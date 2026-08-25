// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2025 The Gleam contributors

/// mimalloc serves both the compiler and, through the JIT, running native
/// target programs, whose per-object heap values make allocator speed part
/// of language performance.
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub fn main() {
    gleam_cli::main();
}
