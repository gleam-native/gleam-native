// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! The ahead-of-time driver: compiles a set of native IR modules to an
//! object file for the host machine or a cross-compilation target, ready to
//! be linked with the `native-runtime-static` library into an executable.
//!
//! The object exports the same entry wrapper the JIT calls, plus a program
//! data blob holding what the JIT passes to the runtime in process: the
//! configured stack size and the interned constructor names. The static
//! library's C `main` decodes the blob and runs the wrapper.

use std::str::FromStr;

use cranelift_codegen::isa;
use cranelift_codegen::settings::{self, Configurable};
use cranelift_module::{DataDescription, Linkage, Module, default_libcall_names};
use cranelift_object::{ObjectBuilder, ObjectModule};
use target_lexicon::Triple;

use crate::translate::Translator;

/// Compiles the given modules to an object file, with `main` in
/// `main_module` reachable through the exported entry wrapper. The object
/// targets the given triple, or the host machine when `triple` is `None`.
/// Returns the object file's bytes.
pub fn compile(
    modules: &[native_ir::Module],
    main_module: &str,
    stack_size_megabytes: u64,
    triple: Option<&str>,
) -> Result<Vec<u8>, String> {
    let mut flag_builder = settings::builder();
    // Position-independent code, so the executable can be linked PIE (the
    // default on modern macOS and Linux toolchains).
    flag_builder.set("is_pic", "true").expect("valid flag");
    flag_builder
        .set("opt_level", "speed")
        .expect("valid flag");
    let flags = settings::Flags::new(flag_builder);
    let isa = match triple {
        // The host builder detects the machine's CPU features; a foreign
        // triple gets the architecture's baseline features.
        None => cranelift_native::builder()
            .map_err(|error| format!("host machine is not supported: {error}"))?
            .finish(flags)
            .map_err(|error| error.to_string())?,
        Some(triple) => {
            let triple = Triple::from_str(triple)
                .map_err(|error| format!("invalid target triple `{triple}`: {error}"))?;
            isa::lookup(triple.clone())
                .map_err(|error| format!("target `{triple}` is not supported: {error}"))?
                .finish(flags)
                .map_err(|error| error.to_string())?
        }
    };

    let builder = ObjectBuilder::new(isa, "gleam", default_libcall_names())
        .map_err(|error| error.to_string())?;
    let mut object_module = ObjectModule::new(builder);

    let mut translator = Translator::new(&mut object_module)?;
    for module in modules {
        translator.declare_module(module)?;
    }
    for module in modules {
        translator.define_module(module)?;
    }

    let main = translator.function_id(main_module, "main").ok_or_else(|| {
        format!("module `{main_module}` has no `main` function compiled for the native target")
    })?;
    let _ = translator.define_entry_wrapper(main)?;

    let program_data = native_runtime::encode_program_data(
        stack_size_megabytes,
        &translator.constructor_names(),
    );
    let data = object_module
        .declare_data(native_runtime::PROGRAM_DATA_SYMBOL, Linkage::Export, false, false)
        .map_err(|error| error.to_string())?;
    let mut description = DataDescription::new();
    description.define(program_data.into_boxed_slice());
    object_module
        .define_data(data, &description)
        .map_err(|error| error.to_string())?;

    object_module
        .finish()
        .emit()
        .map_err(|error| error.to_string())
}
