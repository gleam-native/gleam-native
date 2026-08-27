<!--
  SPDX-License-Identifier: Apache-2.0
  SPDX-FileCopyrightText: 2026 The Gleam contributors
-->

# fuzzing-cli

Differential fuzzer adopted from [daniellionel01/gleam's `fuzzing`
branch](https://github.com/daniellionel01/gleam/tree/fuzzing) (methodology:
[Fuzzing the Gleam Compiler](https://www.kurz.net/posts/fuzzing-gleam-compiler)),
adapted to exercise the native target. It complements `test-fuzz` — the
two generators explore different program spaces — and does not replace it.

`fuzzing-core` provides the pieces: a structure-aware AST "smith" that
builds well-typed programs from a seed via the `arbitrary` crate
(`generator`), an in-memory compiler crash probe (`probe`), and a parser
that turns each target's `echo` output into normalised values (`value`).

Each seed's program runs four ways: Erlang (the semantic reference),
JavaScript on Node.js, the native JIT, and the native JIT under
`GLEAM_DEBUG_RC=1` (checked reference counting). Every non-Erlang leg is
compared against Erlang per `echo` value:

- **nodejs** — cross-target normalisation (JavaScript legitimately prints
  `1.0` as `1`, strings for byte-aligned bit arrays differ, and so on),
  with known upstream issues skipped by heuristic signature.
- **native / checked** — exact value equality: the native target mirrors
  Erlang's `echo` rendering, so any difference at all is a bug. Crashes
  on either side are always flagged (error-report formats are
  target-specific, so they are never string-compared).

```sh
cargo run -p fuzzing-cli --release -- run 947          # one seed, verbose
cargo run -p fuzzing-cli --release -- batch 0 1000     # a seed range
cargo run -p fuzzing-cli --release -- print 947        # print the source
cargo run -p fuzzing-cli --release -- bench 12 0       # benchmark mode
```

Batch runs save every program to `corpus/fuzz/` and each divergence's
program plus all four raw outputs to `artifacts/fuzz/`. The Gleam binary
defaults to `<repo>/target/release/gleam`; override with `--gleam-bin`
or `FUZZ_GLEAM_BIN`.

`bench` reuses the smith's programs as workloads with the same
methodology as `test-fuzz --bench` (see that crate's README): per-
iteration execution as the slope between warm runs at repeat counts 1
and N, cold `gleam build` per target, `gleam export native`, and warm
startup, comparing Erlang against the native JIT and AOT executable.
Results are written to `fuzzing-cli/bench-results.json`.
