// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! Lowering of typed Gleam modules to the native intermediate representation.
//!
//! This is a deliberately small subset of the language so far; anything
//! outside it produces an error naming the unsupported feature rather than
//! generating wrong code. The subset grows with the native backend.

use std::collections::HashMap;

use ecow::EcoString;
use num_bigint::BigInt;
use num_traits::ToPrimitive;

use crate::{
    ast::{
        AssignmentKind, BinOp, Pattern, Statement, TypedClause, TypedExpr, TypedModule,
        TypedStatement,
    },
    error::Error,
    exhaustiveness,
    type_::{ModuleValueConstructor, PRELUDE_MODULE_NAME, Type, ValueConstructorVariant},
};

fn lower_int(value: &BigInt) -> native_ir::Expression {
    match value
        .to_i64()
        .filter(|value| ((i64::MIN >> 1)..=(i64::MAX >> 1)).contains(value))
    {
        Some(value) => native_ir::Expression::Int(value),
        None => native_ir::Expression::BigInt(value.to_signed_bytes_le()),
    }
}

pub fn module(
    module: &TypedModule,
    package_root: &camino::Utf8Path,
    src: &EcoString,
) -> Result<native_ir::Module, Error> {
    // The path `echo` prints, root-relative like the other targets print it.
    let src_path = module
        .type_info
        .src_path
        .strip_prefix(package_root)
        .unwrap_or(&module.type_info.src_path)
        .as_str()
        .replace('\\', "/");
    let mut functions = Vec::new();

    // Imports, type aliases, custom type definitions, and constants
    // generate no code: constructors are lowered to record allocations at
    // their call sites, and constants are inlined where they are used.
    for function in &module.definitions.functions {
        let name = function
            .name
            .as_ref()
            .map(|(_, name)| name.clone())
            .unwrap_or_default();

        if let Some((_module, symbol, _location)) = &function.external_native {
            functions.push(native_ir::Function::External {
                name: name.into(),
                arity: function.arguments.len() as u32,
                symbol: symbol.clone().into(),
            });
            continue;
        }

        let mut parameters = Vec::with_capacity(function.arguments.len());
        for (index, argument) in function.arguments.iter().enumerate() {
            let parameter = match argument.get_variable_name() {
                Some(name) => name.clone().into(),
                None => format!("_discarded${index}"),
            };
            parameters.push(parameter);
        }

        let lowerer = Lowerer {
            module_name: module.name.clone(),
            function_name: name.clone(),
            line_numbers: &module.type_info.line_numbers,
            src: src.clone(),
            src_path: module.type_info.src_path.clone(),
            match_location: std::cell::Cell::new(function.location),
        };
        let body = lowerer.statements(function.body.iter())?;
        functions.push(native_ir::Function::Defined {
            name: name.into(),
            parameters,
            body,
        });
    }

    Ok(native_ir::Module {
        name: module.name.clone().into(),
        src_path,
        functions,
    })
}

struct Lowerer<'a> {
    module_name: EcoString,
    function_name: EcoString,
    line_numbers: &'a src_span::LineNumbers,
    src: EcoString,
    src_path: camino::Utf8PathBuf,
    /// The nearest enclosing pattern match's source location, kept for
    /// errors raised while lowering compiled decision trees, whose
    /// exhaustiveness structures carry no spans of their own. Starts as
    /// the function head's location.
    match_location: std::cell::Cell<src_span::SrcSpan>,
}

impl Lowerer<'_> {
    fn unsupported(&self, feature: &str, location: src_span::SrcSpan) -> Error {
        Error::NativeUnsupportedFeature {
            module: self.module_name.clone(),
            feature: feature.into(),
            path: self.src_path.clone(),
            src: self.src.clone(),
            location,
        }
    }

    /// [`unsupported`](Self::unsupported) located at the nearest enclosing
    /// pattern match, for decision-tree lowering.
    fn unsupported_pattern(&self, feature: &str) -> Error {
        self.unsupported(feature, self.match_location.get())
    }

    fn statements<'a>(
        &self,
        statements: impl Iterator<Item = &'a TypedStatement>,
    ) -> Result<Vec<native_ir::Statement>, Error> {
        statements
            .map(|statement| self.statement(statement))
            .collect()
    }

    fn statement(&self, statement: &TypedStatement) -> Result<native_ir::Statement, Error> {
        let line = self.line_numbers.line_number(statement.location().start);
        match statement {
            Statement::Expression(expression) => Ok(native_ir::Statement::Expression {
                expression: self.expression(expression)?,
                line,
            }),

            Statement::Assignment(assignment) => {
                // A plain variable pattern is irrefutable and binds directly.
                if let Pattern::Variable { name, .. } = &assignment.pattern {
                    return Ok(native_ir::Statement::Let {
                        name: name.clone().into(),
                        value: self.expression(&assignment.value)?,
                        line,
                    });
                }

                let subject_id = assignment
                    .compiled_case
                    .subject_variables
                    .first()
                    .expect("assignment decision tree has a subject")
                    .id as u32;
                let mut prefix_slices = HashMap::new();
                let enclosing = self.match_location.replace(assignment.pattern.location());
                let tree = self.decision(&assignment.compiled_case.tree, None, &mut prefix_slices);
                self.match_location.set(enclosing);
                let tree = tree?;
                let on_failure = match &assignment.kind {
                    AssignmentKind::Let | AssignmentKind::Generated => None,
                    AssignmentKind::Assert {
                        location, message, ..
                    } => Some(native_ir::AssignmentFailure {
                        message: match message {
                            Some(message) => Some(Box::new(self.expression(message)?)),
                            None => None,
                        },
                        function: self.function_name.clone().into(),
                        line: self.line_numbers.line_number(location.start),
                    }),
                };
                Ok(native_ir::Statement::Destructure {
                    subject: self.expression(&assignment.value)?,
                    subject_id,
                    tree,
                    on_failure,
                    line,
                })
            }

            // The type checker has already desugared `use` into a call with
            // a callback function.
            Statement::Use(use_) => Ok(native_ir::Statement::Expression {
                expression: self.expression(&use_.call)?,
                line,
            }),
            // `assert cond` becomes a boolean case: True continues with
            // Nil, False panics with the assert report.
            Statement::Assert(assert) => {
                let message = match &assert.message {
                    Some(message) => Some(Box::new(self.expression(message)?)),
                    None => None,
                };
                let panic = native_ir::Expression::Panic {
                    kind: native_ir::PanicKind::Assert,
                    message,
                    function: self.function_name.clone().into(),
                    line: self.line_numbers.line_number(assert.location.start),
                };
                Ok(native_ir::Statement::Expression {
                    expression: native_ir::Expression::Case {
                        subjects: vec![self.expression(&assert.value)?],
                        subject_ids: vec![0],
                        tree: native_ir::Decision::Switch {
                            var: 0,
                            choices: vec![(
                                native_ir::Check::Bool(true),
                                native_ir::Decision::Run {
                                    bindings: vec![],
                                    body: vec![native_ir::Statement::expression(
                                        native_ir::Expression::Nil,
                                    )],
                                },
                            )],
                            fallback: Box::new(native_ir::Decision::Run {
                                bindings: vec![],
                                body: vec![native_ir::Statement::expression(panic)],
                            }),
                            fallback_fields: vec![],
                        },
                    },
                    line,
                })
            }
        }
    }

    fn expression(&self, expression: &TypedExpr) -> Result<native_ir::Expression, Error> {
        match expression {
            TypedExpr::Int { int_value, .. } => Ok(lower_int(int_value)),

            TypedExpr::Float { float_value, .. } => {
                Ok(native_ir::Expression::Float(float_value.value()))
            }

            TypedExpr::String { value, .. } => Ok(native_ir::Expression::String(
                crate::strings::convert_string_escape_chars(value).into(),
            )),

            TypedExpr::Block { statements, .. } => Ok(native_ir::Expression::Block(
                self.statements(statements.iter())?,
            )),

            TypedExpr::Var {
                constructor, name, ..
            } => match &constructor.variant {
                ValueConstructorVariant::LocalVariable { .. } => {
                    Ok(native_ir::Expression::Variable(name.clone().into()))
                }
                ValueConstructorVariant::Record {
                    name,
                    arity,
                    module,
                    variant_index,
                    ..
                } => Ok(Self::constructor_value(
                    module,
                    name,
                    *arity,
                    *variant_index,
                )),
                ValueConstructorVariant::ModuleFn {
                    module,
                    name,
                    arity,
                    ..
                } => Ok(native_ir::Expression::FunctionReference {
                    module: module.clone().into(),
                    function: name.clone().into(),
                    arity: *arity as u32,
                }),
                ValueConstructorVariant::ModuleConstant { literal, .. } => self.constant(literal),
            },

            // Module-qualified access, `module.name`: the same cases as
            // `Var` above, driven by `ModuleValueConstructor`.
            TypedExpr::ModuleSelect {
                constructor,
                module_name,
                type_,
                ..
            } => match constructor {
                ModuleValueConstructor::Fn { module, name, .. } => {
                    let arity = type_.fn_arity().ok_or_else(|| {
                        self.unsupported("this module access", expression.location())
                    })?;
                    Ok(native_ir::Expression::FunctionReference {
                        module: module.clone().into(),
                        function: name.clone().into(),
                        arity: arity as u32,
                    })
                }
                ModuleValueConstructor::Constant { literal, .. } => self.constant(literal),
                ModuleValueConstructor::Record {
                    name,
                    arity,
                    variant_index,
                    ..
                } => Ok(Self::constructor_value(
                    module_name,
                    name,
                    *arity,
                    *variant_index,
                )),
            },

            TypedExpr::Call { fun, arguments, .. } => {
                let arguments = arguments
                    .iter()
                    .map(|argument| self.expression(&argument.value))
                    .collect::<Result<Vec<_>, _>>()?;
                // A call to a module function or a constructor compiles
                // directly; calling any other expression goes through the
                // closure calling convention.
                if let TypedExpr::Var { constructor, .. } = fun.as_ref() {
                    match &constructor.variant {
                        ValueConstructorVariant::ModuleFn { module, name, .. } => {
                            return Ok(native_ir::Expression::Call {
                                module: module.clone().into(),
                                function: name.clone().into(),
                                arguments,
                            });
                        }
                        ValueConstructorVariant::Record {
                            name,
                            variant_index,
                            ..
                        } => {
                            return Ok(native_ir::Expression::Constructor {
                                tag: *variant_index as u32,
                                display: native_ir::ConstructorDisplay::Record {
                                    name: name.clone().into(),
                                },
                                arguments,
                            });
                        }
                        ValueConstructorVariant::LocalVariable { .. }
                        | ValueConstructorVariant::ModuleConstant { .. } => {}
                    }
                }
                if let TypedExpr::ModuleSelect { constructor, .. } = fun.as_ref() {
                    match constructor {
                        ModuleValueConstructor::Fn { module, name, .. } => {
                            return Ok(native_ir::Expression::Call {
                                module: module.clone().into(),
                                function: name.clone().into(),
                                arguments,
                            });
                        }
                        ModuleValueConstructor::Record {
                            name,
                            variant_index,
                            ..
                        } => {
                            return Ok(native_ir::Expression::Constructor {
                                tag: *variant_index as u32,
                                display: native_ir::ConstructorDisplay::Record {
                                    name: name.clone().into(),
                                },
                                arguments,
                            });
                        }
                        ModuleValueConstructor::Constant { .. } => {}
                    }
                }
                Ok(native_ir::Expression::CallValue {
                    callee: Box::new(self.expression(fun)?),
                    arguments,
                })
            }

            TypedExpr::Fn {
                arguments, body, ..
            } => {
                let mut parameters = Vec::with_capacity(arguments.len());
                for (index, argument) in arguments.iter().enumerate() {
                    let parameter = match argument.get_variable_name() {
                        Some(name) => name.clone().into(),
                        None => format!("_discarded${index}"),
                    };
                    parameters.push(parameter);
                }
                Ok(native_ir::Expression::Lambda {
                    parameters,
                    body: self.statements(body.iter())?,
                })
            }

            // A pipeline is a series of named steps followed by a final
            // expression. An `echo` step prints the latest step's value
            // without binding a new one.
            TypedExpr::Pipeline {
                first_value,
                assignments,
                finally,
                ..
            } => {
                let mut statements = Vec::with_capacity(assignments.len() + 2);
                let mut latest: Option<EcoString> = None;
                let all_assignments =
                    std::iter::once(first_value).chain(assignments.iter().map(|(a, _)| a));
                for assignment in all_assignments {
                    if let TypedExpr::Echo {
                        expression: None,
                        message,
                        location,
                        ..
                    } = assignment.value.as_ref()
                    {
                        let name = latest
                            .clone()
                            .expect("echo with no previous step in a pipe");
                        statements.push(native_ir::Statement::expression(self.echo(
                            native_ir::Expression::Variable(name.into()),
                            message.as_deref(),
                            location,
                        )?));
                    } else {
                        statements.push(native_ir::Statement::Let {
                            name: assignment.name.clone().into(),
                            value: self.expression(&assignment.value)?,
                            line: self.line_numbers.line_number(assignment.location.start),
                        });
                        latest = Some(assignment.name.clone());
                    }
                }
                let finally = if let TypedExpr::Echo {
                    expression: None,
                    message,
                    location,
                    ..
                } = finally.as_ref()
                {
                    let name = latest.expect("echo with no previous step in a pipe");
                    self.echo(
                        native_ir::Expression::Variable(name.into()),
                        message.as_deref(),
                        location,
                    )?
                } else {
                    self.expression(finally)?
                };
                statements.push(native_ir::Statement::expression(finally));
                Ok(native_ir::Expression::Block(statements))
            }

            TypedExpr::Tuple { elements, .. } => {
                // Tuples are records with tag 0.
                let arguments = elements
                    .iter()
                    .map(|element| self.expression(element))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(native_ir::Expression::Constructor {
                    tag: 0,
                    display: native_ir::ConstructorDisplay::Tuple,
                    arguments,
                })
            }

            TypedExpr::TupleIndex { tuple, index, .. } => Ok(native_ir::Expression::FieldAccess {
                record: Box::new(self.expression(tuple)?),
                index: *index as u32,
            }),

            TypedExpr::List { elements, tail, .. } => {
                // Cons cells are two-field records with tag 1, built from
                // the tail outwards.
                let mut list = match tail {
                    Some(tail) => self.expression(tail)?,
                    None => native_ir::Expression::EmptyList,
                };
                for element in elements.iter().rev() {
                    list = native_ir::Expression::Constructor {
                        tag: 1,
                        display: native_ir::ConstructorDisplay::List,
                        arguments: vec![self.expression(element)?, list],
                    };
                }
                Ok(list)
            }

            TypedExpr::PositionalAccess { record, index, .. } => {
                Ok(native_ir::Expression::FieldAccess {
                    record: Box::new(self.expression(record)?),
                    index: *index as u32,
                })
            }

            TypedExpr::RecordUpdate {
                updated_record,
                updated_record_assigned_name,
                constructor,
                arguments,
                ..
            } => {
                // The type checker has already desugared the update into a
                // full constructor argument list where unchanged fields read
                // from the spread record, referenced by name.
                let constructor = constructor.as_ref();
                let (tag, name) = if let TypedExpr::Var { constructor, .. } = constructor {
                    match &constructor.variant {
                        ValueConstructorVariant::Record {
                            name,
                            variant_index,
                            ..
                        } => (*variant_index as u32, name.clone()),
                        ValueConstructorVariant::LocalVariable { .. }
                        | ValueConstructorVariant::ModuleFn { .. }
                        | ValueConstructorVariant::ModuleConstant { .. } => {
                            return Err(
                                self.unsupported("this record update", expression.location())
                            );
                        }
                    }
                } else if let TypedExpr::ModuleSelect {
                    constructor:
                        ModuleValueConstructor::Record {
                            name,
                            variant_index,
                            ..
                        },
                    ..
                } = constructor
                {
                    (*variant_index as u32, name.clone())
                } else {
                    return Err(self.unsupported("this record update", expression.location()));
                };
                let arguments = arguments
                    .iter()
                    .map(|argument| self.expression(&argument.value))
                    .collect::<Result<Vec<_>, _>>()?;
                let construct = native_ir::Expression::Constructor {
                    tag,
                    display: native_ir::ConstructorDisplay::Record { name: name.into() },
                    arguments,
                };
                match updated_record_assigned_name {
                    // The spread expression is not a plain variable: bind it
                    // to the compiler-chosen name the arguments refer to.
                    Some(name) => Ok(native_ir::Expression::Block(vec![
                        native_ir::Statement::Let {
                            name: name.clone().into(),
                            value: self.expression(updated_record)?,
                            line: 0,
                        },
                        native_ir::Statement::expression(construct),
                    ])),
                    None => Ok(construct),
                }
            }

            TypedExpr::BitArray { segments, .. } => {
                let mut lowered = Vec::with_capacity(segments.len());
                for segment in segments {
                    let kind = self.bit_segment_kind(segment)?;
                    lowered.push(native_ir::BitSegment {
                        value: Box::new(self.expression(&segment.value)?),
                        kind,
                    });
                }
                Ok(native_ir::Expression::BitArray(lowered))
            }

            TypedExpr::RecordAccess { record, index, .. } => {
                Ok(native_ir::Expression::FieldAccess {
                    record: Box::new(self.expression(record)?),
                    index: *index as u32,
                })
            }

            TypedExpr::BinOp {
                operator,
                left,
                right,
                ..
            } => {
                let left_type = left.type_();
                let left = self.expression(left)?;
                let right = self.expression(right)?;
                self.binary_operator(*operator, &left_type, left, right)
            }

            TypedExpr::Case {
                subjects,
                clauses,
                compiled_case,
                ..
            } => {
                let subject_ids = compiled_case
                    .subject_variables
                    .iter()
                    .map(|variable| variable.id as u32)
                    .collect();
                let mut prefix_slices = HashMap::new();
                let enclosing = self.match_location.replace(expression.location());
                let tree = self.decision(&compiled_case.tree, Some(clauses), &mut prefix_slices);
                self.match_location.set(enclosing);
                let tree = tree?;
                let subjects = subjects
                    .iter()
                    .map(|subject| self.expression(subject))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(native_ir::Expression::Case {
                    subjects,
                    subject_ids,
                    tree,
                })
            }

            TypedExpr::Echo {
                expression: echo_expression,
                message,
                location,
                ..
            } => {
                // A bare `echo` only occurs in pipelines, handled there.
                let value = echo_expression
                    .as_ref()
                    .expect("bare echo outside a pipeline");
                let value = self.expression(value)?;
                self.echo(value, message.as_deref(), location)
            }

            TypedExpr::Panic {
                location, message, ..
            } => self.panic_expression(native_ir::PanicKind::Panic, message, location),
            TypedExpr::Todo {
                location, message, ..
            } => self.panic_expression(native_ir::PanicKind::Todo, message, location),

            TypedExpr::NegateBool { value, .. } => Ok(native_ir::Expression::BoolNot(Box::new(
                self.expression(value)?,
            ))),

            // Negation shares subtraction's overflow path: negating the most
            // negative small integer promotes to a big integer.
            TypedExpr::NegateInt { value, .. } => Ok(native_ir::Expression::IntBinary {
                operator: native_ir::IntOperator::Subtract,
                left: Box::new(native_ir::Expression::Int(0)),
                right: Box::new(self.expression(value)?),
            }),

            // Only present when analysis already reported a type error, so
            // code generation never runs on it.
            TypedExpr::Invalid { location, .. } => {
                Err(self.unsupported("this kind of expression", *location))
            }
        }
    }

    /// Lowers a binary operator applied to already-lowered operands. Shared
    /// between ordinary expressions and clause guards.
    fn binary_operator(
        &self,
        operator: BinOp,
        left_type: &Type,
        left: native_ir::Expression,
        right: native_ir::Expression,
    ) -> Result<native_ir::Expression, Error> {
        use native_ir::{CompareOperator, FloatOperator, IntOperator};

        fn int_binary(
            operator: IntOperator,
            left: Box<native_ir::Expression>,
            right: Box<native_ir::Expression>,
        ) -> native_ir::Expression {
            native_ir::Expression::IntBinary {
                operator,
                left,
                right,
            }
        }
        fn int_compare(
            operator: CompareOperator,
            left: Box<native_ir::Expression>,
            right: Box<native_ir::Expression>,
        ) -> native_ir::Expression {
            native_ir::Expression::IntCompare {
                operator,
                left,
                right,
            }
        }
        fn float_binary(
            operator: FloatOperator,
            left: Box<native_ir::Expression>,
            right: Box<native_ir::Expression>,
        ) -> native_ir::Expression {
            native_ir::Expression::FloatBinary {
                operator,
                left,
                right,
            }
        }
        fn float_compare(
            operator: CompareOperator,
            left: Box<native_ir::Expression>,
            right: Box<native_ir::Expression>,
        ) -> native_ir::Expression {
            native_ir::Expression::FloatCompare {
                operator,
                left,
                right,
            }
        }

        let left = Box::new(left);
        let right = Box::new(right);
        match operator {
            BinOp::AddInt => Ok(int_binary(IntOperator::Add, left, right)),
            BinOp::SubInt => Ok(int_binary(IntOperator::Subtract, left, right)),
            BinOp::MultInt => Ok(int_binary(IntOperator::Multiply, left, right)),
            BinOp::DivInt => Ok(int_binary(IntOperator::Divide, left, right)),
            BinOp::RemainderInt => Ok(int_binary(IntOperator::Remainder, left, right)),
            BinOp::LtInt => Ok(int_compare(CompareOperator::LessThan, left, right)),
            BinOp::LtEqInt => Ok(int_compare(CompareOperator::LessThanOrEqual, left, right)),
            BinOp::GtInt => Ok(int_compare(CompareOperator::GreaterThan, left, right)),
            BinOp::GtEqInt => Ok(int_compare(
                CompareOperator::GreaterThanOrEqual,
                left,
                right,
            )),
            BinOp::AddFloat => Ok(float_binary(FloatOperator::Add, left, right)),
            BinOp::SubFloat => Ok(float_binary(FloatOperator::Subtract, left, right)),
            BinOp::MultFloat => Ok(float_binary(FloatOperator::Multiply, left, right)),
            BinOp::DivFloat => Ok(float_binary(FloatOperator::Divide, left, right)),
            BinOp::LtFloat => Ok(float_compare(CompareOperator::LessThan, left, right)),
            BinOp::LtEqFloat => Ok(float_compare(CompareOperator::LessThanOrEqual, left, right)),
            BinOp::GtFloat => Ok(float_compare(CompareOperator::GreaterThan, left, right)),
            BinOp::GtEqFloat => Ok(float_compare(
                CompareOperator::GreaterThanOrEqual,
                left,
                right,
            )),
            BinOp::Eq | BinOp::NotEq => {
                let kind = if left_type.is_int() {
                    native_ir::EqualityKind::Int
                } else if left_type.is_float() {
                    native_ir::EqualityKind::Float
                } else if left_type.is_string() {
                    native_ir::EqualityKind::String
                } else if left_type.is_bool() || left_type.is_nil() {
                    native_ir::EqualityKind::Immediate
                } else {
                    // Custom types, lists, tuples, and generics: structural
                    // deep equality in the runtime.
                    native_ir::EqualityKind::Deep
                };
                Ok(native_ir::Expression::Equality {
                    kind,
                    negated: operator == BinOp::NotEq,
                    left,
                    right,
                })
            }
            BinOp::And => Ok(native_ir::Expression::BoolBinary {
                operator: native_ir::BoolOperator::And,
                left,
                right,
            }),
            BinOp::Or => Ok(native_ir::Expression::BoolBinary {
                operator: native_ir::BoolOperator::Or,
                left,
                right,
            }),
            BinOp::Concatenate => Ok(native_ir::Expression::StringConcat(left, right)),
        }
    }

    fn guard(&self, guard: &crate::ast::TypedClauseGuard) -> Result<native_ir::Expression, Error> {
        use crate::ast::ClauseGuard;
        match guard {
            ClauseGuard::Block { value, .. } => self.guard(value),

            ClauseGuard::BinaryOperator {
                operator,
                left,
                right,
                ..
            } => {
                let left_type = left.type_();
                let left = self.guard(left)?;
                let right = self.guard(right)?;
                self.binary_operator(*operator, &left_type, left, right)
            }

            ClauseGuard::Not { expression, .. } => Ok(native_ir::Expression::BoolNot(Box::new(
                self.guard(expression)?,
            ))),

            ClauseGuard::Var { name, .. } => {
                Ok(native_ir::Expression::Variable(name.clone().into()))
            }

            ClauseGuard::Constant(constant) => self.constant(constant),

            ClauseGuard::TupleIndex { tuple, index, .. } => {
                Ok(native_ir::Expression::FieldAccess {
                    record: Box::new(self.guard(tuple)?),
                    index: *index as u32,
                })
            }
            ClauseGuard::FieldAccess {
                container,
                index: Some(index),
                ..
            } => Ok(native_ir::Expression::FieldAccess {
                record: Box::new(self.guard(container)?),
                index: *index as u32,
            }),
            ClauseGuard::FieldAccess { index: None, .. } => {
                Err(self.unsupported("this guard expression", guard.location()))
            }
            ClauseGuard::ModuleSelect { literal, .. } => self.constant(literal),
            ClauseGuard::Invalid { .. } => {
                Err(self.unsupported("this guard expression", guard.location()))
            }
        }
    }

    /// A constructor referenced as a value rather than called: `Nil`,
    /// `True`, and `False` are immediates, other zero-arity constructors
    /// allocate their record directly, and constructors with fields become
    /// a lambda that allocates one.
    fn constructor_value(
        module: &str,
        name: &EcoString,
        arity: u16,
        variant_index: u16,
    ) -> native_ir::Expression {
        if module == PRELUDE_MODULE_NAME && name == "Nil" {
            return native_ir::Expression::Nil;
        }
        if module == PRELUDE_MODULE_NAME && (name == "True" || name == "False") {
            return native_ir::Expression::Bool(name == "True");
        }
        if arity == 0 {
            return native_ir::Expression::Constructor {
                tag: variant_index as u32,
                display: native_ir::ConstructorDisplay::Record {
                    name: name.clone().into(),
                },
                arguments: vec![],
            };
        }
        let parameters: Vec<String> = (0..arity).map(|index| format!("$field{index}")).collect();
        let arguments = parameters
            .iter()
            .map(|name| native_ir::Expression::Variable(name.clone()))
            .collect();
        native_ir::Expression::Lambda {
            parameters,
            body: vec![native_ir::Statement::expression(
                native_ir::Expression::Constructor {
                    tag: variant_index as u32,
                    display: native_ir::ConstructorDisplay::Record {
                        name: name.clone().into(),
                    },
                    arguments,
                },
            )],
        }
    }

    fn constant(
        &self,
        constant: &crate::ast::TypedConstant,
    ) -> Result<native_ir::Expression, Error> {
        use crate::ast::Constant;
        match constant {
            Constant::Int { int_value, .. } => Ok(lower_int(int_value)),
            Constant::Float { float_value, .. } => {
                Ok(native_ir::Expression::Float(float_value.value()))
            }
            Constant::String { value, .. } => Ok(native_ir::Expression::String(
                crate::strings::convert_string_escape_chars(value).into(),
            )),
            Constant::Record { name, type_, .. } if type_.is_bool() => {
                Ok(native_ir::Expression::Bool(name == "True"))
            }
            Constant::Record { type_, .. } if type_.is_nil() => Ok(native_ir::Expression::Nil),
            Constant::Record {
                name,
                arguments,
                record_constructor,
                ..
            } => {
                let (tag, arity, module) = record_constructor
                    .as_deref()
                    .and_then(|constructor| match &constructor.variant {
                        ValueConstructorVariant::Record {
                            variant_index,
                            arity,
                            module,
                            ..
                        } => Some((*variant_index as u32, *arity, module)),
                        ValueConstructorVariant::LocalVariable { .. }
                        | ValueConstructorVariant::ModuleFn { .. }
                        | ValueConstructorVariant::ModuleConstant { .. } => None,
                    })
                    .ok_or_else(|| {
                        self.unsupported("this kind of constant", constant.location())
                    })?;
                // A constructor with fields referenced without arguments is
                // the constructor as a function value, not a record.
                let Some(arguments) = arguments.as_deref() else {
                    return Ok(Self::constructor_value(module, name, arity, tag as u16));
                };
                let arguments = arguments
                    .iter()
                    .map(|argument| self.constant(&argument.value))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(native_ir::Expression::Constructor {
                    tag,
                    display: native_ir::ConstructorDisplay::Record {
                        name: name.clone().into(),
                    },
                    arguments,
                })
            }
            Constant::Tuple { elements, .. } => {
                let arguments = elements
                    .iter()
                    .map(|element| self.constant(element))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(native_ir::Expression::Constructor {
                    tag: 0,
                    display: native_ir::ConstructorDisplay::Tuple,
                    arguments,
                })
            }
            Constant::List { elements, .. } => {
                let mut list = native_ir::Expression::EmptyList;
                for element in elements.iter().rev() {
                    list = native_ir::Expression::Constructor {
                        tag: 1,
                        display: native_ir::ConstructorDisplay::List,
                        arguments: vec![self.constant(element)?, list],
                    };
                }
                Ok(list)
            }
            // A constant referring to another constant or to a function.
            Constant::Var { constructor, .. } => {
                let constructor = constructor.as_deref().ok_or_else(|| {
                    self.unsupported("this kind of constant", constant.location())
                })?;
                match &constructor.variant {
                    ValueConstructorVariant::ModuleConstant { literal, .. } => {
                        self.constant(literal)
                    }
                    ValueConstructorVariant::ModuleFn {
                        module,
                        name,
                        arity,
                        ..
                    } => Ok(native_ir::Expression::FunctionReference {
                        module: module.clone().into(),
                        function: name.clone().into(),
                        arity: *arity as u32,
                    }),
                    ValueConstructorVariant::Record {
                        name,
                        arity,
                        module,
                        variant_index,
                        ..
                    } => Ok(Self::constructor_value(
                        module,
                        name,
                        *arity,
                        *variant_index,
                    )),
                    ValueConstructorVariant::LocalVariable { .. } => {
                        Err(self.unsupported("this kind of constant", constant.location()))
                    }
                }
            }
            Constant::BinaryOperator {
                operator,
                left,
                right,
                ..
            } => {
                let left_type = left.type_();
                let left = self.constant(left)?;
                let right = self.constant(right)?;
                self.binary_operator(*operator, &left_type, left, right)
            }
            Constant::BitArray { segments, .. } => {
                let mut lowered = Vec::with_capacity(segments.len());
                for segment in segments {
                    let kind = self.bit_segment_kind_of(
                        &segment.options,
                        &segment.type_,
                        segment.location,
                        &mut |value| self.constant(value),
                    )?;
                    lowered.push(native_ir::BitSegment {
                        value: Box::new(self.constant(&segment.value)?),
                        kind,
                    });
                }
                Ok(native_ir::Expression::BitArray(lowered))
            }
            // Analysis desugars record updates in constants and rejects
            // `todo` and invalid constants before code generation runs.
            Constant::RecordUpdate { .. } | Constant::Todo { .. } | Constant::Invalid { .. } => {
                Err(self.unsupported("this kind of constant", constant.location()))
            }
        }
    }

    /// Works out how a bit array expression segment is written: its
    /// compile-time size, type, and endianness.
    fn bit_segment_kind(
        &self,
        segment: &crate::ast::TypedExprBitArraySegment,
    ) -> Result<native_ir::BitSegmentKind, Error> {
        self.bit_segment_kind_of(
            &segment.options,
            &segment.type_,
            segment.location,
            &mut |value| self.expression(value),
        )
    }

    /// [`bit_segment_kind`](Self::bit_segment_kind) generalized over the
    /// segment's value representation, so expression and constant bit
    /// arrays share the option handling; `lower` lowers a size option's
    /// value.
    fn bit_segment_kind_of<Value>(
        &self,
        options: &[crate::ast::BitArrayOption<Value>],
        type_: &Type,
        location: src_span::SrcSpan,
        lower: &mut dyn FnMut(&Value) -> Result<native_ir::Expression, Error>,
    ) -> Result<native_ir::BitSegmentKind, Error> {
        use crate::ast::BitArrayOption;
        let mut size: Option<native_ir::Expression> = None;
        let mut unit: u32 = 1;
        let mut endian = native_ir::Endian::Big;
        let mut encoding = None;
        let mut is_codepoint = false;
        let mut is_float = false;
        let mut is_splice = false;
        for option in options {
            match option {
                BitArrayOption::Int { .. } | BitArrayOption::Big { .. } => {}
                BitArrayOption::Signed { .. } | BitArrayOption::Unsigned { .. } => {}
                BitArrayOption::Little { .. } => endian = native_ir::Endian::Little,
                BitArrayOption::Native { .. } => endian = native_ir::Endian::Native,
                BitArrayOption::Float { .. } => is_float = true,
                BitArrayOption::Utf8 { .. } => encoding = Some(native_ir::StringEncoding::Utf8),
                BitArrayOption::Utf16 { .. } => encoding = Some(native_ir::StringEncoding::Utf16),
                BitArrayOption::Utf32 { .. } => encoding = Some(native_ir::StringEncoding::Utf32),
                BitArrayOption::Utf8Codepoint { .. } => {
                    encoding = Some(native_ir::StringEncoding::Utf8);
                    is_codepoint = true;
                }
                BitArrayOption::Utf16Codepoint { .. } => {
                    encoding = Some(native_ir::StringEncoding::Utf16);
                    is_codepoint = true;
                }
                BitArrayOption::Utf32Codepoint { .. } => {
                    encoding = Some(native_ir::StringEncoding::Utf32);
                    is_codepoint = true;
                }
                BitArrayOption::Bytes { .. } | BitArrayOption::Bits { .. } => is_splice = true,
                BitArrayOption::Size { value, .. } => size = Some(lower(value)?),
                BitArrayOption::Unit { value, .. } => unit = *value as u32,
            }
        }
        if is_codepoint {
            return Ok(native_ir::BitSegmentKind::Codepoint {
                encoding: encoding.unwrap_or(native_ir::StringEncoding::Utf8),
                endian,
            });
        }
        if let Some(encoding) = encoding {
            return Ok(native_ir::BitSegmentKind::String { encoding, endian });
        }
        if type_.is_string() {
            return Ok(native_ir::BitSegmentKind::String {
                encoding: native_ir::StringEncoding::Utf8,
                endian,
            });
        }
        if is_splice || type_.is_bit_array() {
            return Ok(native_ir::BitSegmentKind::BitArraySplice {
                bits: size.map(|size| Box::new(Self::multiply(size, unit as u64))),
            });
        }
        if is_float || type_.is_float() {
            let bits = Self::multiply(size.unwrap_or(native_ir::Expression::Int(64)), unit as u64);
            return Ok(native_ir::BitSegmentKind::Float {
                bits: Box::new(bits),
                endian,
            });
        }
        if !type_.is_int() {
            return Err(self.unsupported("this bit array segment", location));
        }
        let bits = Self::multiply(size.unwrap_or(native_ir::Expression::Int(8)), unit as u64);
        Ok(native_ir::BitSegmentKind::Int {
            bits: Box::new(bits),
            endian,
        })
    }

    /// The name of the materialized value of a named bit array segment. The
    /// `bit$` prefix cannot collide with Gleam identifiers.
    fn segment_variable_name(name: &EcoString) -> String {
        format!("bit${name}")
    }

    fn variable_usage_expression(
        variable: &exhaustiveness::VariableUsage,
    ) -> native_ir::Expression {
        match variable {
            exhaustiveness::VariableUsage::PatternSegment(name, _) => {
                native_ir::Expression::Variable(Self::segment_variable_name(name))
            }
            exhaustiveness::VariableUsage::OutsideVariable(name) => {
                native_ir::Expression::Variable(name.clone().into())
            }
        }
    }

    fn multiply(expression: native_ir::Expression, by: u64) -> native_ir::Expression {
        if by == 1 {
            expression
        } else {
            native_ir::Expression::IntBinary {
                operator: native_ir::IntOperator::Multiply,
                left: Box::new(expression),
                right: Box::new(lower_int(&BigInt::from(by))),
            }
        }
    }

    fn add(
        sum: Option<native_ir::Expression>,
        term: native_ir::Expression,
    ) -> Option<native_ir::Expression> {
        Some(match sum {
            None => term,
            Some(sum) => native_ir::Expression::IntBinary {
                operator: native_ir::IntOperator::Add,
                left: Box::new(sum),
                right: Box::new(term),
            },
        })
    }

    /// A bit offset as a tagged integer expression: a constant plus scaled
    /// variables plus nested calculations.
    fn offset_expression(
        &self,
        offset: &exhaustiveness::Offset,
    ) -> Result<native_ir::Expression, Error> {
        let mut sum = None;
        if offset.constant != BigInt::ZERO || offset.is_zero() {
            sum = Self::add(sum, lower_int(&offset.constant));
        }
        let mut variables: Vec<_> = offset.variables.iter().collect();
        variables.sort_by(|(one, _), (other, _)| one.name().cmp(other.name()));
        for (variable, times) in variables {
            let term = Self::multiply(Self::variable_usage_expression(variable), *times as u64);
            sum = Self::add(sum, term);
        }
        for calculation in offset.calculations.iter() {
            let term = self.binary_operator_for_bits(
                calculation.operator,
                self.offset_expression(&calculation.left)?,
                self.offset_expression(&calculation.right)?,
            );
            sum = Self::add(sum, term);
        }
        Ok(sum.unwrap_or(native_ir::Expression::Int(0)))
    }

    fn binary_operator_for_bits(
        &self,
        operator: crate::ast::IntOperator,
        left: native_ir::Expression,
        right: native_ir::Expression,
    ) -> native_ir::Expression {
        use crate::ast::IntOperator;
        native_ir::Expression::IntBinary {
            operator: match operator {
                IntOperator::Add => native_ir::IntOperator::Add,
                IntOperator::Subtract => native_ir::IntOperator::Subtract,
                IntOperator::Multiply => native_ir::IntOperator::Multiply,
                IntOperator::Divide => native_ir::IntOperator::Divide,
                IntOperator::Remainder => native_ir::IntOperator::Remainder,
            },
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// A read size in bits as a tagged integer expression, or `None` for
    /// "all the remaining bits".
    fn read_size_expression(
        &self,
        size: &exhaustiveness::ReadSize,
    ) -> Result<Option<native_ir::Expression>, Error> {
        use exhaustiveness::ReadSize;
        Ok(match size {
            ReadSize::ConstantBits(bits) => Some(lower_int(bits)),
            ReadSize::VariableBits { variable, unit } => Some(Self::multiply(
                Self::variable_usage_expression(variable),
                *unit as u64,
            )),
            ReadSize::BinaryOperator {
                left,
                right,
                operator,
            } => {
                let left = self
                    .read_size_expression(left)?
                    .ok_or_else(|| self.unsupported_pattern("this bit array pattern"))?;
                let right = self
                    .read_size_expression(right)?
                    .ok_or_else(|| self.unsupported_pattern("this bit array pattern"))?;
                Some(self.binary_operator_for_bits(*operator, left, right))
            }
            ReadSize::RemainingBits | ReadSize::RemainingBytes => None,
        })
    }

    fn endian(endianness: crate::ast::Endianness) -> native_ir::Endian {
        match endianness {
            crate::ast::Endianness::Big => native_ir::Endian::Big,
            crate::ast::Endianness::Little => native_ir::Endian::Little,
        }
    }

    /// The named segment reads a test refers to, materialized before it runs.
    fn segment_reads(
        &self,
        references: Vec<(&EcoString, &exhaustiveness::ReadAction)>,
    ) -> Result<Vec<native_ir::SegmentRead>, Error> {
        let mut reads = Vec::with_capacity(references.len());
        for (name, action) in references {
            if action.type_ != exhaustiveness::ReadType::Int {
                return Err(self.unsupported_pattern("this bit array pattern"));
            }
            let bits = self
                .read_size_expression(&action.size)?
                .ok_or_else(|| self.unsupported_pattern("this bit array pattern"))?;
            reads.push(native_ir::SegmentRead {
                name: Self::segment_variable_name(name),
                offset: Box::new(self.offset_expression(&action.from)?),
                bits: Box::new(bits),
                endian: Self::endian(action.endianness),
                signed: action.signed,
            });
        }
        Ok(reads)
    }

    fn bit_array_test(
        &self,
        test: &exhaustiveness::BitArrayTest,
    ) -> Result<native_ir::Check, Error> {
        use exhaustiveness::{BitArrayMatchedValue, BitArrayTest, SizeOperator};
        let reads = self.segment_reads(test.referenced_segment_patterns())?;
        let test = match test {
            BitArrayTest::Size(size_test) => native_ir::BitsTest::Size {
                bits: Box::new(self.offset_expression(&size_test.size)?),
                exact: size_test.operator == SizeOperator::Equal,
            },
            BitArrayTest::CatchAllIsBytes { size_so_far } => native_ir::BitsTest::RestIsBytes {
                offset: Box::new(self.offset_expression(size_so_far)?),
            },
            BitArrayTest::ReadSizeIsNotNegative { size } => {
                let value = self
                    .read_size_expression(size)?
                    .ok_or_else(|| self.unsupported_pattern("this bit array pattern"))?;
                native_ir::BitsTest::NonNegative {
                    value: Box::new(value),
                }
            }
            BitArrayTest::Match(match_test) => {
                let offset = Box::new(self.offset_expression(&match_test.read_action.from)?);
                let mut value = &match_test.value;
                while let BitArrayMatchedValue::Assign { value: inner, .. } = value {
                    value = inner;
                }
                match value {
                    BitArrayMatchedValue::LiteralInt { bits: Ok(bits), .. } => {
                        // Pack MSB-first, zero-padding the last byte.
                        let mut bytes = vec![0u8; bits.len().div_ceil(8)];
                        for (index, bit) in bits.iter().enumerate() {
                            if *bit && let Some(byte) = bytes.get_mut(index / 8) {
                                *byte |= 1 << (7 - index % 8);
                            }
                        }
                        native_ir::BitsTest::Bytes {
                            offset,
                            bytes,
                            bit_length: bits.len() as u64,
                        }
                    }
                    // The compiler pre-encodes literal strings with their
                    // encoding and endianness.
                    BitArrayMatchedValue::LiteralString { bytes, .. } => {
                        native_ir::BitsTest::Bytes {
                            offset,
                            bytes: bytes.clone(),
                            bit_length: bytes.len() as u64 * 8,
                        }
                    }
                    // Variables and discards always match; the check exists
                    // to materialize its reads.
                    BitArrayMatchedValue::Variable(_) | BitArrayMatchedValue::Discard(_) => {
                        native_ir::BitsTest::AlwaysTrue
                    }
                    // A literal float matches by reading the float and
                    // comparing numerically, like the other targets.
                    BitArrayMatchedValue::LiteralFloat(value) => {
                        let value = crate::parse::LiteralFloatValue::parse(value)
                            .ok_or_else(|| self.unsupported_pattern("this float literal"))?;
                        let bits = self
                            .read_size_expression(&match_test.read_action.size)?
                            .ok_or_else(|| self.unsupported_pattern("this bit array pattern"))?;
                        native_ir::BitsTest::FloatEquals {
                            offset,
                            bits: Box::new(bits),
                            endian: Self::endian(match_test.read_action.endianness),
                            value: value.value(),
                        }
                    }
                    // A literal int whose bit encoding could not be
                    // precomputed (its size is only known at run time)
                    // matches by reading the integer and comparing
                    // numerically, like the other targets.
                    BitArrayMatchedValue::LiteralInt { value, .. } => {
                        let bits = self
                            .read_size_expression(&match_test.read_action.size)?
                            .ok_or_else(|| self.unsupported_pattern("this bit array pattern"))?;
                        native_ir::BitsTest::IntEquals {
                            offset,
                            bits: Box::new(bits),
                            endian: Self::endian(match_test.read_action.endianness),
                            signed: match_test.read_action.signed,
                            value: Box::new(lower_int(value)),
                        }
                    }
                    // Unreachable: assignments were unwrapped above.
                    BitArrayMatchedValue::Assign { .. } => {
                        return Err(self.unsupported_pattern("this bit array pattern"));
                    }
                }
            }
            BitArrayTest::SegmentIsFiniteFloat { read_action } => {
                let bits = self
                    .read_size_expression(&read_action.size)?
                    .ok_or_else(|| self.unsupported_pattern("this bit array pattern"))?;
                native_ir::BitsTest::IsFiniteFloat {
                    offset: Box::new(self.offset_expression(&read_action.from)?),
                    bits: Box::new(bits),
                    endian: Self::endian(read_action.endianness),
                }
            }
        };
        Ok(native_ir::Check::BitArray { reads, test })
    }

    fn decision(
        &self,
        decision: &exhaustiveness::Decision,
        clauses: Option<&[TypedClause]>,
        prefix_slices: &mut HashMap<usize, (u32, u32)>,
    ) -> Result<native_ir::Decision, Error> {
        match decision {
            exhaustiveness::Decision::Run { body } => {
                self.decision_body(body, clauses, prefix_slices)
            }

            exhaustiveness::Decision::Guard {
                guard,
                if_true,
                if_false,
            } => {
                let clauses = clauses
                    .ok_or_else(|| self.unsupported_pattern("guards outside case expressions"))?;
                let guard_expression = clauses
                    .get(*guard)
                    .expect("guard clause index in range")
                    .guard
                    .as_ref()
                    .expect("guard decision on clause with a guard");
                let bindings = self.bound_values(&if_true.bindings, prefix_slices)?;
                let clause = clauses
                    .get(if_true.clause_index)
                    .expect("decision tree clause index in range");
                let body = vec![native_ir::Statement::expression(
                    self.expression(&clause.then)?,
                )];
                let if_false = self.decision(if_false, Some(clauses), prefix_slices)?;
                Ok(native_ir::Decision::Guard {
                    bindings,
                    guard: Box::new(self.guard(guard_expression)?),
                    if_true: body,
                    if_false: Box::new(if_false),
                })
            }

            exhaustiveness::Decision::Fail => Ok(native_ir::Decision::Fail),

            exhaustiveness::Decision::Switch {
                var,
                choices,
                fallback,
                fallback_check,
            } => {
                let subject = var.id as u32;
                let choices = choices
                    .iter()
                    .map(|(check, decision)| {
                        let check =
                            self.runtime_check(check, &var.type_, subject, prefix_slices)?;
                        let decision = self.decision(decision, clauses, prefix_slices)?;
                        Ok((check, decision))
                    })
                    .collect::<Result<Vec<_>, Error>>()?;
                // When the fallback is the final variant of an exhaustive
                // match its check is not performed, but the fields it
                // extracts must still be made available.
                let fallback_fields = match fallback_check.as_ref() {
                    exhaustiveness::FallbackCheck::RuntimeCheck { check } => {
                        match self.runtime_check(check, &var.type_, subject, prefix_slices)? {
                            native_ir::Check::Variant { fields, .. }
                            | native_ir::Check::Always { fields } => fields,
                            native_ir::Check::NonEmptyList { first, rest } => vec![first, rest],
                            native_ir::Check::Int(_)
                            | native_ir::Check::BigInt(_)
                            | native_ir::Check::Float(_)
                            | native_ir::Check::String(_)
                            | native_ir::Check::Bool(_)
                            | native_ir::Check::Nil
                            | native_ir::Check::EmptyList
                            | native_ir::Check::StringPrefix { .. }
                            | native_ir::Check::BitArray { .. } => vec![],
                        }
                    }
                    exhaustiveness::FallbackCheck::InfiniteCatchAll
                    | exhaustiveness::FallbackCheck::CatchAll { .. } => vec![],
                };
                let fallback = self.decision(fallback, clauses, prefix_slices)?;
                Ok(native_ir::Decision::Switch {
                    var: subject,
                    choices,
                    fallback: Box::new(fallback),
                    fallback_fields,
                })
            }
        }
    }

    fn runtime_check(
        &self,
        check: &exhaustiveness::RuntimeCheck,
        subject_type: &Type,
        subject: u32,
        prefix_slices: &mut HashMap<usize, (u32, u32)>,
    ) -> Result<native_ir::Check, Error> {
        match check {
            exhaustiveness::RuntimeCheck::Int { int_value } => Ok(
                if let native_ir::Expression::Int(value) = lower_int(int_value) {
                    native_ir::Check::Int(value)
                } else {
                    native_ir::Check::BigInt(int_value.to_signed_bytes_le())
                },
            ),
            exhaustiveness::RuntimeCheck::Float { float_value } => {
                Ok(native_ir::Check::Float(float_value.value()))
            }
            exhaustiveness::RuntimeCheck::String { value } => Ok(native_ir::Check::String(
                crate::strings::convert_string_escape_chars(value).into(),
            )),
            exhaustiveness::RuntimeCheck::Variant { index, fields, .. } => {
                // Bool and Nil are immediate words (`True` is variant 0);
                // every other custom type is a heap record with a variant
                // tag word.
                if subject_type.is_bool() {
                    Ok(native_ir::Check::Bool(*index == 0))
                } else if subject_type.is_nil() {
                    Ok(native_ir::Check::Nil)
                } else {
                    Ok(native_ir::Check::Variant {
                        tag: *index as u32,
                        fields: fields.iter().map(|field| field.id as u32).collect(),
                    })
                }
            }
            exhaustiveness::RuntimeCheck::StringPrefix { prefix, rest } => {
                let prefix: String = crate::strings::convert_string_escape_chars(prefix).into();
                // The rest variable, when bound, becomes a slice of the
                // subject past the prefix.
                let _ = prefix_slices.insert(rest.id, (subject, prefix.len() as u32));
                Ok(native_ir::Check::StringPrefix { prefix })
            }
            exhaustiveness::RuntimeCheck::Tuple { elements, .. } => Ok(native_ir::Check::Always {
                fields: elements.iter().map(|element| element.id as u32).collect(),
            }),
            exhaustiveness::RuntimeCheck::BitArray { test } => self.bit_array_test(test),
            exhaustiveness::RuntimeCheck::EmptyList => Ok(native_ir::Check::EmptyList),
            exhaustiveness::RuntimeCheck::NonEmptyList { first, rest } => {
                Ok(native_ir::Check::NonEmptyList {
                    first: first.id as u32,
                    rest: rest.id as u32,
                })
            }
        }
    }

    fn decision_body(
        &self,
        body: &exhaustiveness::Body,
        clauses: Option<&[TypedClause]>,
        prefix_slices: &HashMap<usize, (u32, u32)>,
    ) -> Result<native_ir::Decision, Error> {
        let bindings = self.bound_values(&body.bindings, prefix_slices)?;
        // Assignments have no clause bodies: only the bindings matter.
        let body = match clauses {
            Some(clauses) => {
                let clause = clauses
                    .get(body.clause_index)
                    .expect("decision tree clause index in range");
                vec![native_ir::Statement::expression(
                    self.expression(&clause.then)?,
                )]
            }
            None => vec![],
        };
        Ok(native_ir::Decision::Run { bindings, body })
    }

    fn bound_values(
        &self,
        body_bindings: &[(EcoString, exhaustiveness::BoundValue)],
        prefix_slices: &HashMap<usize, (u32, u32)>,
    ) -> Result<Vec<(String, native_ir::Bound)>, Error> {
        let mut bindings = Vec::with_capacity(body_bindings.len());
        for (name, value) in body_bindings {
            let bound = match value {
                exhaustiveness::BoundValue::Variable(variable) => {
                    match prefix_slices.get(&variable.id) {
                        Some((subject, offset)) => native_ir::Bound::StringSlice {
                            subject: *subject,
                            offset: *offset,
                        },
                        None => native_ir::Bound::Variable(variable.id as u32),
                    }
                }
                exhaustiveness::BoundValue::LiteralInt(value) => {
                    native_ir::Bound::Value(lower_int(value))
                }
                exhaustiveness::BoundValue::LiteralFloat(value) => {
                    let value = crate::parse::LiteralFloatValue::parse(value)
                        .ok_or_else(|| self.unsupported_pattern("this float literal"))?;
                    native_ir::Bound::Value(native_ir::Expression::Float(value.value()))
                }
                exhaustiveness::BoundValue::LiteralString(value) => {
                    native_ir::Bound::Value(native_ir::Expression::String(
                        crate::strings::convert_string_escape_chars(value).into(),
                    ))
                }
                exhaustiveness::BoundValue::BitArraySlice {
                    bit_array,
                    read_action,
                } => {
                    use exhaustiveness::ReadType;
                    let subject = bit_array.id as u32;
                    let offset = Box::new(self.offset_expression(&read_action.from)?);
                    let bits = self.read_size_expression(&read_action.size)?;
                    match (&read_action.type_, bits) {
                        (ReadType::Int, Some(bits)) => native_ir::Bound::BitsReadInt {
                            subject,
                            offset,
                            bits: Box::new(bits),
                            endian: Self::endian(read_action.endianness),
                            signed: read_action.signed,
                        },
                        (ReadType::Float, Some(bits)) => native_ir::Bound::BitsReadFloat {
                            subject,
                            offset,
                            bits: Box::new(bits),
                            endian: Self::endian(read_action.endianness),
                        },
                        (ReadType::BitArray, bits) => native_ir::Bound::BitsSlice {
                            subject,
                            offset,
                            bits: bits.map(Box::new),
                        },
                        _ => return Err(self.unsupported_pattern("this bit array pattern")),
                    }
                }
                exhaustiveness::BoundValue::StringSlice { subject, prefix } => {
                    let prefix: String = crate::strings::convert_string_escape_chars(prefix).into();
                    native_ir::Bound::StringSlice {
                        subject: subject.id as u32,
                        offset: prefix.len() as u32,
                    }
                }
            };
            bindings.push((name.clone().into(), bound));
        }
        Ok(bindings)
    }

    fn echo(
        &self,
        value: native_ir::Expression,
        message: Option<&TypedExpr>,
        location: &src_span::SrcSpan,
    ) -> Result<native_ir::Expression, Error> {
        let message = match message {
            Some(message) => Some(Box::new(self.expression(message)?)),
            None => None,
        };
        Ok(native_ir::Expression::Echo {
            value: Box::new(value),
            message,
            line: self.line_numbers.line_number(location.start),
        })
    }

    fn panic_expression(
        &self,
        kind: native_ir::PanicKind,
        message: &Option<Box<TypedExpr>>,
        location: &src_span::SrcSpan,
    ) -> Result<native_ir::Expression, Error> {
        let message = match message {
            Some(message) => Some(Box::new(self.expression(message)?)),
            None => None,
        };
        Ok(native_ir::Expression::Panic {
            kind,
            message,
            function: self.function_name.clone().into(),
            line: self.line_numbers.line_number(location.start),
        })
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigInt;

    use crate::{analyse::TargetSupport, build::Target, type_::tests::compile_module_with_opts};

    fn lower(src: &str) -> native_ir::Module {
        let module = compile_module_with_opts(
            "test_module",
            src,
            None,
            vec![],
            Target::Native,
            TargetSupport::NotEnforced,
            None,
        )
        .expect("should compile");
        super::module(&module, camino::Utf8Path::new("/root"), &src.into()).expect("should lower")
    }

    /// The statements with their debug source lines zeroed (recursively,
    /// through nested bodies), so assertions stay independent of the test
    /// source's exact layout. `Panic` and assignment-failure lines are
    /// semantic — they reach the runtime's error reports — and are kept.
    fn without_lines(statements: &[native_ir::Statement]) -> Vec<native_ir::Statement> {
        fn zero_statement_lines(value: &mut serde_json::Value) {
            match value {
                serde_json::Value::Object(map) => {
                    for (key, inner) in map.iter_mut() {
                        if matches!(key.as_str(), "Let" | "Destructure" | "Expression")
                            && let serde_json::Value::Object(fields) = inner
                            && let Some(line) = fields.get_mut("line")
                        {
                            *line = serde_json::Value::from(0);
                        }
                        zero_statement_lines(inner);
                    }
                }
                serde_json::Value::Array(values) => {
                    values.iter_mut().for_each(zero_statement_lines)
                }
                _ => {}
            }
        }
        let mut value = serde_json::to_value(statements).expect("statements to json");
        zero_statement_lines(&mut value);
        serde_json::from_value(value).expect("statements from json")
    }

    #[test]
    fn integer_literals() {
        let module = lower(
            "pub fn main() {
  let small = 42
  let big = 9223372036854775808
  0
}",
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        assert_eq!(
            body[0],
            native_ir::Statement::Let {
                line: 0,
                name: "small".into(),
                value: native_ir::Expression::Int(42),
            }
        );
        assert_eq!(
            body[1],
            native_ir::Statement::Let {
                line: 0,
                name: "big".into(),
                value: native_ir::Expression::BigInt(
                    BigInt::from(9223372036854775808_u64).to_signed_bytes_le(),
                ),
            }
        );
    }

    #[test]
    fn float_literals() {
        let module = lower(
            "pub fn main() {
  let x = 1.5
  let y = -3.0e2
  0
}",
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        assert_eq!(
            body[0],
            native_ir::Statement::Let {
                line: 0,
                name: "x".into(),
                value: native_ir::Expression::Float(1.5),
            }
        );
        assert_eq!(
            body[1],
            native_ir::Statement::Let {
                line: 0,
                name: "y".into(),
                value: native_ir::Expression::Float(-300.0),
            }
        );
    }

    #[test]
    fn string_literals_are_unescaped() {
        let module = lower(
            r#"pub fn main() {
  let x = "hello\n\u{1F30D}"
  0
}"#,
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        assert_eq!(
            body[0],
            native_ir::Statement::Let {
                line: 0,
                name: "x".into(),
                value: native_ir::Expression::String("hello\n🌍".into()),
            }
        );
    }

    #[test]
    fn string_concatenation() {
        let module = lower(
            r#"pub fn main() {
  let x = "Hello, " <> "world!"
  0
}"#,
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        assert_eq!(
            body[0],
            native_ir::Statement::Let {
                line: 0,
                name: "x".into(),
                value: native_ir::Expression::StringConcat(
                    Box::new(native_ir::Expression::String("Hello, ".into())),
                    Box::new(native_ir::Expression::String("world!".into())),
                ),
            }
        );
    }

    #[test]
    fn integer_arithmetic_operators() {
        let module = lower(
            "pub fn main() {
  40 - 2 * 3
  84 / 2
  85 % 43
}",
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        assert_eq!(
            body[0],
            native_ir::Statement::expression(native_ir::Expression::IntBinary {
                operator: native_ir::IntOperator::Subtract,
                left: Box::new(native_ir::Expression::Int(40)),
                right: Box::new(native_ir::Expression::IntBinary {
                    operator: native_ir::IntOperator::Multiply,
                    left: Box::new(native_ir::Expression::Int(2)),
                    right: Box::new(native_ir::Expression::Int(3)),
                }),
            })
        );
        assert_eq!(
            body[1],
            native_ir::Statement::expression(native_ir::Expression::IntBinary {
                operator: native_ir::IntOperator::Divide,
                left: Box::new(native_ir::Expression::Int(84)),
                right: Box::new(native_ir::Expression::Int(2)),
            })
        );
        assert_eq!(
            body[2],
            native_ir::Statement::expression(native_ir::Expression::IntBinary {
                operator: native_ir::IntOperator::Remainder,
                left: Box::new(native_ir::Expression::Int(85)),
                right: Box::new(native_ir::Expression::Int(43)),
            })
        );
    }

    #[test]
    fn float_operators() {
        let module = lower(
            "pub fn main() {
  1.5 +. 2.0
  10.0 /. 4.0
  1.5 <. 2.5
}",
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        assert_eq!(
            body[0],
            native_ir::Statement::expression(native_ir::Expression::FloatBinary {
                operator: native_ir::FloatOperator::Add,
                left: Box::new(native_ir::Expression::Float(1.5)),
                right: Box::new(native_ir::Expression::Float(2.0)),
            })
        );
        assert_eq!(
            body[1],
            native_ir::Statement::expression(native_ir::Expression::FloatBinary {
                operator: native_ir::FloatOperator::Divide,
                left: Box::new(native_ir::Expression::Float(10.0)),
                right: Box::new(native_ir::Expression::Float(4.0)),
            })
        );
        assert_eq!(
            body[2],
            native_ir::Statement::expression(native_ir::Expression::FloatCompare {
                operator: native_ir::CompareOperator::LessThan,
                left: Box::new(native_ir::Expression::Float(1.5)),
                right: Box::new(native_ir::Expression::Float(2.5)),
            })
        );
    }

    #[test]
    fn integer_comparison_operators() {
        let module = lower(
            "pub fn main() {
  1 < 2
  3 >= 4
}",
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        assert_eq!(
            body[0],
            native_ir::Statement::expression(native_ir::Expression::IntCompare {
                operator: native_ir::CompareOperator::LessThan,
                left: Box::new(native_ir::Expression::Int(1)),
                right: Box::new(native_ir::Expression::Int(2)),
            })
        );
        assert_eq!(
            body[1],
            native_ir::Statement::expression(native_ir::Expression::IntCompare {
                operator: native_ir::CompareOperator::GreaterThanOrEqual,
                left: Box::new(native_ir::Expression::Int(3)),
                right: Box::new(native_ir::Expression::Int(4)),
            })
        );
    }

    #[test]
    fn equality_is_type_directed() {
        let module = lower(
            r#"pub fn main() {
  1 == 2
  1.5 != 2.5
  "a" == "b"
  True != False
}"#,
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        let kinds: Vec<_> = body
            .iter()
            .map(|statement| match statement {
                native_ir::Statement::Expression {
                    expression: native_ir::Expression::Equality { kind, negated, .. },
                    ..
                } => (*kind, *negated),
                _ => panic!("expected an equality expression"),
            })
            .collect();
        assert_eq!(
            kinds,
            vec![
                (native_ir::EqualityKind::Int, false),
                (native_ir::EqualityKind::Float, true),
                (native_ir::EqualityKind::String, false),
                (native_ir::EqualityKind::Immediate, true),
            ]
        );
    }

    #[test]
    fn boolean_operators() {
        let module = lower(
            "pub fn main() {
  True && False || True
}",
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        // `&&` binds tighter than `||`.
        assert_eq!(
            body[0],
            native_ir::Statement::expression(native_ir::Expression::BoolBinary {
                operator: native_ir::BoolOperator::Or,
                left: Box::new(native_ir::Expression::BoolBinary {
                    operator: native_ir::BoolOperator::And,
                    left: Box::new(native_ir::Expression::Bool(true)),
                    right: Box::new(native_ir::Expression::Bool(false)),
                }),
                right: Box::new(native_ir::Expression::Bool(true)),
            })
        );
    }

    #[test]
    fn boolean_literals() {
        let module = lower(
            "pub fn main() {
  let yes = True
  let no = False
  0
}",
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        assert_eq!(
            body[0],
            native_ir::Statement::Let {
                line: 0,
                name: "yes".into(),
                value: native_ir::Expression::Bool(true),
            }
        );
        assert_eq!(
            body[1],
            native_ir::Statement::Let {
                line: 0,
                name: "no".into(),
                value: native_ir::Expression::Bool(false),
            }
        );
    }

    #[test]
    fn case_expressions() {
        let module = lower(
            r#"pub fn main() {
  case 5 {
    1 -> 10
    n -> n
  }
}"#,
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        assert_eq!(
            body[0],
            native_ir::Statement::expression(native_ir::Expression::Case {
                subjects: vec![native_ir::Expression::Int(5)],
                subject_ids: vec![0],
                tree: native_ir::Decision::Switch {
                    var: 0,
                    choices: vec![(
                        native_ir::Check::Int(1),
                        native_ir::Decision::Run {
                            bindings: vec![],
                            body: vec![native_ir::Statement::expression(
                                native_ir::Expression::Int(10)
                            )],
                        },
                    )],
                    fallback: Box::new(native_ir::Decision::Run {
                        bindings: vec![("n".into(), native_ir::Bound::Variable(0))],
                        body: vec![native_ir::Statement::expression(
                            native_ir::Expression::Variable("n".into())
                        )],
                    }),
                    fallback_fields: vec![],
                },
            })
        );
    }

    #[test]
    fn custom_types() {
        let module = lower(
            r#"pub type Pair {
  Pair(first: Int, second: Int)
}

pub fn main() {
  let pair = Pair(1, 2)
  let total = case pair {
    Pair(first, second) -> first + second
  }
  total + pair.first
}"#,
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        assert_eq!(
            body[0],
            native_ir::Statement::Let {
                line: 0,
                name: "pair".into(),
                value: native_ir::Expression::Constructor {
                    tag: 0,
                    display: native_ir::ConstructorDisplay::Record {
                        name: "Pair".into()
                    },
                    arguments: vec![native_ir::Expression::Int(1), native_ir::Expression::Int(2)],
                },
            }
        );
        // The single-variant match is not tag-tested: the fields arrive via
        // the fallback.
        let native_ir::Statement::Let { value, .. } = &body[1] else {
            panic!("expected a let");
        };
        let native_ir::Expression::Case { tree, .. } = value else {
            panic!("expected a case");
        };
        let native_ir::Decision::Switch {
            choices,
            fallback_fields,
            ..
        } = tree
        else {
            panic!("expected a switch");
        };
        assert!(choices.is_empty());
        assert_eq!(fallback_fields.len(), 2);
        // `pair.first` reads field 0.
        let native_ir::Statement::Expression {
            expression: native_ir::Expression::IntBinary { right, .. },
            ..
        } = &body[2]
        else {
            panic!("expected an addition");
        };
        assert_eq!(
            right.as_ref(),
            &native_ir::Expression::FieldAccess {
                record: Box::new(native_ir::Expression::Variable("pair".into())),
                index: 0,
            }
        );
    }

    #[test]
    fn tuple_construction_and_access() {
        let module = lower(
            "pub fn main() {
  let pair = #(1, #(2, 3))
  pair.0
}",
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        assert_eq!(
            body[0],
            native_ir::Statement::Let {
                line: 0,
                name: "pair".into(),
                value: native_ir::Expression::Constructor {
                    tag: 0,
                    display: native_ir::ConstructorDisplay::Tuple,
                    arguments: vec![
                        native_ir::Expression::Int(1),
                        native_ir::Expression::Constructor {
                            tag: 0,
                            display: native_ir::ConstructorDisplay::Tuple,
                            arguments: vec![
                                native_ir::Expression::Int(2),
                                native_ir::Expression::Int(3),
                            ],
                        },
                    ],
                },
            }
        );
        assert_eq!(
            body[1],
            native_ir::Statement::expression(native_ir::Expression::FieldAccess {
                record: Box::new(native_ir::Expression::Variable("pair".into())),
                index: 0,
            })
        );
    }

    #[test]
    fn tuple_patterns_extract_fields_without_a_test() {
        let module = lower(
            "pub fn main() {
  case #(1, 2) {
    #(a, b) -> a + b
  }
}",
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        let native_ir::Statement::Expression {
            expression: native_ir::Expression::Case { tree, .. },
            ..
        } = &body[0]
        else {
            panic!("expected a case expression");
        };
        let native_ir::Decision::Switch {
            choices,
            fallback_fields,
            ..
        } = tree
        else {
            panic!("expected a switch");
        };
        // A tuple pattern cannot fail, so its element extraction arrives
        // via the untested fallback.
        assert!(choices.is_empty());
        assert_eq!(fallback_fields.len(), 2);
    }

    #[test]
    fn tuple_index_in_guards() {
        let module = lower(
            "pub fn main() {
  case #(1, 2) {
    pair if pair.1 > 1 -> 1
    _ -> 0
  }
}",
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        let native_ir::Statement::Expression {
            expression: native_ir::Expression::Case { tree, .. },
            ..
        } = &body[0]
        else {
            panic!("expected a case expression");
        };
        let native_ir::Decision::Guard { guard, .. } = tree else {
            panic!("expected a guard");
        };
        assert_eq!(
            guard.as_ref(),
            &native_ir::Expression::IntCompare {
                operator: native_ir::CompareOperator::GreaterThan,
                left: Box::new(native_ir::Expression::FieldAccess {
                    record: Box::new(native_ir::Expression::Variable("pair".into())),
                    index: 1,
                }),
                right: Box::new(native_ir::Expression::Int(1)),
            }
        );
    }

    #[test]
    fn list_literals() {
        let module = lower(
            "pub fn main() {
  let rest = [4]
  [3, ..rest]
}",
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        assert_eq!(
            body[0],
            native_ir::Statement::Let {
                line: 0,
                name: "rest".into(),
                value: native_ir::Expression::Constructor {
                    tag: 1,
                    display: native_ir::ConstructorDisplay::List,
                    arguments: vec![
                        native_ir::Expression::Int(4),
                        native_ir::Expression::EmptyList,
                    ],
                },
            }
        );
        // The spread tail is used directly rather than rebuilt.
        assert_eq!(
            body[1],
            native_ir::Statement::expression(native_ir::Expression::Constructor {
                tag: 1,
                display: native_ir::ConstructorDisplay::List,
                arguments: vec![
                    native_ir::Expression::Int(3),
                    native_ir::Expression::Variable("rest".into()),
                ],
            })
        );
    }

    #[test]
    fn list_patterns() {
        let module = lower(
            "pub fn main(list: List(Int)) {
  case list {
    [] -> 0
    [first, ..rest] -> first
  }
}",
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        let native_ir::Statement::Expression {
            expression: native_ir::Expression::Case { tree, .. },
            ..
        } = &body[0]
        else {
            panic!("expected a case expression");
        };
        let native_ir::Decision::Switch {
            choices,
            fallback_fields,
            ..
        } = tree
        else {
            panic!("expected a switch");
        };
        // The empty list is an immediate word; the cons case is the
        // exhaustive fallback whose head and tail arrive untested.
        assert_eq!(choices.len(), 1);
        assert_eq!(choices[0].0, native_ir::Check::EmptyList);
        assert_eq!(fallback_fields.len(), 2);
    }

    #[test]
    fn nested_list_patterns_switch_on_extracted_variables() {
        let module = lower(
            "pub fn main(list: List(Int)) {
  case list {
    [_, _] -> 2
    _ -> 0
  }
}",
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        let native_ir::Statement::Expression {
            expression:
                native_ir::Expression::Case {
                    subject_ids, tree, ..
                },
            ..
        } = &body[0]
        else {
            panic!("expected a case expression");
        };
        // The outer switch checks the subject; some inner switch must check
        // a variable that is not the subject: the extracted tail.
        let native_ir::Decision::Switch {
            var,
            choices,
            fallback,
            ..
        } = tree
        else {
            panic!("expected a switch");
        };
        assert_eq!(subject_ids, &vec![*var]);
        fn has_non_subject_switch(decision: &native_ir::Decision, subject: u32) -> bool {
            match decision {
                native_ir::Decision::Switch {
                    var,
                    choices,
                    fallback,
                    ..
                } => {
                    *var != subject
                        || choices
                            .iter()
                            .any(|(_, decision)| has_non_subject_switch(decision, subject))
                        || has_non_subject_switch(fallback, subject)
                }
                _ => false,
            }
        }
        assert!(
            choices
                .iter()
                .any(|(_, decision)| has_non_subject_switch(decision, *var))
                || has_non_subject_switch(fallback, *var)
        );
    }

    #[test]
    fn record_updates() {
        let module = lower(
            r#"pub type Person {
  Person(name: String, age: Int)
}

pub fn main() {
  let alice = Person(name: "Alice", age: 30)
  Person(..alice, age: 31)
}"#,
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        // The unchanged field reads from the spread record; the updated one
        // uses the new value.
        assert_eq!(
            body[1],
            native_ir::Statement::expression(native_ir::Expression::Constructor {
                tag: 0,
                display: native_ir::ConstructorDisplay::Record {
                    name: "Person".into()
                },
                arguments: vec![
                    native_ir::Expression::FieldAccess {
                        record: Box::new(native_ir::Expression::Variable("alice".into())),
                        index: 0,
                    },
                    native_ir::Expression::Int(31),
                ],
            })
        );
    }

    #[test]
    fn destructuring_assignments() {
        let module = lower(
            r#"pub fn main() {
  let #(a, b) = #(1, 2)
  let assert Ok(value) = Ok(a + b) as "always fine"
  value
}"#,
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        // Irrefutable tuple destructuring: no failure handler.
        let native_ir::Statement::Destructure {
            on_failure: None,
            tree,
            ..
        } = &body[0]
        else {
            panic!("expected an infallible destructure, got {:?}", body[0]);
        };
        let native_ir::Decision::Switch {
            fallback_fields, ..
        } = tree
        else {
            panic!("expected a switch");
        };
        assert_eq!(fallback_fields.len(), 2);

        // `let assert` carries its failure metadata and message.
        let native_ir::Statement::Destructure {
            on_failure: Some(failure),
            ..
        } = &body[1]
        else {
            panic!("expected a fallible destructure, got {:?}", body[1]);
        };
        assert_eq!(
            failure.message.as_deref(),
            Some(&native_ir::Expression::String("always fine".into()))
        );
        assert_eq!(failure.function, "main");
        assert_eq!(failure.line, 3);
    }

    #[test]
    fn closures_and_function_values() {
        let module = lower(
            "fn double(x: Int) -> Int {
  x * 2
}

pub fn main() {
  let n = 10
  let adder = fn(x) { x + n }
  let doubler = double
  adder(doubler(1))
}",
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[1] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        assert_eq!(
            body[1],
            native_ir::Statement::Let {
                line: 0,
                name: "adder".into(),
                value: native_ir::Expression::Lambda {
                    parameters: vec!["x".into()],
                    body: vec![native_ir::Statement::expression(
                        native_ir::Expression::IntBinary {
                            operator: native_ir::IntOperator::Add,
                            left: Box::new(native_ir::Expression::Variable("x".into())),
                            right: Box::new(native_ir::Expression::Variable("n".into())),
                        }
                    )],
                },
            }
        );
        assert_eq!(
            body[2],
            native_ir::Statement::Let {
                line: 0,
                name: "doubler".into(),
                value: native_ir::Expression::FunctionReference {
                    module: "test_module".into(),
                    function: "double".into(),
                    arity: 1,
                },
            }
        );
        assert_eq!(
            body[3],
            native_ir::Statement::expression(native_ir::Expression::CallValue {
                callee: Box::new(native_ir::Expression::Variable("adder".into())),
                arguments: vec![native_ir::Expression::CallValue {
                    callee: Box::new(native_ir::Expression::Variable("doubler".into())),
                    arguments: vec![native_ir::Expression::Int(1)],
                }],
            })
        );
    }

    #[test]
    fn module_constants_are_inlined() {
        let module = lower(
            r#"const answer = 42

const long_greeting = greeting <> "!"

const greeting = "hello"

const words = ["forty", "two"]

pub fn main() {
  let x = answer
  let y = long_greeting
  let z = words
  x
}"#,
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        assert_eq!(
            body[0],
            native_ir::Statement::Let {
                line: 0,
                name: "x".into(),
                value: native_ir::Expression::Int(42),
            }
        );
        // A constant referencing another constant inlines it recursively.
        assert_eq!(
            body[1],
            native_ir::Statement::Let {
                line: 0,
                name: "y".into(),
                value: native_ir::Expression::StringConcat(
                    Box::new(native_ir::Expression::String("hello".into())),
                    Box::new(native_ir::Expression::String("!".into())),
                ),
            }
        );
        assert_eq!(
            body[2],
            native_ir::Statement::Let {
                line: 0,
                name: "z".into(),
                value: native_ir::Expression::Constructor {
                    tag: 1,
                    display: native_ir::ConstructorDisplay::List,
                    arguments: vec![
                        native_ir::Expression::String("forty".into()),
                        native_ir::Expression::Constructor {
                            tag: 1,
                            display: native_ir::ConstructorDisplay::List,
                            arguments: vec![
                                native_ir::Expression::String("two".into()),
                                native_ir::Expression::EmptyList,
                            ],
                        },
                    ],
                },
            }
        );
    }

    #[test]
    fn bit_arrays() {
        let module = lower(
            r#"pub fn main(input: BitArray) {
  let constructed = <<1, 513:16-little, "hi":utf16, 2.5:32-float, input:bits-size(4)>>
  case input {
    <<length, payload:bytes-size(length), _:bits>> -> payload
    _ -> constructed
  }
}"#,
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        // Construction: five segments with the right kinds.
        let native_ir::Statement::Let { value, .. } = &body[0] else {
            panic!("expected a let");
        };
        let native_ir::Expression::BitArray(segments) = value else {
            panic!("expected a bit array, got {value:?}");
        };
        let kinds: Vec<_> = segments.iter().map(|segment| &segment.kind).collect();
        assert!(matches!(
            kinds[0],
            native_ir::BitSegmentKind::Int {
                endian: native_ir::Endian::Big,
                ..
            }
        ));
        assert!(matches!(
            kinds[1],
            native_ir::BitSegmentKind::Int {
                endian: native_ir::Endian::Little,
                ..
            }
        ));
        assert!(matches!(
            kinds[2],
            native_ir::BitSegmentKind::String {
                encoding: native_ir::StringEncoding::Utf16,
                ..
            }
        ));
        assert!(matches!(kinds[3], native_ir::BitSegmentKind::Float { .. }));
        assert!(matches!(
            kinds[4],
            native_ir::BitSegmentKind::BitArraySplice { bits: Some(_) }
        ));

        // The pattern produces bit array checks, and the dynamic
        // `bytes-size(length)` payload read refers to the materialized
        // `length` segment.
        let native_ir::Statement::Expression {
            expression: native_ir::Expression::Case { tree, .. },
            ..
        } = &body[1]
        else {
            panic!("expected a case");
        };
        fn find_dynamic_read(decision: &native_ir::Decision) -> bool {
            match decision {
                native_ir::Decision::Run { bindings, .. } => {
                    bindings.iter().any(|(_, bound)| match bound {
                        native_ir::Bound::BitsSlice {
                            bits: Some(bits), ..
                        } => format!("{bits:?}").contains("bit$length"),
                        _ => false,
                    })
                }
                native_ir::Decision::Switch {
                    choices, fallback, ..
                } => {
                    choices
                        .iter()
                        .any(|(_, decision)| find_dynamic_read(decision))
                        || find_dynamic_read(fallback)
                }
                native_ir::Decision::Guard { if_false, .. } => find_dynamic_read(if_false),
                native_ir::Decision::Fail => false,
            }
        }
        assert!(find_dynamic_read(tree), "tree: {tree:#?}");
    }

    #[test]
    fn case_guards() {
        let module = lower(
            r#"pub fn main() {
  case 5 {
    n if n > 3 && n != 4 -> n
    _ -> 0
  }
}"#,
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        let native_ir::Statement::Expression {
            expression: native_ir::Expression::Case { tree, .. },
            ..
        } = &body[0]
        else {
            panic!("expected a case expression");
        };
        assert_eq!(
            tree,
            &native_ir::Decision::Guard {
                bindings: vec![("n".into(), native_ir::Bound::Variable(0))],
                guard: Box::new(native_ir::Expression::BoolBinary {
                    operator: native_ir::BoolOperator::And,
                    left: Box::new(native_ir::Expression::IntCompare {
                        operator: native_ir::CompareOperator::GreaterThan,
                        left: Box::new(native_ir::Expression::Variable("n".into())),
                        right: Box::new(native_ir::Expression::Int(3)),
                    }),
                    right: Box::new(native_ir::Expression::Equality {
                        kind: native_ir::EqualityKind::Int,
                        negated: true,
                        left: Box::new(native_ir::Expression::Variable("n".into())),
                        right: Box::new(native_ir::Expression::Int(4)),
                    }),
                }),
                if_true: vec![native_ir::Statement::expression(
                    native_ir::Expression::Variable("n".into())
                )],
                if_false: Box::new(native_ir::Decision::Run {
                    bindings: vec![],
                    body: vec![native_ir::Statement::expression(
                        native_ir::Expression::Int(0)
                    )],
                }),
            }
        );
    }

    #[test]
    fn field_access_in_guards() {
        let module = lower(
            r#"pub type Person {
  Person(name: String, age: Int)
}

pub fn main() {
  case Person("Ada", 36) {
    person if person.age > 18 -> 1
    _ -> 0
  }
}"#,
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        let native_ir::Statement::Expression {
            expression: native_ir::Expression::Case { tree, .. },
            ..
        } = &body[0]
        else {
            panic!("expected a case expression");
        };
        // The guard sits below the single-variant switch.
        fn find_guard(decision: &native_ir::Decision) -> Option<&native_ir::Expression> {
            match decision {
                native_ir::Decision::Guard { guard, .. } => Some(guard),
                native_ir::Decision::Switch {
                    choices, fallback, ..
                } => choices
                    .iter()
                    .find_map(|(_, decision)| find_guard(decision))
                    .or_else(|| find_guard(fallback)),
                _ => None,
            }
        }
        let guard = find_guard(tree).expect("a guard in the tree");
        assert_eq!(
            guard,
            &native_ir::Expression::IntCompare {
                operator: native_ir::CompareOperator::GreaterThan,
                left: Box::new(native_ir::Expression::FieldAccess {
                    record: Box::new(native_ir::Expression::Variable("person".into())),
                    index: 1,
                }),
                right: Box::new(native_ir::Expression::Int(18)),
            }
        );
    }

    #[test]
    fn case_on_booleans_maps_variant_indices() {
        let module = lower(
            "pub fn check(value: Bool) {
  case value {
    True -> 1
    False -> 0
  }
}",
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        let native_ir::Statement::Expression {
            expression: native_ir::Expression::Case { tree, .. },
            ..
        } = &body[0]
        else {
            panic!("expected a case expression");
        };
        let native_ir::Decision::Switch { choices, .. } = tree else {
            panic!("expected a switch");
        };
        // The `True` pattern checks for the `True` immediate word.
        assert_eq!(choices[0].0, native_ir::Check::Bool(true));
    }

    #[test]
    fn panic_and_todo() {
        let module = lower(
            r#"pub fn main() {
  let x = todo
  panic as "boom"
}"#,
        );
        let native_ir::Function::Defined { body, .. } = &module.functions[0] else {
            panic!("expected a defined function");
        };
        let body = without_lines(body);
        assert_eq!(
            body[0],
            native_ir::Statement::Let {
                line: 0,
                name: "x".into(),
                value: native_ir::Expression::Panic {
                    kind: native_ir::PanicKind::Todo,
                    message: None,
                    function: "main".into(),
                    line: 2,
                },
            }
        );
        assert_eq!(
            body[1],
            native_ir::Statement::expression(native_ir::Expression::Panic {
                kind: native_ir::PanicKind::Panic,
                message: Some(Box::new(native_ir::Expression::String("boom".into()))),
                function: "main".into(),
                line: 3,
            })
        );
    }
}
