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
Cranelift IR by the `native-generation` crate against the `native-runtime`
crate — either JIT-executed in process (`gleam run`) or compiled to an
object file and linked with the `native-runtime-static` library into a
standalone executable (`gleam export native`). Values are tagged 64-bit words (low bit 1 = 63-bit
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
- ✅ Float literals (boxed f64 heap objects)
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
- ✅ Unary negation in expression position: `!` on `Bool`, `-` on `Int`
  (lowered as `0 - x`, sharing subtraction's overflow promotion at the
  small-integer minimum)
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
- ✅ `case` expressions — lowered from the `CompiledCase` decision tree as
  planned: multi-subject matches, literal checks (small and big integers,
  floats, strings), `Bool`/`Nil` variant checks (with the
  variant-index-to-tagged-word mapping: `True` is variant 0 but encodes
  as 1), variable and discard patterns, pattern bindings, alternative
  patterns (`a | b`)
- ✅ Destructuring pattern kinds: constructors with fields, tuples, lists
  (`[x, ..rest]`, nested patterns), string prefixes (`"pre" <> rest`,
  including `as` bindings, in `case` and `let assert`), and bit arrays —
  including untested final variants of exhaustive matches, whose fields
  arrive via the fallback
- ✅ Guards (`if` clauses): operators, negation, variables, tuple indexing,
  record field access (including chained access), and constants of every
  kind — scalar and composite literals and module constants, local and
  module-qualified (sharing the expression operator and constant lowering)
- ✅ `use` expressions (the type checker's desugared callback call is
  lowered directly)
- ✅ `assert` (with the optional `as` message, location metadata, exit
  code 1; reporting the values of the failing expression's operands, as the
  other targets do, needs more site metadata and is future work)

## Functions

- ✅ Module function definitions with parameters
- ✅ Direct calls, including cross-module calls
- ✅ Module-qualified access (`module.function(..)`, `module.constant`,
  `module.Constructor`, and qualified functions or constructors used as
  values), sharing the unqualified lowering paths
- ✅ Guaranteed tail calls between Gleam functions: calls in tail position
  (including through blocks and case clauses) compile to genuine
  `return_call`/`return_call_indirect` transfers, so self- and mutual tail
  recursion run in constant stack space. This required flipping argument
  ownership — Gleam callees own their arguments (externals still borrow) —
  and tail paths release remaining scope bindings and case subjects between
  argument evaluation and the transfer
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
- ✅ Recursion depth: the program runs on a dedicated thread with a 1 GiB
  stack by default, configurable per project with `stack_size_megabytes`
  under `[native]` in `gleam.toml`; deep non-tail recursion that exhausts
  it is caught by a `sigaltstack` handler that reports
  `runtime error: stack overflow` and exits with code 1 instead of
  crashing raw

## Data types

- ✅ Tuples (records with tag 0; construction, positional access
  `#(1, 2).0`, destructuring, tuple index in guards)
- ✅ Lists (empty list is a tagged immediate, cons cells are two-field
  records; literals, spread construction `[x, ..rest]`, and patterns
  including nested ones like `[_, _]`)
- ✅ Custom types: heap records (variant tag word + field words),
  constructors (labelled and positional), field access (`wibble.name`),
  destructuring in `case`; `Result` works as a plain custom type;
  constructors as function values; structural equality
- ✅ Record updates (`Wibble(..old, name: "new")`): the type checker's
  desugaring lowers onto existing constructor and field access nodes,
  including non-variable spread expressions and module-qualified
  constructors (`module.Wibble(..old, …)`)
- ✅ Strings: UTF-8 heap objects with literals, concatenation, equality,
  and prefix patterns; plus the runtime external suite a standard library
  port binds to — grapheme-aware length/reverse/slice/pop_grapheme/
  graphemes (via unicode-segmentation), three-way comparison (bytewise
  UTF-8, which is code point order — exactly Erlang's binary comparison),
  case mapping, contains/ends_with, trim family, replace, split, code point
  conversions both ways, and int/float rendering. Runtime-built `Result`,
  tuple, and list values follow the standard layouts and destructure in
  ordinary Gleam patterns
- ✅ Bit arrays: bit-granular storage with Erlang semantics — construction
  and patterns support arbitrary (unaligned) constant and dynamic sizes,
  big/little/native endianness, signedness, truncation, wide segments via
  big integers, floats at 16 (half precision), 32, and 64 bits (only
  finite floats match, via the finiteness test), UTF-8/16/32 strings
  (literal pattern matches use the compiler's pre-encoded bytes),
  codepoint construction segments, whole and sized bit array splices,
  `bytes-size(n)` payloads, `rest:bits` captures, sizes referring to
  earlier segments and outer variables with arithmetic, structural
  equality, and `echo` (partial trailing bits print as `n:size(b)`).
  Sizes are validated at run time. Codepoint and string *variable* reads in
  patterns are rejected at code generation — exactly as on the JavaScript
  target
- ✅ Constants (`const x = …`): inlined at use sites like the Erlang
  target, covering scalars, tuples, lists, records, bit arrays,
  references to other constants, to functions, and to constructors (a
  constructor with fields referenced by a constant is the constructor as
  a function value), `<>`, and module-qualified constants in guards

## Runtime semantics and services

- ✅ Arbitrary precision integers (representation, overflow promotion, and
  all arithmetic and comparison operators)
- ✅ `panic`, `todo` with module/function/line metadata baked into the call
  site, lazily-evaluated message expressions, exit code 1
- ✅ `echo`: prints `module:line` (plus the optional `as` message) and the
  value to standard error, returning the value — in expression position and
  inside pipelines (`|> echo |>` prints the mid-pipeline value). Output is
  Gleam-pretty: record headers carry a display-only constructor id (masked
  out of variant checks and equality) resolved through a name registry the
  JIT hands the runtime, so custom types print as `Person("Ada", 36)`,
  lists as `[1, 2]`, tuples as `#(...)`, and `Ok`/`Error` by name. Scalars
  use the static type at the echo site. One representational limit remains:
  `Bool`, `Nil`, and the empty list are bare tagged integers, so *nested*
  inside structures they print as their integer encoding
- ✅ Value formatting: the runtime's `inspect` now renders constructor
  names, ready to back a `gleam/string.inspect` external
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
- ✅ Command line arguments and exit codes: `gleam run -- args...` passes
  the arguments through to the program, readable as a `List(String)` via
  the `gleam_native_start_arguments` external; the `gleam_native_exit`
  external ends the program with a chosen code, and a normally-completing
  `main` exits 0 (runtime failures exit 1)

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
- ✅ Ahead-of-time compilation: `gleam export native` builds the project in
  production mode, compiles the `.nir` modules to a host object file via
  `cranelift-object` (position-independent, optimized), and links it with
  the system C compiler driver against the `native-runtime-static` library
  — the runtime as a `staticlib` whose C `main` decodes an embedded program
  data blob (configured stack size, interned constructor names) before
  running the program thread. The executable lands in the project root,
  named after the package. The static library is looked up next to the
  `gleam` binary (`libnative_runtime_static.a`), overridable with
  `GLEAM_NATIVE_RUNTIME_LIB`
- ✅ `gleam test` — a built-in runner (gleeunit needs the standard library,
  which native does not support yet): the harness discovers public
  zero-argument `*_test` functions in the root package's `test`-directory
  modules — gleeunit's convention, so suites stay compatible — compiles
  them with a C-convention wrapper each, and runs them in order on the
  program thread. A failing test's panic report is printed and the run
  resumes with the next test: in test mode `gleam_native_panic` re-enters
  the runner loop instead of exiting, abandoning the failed test's frames
  and allocations (the process exits at the end of the run, so the leak is
  harmless). Output is a `PASS`/`FAIL` line per test plus a
  `Ran N tests, F failures` summary; exit code 1 if anything failed. A
  stack overflow still aborts the whole run, and `gleeunit` itself remains
  unsupported
- ❌ Debug info (DWARF via `gimli`) — spans must keep flowing through the IR
  so this stays possible
- ✅ Cross-compilation: `gleam export native --platform <name>` with the
  curated platforms `linux-arm64` / `linux-x64` (musl, fully static),
  `linux-arm64-gnu` / `linux-x64-gnu` (dynamic glibc), and `macos-arm64` /
  `macos-x64`. Cranelift compiles the object for the platform's triple
  (baseline CPU features); the runtime library is resolved per platform —
  `--runtime-lib` flag, `GLEAM_NATIVE_RUNTIME_LIB`,
  `libnative_runtime_static-<triple>.a` beside the gleam binary, or cargo's
  `target/<triple>/<profile>/` development layout (built with
  `cargo build -p native-runtime-static --target <triple>`). Linking uses
  the host `cc` where it can (its own platform; both macOS architectures
  via `-arch`), otherwise `zig cc -target` when zig is on PATH (with zig's
  bundled libunwind supplying the `_Unwind_*` symbols Rust references), and
  `--linker` / `GLEAM_NATIVE_LINKER` overrides everything. Without a usable
  linker the object file is kept and the exact manual link command is
  printed. All platforms are 64-bit little-endian Unix; Windows needs a
  runtime port (signal-based stack overflow handling) first
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
