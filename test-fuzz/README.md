<!--
  SPDX-License-Identifier: Apache-2.0
  SPDX-FileCopyrightText: 2026 The Gleam contributors
-->

# Differential fuzzer for the native target

Generates seeded, type-directed, total Gleam programs (language features
only, no standard library) and runs each on Erlang (the semantic
reference), the native JIT, and the native JIT under `GLEAM_DEBUG_RC=1`
(checked reference counting). The standard output, standard error (with
build-progress lines filtered), and exit codes of all three runs must be
identical; any mismatch, crash, or timeout is saved as a reproducer under
`test-fuzz/failures/<seed>/` together with all three outputs.

```sh
cargo run -p test-fuzz --release -- --iterations 500   # explore (time-based seed)
cargo run -p test-fuzz --release -- --seed 12345       # reproduce one program
cargo run -p test-fuzz --release -- --emit 12345       # print a seed's program
```

The first iteration of a run uses the base seed directly, so the seed a
failure prints reproduces its exact program with `--seed`.

## What programs contain

Custom types (single-variant for field access and record updates,
multi-variant for case matching), fuel-bounded recursive functions with
tail and non-tail self calls, plain helpers, closures and function
references called through variables, pipes, `case` with literal /
binding / constructor / tuple / list / string-prefix patterns, guards,
top-level alternative patterns, structural equality at random types, big
integer literals and full integer arithmetic, bounded floats, string
concatenation and escapes, and bit array construction plus matching
(sized, signed, endian, and dynamically-sized segments) with the
extracted values echoed.

## What is deliberately avoided

- Runtime failures (`panic`, `todo`, failing `let assert`): report
  formats are target-specific, so every generated program is total —
  matches always end in a catch-all and recursion is fuelled.
- `echo` of bare bit arrays and functions, and of tuples whose first
  element can be a zero-arity constructor: their renderings legitimately
  differ (the BEAM cannot distinguish such a tuple from a record). The
  values themselves are still exercised; only `echo` positions exclude
  them.
- Float multiplication and division: Erlang raises on overflow to
  infinity where native saturates, so float growth must stay linear.
- Exponential accumulator growth in recursion (`acc <> acc`): recursive
  steps combine the accumulator linearly.
