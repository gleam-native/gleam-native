// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! The JIT driver: compiles a set of native IR modules in memory and runs the
//! project's `main` function.

use cranelift_codegen::settings::{self, Configurable};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::default_libcall_names;

use crate::translate::Translator;

/// The default stack size for the program thread, in megabytes.
pub const DEFAULT_STACK_MEGABYTES: u64 = 1024;

/// JIT-compiles the given modules and calls `main` in `main_module`, with
/// the given command line arguments available to the program and the given
/// stack size (in megabytes) for its thread.
pub fn run(
    modules: &[native_ir::Module],
    main_module: &str,
    arguments: Vec<String>,
    stack_size_megabytes: u64,
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

    let pointer = jit_module.get_finalized_function(entry) as usize;
    // Run on a dedicated thread with a large stack; tail calls run in
    // constant space, and deep non-tail recursion gets generous room before
    // the overflow handler reports it. A minimum of one megabyte keeps a
    // misconfigured project able to reach `main` at all.
    let stack_size = usize::try_from(stack_size_megabytes.max(1))
        .unwrap_or(usize::MAX)
        .saturating_mul(1024 * 1024);
    std::thread::Builder::new()
        .name("gleam-main".into())
        .stack_size(stack_size)
        .spawn(move || {
            native_runtime::install_stack_overflow_handler();
            let entry_function =
                unsafe { std::mem::transmute::<usize, extern "C" fn() -> u64>(pointer) };
            let _ = entry_function();
        })
        .map_err(|error| format!("could not start the program thread: {error}"))?
        .join()
        .map_err(|_| "the program crashed".to_string())?;
    Ok(())
}
