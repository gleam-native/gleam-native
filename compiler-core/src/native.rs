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
    let lowerer = Lowerer {
        module_name: module.name.clone(),
    };
    let mut functions = Vec::new();

    // Imports and type aliases generate no code.
    if !module.definitions.custom_types.is_empty() {
        return Err(lowerer.unsupported("custom types"));
    }
    if !module.definitions.constants.is_empty() {
        return Err(lowerer.unsupported("module constants"));
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

struct Lowerer {
    module_name: EcoString,
}

impl Lowerer {
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
                } if name == "Nil" && module == PRELUDE_MODULE_NAME => {
                    Ok(native_ir::Expression::Nil)
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
                operator: BinOp::AddInt,
                left,
                right,
                ..
            } => Ok(native_ir::Expression::IntAdd(
                Box::new(self.expression(left)?),
                Box::new(self.expression(right)?),
            )),
            TypedExpr::BinOp { operator, .. } => {
                Err(self.unsupported(&format!("the `{}` operator", operator.name())))
            }

            _ => Err(self.unsupported("this kind of expression")),
        }
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
}
