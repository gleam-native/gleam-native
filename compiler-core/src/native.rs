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
    type_::{PRELUDE_MODULE_NAME, Type, ValueConstructorVariant},
};

/// The tagged word encoding of a small integer; see `native-runtime`.
fn tag_small_int(value: i64) -> i64 {
    (value << 1) | 1
}

fn lower_int(value: &BigInt) -> native_ir::Expression {
    match value
        .to_i64()
        .filter(|value| ((i64::MIN >> 1)..=(i64::MAX >> 1)).contains(value))
    {
        Some(value) => native_ir::Expression::Int(value),
        None => native_ir::Expression::BigInt(value.to_signed_bytes_le()),
    }
}

pub fn module(module: &TypedModule) -> Result<native_ir::Module, Error> {
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
        functions,
    })
}

struct Lowerer<'a> {
    module_name: EcoString,
    function_name: EcoString,
    line_numbers: &'a src_span::LineNumbers,
}

impl Lowerer<'_> {
    fn unsupported(&self, feature: &str) -> Error {
        Error::NativeUnsupportedFeature {
            module: self.module_name.clone(),
            feature: feature.into(),
        }
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
        match statement {
            Statement::Expression(expression) => Ok(native_ir::Statement::Expression(
                self.expression(expression)?,
            )),

            Statement::Assignment(assignment) => {
                // A plain variable pattern is irrefutable and binds directly.
                if let Pattern::Variable { name, .. } = &assignment.pattern {
                    return Ok(native_ir::Statement::Let {
                        name: name.clone().into(),
                        value: self.expression(&assignment.value)?,
                    });
                }

                let subject_id = assignment
                    .compiled_case
                    .subject_variables
                    .first()
                    .expect("assignment decision tree has a subject")
                    .id as u32;
                let mut prefix_slices = HashMap::new();
                let tree =
                    self.decision(&assignment.compiled_case.tree, None, &mut prefix_slices)?;
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
                })
            }

            // The type checker has already desugared `use` into a call with
            // a callback function.
            Statement::Use(use_) => Ok(native_ir::Statement::Expression(
                self.expression(&use_.call)?,
            )),
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
                Ok(native_ir::Statement::Expression(native_ir::Expression::Case {
                    subjects: vec![self.expression(&assert.value)?],
                    subject_ids: vec![0],
                    tree: native_ir::Decision::Switch {
                        var: 0,
                        choices: vec![(
                            native_ir::Check::Immediate(tag_small_int(1)),
                            native_ir::Decision::Run {
                                bindings: vec![],
                                body: vec![native_ir::Statement::Expression(
                                    native_ir::Expression::Nil,
                                )],
                            },
                        )],
                        fallback: Box::new(native_ir::Decision::Run {
                            bindings: vec![],
                            body: vec![native_ir::Statement::Expression(panic)],
                        }),
                        fallback_fields: vec![],
                    },
                }))
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
                    arity: 0,
                    module,
                    ..
                } if module == PRELUDE_MODULE_NAME && name == "Nil" => {
                    Ok(native_ir::Expression::Nil)
                }
                ValueConstructorVariant::Record {
                    name,
                    arity: 0,
                    module,
                    ..
                } if module == PRELUDE_MODULE_NAME && (name == "True" || name == "False") => {
                    Ok(native_ir::Expression::Bool(name == "True"))
                }
                ValueConstructorVariant::Record {
                    arity: 0,
                    variant_index,
                    ..
                } => Ok(native_ir::Expression::Constructor {
                    tag: *variant_index as u32,
                    arguments: vec![],
                }),
                // A constructor used as a function value: a lambda that
                // allocates the record.
                ValueConstructorVariant::Record {
                    arity,
                    variant_index,
                    ..
                } => {
                    let parameters: Vec<String> =
                        (0..*arity).map(|index| format!("$field{index}")).collect();
                    let arguments = parameters
                        .iter()
                        .map(|name| native_ir::Expression::Variable(name.clone()))
                        .collect();
                    Ok(native_ir::Expression::Lambda {
                        parameters,
                        body: vec![native_ir::Statement::Expression(
                            native_ir::Expression::Constructor {
                                tag: *variant_index as u32,
                                arguments,
                            },
                        )],
                    })
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
                ValueConstructorVariant::ModuleConstant { literal, .. } => {
                    self.constant(literal)
                }
            },

            TypedExpr::Call {
                fun, arguments, ..
            } => {
                let arguments = arguments
                    .iter()
                    .map(|argument| self.expression(&argument.value))
                    .collect::<Result<Vec<_>, _>>()?;
                match fun.as_ref() {
                    TypedExpr::Var { constructor, .. } => match &constructor.variant {
                        ValueConstructorVariant::ModuleFn { module, name, .. } => {
                            Ok(native_ir::Expression::Call {
                                module: module.clone().into(),
                                function: name.clone().into(),
                                arguments,
                            })
                        }
                        ValueConstructorVariant::Record { variant_index, .. } => {
                            Ok(native_ir::Expression::Constructor {
                                tag: *variant_index as u32,
                                arguments,
                            })
                        }
                        _ => Ok(native_ir::Expression::CallValue {
                            callee: Box::new(self.expression(fun)?),
                            arguments,
                        }),
                    },
                    _ => Ok(native_ir::Expression::CallValue {
                        callee: Box::new(self.expression(fun)?),
                        arguments,
                    }),
                }
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
            // expression.
            TypedExpr::Pipeline {
                first_value,
                assignments,
                finally,
                ..
            } => {
                let mut statements = Vec::with_capacity(assignments.len() + 2);
                statements.push(native_ir::Statement::Let {
                    name: first_value.name.clone().into(),
                    value: self.expression(&first_value.value)?,
                });
                for (assignment, _kind) in assignments {
                    statements.push(native_ir::Statement::Let {
                        name: assignment.name.clone().into(),
                        value: self.expression(&assignment.value)?,
                    });
                }
                statements.push(native_ir::Statement::Expression(
                    self.expression(finally)?,
                ));
                Ok(native_ir::Expression::Block(statements))
            }

            TypedExpr::Tuple { elements, .. } => {
                // Tuples are records with tag 0.
                let arguments = elements
                    .iter()
                    .map(|element| self.expression(element))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(native_ir::Expression::Constructor { tag: 0, arguments })
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
                let tag = match constructor.as_ref() {
                    TypedExpr::Var { constructor, .. } => match &constructor.variant {
                        ValueConstructorVariant::Record { variant_index, .. } => {
                            *variant_index as u32
                        }
                        _ => return Err(self.unsupported("this record update")),
                    },
                    _ => return Err(self.unsupported("this record update")),
                };
                let arguments = arguments
                    .iter()
                    .map(|argument| self.expression(&argument.value))
                    .collect::<Result<Vec<_>, _>>()?;
                let construct = native_ir::Expression::Constructor { tag, arguments };
                match updated_record_assigned_name {
                    // The spread expression is not a plain variable: bind it
                    // to the compiler-chosen name the arguments refer to.
                    Some(name) => Ok(native_ir::Expression::Block(vec![
                        native_ir::Statement::Let {
                            name: name.clone().into(),
                            value: self.expression(updated_record)?,
                        },
                        native_ir::Statement::Expression(construct),
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
                let tree = self.decision(&compiled_case.tree, Some(clauses), &mut prefix_slices)?;
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
                let value = echo_expression
                    .as_ref()
                    .ok_or_else(|| self.unsupported("echo inside a pipeline"))?;
                let type_ = value.type_();
                let kind = if type_.is_int() {
                    native_ir::EchoKind::Int
                } else if type_.is_float() {
                    native_ir::EchoKind::Float
                } else if type_.is_string() {
                    native_ir::EchoKind::String
                } else if type_.is_bool() {
                    native_ir::EchoKind::Bool
                } else if type_.is_nil() {
                    native_ir::EchoKind::Nil
                } else {
                    native_ir::EchoKind::Structural
                };
                let message = match message {
                    Some(message) => Some(Box::new(self.expression(message)?)),
                    None => None,
                };
                Ok(native_ir::Expression::Echo {
                    kind,
                    value: Box::new(self.expression(value)?),
                    message,
                    line: self.line_numbers.line_number(location.start),
                })
            }

            TypedExpr::Panic {
                location, message, ..
            } => self.panic_expression(native_ir::PanicKind::Panic, message, location),
            TypedExpr::Todo {
                location, message, ..
            } => self.panic_expression(native_ir::PanicKind::Todo, message, location),

            _ => Err(self.unsupported("this kind of expression")),
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
        let left = Box::new(left);
        let right = Box::new(right);
        match operator {
            BinOp::AddInt | BinOp::SubInt | BinOp::MultInt | BinOp::DivInt
            | BinOp::RemainderInt => Ok(native_ir::Expression::IntBinary {
                operator: match operator {
                    BinOp::AddInt => native_ir::IntOperator::Add,
                    BinOp::SubInt => native_ir::IntOperator::Subtract,
                    BinOp::MultInt => native_ir::IntOperator::Multiply,
                    BinOp::DivInt => native_ir::IntOperator::Divide,
                    _ => native_ir::IntOperator::Remainder,
                },
                left,
                right,
            }),
            BinOp::LtInt | BinOp::LtEqInt | BinOp::GtInt | BinOp::GtEqInt => {
                Ok(native_ir::Expression::IntCompare {
                    operator: match operator {
                        BinOp::LtInt => native_ir::CompareOperator::LessThan,
                        BinOp::LtEqInt => native_ir::CompareOperator::LessThanOrEqual,
                        BinOp::GtInt => native_ir::CompareOperator::GreaterThan,
                        _ => native_ir::CompareOperator::GreaterThanOrEqual,
                    },
                    left,
                    right,
                })
            }
            BinOp::AddFloat | BinOp::SubFloat | BinOp::MultFloat | BinOp::DivFloat => {
                Ok(native_ir::Expression::FloatBinary {
                    operator: match operator {
                        BinOp::AddFloat => native_ir::FloatOperator::Add,
                        BinOp::SubFloat => native_ir::FloatOperator::Subtract,
                        BinOp::MultFloat => native_ir::FloatOperator::Multiply,
                        _ => native_ir::FloatOperator::Divide,
                    },
                    left,
                    right,
                })
            }
            BinOp::LtFloat | BinOp::LtEqFloat | BinOp::GtFloat | BinOp::GtEqFloat => {
                Ok(native_ir::Expression::FloatCompare {
                    operator: match operator {
                        BinOp::LtFloat => native_ir::CompareOperator::LessThan,
                        BinOp::LtEqFloat => native_ir::CompareOperator::LessThanOrEqual,
                        BinOp::GtFloat => native_ir::CompareOperator::GreaterThan,
                        _ => native_ir::CompareOperator::GreaterThanOrEqual,
                    },
                    left,
                    right,
                })
            }
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
            BinOp::And | BinOp::Or => Ok(native_ir::Expression::BoolBinary {
                operator: match operator {
                    BinOp::And => native_ir::BoolOperator::And,
                    _ => native_ir::BoolOperator::Or,
                },
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
                Err(self.unsupported("this guard expression"))
            }
            ClauseGuard::ModuleSelect { literal, .. } => self.constant(literal),
            ClauseGuard::Invalid { .. } => Err(self.unsupported("this guard expression")),
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
                arguments,
                record_constructor,
                ..
            } => {
                let tag = record_constructor
                    .as_deref()
                    .and_then(|constructor| match &constructor.variant {
                        ValueConstructorVariant::Record { variant_index, .. } => {
                            Some(*variant_index as u32)
                        }
                        _ => None,
                    })
                    .ok_or_else(|| self.unsupported("this kind of constant"))?;
                let arguments = arguments
                    .as_deref()
                    .unwrap_or_default()
                    .iter()
                    .map(|argument| self.constant(&argument.value))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(native_ir::Expression::Constructor { tag, arguments })
            }
            Constant::Tuple { elements, .. } => {
                let arguments = elements
                    .iter()
                    .map(|element| self.constant(element))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(native_ir::Expression::Constructor { tag: 0, arguments })
            }
            Constant::List { elements, .. } => {
                let mut list = native_ir::Expression::EmptyList;
                for element in elements.iter().rev() {
                    list = native_ir::Expression::Constructor {
                        tag: 1,
                        arguments: vec![self.constant(element)?, list],
                    };
                }
                Ok(list)
            }
            // A constant referring to another constant or to a function.
            Constant::Var { constructor, .. } => {
                let constructor = constructor
                    .as_deref()
                    .ok_or_else(|| self.unsupported("this kind of constant"))?;
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
                    _ => Err(self.unsupported("this kind of constant")),
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
            _ => Err(self.unsupported("this kind of constant")),
        }
    }

    /// Works out how a bit array expression segment is written: its
    /// compile-time size, type, and endianness. Anything dynamic or not
    /// byte-aligned is unsupported for now.
    fn bit_segment_kind(
        &self,
        segment: &crate::ast::TypedExprBitArraySegment,
    ) -> Result<native_ir::BitSegmentKind, Error> {
        use crate::ast::BitArrayOption;
        let mut size: Option<native_ir::Expression> = None;
        let mut unit: u32 = 1;
        let mut endian = native_ir::Endian::Big;
        let mut encoding = None;
        let mut is_codepoint = false;
        let mut is_float = false;
        let mut is_splice = false;
        for option in &segment.options {
            match option {
                BitArrayOption::Int { .. } | BitArrayOption::Big { .. } => {}
                BitArrayOption::Signed { .. } | BitArrayOption::Unsigned { .. } => {}
                BitArrayOption::Little { .. } => endian = native_ir::Endian::Little,
                BitArrayOption::Native { .. } => endian = native_ir::Endian::Native,
                BitArrayOption::Float { .. } => is_float = true,
                BitArrayOption::Utf8 { .. } => encoding = Some(native_ir::StringEncoding::Utf8),
                BitArrayOption::Utf16 { .. } => {
                    encoding = Some(native_ir::StringEncoding::Utf16)
                }
                BitArrayOption::Utf32 { .. } => {
                    encoding = Some(native_ir::StringEncoding::Utf32)
                }
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
                BitArrayOption::Size { value, .. } => size = Some(self.expression(value)?),
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
        if segment.type_.is_string() {
            return Ok(native_ir::BitSegmentKind::String {
                encoding: native_ir::StringEncoding::Utf8,
                endian,
            });
        }
        if is_splice || segment.type_.is_bit_array() {
            return Ok(native_ir::BitSegmentKind::BitArraySplice {
                bits: match size {
                    Some(size) => Some(Box::new(Self::multiply(size, unit as u64))),
                    None => None,
                },
            });
        }
        if is_float || segment.type_.is_float() {
            let bits = Self::multiply(
                size.unwrap_or(native_ir::Expression::Int(64)),
                unit as u64,
            );
            return Ok(native_ir::BitSegmentKind::Float {
                bits: Box::new(bits),
                endian,
            });
        }
        if !segment.type_.is_int() {
            return Err(self.unsupported("this bit array segment"));
        }
        let bits = Self::multiply(
            size.unwrap_or(native_ir::Expression::Int(8)),
            unit as u64,
        );
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
            let term = Self::multiply(
                Self::variable_usage_expression(variable),
                *times as u64,
            );
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
                    .ok_or_else(|| self.unsupported("this bit array pattern"))?;
                let right = self
                    .read_size_expression(right)?
                    .ok_or_else(|| self.unsupported("this bit array pattern"))?;
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
                return Err(self.unsupported("this bit array pattern"));
            }
            let bits = self
                .read_size_expression(&action.size)?
                .ok_or_else(|| self.unsupported("this bit array pattern"))?;
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
            BitArrayTest::CatchAllIsBytes { size_so_far } => {
                native_ir::BitsTest::RestIsBytes {
                    offset: Box::new(self.offset_expression(size_so_far)?),
                }
            }
            BitArrayTest::ReadSizeIsNotNegative { size } => {
                let value = self
                    .read_size_expression(size)?
                    .ok_or_else(|| self.unsupported("this bit array pattern"))?;
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
                            if *bit {
                                bytes[index / 8] |= 1 << (7 - index % 8);
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
                    _ => return Err(self.unsupported("this bit array pattern")),
                }
            }
            BitArrayTest::SegmentIsFiniteFloat { read_action } => {
                let bits = self
                    .read_size_expression(&read_action.size)?
                    .ok_or_else(|| self.unsupported("this bit array pattern"))?;
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
                    .ok_or_else(|| self.unsupported("guards outside case expressions"))?;
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
                let body = vec![native_ir::Statement::Expression(
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
                            _ => vec![],
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
            exhaustiveness::RuntimeCheck::Int { int_value } => {
                Ok(match lower_int(int_value) {
                    native_ir::Expression::Int(value) => native_ir::Check::Int(value),
                    _ => native_ir::Check::BigInt(int_value.to_signed_bytes_le()),
                })
            }
            exhaustiveness::RuntimeCheck::Float { float_value } => {
                Ok(native_ir::Check::Float(float_value.value()))
            }
            exhaustiveness::RuntimeCheck::String { value } => Ok(native_ir::Check::String(
                crate::strings::convert_string_escape_chars(value).into(),
            )),
            exhaustiveness::RuntimeCheck::Variant { index, fields, .. } => {
                // Bool and Nil are tagged immediates (True is variant 0 but
                // encodes as 1); every other custom type is a heap record
                // with a variant tag word.
                if subject_type.is_bool() {
                    Ok(native_ir::Check::Immediate(tag_small_int(
                        if *index == 0 { 1 } else { 0 },
                    )))
                } else if subject_type.is_nil() {
                    Ok(native_ir::Check::Immediate(tag_small_int(0)))
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
            exhaustiveness::RuntimeCheck::Tuple { elements, .. } => {
                Ok(native_ir::Check::Always {
                    fields: elements.iter().map(|element| element.id as u32).collect(),
                })
            }
            exhaustiveness::RuntimeCheck::BitArray { test } => self.bit_array_test(test),
            exhaustiveness::RuntimeCheck::EmptyList => {
                Ok(native_ir::Check::Immediate(tag_small_int(0)))
            }
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
                vec![native_ir::Statement::Expression(
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
                        .ok_or_else(|| self.unsupported("this float literal"))?;
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
                        _ => return Err(self.unsupported("this bit array pattern")),
                    }
                }
                exhaustiveness::BoundValue::StringSlice { subject, prefix } => {
                    let prefix: String =
                        crate::strings::convert_string_escape_chars(prefix).into();
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
        super::module(&module).expect("should lower")
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
        assert_eq!(
            body[0],
            native_ir::Statement::Let {
                name: "small".into(),
                value: native_ir::Expression::Int(42),
            }
        );
        assert_eq!(
            body[1],
            native_ir::Statement::Let {
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
        assert_eq!(
            body[0],
            native_ir::Statement::Let {
                name: "x".into(),
                value: native_ir::Expression::Float(1.5),
            }
        );
        assert_eq!(
            body[1],
            native_ir::Statement::Let {
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
        assert_eq!(
            body[0],
            native_ir::Statement::Let {
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
        assert_eq!(
            body[0],
            native_ir::Statement::Let {
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
        assert_eq!(
            body[0],
            native_ir::Statement::Expression(native_ir::Expression::IntBinary {
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
            native_ir::Statement::Expression(native_ir::Expression::IntBinary {
                operator: native_ir::IntOperator::Divide,
                left: Box::new(native_ir::Expression::Int(84)),
                right: Box::new(native_ir::Expression::Int(2)),
            })
        );
        assert_eq!(
            body[2],
            native_ir::Statement::Expression(native_ir::Expression::IntBinary {
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
        assert_eq!(
            body[0],
            native_ir::Statement::Expression(native_ir::Expression::FloatBinary {
                operator: native_ir::FloatOperator::Add,
                left: Box::new(native_ir::Expression::Float(1.5)),
                right: Box::new(native_ir::Expression::Float(2.0)),
            })
        );
        assert_eq!(
            body[1],
            native_ir::Statement::Expression(native_ir::Expression::FloatBinary {
                operator: native_ir::FloatOperator::Divide,
                left: Box::new(native_ir::Expression::Float(10.0)),
                right: Box::new(native_ir::Expression::Float(4.0)),
            })
        );
        assert_eq!(
            body[2],
            native_ir::Statement::Expression(native_ir::Expression::FloatCompare {
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
        assert_eq!(
            body[0],
            native_ir::Statement::Expression(native_ir::Expression::IntCompare {
                operator: native_ir::CompareOperator::LessThan,
                left: Box::new(native_ir::Expression::Int(1)),
                right: Box::new(native_ir::Expression::Int(2)),
            })
        );
        assert_eq!(
            body[1],
            native_ir::Statement::Expression(native_ir::Expression::IntCompare {
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
        let kinds: Vec<_> = body
            .iter()
            .map(|statement| match statement {
                native_ir::Statement::Expression(native_ir::Expression::Equality {
                    kind,
                    negated,
                    ..
                }) => (*kind, *negated),
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
        // `&&` binds tighter than `||`.
        assert_eq!(
            body[0],
            native_ir::Statement::Expression(native_ir::Expression::BoolBinary {
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
        assert_eq!(
            body[0],
            native_ir::Statement::Let {
                name: "yes".into(),
                value: native_ir::Expression::Bool(true),
            }
        );
        assert_eq!(
            body[1],
            native_ir::Statement::Let {
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
        assert_eq!(
            body[0],
            native_ir::Statement::Expression(native_ir::Expression::Case {
                subjects: vec![native_ir::Expression::Int(5)],
                subject_ids: vec![0],
                tree: native_ir::Decision::Switch {
                    var: 0,
                    choices: vec![(
                        native_ir::Check::Int(1),
                        native_ir::Decision::Run {
                            bindings: vec![],
                            body: vec![native_ir::Statement::Expression(
                                native_ir::Expression::Int(10)
                            )],
                        },
                    )],
                    fallback: Box::new(native_ir::Decision::Run {
                        bindings: vec![("n".into(), native_ir::Bound::Variable(0))],
                        body: vec![native_ir::Statement::Expression(
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
        assert_eq!(
            body[0],
            native_ir::Statement::Let {
                name: "pair".into(),
                value: native_ir::Expression::Constructor {
                    tag: 0,
                    arguments: vec![
                        native_ir::Expression::Int(1),
                        native_ir::Expression::Int(2)
                    ],
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
        let native_ir::Statement::Expression(native_ir::Expression::IntBinary {
            right, ..
        }) = &body[2]
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
        assert_eq!(
            body[0],
            native_ir::Statement::Let {
                name: "pair".into(),
                value: native_ir::Expression::Constructor {
                    tag: 0,
                    arguments: vec![
                        native_ir::Expression::Int(1),
                        native_ir::Expression::Constructor {
                            tag: 0,
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
            native_ir::Statement::Expression(native_ir::Expression::FieldAccess {
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
        let native_ir::Statement::Expression(native_ir::Expression::Case { tree, .. }) = &body[0]
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
        let native_ir::Statement::Expression(native_ir::Expression::Case { tree, .. }) = &body[0]
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
        assert_eq!(
            body[0],
            native_ir::Statement::Let {
                name: "rest".into(),
                value: native_ir::Expression::Constructor {
                    tag: 1,
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
            native_ir::Statement::Expression(native_ir::Expression::Constructor {
                tag: 1,
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
        let native_ir::Statement::Expression(native_ir::Expression::Case { tree, .. }) = &body[0]
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
        // The empty list is a tagged immediate; the cons case is the
        // exhaustive fallback whose head and tail arrive untested.
        assert_eq!(choices.len(), 1);
        assert_eq!(choices[0].0, native_ir::Check::Immediate(1));
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
        let native_ir::Statement::Expression(native_ir::Expression::Case {
            subject_ids,
            tree,
            ..
        }) = &body[0]
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
                native_ir::Decision::Switch { var, choices, fallback, .. } => {
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
        // The unchanged field reads from the spread record; the updated one
        // uses the new value.
        assert_eq!(
            body[1],
            native_ir::Statement::Expression(native_ir::Expression::Constructor {
                tag: 0,
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
        assert_eq!(
            body[1],
            native_ir::Statement::Let {
                name: "adder".into(),
                value: native_ir::Expression::Lambda {
                    parameters: vec!["x".into()],
                    body: vec![native_ir::Statement::Expression(
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
            native_ir::Statement::Expression(native_ir::Expression::CallValue {
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
        assert_eq!(
            body[0],
            native_ir::Statement::Let {
                name: "x".into(),
                value: native_ir::Expression::Int(42),
            }
        );
        // A constant referencing another constant inlines it recursively.
        assert_eq!(
            body[1],
            native_ir::Statement::Let {
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
                name: "z".into(),
                value: native_ir::Expression::Constructor {
                    tag: 1,
                    arguments: vec![
                        native_ir::Expression::String("forty".into()),
                        native_ir::Expression::Constructor {
                            tag: 1,
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
        let native_ir::Statement::Expression(native_ir::Expression::Case { tree, .. }) = &body[1]
        else {
            panic!("expected a case");
        };
        fn find_dynamic_read(decision: &native_ir::Decision) -> bool {
            match decision {
                native_ir::Decision::Run { bindings, .. } => {
                    bindings.iter().any(|(_, bound)| match bound {
                        native_ir::Bound::BitsSlice { bits: Some(bits), .. } => {
                            format!("{bits:?}").contains("bit$length")
                        }
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
                native_ir::Decision::Guard {
                    if_false, ..
                } => find_dynamic_read(if_false),
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
        let native_ir::Statement::Expression(native_ir::Expression::Case { tree, .. }) = &body[0]
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
                if_true: vec![native_ir::Statement::Expression(
                    native_ir::Expression::Variable("n".into())
                )],
                if_false: Box::new(native_ir::Decision::Run {
                    bindings: vec![],
                    body: vec![native_ir::Statement::Expression(
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
        let native_ir::Statement::Expression(native_ir::Expression::Case { tree, .. }) = &body[0]
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
        let native_ir::Statement::Expression(native_ir::Expression::Case { tree, .. }) = &body[0]
        else {
            panic!("expected a case expression");
        };
        let native_ir::Decision::Switch { choices, .. } = tree else {
            panic!("expected a switch");
        };
        // The `True` pattern must check for the tagged word 3.
        assert_eq!(choices[0].0, native_ir::Check::Immediate(3));
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
        assert_eq!(
            body[0],
            native_ir::Statement::Let {
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
            native_ir::Statement::Expression(native_ir::Expression::Panic {
                kind: native_ir::PanicKind::Panic,
                message: Some(Box::new(native_ir::Expression::String("boom".into()))),
                function: "main".into(),
                line: 3,
            })
        );
    }
}
