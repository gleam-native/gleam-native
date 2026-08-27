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
and N, cold `gleam build` per target, `gleam export native`, warm
startup, and peak RSS (one run at the calibrated repeat count through
`/usr/bin/time -l`, child processes included), comparing Erlang, the
native JIT and AOT executable, and JavaScript on Node.js. Results are
written to `fuzzing-cli/bench-results.json`.

## Results

On an Apple Silicon Mac (M3, macOS 14, release-built compiler,
Erlang/OTP 29, Node 24), 2026-08-28, over 24 generated programs
(seeds 0–23; seed 21 skipped — its Erlang leg dies on the known OTP
compiler bug) — after the day's optimizations (single-write `echo`,
in-place float updates, inline float free, scalar replacement,
float/big-integer literal interning):

| configuration | exec vs erlang | peak RSS | cold compile | startup |
| ------------- | -------------- | -------- | ------------ | ------- |
| erlang (BEAM) |          1.00× |    83 MB |       196 ms |  256 ms |
| node 24       |          6.13× |    65 MB |         7 ms |   26 ms |
| native (JIT)  |         10.47× |    16 MB |         6 ms |  6.5 ms |
| native (AOT)  |         10.75× |     3 MB |       247 ms |  6.6 ms |

Execution is the geometric mean of per-iteration time ratios (the
`echo` rendering included); the native JIT is 1.71× faster than
Node.js and ahead on every program. Peak RSS is one run at the
calibrated repeat count, children included. The AOT compile figure is
`gleam export native`: a from-scratch build plus linking and the
runtime-library freshness check. Full charts:
https://claude.ai/code/artifact/6051ad4d-9293-47d6-a7e9-fb9be34a5099
