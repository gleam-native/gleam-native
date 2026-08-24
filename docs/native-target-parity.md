<!--
  SPDX-License-Identifier: Apache-2.0
  SPDX-FileCopyrightText: 2026 The Gleam contributors
-->

# Native target feature parity

This document tracks the Gleam language and tooling features the native
(Cranelift) backend must support to reach parity with the Erlang and
JavaScript targets. The Erlang target is the semantic reference: where the
two existing targets disagree (integer precision, string encoding), native
follows Erlang.

Status legend:

- ✅ implemented
- 🚧 partially implemented
- ❌ not yet implemented

Architecture recap: typed modules are lowered in `compiler-core/src/native.rs`
to the IR in the `native-ir` crate, serialized per module into
`build/{mode}/native/{package}/_gleam_artefacts/*.nir`, then translated to
Cranelift IR and JIT-executed by the `native-generation` crate against the
`native-runtime` crate. Values are tagged 64-bit words (low bit 1 = 63-bit
small integer, low bit 0 = 8-byte-aligned heap pointer). Gleam functions use
Cranelift's `tail` calling convention; runtime and external calls use the
platform C convention.

## Literals and basic values

- ✅ Integer literals within the 63-bit small integer range
- ✅ Integer literals beyond the small range (heap big integers built from
  constant data at run time)
- ✅ `Nil`
- ✅ Float literals (boxed f64 heap objects; heap objects carry no kind
  header yet — the type system separates them until polymorphic equality and
  `echo` exist)
- ✅ String literals (immutable UTF-8 heap strings built from constant data;
  escape sequences processed at lowering time; `println` runtime external)
- ✅ `True` / `False` (tagged small integers 1 and 0; `print_bool` runtime
  external)
- ✅ Number literal notations already accepted by the parser: `0x`, `0o`,
  `0b`, underscores, scientific notation for floats (parsed values arrive
  pre-decoded in the typed AST)

## Operators

- ✅ `+`, `-`, `*` on `Int` (fast path plus overflow promotion to big
  integers)
- ✅ `/`, `%` on `Int` (runtime calls; truncating, division by zero returns
  zero, remainder takes the dividend's sign)
- ✅ Comparison operators on `Int` (`<`, `<=`, `>`, `>=`): tagged words
  compare directly on the fast path; big integer operands go through the
  runtime's three-way comparison
- ✅ Float arithmetic and comparison (`+.`, `-.`, `*.`, `/.`, `<.`, `<=.`,
  `>.`, `>=.`): unboxed register arithmetic between loads, division by zero
  yields 0.0, IEEE ordered comparisons
- ❌ `==` / `!=` structural equality for every type (deep runtime equality,
  the same semantics as Erlang `=:=` on Gleam data / `isEqual` in the JS
  prelude)
- ✅ `<>` string concatenation
- ✅ `&&`, `||` with short-circuit evaluation (pure codegen: the right
  operand's evaluation sits in a conditionally-executed block)
- ❌ Pipe operator `|>` (desugared by the compiler; needs function values)

## Bindings and control flow

- ✅ `let` with a variable pattern
- ✅ Blocks
- ❌ `let` with arbitrary patterns (tuple destructuring etc.)
- ❌ `let assert` (pattern match or panic, with optional `as` message)
- ❌ `case` expressions — lower the `CompiledCase` decision tree attached to
  the typed AST (`compiler-core/src/exhaustiveness.rs`); do not reimplement
  pattern matching. Blocks-with-parameters map directly onto decision nodes
- ❌ Pattern kinds within `case`: literals, variables, discards, tuples,
  constructors, lists (`[x, ..rest]`), strings and string prefixes
  (UTF-8, `StringEncoding::Utf8`), bit arrays, `as` bindings, alternative
  patterns (`a | b`)
- ❌ Guards (`if` clauses), including guard-legal constants and operators
- ❌ `use` expressions (desugared to callbacks; needs closures)
- ❌ `assert` (boolean assertion with structured failure information)

## Functions

- ✅ Module function definitions with parameters
- ✅ Direct calls, including cross-module calls
- ✅ Guaranteed tail calls between Gleam functions (`tail` calling
  convention, in place from day one)
- ❌ Anonymous functions and closures (closure conversion pass; closure
  environment as a heap object)
- ❌ Function values / references (`let f = some_function`) and calls through
  values (`call_indirect`)
- ❌ Function captures (`add(1, _)`)
- ❌ Labelled arguments (compile-time reordering; mostly free once calls are
  complete)
- ❌ Generic functions — handled by the uniform value representation, but
  needs coverage the moment closures and data structures exist
- ❌ Recursion depth: native stacks are finite where the BEAM's are not;
  self- and mutual tail recursion must not grow the stack (covered by tail
  calls), and deep non-tail recursion needs at least a sensible crash

## Data types

- ❌ Tuples (heap objects, positional access `#(1, 2).0`)
- ❌ Lists (cons cells; literals, prepend, pattern support)
- ❌ Custom types: constructor functions, tag words, field access by label,
  record accessors (`wibble.name`)
- ❌ Record updates (`Wibble(..old, name: "new")`)
- ❌ `Result` (prelude custom type; no special casing beyond the prelude)
- ❌ Strings: UTF-8 heap objects, concatenation, comparison, the grapheme
  guarantees the stdlib relies on come from stdlib externals
- ❌ Bit arrays: construction and patterns with size/unit/signedness/
  endianness options, UTF codepoint segments; the per-target feature gates in
  `compiler-core/src/bit_array.rs` need `Native` rules
- ❌ Constants (`const x = …`): compile-time construction into the data
  section, or lazily-initialized globals

## Runtime semantics and services

- ✅ Arbitrary precision integers (representation and overflow promotion;
  only `+` wired so far)
- ✅ `panic`, `todo` with module/function/line metadata baked into the call
  site, lazily-evaluated message expressions, exit code 1
- ❌ `echo` (debug printing of any value; needs runtime value inspection)
- ❌ String formatting of values for panics and `echo` (the native
  equivalent of `gleam/string.inspect`)
- ❌ Reference counting — currently every heap allocation leaks. Gleam data
  is immutable and acyclic so plain RC is sound; insertion as an IR-to-IR
  pass with liveness-based drops, Perceus-style reuse later
- ❌ Stack traces or at least source positions on panics
- ❌ `main` receiving arguments / `argv` access, exit code propagation
  (`gleam run` currently always exits 0 on success)

## Externals and FFI

- ✅ `@external(native, "module", "symbol")` parsing, analysis, formatting,
  metadata round-trip
- ✅ Calling native externals through the platform C convention (symbol
  resolved by name in the JIT; the module string is reserved for a future
  library hint)
- ✅ `@target(native)` conditional compilation
- ❌ Validation rules for native external names (the
  `assert_valid_javascript_external` analogue: function must be a valid
  C symbol)
- ❌ A defined C ABI contract for user-provided externals (value
  representation, who allocates, panic behavior) — required before anyone
  can write FFI outside `native-runtime`
- ❌ Native FFI source files: decide what `native_file_copier` should accept
  for the native target (e.g. `.c`, `.o`, prebuilt static libraries) once
  AOT exists

## Build and tooling

- ✅ `gleam check --target native`
- ✅ `gleam build --target native` producing per-module `.nir` artifacts
  with incremental reuse
- ✅ `gleam run --target native` (in-process JIT)
- 🚧 Unsupported features fail with a named error rather than miscompiling
  (no source location yet in `NativeUnsupportedFeature`)
- ❌ Ahead-of-time compilation: `.nir` → object files via `cranelift-object`,
  `native-runtime` as a staticlib, system linker; `gleam build` emits a real
  executable
- ❌ `gleam test` (needs a gleeunit story or a native test runner)
- ❌ `gleam export` equivalent (shipment = the AOT binary)
- ❌ Debug info (DWARF via `gimli`) — spans must keep flowing through the IR
  so this stays possible
- ❌ Cross-compilation (Cranelift supports foreign ISAs; needs target triple
  plumbing and linking strategy)
- ❌ Re-enable the native row in `test-output`'s all-target macro and add
  language-conformance execution tests (the real proof of parity)

## Ecosystem

- ❌ `gleam_stdlib` support — the largest single work item. The stdlib is
  gated on `@target(erlang)` / `@target(javascript)` and their externals, so
  today it fails analysis under the native target. Requires a fork or
  upstream additions: `@target(native)` branches plus a native externals
  library covering strings, lists, dicts, IO, etc.
- ❌ The prelude types' runtime contract (`Result`, `Bool`, `Order`, …)
  documented for FFI authors, as `prelude.mjs` does for JavaScript
- ❌ Concurrency story — explicitly out of scope for language parity;
  whatever concurrency native offers will arrive as libraries over externals,
  not compiler features

## Explicit non-goals for parity

These are quality work, not parity requirements, and are tracked separately:
unboxed fast paths for floats and ints, Perceus-style RC optimization,
inlining at the native IR level (reusing `compiler-core/src/inline.rs`),
constant caching for big integer literals (currently rebuilt from bytes at
each evaluation), and code size or compile speed tuning.
