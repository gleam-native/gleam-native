// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! Native code generation for the Gleam native target, built on Cranelift.
//!
//! This crate must not be depended upon by `compiler-core`, which has to keep
//! compiling to WebAssembly for `compiler-wasm`.

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
}
