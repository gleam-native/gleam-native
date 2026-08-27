// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! Fuzzing binary adopted from daniellionel01/gleam's `fuzzing` branch
//! (see <https://www.kurz.net/posts/fuzzing-gleam-compiler>): generate
//! well-typed Gleam programs with the `fuzzing-core` smith and run each on
//! Erlang (the semantic reference), JavaScript on Node.js, the native JIT,
//! and the native JIT under `GLEAM_DEBUG_RC=1` (checked reference
//! counting), reporting divergences against Erlang per pair.
//!
//! The JavaScript pair is compared value-by-value with cross-target
//! normalisation (`1.0` prints as `1` on JavaScript, and so on). The two
//! native pairs must match Erlang's `echo` output exactly: the native
//! target deliberately mirrors Erlang's rendering, so any difference is a
//! bug.
//!
//! Usage (run from the workspace root):
//!   cargo run -p fuzzing-cli --release -- run <seed>     one seed, verbose
//!   cargo run -p fuzzing-cli --release -- batch <start> <count>
//!   cargo run -p fuzzing-cli --release -- print <seed>   print the source
//!   cargo run -p fuzzing-cli --release -- bench <count> [base_seed]
//!   cargo run -p fuzzing-cli --release -- --gleam-bin <path> ...
//!
//! By default `fuzzing-cli` looks for a Gleam build at
//! `<repo>/target/release/gleam`. If that doesn't exist, it falls back to
//! `gleam` from $PATH. Override either with `--gleam-bin <path>` or the
//! `FUZZ_GLEAM_BIN` environment variable.
//!
//! Exit codes: 0 = match (or batch completed), 1 = mismatch, 2 = usage
//! error.

mod bench;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use fuzzing_core::generator::Module;
use fuzzing_core::value;
use gleam_core::build::Target;

const RUN_TIMEOUT: Duration = Duration::from_secs(120);

fn main() {
    let args: Vec<String> = env::args().collect();
    let (gleam_bin, rest) = resolve_gleam_bin(&args);
    match rest.get(0).map(String::as_str) {
        Some("run") => cmd_run(&rest[1..], &gleam_bin),
        Some("batch") => cmd_batch(&rest[1..], &gleam_bin),
        Some("print") => cmd_print(&rest[1..]),
        Some("bench") => bench::cmd_bench(&rest[1..], &gleam_bin),
        _ => {
            eprintln!(
                "usage:\n  cargo run -p fuzzing-cli -- run <seed>\n  cargo run -p fuzzing-cli -- batch <start> <count>\n  cargo run -p fuzzing-cli -- print <seed>\n  cargo run -p fuzzing-cli -- bench <count> [base_seed]\n  cargo run -p fuzzing-cli -- --gleam-bin <path> ..."
            );
            std::process::exit(2);
        }
    }
}

fn resolve_gleam_bin(args: &[String]) -> (PathBuf, Vec<String>) {
    let mut override_path: Option<PathBuf> = None;
    let mut rest_start = args.len();
    for (i, a) in args.iter().enumerate() {
        if a == "--gleam-bin" {
            if let Some(path) = args.get(i + 1) {
                override_path = Some(PathBuf::from(path));
                rest_start = i + 2;
                break;
            }
        }
        if let Some(value) = a.strip_prefix("--gleam-bin=") {
            override_path = Some(PathBuf::from(value));
            rest_start = i + 1;
            break;
        }
    }
    if override_path.is_none() {
        if let Ok(value) = env::var("FUZZ_GLEAM_BIN") {
            override_path = Some(PathBuf::from(value));
        }
    }
    let rest: Vec<String> = if rest_start >= args.len() {
        args[1..].to_vec()
    } else if override_path.is_some() {
        args[rest_start..].to_vec()
    } else {
        args[1..].to_vec()
    };
    let bin = override_path.unwrap_or_else(default_gleam_bin);
    (bin, rest)
}

fn default_gleam_bin() -> PathBuf {
    // Walk up from CWD looking for a sibling target/release/gleam.
    let mut dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    loop {
        let candidate = dir.join("target").join("release").join("gleam");
        if candidate.is_file() {
            return candidate;
        }
        if !dir.pop() {
            break;
        }
    }
    PathBuf::from("gleam")
}

// -- Commands ---------------------------------------------------------

fn cmd_run(args: &[String], gleam_bin: &Path) {
    let seed: u64 = args
        .first()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| {
            eprintln!("usage: cargo run -p fuzzing-cli -- run <seed>");
            std::process::exit(2);
        });

    let module = Module::from_seed(seed);
    let outcome = run_and_compare(&module, gleam_bin);

    // ANSI colors: green for match, yellow for skip, red for divergence.
    // The codes are inert when piped to a non-terminal, so CI logs stay
    // readable.
    const RED: &str = "\x1b[31m";
    const GREEN: &str = "\x1b[32m";
    const YELLOW: &str = "\x1b[33m";
    const RESET: &str = "\x1b[0m";

    println!("=== fuzz ===");
    println!("gleam:       {}", gleam_bin.display());
    println!("seed:        {seed}");
    for run in &outcome.runs {
        let status = if run.timed_out {
            "timed out".to_string()
        } else {
            format!("exit {}", run.status)
        };
        println!("{:<12} {status}", format!("{}:", run.label));
    }
    if let Some(note) = &outcome.whole_skip {
        println!("match:       {YELLOW}SKIP{RESET} ({note})");
    } else {
        for (label, pair) in &outcome.pairs {
            match pair {
                PairResult::Match => println!("{:<12} {GREEN}OK{RESET}", format!("{label}:")),
                PairResult::Skip(note) => {
                    println!("{:<12} {YELLOW}SKIP{RESET} ({note})", format!("{label}:"))
                }
                PairResult::Divergence(note) => {
                    println!("{:<12} {RED}NO{RESET} ({note})", format!("{label}:"))
                }
            }
        }
    }

    // Show per-echo values for every leg that ran cleanly, and raw output
    // for any leg that did not.
    let reference = &outcome.runs[0];
    if reference.status == 0 {
        println!("\n--- values (erlang reference first) ---");
        let max = outcome
            .runs
            .iter()
            .filter(|run| run.status == 0)
            .map(|run| run.values.len())
            .max()
            .unwrap_or(0);
        for i in 0..max {
            let mut parts = Vec::new();
            for run in &outcome.runs {
                if run.status != 0 {
                    continue;
                }
                let rendered = run
                    .values
                    .get(i)
                    .map(|v| format!("{v:?}"))
                    .unwrap_or_else(|| "<missing>".into());
                parts.push(format!("{}={rendered}", run.label));
            }
            println!("[{i}] {}", parts.join("  "));
        }
    }
    for run in &outcome.runs {
        if run.status != 0 && outcome.whole_skip.is_none() {
            println!("\n--- {} output ---\n{}", run.label, run.raw);
        }
    }

    let failed = outcome.whole_skip.is_none()
        && outcome
            .pairs
            .iter()
            .any(|(_, pair)| matches!(pair, PairResult::Divergence(_)));
    std::process::exit(if failed { 1 } else { 0 });
}

fn cmd_print(args: &[String]) {
    let seed: u64 = match args.first().and_then(|s| s.parse().ok()) {
        Some(s) => s,
        None => {
            eprintln!("usage: cargo run -p fuzzing-cli -- print <seed>");
            std::process::exit(2);
        }
    };
    let module = Module::from_seed(seed);
    print!("{}", module.to_source());
}

fn cmd_batch(args: &[String], gleam_bin: &Path) {
    let start: u64 = args
        .first()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| {
            eprintln!("usage: cargo run -p fuzzing-cli -- batch <start> <count>");
            std::process::exit(2);
        });
    let count: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or_else(|| {
        eprintln!("usage: cargo run -p fuzzing-cli -- batch <start> <count>");
        std::process::exit(2);
    });

    let base = Path::new(env!("CARGO_MANIFEST_DIR"));
    let corpus_dir = base.join("corpus").join("fuzz");
    let artifacts_dir = base.join("artifacts").join("fuzz");
    fs::create_dir_all(&corpus_dir).expect("create corpus dir");
    fs::create_dir_all(&artifacts_dir).expect("create artifacts dir");

    eprintln!("[fuzzing-cli] gleam: {}", gleam_bin.display());
    eprintln!("[fuzzing-cli] seeds {start}..{}", start + count - 1);
    let started = Instant::now();
    let mut mismatches = 0u64;
    let mut skipped = 0u64;
    let mut ran = 0u64;

    for seed in start..start + count {
        let module = Module::from_seed(seed);
        let src = module.to_source();

        let corpus_path = corpus_dir.join(format!("seed_{seed}.gleam"));
        fs::write(&corpus_path, &src).expect("write corpus");

        let outcome = run_and_compare(&module, gleam_bin);

        if let Some(note) = &outcome.whole_skip {
            eprintln!("[fuzzing-cli] seed {seed} skipped: {note}");
            skipped += 1;
            continue;
        }
        let mut problems = Vec::new();
        for (label, pair) in &outcome.pairs {
            match pair {
                PairResult::Match => {}
                PairResult::Skip(note) => {
                    eprintln!("[fuzzing-cli] seed {seed} pair {label} skipped: {note}")
                }
                PairResult::Divergence(note) => problems.push(format!("{label}: {note}")),
            }
        }
        if !problems.is_empty() {
            let artifact_path = artifacts_dir.join(format!("seed_{seed}.gleam"));
            fs::write(&artifact_path, &src).expect("write artifact");
            for run in &outcome.runs {
                fs::write(
                    artifacts_dir.join(format!("seed_{seed}.{}.txt", run.label)),
                    &run.raw,
                )
                .expect("write artifact output");
            }
            fs::write(
                artifacts_dir.join(format!("seed_{seed}.problems.txt")),
                problems.join("\n"),
            )
            .expect("write artifact problems");
            eprintln!(
                "[fuzzing-cli] DIVERGENCE seed {seed} -> {}",
                problems.join("; ")
            );
            mismatches += 1;
        }

        ran += 1;
        if ran % 10 == 0 {
            eprintln!(
                "[fuzzing-cli] {ran}/{count} run, {mismatches} mismatches, {skipped} skipped ({:.0}s)",
                started.elapsed().as_secs_f64()
            );
        }
    }

    eprintln!(
        "[fuzzing-cli] done: {ran} programs, {mismatches} mismatch(es), {skipped} skipped in {:.0}s",
        started.elapsed().as_secs_f64()
    );
}

// -- Running and comparing --------------------------------------------

struct TargetRun {
    label: &'static str,
    status: i32,
    timed_out: bool,
    raw: String,
    values: Vec<value::Value>,
}

enum PairResult {
    Match,
    Skip(String),
    Divergence(String),
}

struct Outcome {
    /// erlang, nodejs, native, checked - in that order.
    runs: Vec<TargetRun>,
    /// Non-erlang legs compared against the Erlang reference.
    pairs: Vec<(&'static str, PairResult)>,
    /// The Erlang reference itself hit a known runtime bug, so no pair
    /// can be judged.
    whole_skip: Option<String>,
}

fn run_and_compare(module: &Module, gleam_bin: &Path) -> Outcome {
    let src = module.to_source();
    let work = tempfile::tempdir().expect("create temp dir");
    let project_dir = work.path();

    let src_dir = project_dir.join("src");
    fs::create_dir_all(&src_dir).expect("create src dir");
    fs::write(src_dir.join("main.gleam"), &src).expect("write main.gleam");
    fs::write(project_dir.join("gleam.toml"), "name = \"fuzzing_case\"\n")
        .expect("write gleam.toml");

    let echo_lines = value::echo_line_numbers(&src);
    let run = |label: &'static str, target: Target, extra: &[&str], checked_rc: bool| {
        let target_flag = match target {
            Target::Erlang => "erlang",
            Target::JavaScript => "javascript",
            Target::Native => "native",
        };
        let mut command = Command::new(gleam_bin);
        let _ = command
            .args(["run", "--target", target_flag, "--module", "main"])
            .args(extra)
            .current_dir(project_dir);
        if checked_rc {
            let _ = command.env("GLEAM_DEBUG_RC", "1");
        }
        let (raw, status, timed_out) = run_with_timeout(command);
        let values = if status == Some(0) {
            value::parse_output_with_echo_lines(&raw, target, &echo_lines)
        } else {
            Vec::new()
        };
        TargetRun {
            label,
            status: status.unwrap_or(-1),
            timed_out,
            raw,
            values,
        }
    };

    let erlang = run("erlang", Target::Erlang, &[], false);
    let nodejs = run(
        "nodejs",
        Target::JavaScript,
        &["--runtime", "nodejs"],
        false,
    );
    let native = run("native", Target::Native, &[], false);
    let checked = run("checked", Target::Native, &[], true);

    // The Erlang run is the reference; when it dies on a known
    // OTP-compiler bug nothing can be compared for this seed.
    // https://github.com/erlang/otp/issues/11494
    let whole_skip = if erlang.status != 0 && is_erlang_otp_issue_11494(&erlang.raw) {
        Some("otp issue #11494".into())
    } else {
        None
    };

    let pairs = vec![
        ("nodejs", compare_javascript(&erlang, &nodejs)),
        ("native", compare_native(&erlang, &native)),
        ("checked", compare_native(&erlang, &checked)),
    ];

    Outcome {
        runs: vec![erlang, nodejs, native, checked],
        pairs,
        whole_skip,
    }
}

/// The JavaScript leg: value-by-value comparison with cross-target
/// normalisation, and known upstream issues skipped by heuristic.
fn compare_javascript(erlang: &TargetRun, nodejs: &TargetRun) -> PairResult {
    let erl_ok = erlang.status == 0;
    let js_ok = nodejs.status == 0;

    // https://github.com/gleam-lang/gleam/issues/6182
    if erl_ok && !js_ok && is_gleam_issue_6182(&nodejs.raw) {
        return PairResult::Skip("gleam issue #6182".into());
    }
    // https://github.com/gleam-lang/gleam/issues/6212
    if erl_ok && !js_ok && is_gleam_issue_6212(&nodejs.raw) {
        return PairResult::Skip("gleam issue #6212".into());
    }

    if erl_ok && js_ok {
        if value::outputs_match_cross_target(&erlang.values, &nodejs.values) {
            PairResult::Match
        } else {
            PairResult::Divergence("values differ".into())
        }
    } else if !erl_ok && !js_ok {
        if value::strip_build_noise(&erlang.raw) == value::strip_build_noise(&nodejs.raw) {
            PairResult::Match
        } else {
            PairResult::Divergence("both crashed differently".into())
        }
    } else {
        PairResult::Divergence(status_note(erlang, nodejs))
    }
}

/// A native leg: the native target mirrors Erlang's `echo` rendering
/// exactly, so the parsed values must be identical - no cross-target
/// leniency that could mask a real difference.
fn compare_native(erlang: &TargetRun, native: &TargetRun) -> PairResult {
    let erl_ok = erlang.status == 0;
    let native_ok = native.status == 0;
    if erl_ok && native_ok {
        if erlang.values == native.values {
            PairResult::Match
        } else {
            PairResult::Divergence("values differ".into())
        }
    } else {
        // Generated programs are meant to be total; error-report formats
        // are target-specific, so any crash on either side is flagged
        // for a human rather than string-compared.
        PairResult::Divergence(status_note(erlang, native))
    }
}

fn status_note(erlang: &TargetRun, other: &TargetRun) -> String {
    let describe = |run: &TargetRun| {
        if run.timed_out {
            "timeout".to_string()
        } else {
            format!("exit {}", run.status)
        }
    };
    format!("erlang {} vs {}", describe(erlang), describe(other))
}

/// Runs the command with a timeout, returning combined stdout+stderr,
/// the exit code, and whether it timed out. The child gets its own
/// process group so a hanging runtime is killed with its whole tree.
fn run_with_timeout(mut command: Command) -> (String, Option<i32>, bool) {
    use std::process::Stdio;
    let _ = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        let _ = command.process_group(0);
    }
    let mut child = command.spawn().expect("spawn gleam");

    let mut stdout_pipe = child.stdout.take().expect("stdout pipe");
    let mut stderr_pipe = child.stderr.take().expect("stderr pipe");
    let stdout_reader = std::thread::spawn(move || {
        use std::io::Read;
        let mut bytes = Vec::new();
        let _ = stdout_pipe.read_to_end(&mut bytes);
        bytes
    });
    let stderr_reader = std::thread::spawn(move || {
        use std::io::Read;
        let mut bytes = Vec::new();
        let _ = stderr_pipe.read_to_end(&mut bytes);
        bytes
    });

    let deadline = Instant::now() + RUN_TIMEOUT;
    let status = loop {
        match child.try_wait().expect("wait for gleam") {
            Some(status) => break Some(status),
            None if Instant::now() > deadline => {
                #[cfg(unix)]
                {
                    let _ = Command::new("kill")
                        .args(["-KILL", &format!("-{}", child.id())])
                        .status();
                }
                let _ = child.kill();
                let _ = child.wait();
                break None;
            }
            None => std::thread::sleep(Duration::from_millis(10)),
        }
    };
    let stdout = String::from_utf8_lossy(&stdout_reader.join().expect("stdout")).into_owned();
    let stderr = String::from_utf8_lossy(&stderr_reader.join().expect("stderr")).into_owned();
    (
        format!("{stdout}{stderr}"),
        status.and_then(|status| status.code()),
        status.is_none(),
    )
}

// -- Known-issue heuristics -------------------------------------------
// Add a new entry here to filter a known issue: return true when raw
// output matches the issue's signature.

// https://github.com/erlang/otp/issues/11494
fn is_erlang_otp_issue_11494(raw: &str) -> bool {
    raw.contains("Internal consistency check failed")
        && raw.contains("call_only")
        && raw.contains("bad_arg_type")
        && raw.contains("{x,")
        && raw.contains("t_union")
        && raw.contains("t_bitstring")
}

// https://github.com/gleam-lang/gleam/issues/6182
fn is_gleam_issue_6182(raw: &str) -> bool {
    raw.contains("SyntaxError: Unexpected token '&&'")
        || raw.contains("SyntaxError: Unexpected token ')'")
}

// https://github.com/gleam-lang/gleam/issues/6212
fn is_gleam_issue_6212(raw: &str) -> bool {
    raw.contains("TypeError:") && raw.contains("is not a function")
}
