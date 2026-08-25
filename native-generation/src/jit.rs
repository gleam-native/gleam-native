// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! The JIT driver: compiles a set of native IR modules in memory and either
//! runs the project's `main` function or runs its test suite.

use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::default_libcall_names;
use cranelift_codegen::settings::{self, Configurable};

use crate::translate::Translator;

/// The default stack size for the program thread, in megabytes.
pub const DEFAULT_STACK_MEGABYTES: u64 = 1024;

/// An empty JIT module for the host machine with the runtime's symbols
/// registered, ready for the translator.
fn make_jit_module() -> Result<JITModule, String> {
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
    Ok(JITModule::new(jit_builder))
}

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
    let mut jit_module = make_jit_module()?;

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
    let entry_function = unsafe { std::mem::transmute::<usize, extern "C" fn() -> u64>(pointer) };
    native_runtime::run_program_thread(stack_size_megabytes, entry_function)
}

/// JIT-compiles the given modules and runs the given `(module, function)`
/// tests through the runtime's test runner, which reports each outcome and
/// exits the process with 0 if every test passed and 1 otherwise.
pub fn run_tests(
    modules: &[native_ir::Module],
    tests: &[(String, String)],
    arguments: Vec<String>,
    stack_size_megabytes: u64,
) -> Result<(), String> {
    native_runtime::set_start_arguments(arguments);
    let mut jit_module = make_jit_module()?;

    let mut translator = Translator::new(&mut jit_module)?;
    for module in modules {
        translator.declare_module(module)?;
    }
    for module in modules {
        translator.define_module(module)?;
    }

    // One C-convention wrapper per test so the runner can call each
    // compiled test directly.
    let mut wrappers = Vec::with_capacity(tests.len());
    for (index, (module, function)) in tests.iter().enumerate() {
        let id = translator.function_id(module, function).ok_or_else(|| {
            format!(
                "module `{module}` has no `{function}` function compiled for the native target"
            )
        })?;
        let wrapper = translator.define_c_wrapper(&format!("gleam_test_wrapper${index}"), id)?;
        wrappers.push(wrapper);
    }

    native_runtime::set_constructor_names(translator.constructor_names());
    jit_module
        .finalize_definitions()
        .map_err(|error| error.to_string())?;

    let tests: Vec<(String, extern "C" fn() -> u64)> = tests
        .iter()
        .zip(wrappers)
        .map(|((module, function), wrapper)| {
            let pointer = jit_module.get_finalized_function(wrapper) as usize;
            let test_function =
                unsafe { std::mem::transmute::<usize, extern "C" fn() -> u64>(pointer) };
            (format!("{module}.{function}"), test_function)
        })
        .collect();

    // The runner exits the process when the run finishes, so this never
    // returns normally; the JIT module stays alive throughout.
    native_runtime::run_program_thread_with(stack_size_megabytes, move || {
        native_runtime::run_tests(tests);
    })
}
