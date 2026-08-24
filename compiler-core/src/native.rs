// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! Lowering of typed Gleam modules to the native intermediate representation.
//!
//! This is a deliberately small subset of the language so far; anything
//! outside it produces an error naming the unsupported feature rather than
//! generating wrong code. The subset grows with the native backend.

use ecow::EcoString;
use num_traits::ToPrimitive;

use crate::{
    ast::{BinOp, Pattern, Statement, TypedExpr, TypedModule, TypedStatement},
    error::Error,
    type_::{PRELUDE_MODULE_NAME, ValueConstructorVariant},
};

pub fn module(module: &TypedModule) -> Result<native_ir::Module, Error> {
    let mut functions = Vec::new();

    // Imports and type aliases generate no code.
    if !module.definitions.custom_types.is_empty() {
        return Err(Error::NativeUnsupportedFeature {
            module: module.name.clone(),
            feature: "custom types".into(),
        });
    }
    if !module.definitions.constants.is_empty() {
        return Err(Error::NativeUnsupportedFeature {
            module: module.name.clone(),
            feature: "module constants".into(),
        });
    }

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
                let name = match &assignment.pattern {
                    Pattern::Variable { name, .. } => name.clone(),
                    _ => return Err(self.unsupported("pattern matching in assignments")),
                };
                Ok(native_ir::Statement::Let {
                    name: name.into(),
                    value: self.expression(&assignment.value)?,
                })
            }

            Statement::Use(_) => Err(self.unsupported("use expressions")),
            Statement::Assert(_) => Err(self.unsupported("assert")),
        }
    }

    fn expression(&self, expression: &TypedExpr) -> Result<native_ir::Expression, Error> {
        match expression {
            TypedExpr::Int { int_value, .. } => {
                match int_value
                    .to_i64()
                    .filter(|value| ((i64::MIN >> 1)..=(i64::MAX >> 1)).contains(value))
                {
                    Some(value) => Ok(native_ir::Expression::Int(value)),
                    None => Ok(native_ir::Expression::BigInt(
                        int_value.to_signed_bytes_le(),
                    )),
                }
            }

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
                ValueConstructorVariant::ModuleFn { .. } => {
                    Err(self.unsupported("function values"))
                }
                _ => Err(self.unsupported("this kind of value")),
            },

            TypedExpr::Call {
                fun, arguments, ..
            } => {
                let (module, function) = match fun.as_ref() {
                    TypedExpr::Var { constructor, .. } => match &constructor.variant {
                        ValueConstructorVariant::ModuleFn { module, name, .. } => {
                            (module.clone(), name.clone())
                        }
                        _ => return Err(self.unsupported("calling non-function values")),
                    },
                    _ => return Err(self.unsupported("calling expressions")),
                };
                let arguments = arguments
                    .iter()
                    .map(|argument| self.expression(&argument.value))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(native_ir::Expression::Call {
                    module: module.into(),
                    function: function.into(),
                    arguments,
                })
            }

            TypedExpr::BinOp {
                operator:
                    operator @ (BinOp::AddInt
                    | BinOp::SubInt
                    | BinOp::MultInt
                    | BinOp::DivInt
                    | BinOp::RemainderInt),
                left,
                right,
                ..
            } => Ok(native_ir::Expression::IntBinary {
                operator: match operator {
                    BinOp::AddInt => native_ir::IntOperator::Add,
                    BinOp::SubInt => native_ir::IntOperator::Subtract,
                    BinOp::MultInt => native_ir::IntOperator::Multiply,
                    BinOp::DivInt => native_ir::IntOperator::Divide,
                    _ => native_ir::IntOperator::Remainder,
                },
                left: Box::new(self.expression(left)?),
                right: Box::new(self.expression(right)?),
            }),
            TypedExpr::BinOp {
                operator:
                    operator @ (BinOp::LtInt | BinOp::LtEqInt | BinOp::GtInt | BinOp::GtEqInt),
                left,
                right,
                ..
            } => Ok(native_ir::Expression::IntCompare {
                operator: match operator {
                    BinOp::LtInt => native_ir::CompareOperator::LessThan,
                    BinOp::LtEqInt => native_ir::CompareOperator::LessThanOrEqual,
                    BinOp::GtInt => native_ir::CompareOperator::GreaterThan,
                    _ => native_ir::CompareOperator::GreaterThanOrEqual,
                },
                left: Box::new(self.expression(left)?),
                right: Box::new(self.expression(right)?),
            }),
            TypedExpr::BinOp {
                operator:
                    operator @ (BinOp::AddFloat
                    | BinOp::SubFloat
                    | BinOp::MultFloat
                    | BinOp::DivFloat),
                left,
                right,
                ..
            } => Ok(native_ir::Expression::FloatBinary {
                operator: match operator {
                    BinOp::AddFloat => native_ir::FloatOperator::Add,
                    BinOp::SubFloat => native_ir::FloatOperator::Subtract,
                    BinOp::MultFloat => native_ir::FloatOperator::Multiply,
                    _ => native_ir::FloatOperator::Divide,
                },
                left: Box::new(self.expression(left)?),
                right: Box::new(self.expression(right)?),
            }),
            TypedExpr::BinOp {
                operator:
                    operator @ (BinOp::LtFloat
                    | BinOp::LtEqFloat
                    | BinOp::GtFloat
                    | BinOp::GtEqFloat),
                left,
                right,
                ..
            } => Ok(native_ir::Expression::FloatCompare {
                operator: match operator {
                    BinOp::LtFloat => native_ir::CompareOperator::LessThan,
                    BinOp::LtEqFloat => native_ir::CompareOperator::LessThanOrEqual,
                    BinOp::GtFloat => native_ir::CompareOperator::GreaterThan,
                    _ => native_ir::CompareOperator::GreaterThanOrEqual,
                },
                left: Box::new(self.expression(left)?),
                right: Box::new(self.expression(right)?),
            }),
            TypedExpr::BinOp {
                operator: BinOp::Concatenate,
                left,
                right,
                ..
            } => Ok(native_ir::Expression::StringConcat(
                Box::new(self.expression(left)?),
                Box::new(self.expression(right)?),
            )),
            TypedExpr::BinOp { operator, .. } => {
                Err(self.unsupported(&format!("the `{}` operator", operator.name())))
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
