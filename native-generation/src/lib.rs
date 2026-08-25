// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! Native code generation for the Gleam native target, built on Cranelift.
//!
//! This crate must not be depended upon by `compiler-core`, which has to keep
//! compiling to WebAssembly for `compiler-wasm`. It consumes the serialized
//! native IR that `compiler-core` writes at build time (see the `native-ir`
//! crate) and either JIT-compiles and runs it ([`jit`]) or, in the future,
//! emits object files for ahead-of-time compilation.

pub mod jit;
pub mod translate;

#[cfg(test)]
mod tests {
    use cranelift_codegen::ir::{AbiParam, InstBuilder, types};
    use cranelift_codegen::settings::{self, Configurable};
    use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
    use cranelift_jit::{JITBuilder, JITModule};
    use cranelift_module::{Linkage, Module, default_libcall_names};

    /// JIT-compile and run a function adding two i64s, proving the Cranelift
    /// toolchain works end to end on the host.
    #[test]
    fn jit_add_two_integers() {
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "false").unwrap();
        let isa_builder = cranelift_native::builder().unwrap();
        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .unwrap();
        let mut module = JITModule::new(JITBuilder::with_isa(isa, default_libcall_names()));

        let frontend_config = module.target_config();
        let mut context = module.make_context();
        context
            .func
            .signature
            .params
            .push(AbiParam::new(types::I64));
        context
            .func
            .signature
            .params
            .push(AbiParam::new(types::I64));
        context
            .func
            .signature
            .returns
            .push(AbiParam::new(types::I64));

        let mut builder_context = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut context.func, &mut builder_context);
        let block = builder.create_block();
        builder.append_block_params_for_function_params(block);
        builder.switch_to_block(block);
        builder.seal_block(block);
        let left = builder.block_params(block)[0];
        let right = builder.block_params(block)[1];
        let sum = builder.ins().iadd(left, right);
        builder.ins().return_(&[sum]);
        builder.finalize(frontend_config);

        let function = module
            .declare_function("add", Linkage::Export, &context.func.signature)
            .unwrap();
        module.define_function(function, &mut context).unwrap();
        module.clear_context(&mut context);
        module.finalize_definitions().unwrap();

        let pointer = module.get_finalized_function(function);
        let add = unsafe { std::mem::transmute::<*const u8, fn(i64, i64) -> i64>(pointer) };
        assert_eq!(add(20, 22), 42);
    }

    /// End-to-end: build a tiny two-module program in native IR, JIT it, and
    /// run `main`. Uses the runtime's `print_int` external and an addition
    /// that stays on the fast path plus one that overflows to a big integer.
    #[test]
    fn jit_runs_main_of_a_program() {
        let dependency = native_ir::Module {
            name: "wibble/wobble".into(),
            functions: vec![
                native_ir::Function::External {
                    name: "print_int".into(),
                    arity: 1,
                    symbol: "print_int".into(),
                },
                native_ir::Function::Defined {
                    name: "add".into(),
                    parameters: vec!["x".into(), "y".into()],
                    body: vec![native_ir::Statement::Expression(
                        native_ir::Expression::IntBinary {
                            operator: native_ir::IntOperator::Add,
                            left: Box::new(native_ir::Expression::Variable("x".into())),
                            right: Box::new(native_ir::Expression::Variable("y".into())),
                        },
                    )],
                },
            ],
        };
        let root = native_ir::Module {
            name: "app".into(),
            functions: vec![native_ir::Function::Defined {
                name: "main".into(),
                parameters: vec![],
                body: vec![
                    native_ir::Statement::Let {
                        name: "total".into(),
                        value: native_ir::Expression::Call {
                            module: "wibble/wobble".into(),
                            function: "add".into(),
                            arguments: vec![
                                native_ir::Expression::Int(40),
                                native_ir::Expression::Int(2),
                            ],
                        },
                    },
                    native_ir::Statement::Let {
                        name: "huge".into(),
                        value: native_ir::Expression::IntBinary {
                            operator: native_ir::IntOperator::Add,
                            left: Box::new(native_ir::Expression::Int(i64::MAX >> 1)),
                            right: Box::new(native_ir::Expression::Variable("total".into())),
                        },
                    },
                    native_ir::Statement::Expression(native_ir::Expression::Call {
                        module: "wibble/wobble".into(),
                        function: "print_int".into(),
                        arguments: vec![native_ir::Expression::Variable("huge".into())],
                    }),
                ],
            }],
        };

        crate::jit::run(&[dependency, root], "app", Vec::new(), crate::jit::DEFAULT_STACK_MEGABYTES).unwrap();
    }

    /// A big integer literal flows from the data section through the runtime
    /// constructor and the addition slow path.
    #[test]
    fn jit_runs_big_integer_literals() {
        let module = native_ir::Module {
            name: "app".into(),
            functions: vec![native_ir::Function::Defined {
                name: "main".into(),
                parameters: vec![],
                body: vec![native_ir::Statement::Expression(
                    native_ir::Expression::IntBinary {
                        operator: native_ir::IntOperator::Add,
                        // 2^70, as signed little-endian bytes.
                        left: Box::new(native_ir::Expression::BigInt(vec![
                            0, 0, 0, 0, 0, 0, 0, 0, 64,
                        ])),
                        right: Box::new(native_ir::Expression::Int(1)),
                    },
                )],
            }],
        };
        crate::jit::run(&[module], "app", Vec::new(), crate::jit::DEFAULT_STACK_MEGABYTES).unwrap();
    }

    /// A float literal is boxed via the runtime constructor and can be
    /// passed to an external.
    #[test]
    fn jit_runs_float_literals() {
        let module = native_ir::Module {
            name: "app".into(),
            functions: vec![
                native_ir::Function::External {
                    name: "print_float".into(),
                    arity: 1,
                    symbol: "print_float".into(),
                },
                native_ir::Function::Defined {
                    name: "main".into(),
                    parameters: vec![],
                    body: vec![native_ir::Statement::Expression(
                        native_ir::Expression::Call {
                            module: "app".into(),
                            function: "print_float".into(),
                            arguments: vec![native_ir::Expression::Float(1.5)],
                        },
                    )],
                },
            ],
        };
        crate::jit::run(&[module], "app", Vec::new(), crate::jit::DEFAULT_STACK_MEGABYTES).unwrap();
    }

    /// A case expression's decision tree compiles and runs: a boolean switch
    /// with an immediate check and a fallback.
    #[test]
    fn jit_runs_case_expressions() {
        let module = native_ir::Module {
            name: "app".into(),
            functions: vec![native_ir::Function::Defined {
                name: "main".into(),
                parameters: vec![],
                body: vec![native_ir::Statement::Expression(
                    native_ir::Expression::Case {
                        subjects: vec![native_ir::Expression::Bool(true)],
                        subject_ids: vec![0],
                        tree: native_ir::Decision::Switch {
                            var: 0,
                            choices: vec![(
                                native_ir::Check::Immediate(3),
                                native_ir::Decision::Run {
                                    bindings: vec![(
                                        "x".into(),
                                        native_ir::Bound::Variable(0),
                                    )],
                                    body: vec![native_ir::Statement::Expression(
                                        native_ir::Expression::Variable("x".into()),
                                    )],
                                },
                            )],
                            fallback: Box::new(native_ir::Decision::Run {
                                bindings: vec![],
                                body: vec![native_ir::Statement::Expression(
                                    native_ir::Expression::Int(0),
                                )],
                            }),
                            fallback_fields: vec![],
                        },
                    },
                )],
            }],
        };
        crate::jit::run(&[module], "app", Vec::new(), crate::jit::DEFAULT_STACK_MEGABYTES).unwrap();
    }

    /// A panic expression translates and compiles; `main` must not call it,
    /// as the runtime panic aborts the whole process, test runner included.
    #[test]
    fn jit_compiles_panic_expressions() {
        let module = native_ir::Module {
            name: "app".into(),
            functions: vec![
                native_ir::Function::Defined {
                    name: "explode".into(),
                    parameters: vec![],
                    body: vec![native_ir::Statement::Expression(
                        native_ir::Expression::Panic {
                            kind: native_ir::PanicKind::Panic,
                            message: Some(Box::new(native_ir::Expression::String(
                                "boom".into(),
                            ))),
                            function: "explode".into(),
                            line: 2,
                        },
                    )],
                },
                native_ir::Function::Defined {
                    name: "main".into(),
                    parameters: vec![],
                    body: vec![native_ir::Statement::Expression(native_ir::Expression::Nil)],
                },
            ],
        };
        crate::jit::run(&[module], "app", Vec::new(), crate::jit::DEFAULT_STACK_MEGABYTES).unwrap();
    }

    /// A string literal is built from constant data and can be passed to the
    /// runtime's `println` external.
    #[test]
    fn jit_runs_string_literals() {
        let module = native_ir::Module {
            name: "app".into(),
            functions: vec![
                native_ir::Function::External {
                    name: "println".into(),
                    arity: 1,
                    symbol: "println".into(),
                },
                native_ir::Function::Defined {
                    name: "main".into(),
                    parameters: vec![],
                    body: vec![native_ir::Statement::Expression(
                        native_ir::Expression::Call {
                            module: "app".into(),
                            function: "println".into(),
                            arguments: vec![native_ir::Expression::StringConcat(
                                Box::new(native_ir::Expression::String(
                                    "Hello from ".into(),
                                )),
                                Box::new(native_ir::Expression::String(
                                    "the JIT! 🌍".into(),
                                )),
                            )],
                        },
                    )],
                },
            ],
        };
        crate::jit::run(&[module], "app", Vec::new(), crate::jit::DEFAULT_STACK_MEGABYTES).unwrap();
    }
}
