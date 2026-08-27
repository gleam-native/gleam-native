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
    /// The closure-invocation thunks the object exports; registered with
    /// the runtime before the program runs so externals can call Gleam
    /// callbacks. Names from `native_generation`'s `invoke_symbol`.
    safe fn gleam_native_invoke_0(closure: u64) -> u64;
    safe fn gleam_native_invoke_1(closure: u64, a: u64) -> u64;
    safe fn gleam_native_invoke_2(closure: u64, a: u64, b: u64) -> u64;
    safe fn gleam_native_invoke_3(closure: u64, a: u64, b: u64, c: u64) -> u64;
    safe fn gleam_native_invoke_4(closure: u64, a: u64, b: u64, c: u64, d: u64) -> u64;
    safe fn gleam_native_invoke_5(closure: u64, a: u64, b: u64, c: u64, d: u64, e: u64) -> u64;
    safe fn gleam_native_invoke_6(
        closure: u64,
        a: u64,
        b: u64,
        c: u64,
        d: u64,
        e: u64,
        f: u64,
    ) -> u64;

    /// The C-convention wrapper around the program's `main` function; the
    /// symbol name is `native_generation::translate::ENTRY_SYMBOL`. Declared
    /// `safe` so it coerces to the plain function pointer
    /// [`native_runtime::start`] expects.
    safe fn gleam_native_main_wrapper() -> u64;

    /// The generated literal-init function; the symbol name is
    /// [`native_runtime::LITERAL_INIT_SYMBOL`]. Builds every interned
    /// literal (marked permanent) before the program runs.
    safe fn gleam_native_literal_init() -> u64;

    /// The program data blob; the symbol name is
    /// [`native_runtime::PROGRAM_DATA_SYMBOL`].
    static gleam_native_program_data: u8;

    /// The frame table blob for panic stack traces; the symbol name is
    /// [`native_runtime::FRAME_TABLE_SYMBOL`].
    static gleam_native_frame_table: u8;
}

/// The executable's C entry point.
///
/// # Safety
///
/// Called by the C startup code with the process's real `argc`/`argv`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: i32, argv: *const *const std::ffi::c_char) -> i32 {
    native_runtime::set_invoker(0, gleam_native_invoke_0 as *const u8);
    native_runtime::set_invoker(1, gleam_native_invoke_1 as *const u8);
    native_runtime::set_invoker(2, gleam_native_invoke_2 as *const u8);
    native_runtime::set_invoker(3, gleam_native_invoke_3 as *const u8);
    native_runtime::set_invoker(4, gleam_native_invoke_4 as *const u8);
    native_runtime::set_invoker(5, gleam_native_invoke_5 as *const u8);
    native_runtime::set_invoker(6, gleam_native_invoke_6 as *const u8);
    unsafe {
        native_runtime::start(
            argc,
            argv,
            &raw const gleam_native_program_data,
            &raw const gleam_native_frame_table,
            gleam_native_literal_init,
            gleam_native_main_wrapper,
        )
    }
}
