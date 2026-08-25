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
small integer, low bit 0 = 8-byte-aligned heap pointer). Every heap object
starts with a header word (kind, and for records the variant tag and field
count), enabling polymorphic deep equality and `echo`, and carries a
reference count in the word before the pointer. Gleam functions use
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
- ✅ `==` / `!=`: type-directed fast paths for `Int` (including big
  integers), `Float` (IEEE), `String` (by contents), `Bool`, and `Nil`;
  every other type (custom types, lists, tuples, generics) uses the
  runtime's structural deep equality, which walks heap object headers.
  Closures compare by identity
- ✅ `<>` string concatenation
- ✅ `&&`, `||` with short-circuit evaluation (pure codegen: the right
  operand's evaluation sits in a conditionally-executed block)
- ✅ Pipe operator `|>` (lowered from the type checker's step assignments)

## Bindings and control flow

- ✅ `let` with a variable pattern
- ✅ Blocks
- ✅ `let` with arbitrary irrefutable patterns (tuple, single-variant
  constructor destructuring); bindings persist in the enclosing scope
- ✅ `let assert` (pattern match or panic with the optional `as` message,
  module/function/line metadata, exit code 1)
- 🚧 `case` expressions — lowered from the `CompiledCase` decision tree as
  planned. Working: multi-subject matches, literal checks (small and big
  integers, floats, strings), `Bool`/`Nil` variant checks (with the
  variant-index-to-tagged-word mapping: `True` is variant 0 but encodes
  as 1), variable and discard patterns, pattern bindings, alternative
  patterns (`a | b`)
- 🚧 Destructuring pattern kinds: constructors with fields, tuples, lists
  (`[x, ..rest]`, nested patterns), and string prefixes
  (`"pre" <> rest`, including `as` bindings, in `case` and `let assert`)
  work, including untested final variants of exhaustive matches, whose
  fields arrive via the fallback. Bit array patterns work within the
  byte-aligned constant-size subset
- 🚧 Guards (`if` clauses): operators, negation, variables, tuple indexing,
  and scalar literal constants work (sharing the expression operator
  lowering). Not yet: field access and module constants in guards
- ✅ `use` expressions (the type checker's desugared callback call is
  lowered directly)
- ✅ `assert` (with the optional `as` message, location metadata, exit
  code 1; reporting the values of the failing expression's operands, as the
  other targets do, needs more site metadata and is future work)

## Functions

- ✅ Module function definitions with parameters
- ✅ Direct calls, including cross-module calls
- ✅ Guaranteed tail calls between Gleam functions (`tail` calling
  convention, in place from day one)
- ✅ Anonymous functions and closures: lambda-lifted at code generation
  time with free-variable analysis for captures; a closure is a heap object
  `[code pointer, captures...]` and indirect calls use the closure calling
  convention (closure first, `tail` convention, `call_indirect`)
- ✅ Function values: module functions and constructors used as values get
  generated adapter wrappers; calls through values work
- ✅ Function captures (`add(1, _)`, via the type checker's desugaring)
- ✅ Labelled arguments (the type checker reorders them)
- ✅ Generic functions through the uniform value representation (exercised
  by generic `map`/`fold` over closures and module functions)
- ❌ Recursion depth: native stacks are finite where the BEAM's are not;
  self- and mutual tail recursion must not grow the stack (covered by tail
  calls), and deep non-tail recursion needs at least a sensible crash

## Data types

- ✅ Tuples (records with tag 0; construction, positional access
  `#(1, 2).0`, destructuring, tuple index in guards)
- ✅ Lists (empty list is a tagged immediate, cons cells are two-field
  records; literals, spread construction `[x, ..rest]`, and patterns
  including nested ones like `[_, _]`)
- ✅ Custom types: heap records (variant tag word + field words),
  constructors (labelled and positional), field access (`wibble.name`),
  destructuring in `case`; `Result` works as a plain custom type;
  constructors as function values. Not yet: equality (needs polymorphic
  deep equality)
- ✅ Record updates (`Wibble(..old, name: "new")`): the type checker's
  desugaring lowers onto existing constructor and field access nodes,
  including non-variable spread expressions
- 🚧 Strings: UTF-8 heap objects with literals, concatenation, and equality
  done; ordering comparison and the grapheme operations the stdlib relies on
  still need runtime support
- 🚧 Bit arrays: byte-aligned, constant-sized segments work — construction
  (int segments with size/unit and endianness, truncation, utf8 strings,
  bit array splices) and patterns (literal int/string matches via the
  compiler's pre-encoded bytes, sized and endian/signed integer reads,
  `rest:bits` captures, in `case` and `let assert`), plus structural
  equality and `echo`. Not yet: dynamic sizes (`size(n)`), non-byte-aligned
  segments, floats, UTF-16/32, codepoints; the per-target gates in
  `compiler-core/src/bit_array.rs` still treat native like Erlang
- ✅ Constants (`const x = …`): inlined at use sites like the Erlang
  target, covering scalars, tuples, lists, records, references to other
  constants and to functions, `<>`, and module-qualified constants in
  guards

## Runtime semantics and services

- ✅ Arbitrary precision integers (representation, overflow promotion, and
  all arithmetic and comparison operators)
- ✅ `panic`, `todo` with module/function/line metadata baked into the call
  site, lazily-evaluated message expressions, exit code 1
- 🚧 `echo`: prints `module:line` (plus the optional `as` message) and the
  value to standard error, returning the value. Scalars and strings print
  exactly (using the static type at the echo site); composites print
  structurally as `@tag(field, ...)` because constructor names do not exist
  at run time — Gleam-pretty output needs constructor-name metadata in
  record headers or static descriptors. `echo` inside a pipeline is not yet
  lowered
- 🚧 Value formatting exists as the runtime's structural `inspect`; a
  `gleam/string.inspect` equivalent with constructor names is future work
- ✅ Reference counting: a count word before every heap object, with
  mechanical local ownership rules inserted during code generation
  (expressions yield owned references, variable reads share, container
  stores consume, scopes release bindings on exit, callers release borrowed
  arguments). Bindings a sequence's final statement does not mention are
  released early, so tail-recursive loops free their garbage every
  iteration (verified: an allocation-churn program runs at baseline memory).
  Destruction is worklist-based, so long lists do not overflow the stack.
  Counts are runtime calls today; Perceus-style reuse and inline fast paths
  are future optimization work
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
