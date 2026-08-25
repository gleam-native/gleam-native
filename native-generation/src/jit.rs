// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! The JIT driver: compiles a set of native IR modules in memory and runs the
//! project's `main` function.

use cranelift_codegen::settings::{self, Configurable};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::default_libcall_names;

use crate::translate::Translator;

/// JIT-compiles the given modules and calls `main` in `main_module`, with
/// the given command line arguments available to the program.
pub fn run(
    modules: &[native_ir::Module],
    main_module: &str,
    arguments: Vec<String>,
) -> Result<(), String> {
    native_runtime::set_start_arguments(arguments);
    let mut flag_builder = settings::builder();
    flag_builder
        .set("use_colocated_libcalls", "false")
        .expect("valid flag");
    flag_builder.set("is_pic", "false").expect("valid flag");
    let isa = cranelift_native::builder()
        .map_err(|error| format!("host machine is not supported: {error}"))?
        .finish(settings::Flags::new(flag_builder))
        .map_err(|error| error.to_string())?;

    let mut jit_builder = JITBuilder::with_isa(isa, default_libcall_names());
    for (name, pointer) in native_runtime::symbols() {
        let _ = jit_builder.symbol(name, pointer);
    }
    let mut jit_module = JITModule::new(jit_builder);

    let mut translator = Translator::new(&mut jit_module)?;
    for module in modules {
        translator.declare_module(module)?;
    }
    for module in modules {
        translator.define_module(module)?;
    }

    native_runtime::set_constructor_names(translator.constructor_names());
    let main = translator.function_id(main_module, "main").ok_or_else(|| {
        format!("module `{main_module}` has no `main` function compiled for the native target")
    })?;
    let entry = translator.define_entry_wrapper(main)?;

    jit_module
        .finalize_definitions()
        .map_err(|error| error.to_string())?;

    let pointer = jit_module.get_finalized_function(entry);
    let entry_function =
        unsafe { std::mem::transmute::<*const u8, extern "C" fn() -> u64>(pointer) };
    let _ = entry_function();
    Ok(())
}
