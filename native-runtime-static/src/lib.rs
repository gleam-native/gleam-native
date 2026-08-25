// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! The static library that `gleam export native` links into ahead-of-time
//! compiled executables. It pulls in the whole `native-runtime` (whose
//! `#[no_mangle]` symbols the generated code references) and provides the C
//! `main`, which hands over to [`native_runtime::start`] with the two
//! symbols the generated object file exports: the entry wrapper around the
//! program's `main` function and the program data blob.
//!
//! This must never be linked into the `gleam` binary itself: its `main`
//! would clash. It is compiled only as a standalone `staticlib` artifact
//! that the `gleam` binary locates at export time.

// Linked for its `#[no_mangle]` symbols alone: the standard library's
// `@external(native, ...)` implementations, which generated code references
// by name.
use native_runtime_stdlib as _;

/// Gleam programs allocate a heap object per value, so compiled executables
/// get a fast allocator behind the runtime's pool rather than the system
/// one. This mirrors the `gleam` binary, keeping `gleam run` and exported
/// executables at the same performance.
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

unsafe extern "C" {
    /// The C-convention wrapper around the program's `main` function; the
    /// symbol name is `native_generation::translate::ENTRY_SYMBOL`. Declared
    /// `safe` so it coerces to the plain function pointer
    /// [`native_runtime::start`] expects.
    safe fn gleam_native_main_wrapper() -> u64;

    /// The program data blob; the symbol name is
    /// [`native_runtime::PROGRAM_DATA_SYMBOL`].
    static gleam_native_program_data: u8;
}

/// The executable's C entry point.
///
/// # Safety
///
/// Called by the C startup code with the process's real `argc`/`argv`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: i32, argv: *const *const std::ffi::c_char) -> i32 {
    unsafe {
        native_runtime::start(
            argc,
            argv,
            &raw const gleam_native_program_data,
            gleam_native_main_wrapper,
        )
    }
}
