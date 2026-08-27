// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! Benchmark mode: uses the `fuzzing-core` smith's generated programs as
//! workloads and compares the Erlang target, the native JIT (`gleam run`),
//! and the AOT executable (`gleam export native`) with the same
//! methodology as `test-fuzz --bench`:
//!
//! - Steady-state execution: each program's `main` is renamed and driven
//!   by a tail-recursive repeat loop; a warm `gleam run` is timed at
//!   repeat counts 1 and N (N calibrated on Erlang so the run takes
//!   around a second) and the per-iteration time is the slope
//!   `(t_N - t_1) / (N - 1)`, which cancels VM startup, build-freshness
//!   checks, and JIT compilation. `echo` formatting and writes are part
//!   of the workload (output goes to the null device).
//! - Compile time: a cold `gleam build` per target, plus
//!   `gleam export native` (inherently cold, includes linking).
//! - Startup time: a warm run of a trivial program per target.
//!
//! Timed samples use the minimum over several runs. Results print as
//! tables and are saved as JSON next to this crate.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use fuzzing_core::generator::Module;

const NODE_RUN: [&str; 7] = [
    "run", "--target", "javascript", "--module", "main", "--runtime", "nodejs",
];

/// Timed runs per measurement point; the minimum is used.
const SAMPLES: usize = 3;
const STARTUP_SAMPLES: usize = 7;
/// Calibration grows the repeat count until the Erlang run is this much
/// slower than the single-repeat run.
const TARGET_SECONDS: f64 = 1.0;
const MAX_REPEATS: u64 = 1 << 20;
const TIMEOUT: Duration = Duration::from_secs(300);

struct ProgramResult {
    seed: u64,
    repeats: u64,
    /// Per-iteration execution time in seconds, per target.
    exec: Targets<f64>,
    /// Cold compile time in seconds, per target.
    compile: Targets<f64>,
    /// Peak resident set size in bytes of one run at the calibrated
    /// repeat count (child processes included), or `None` where the
    /// platform cannot measure it.
    rss: Targets<Option<u64>>,
}

struct Targets<T> {
    erlang: T,
    node: T,
    jit: T,
    aot: T,
}

pub fn cmd_bench(args: &[String], gleam: &Path) {
    let count: u64 = args
        .first()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| {
            eprintln!("usage: cargo run -p fuzzing-cli -- bench <count> [base_seed]");
            std::process::exit(2);
        });
    let base_seed: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);

    println!("gleam: {}", gleam.display());
    println!(
        "benchmarking {count} smith program(s) from seed {base_seed}: erlang vs native JIT vs native AOT"
    );

    let project = scratch_project();
    let startup = measure_startup(gleam, &project);
    println!(
        "startup (trivial program, warm): erlang {:.1}ms, node {:.1}ms, jit {:.1}ms, aot {:.1}ms",
        startup.erlang * 1000.0,
        startup.node * 1000.0,
        startup.jit * 1000.0,
        startup.aot * 1000.0
    );

    let mut results: Vec<ProgramResult> = Vec::new();
    let started = Instant::now();
    for (index, seed) in (base_seed..base_seed + count).enumerate() {
        let program = Module::from_seed(seed).to_source();
        match measure_program(gleam, &project, &program, seed) {
            Some(result) => {
                println!(
                    "  [{}/{count}] seed {seed}: repeats {}, per-iteration erlang {}, node {}, jit {}, aot {} ({:.0}s elapsed)",
                    index + 1,
                    result.repeats,
                    format_seconds(result.exec.erlang),
                    format_seconds(result.exec.node),
                    format_seconds(result.exec.jit),
                    format_seconds(result.exec.aot),
                    started.elapsed().as_secs_f64()
                );
                results.push(result);
            }
            None => println!(
                "  [{}/{count}] seed {seed}: SKIPPED (a run failed or timed out; check with: cargo run -p fuzzing-cli -- run {seed})",
                index + 1
            ),
        }
    }

    if results.is_empty() {
        println!("no programs completed; nothing to report");
        std::process::exit(1);
    }

    print_report(&results, &startup);
    save_json(&results, &startup, base_seed);
}

// -- Measurement ------------------------------------------------------

/// Warm-run startup times for a trivial program on each target.
fn measure_startup(gleam: &Path, project: &Path) -> Targets<f64> {
    write_source(project, "pub fn main() {\n  Nil\n}\n");
    let erlang = ["run", "--target", "erlang", "--module", "main"];
    let node = NODE_RUN;
    let native = ["run", "--target", "native", "--module", "main"];
    let export = ["export", "native", "--module", "main"];
    assert!(warm(gleam, project, &erlang), "trivial erlang run failed");
    assert!(warm(gleam, project, &node), "trivial node run failed");
    assert!(warm(gleam, project, &native), "trivial native run failed");
    assert!(warm(gleam, project, &export), "trivial export failed");
    let binary = project.join("main");
    Targets {
        erlang: sample(STARTUP_SAMPLES, || timed(gleam, project, &erlang))
            .expect("trivial erlang run failed"),
        node: sample(STARTUP_SAMPLES, || timed(gleam, project, &node))
            .expect("trivial node run failed"),
        jit: sample(STARTUP_SAMPLES, || timed(gleam, project, &native))
            .expect("trivial native run failed"),
        aot: sample(STARTUP_SAMPLES, || timed(&binary, project, &[]))
            .expect("trivial aot run failed"),
    }
}

/// All measurements for one generated program, or `None` if any step
/// fails (the `run` command is the tool for diagnosing those seeds).
fn measure_program(
    gleam: &Path,
    project: &Path,
    program: &str,
    seed: u64,
) -> Option<ProgramResult> {
    let erlang = ["run", "--target", "erlang", "--module", "main"];
    let node = NODE_RUN;
    let native = ["run", "--target", "native", "--module", "main"];
    let export = ["export", "native", "--module", "main"];
    let binary = project.join("main");

    // Single-repeat baselines.
    write_source(project, &with_repeats(program, 1));
    if !warm(gleam, project, &erlang)
        || !warm(gleam, project, &node)
        || !warm(gleam, project, &native)
    {
        return None;
    }
    let erlang_one = sample(SAMPLES, || timed(gleam, project, &erlang))?;
    let node_one = sample(SAMPLES, || timed(gleam, project, &node))?;
    let jit_one = sample(SAMPLES, || timed(gleam, project, &native))?;

    // Cold compile times, while the single-repeat source is in place.
    let compile = Targets {
        erlang: sample(SAMPLES, || {
            remove_build(project, "dev/erlang");
            timed(gleam, project, &["build", "--target", "erlang"])
        })?,
        node: sample(SAMPLES, || {
            remove_build(project, "dev/javascript");
            timed(gleam, project, &["build", "--target", "javascript"])
        })?,
        jit: sample(SAMPLES, || {
            remove_build(project, "dev/native");
            timed(gleam, project, &["build", "--target", "native"])
        })?,
        // `gleam export native` deletes its own build directory, so it is
        // always cold; it also includes linking the executable.
        aot: sample(SAMPLES, || timed(gleam, project, &export))?,
    };
    let aot_one = sample(SAMPLES, || timed(&binary, project, &[]))?;

    // Calibrate the repeat count on Erlang: grow until the run is
    // TARGET_SECONDS slower than the single-repeat baseline.
    let mut repeats: u64 = 8;
    loop {
        write_source(project, &with_repeats(program, repeats));
        if !warm(gleam, project, &erlang) {
            return None;
        }
        let elapsed = timed(gleam, project, &erlang)?;
        let delta = elapsed - erlang_one;
        if delta >= TARGET_SECONDS || repeats >= MAX_REPEATS {
            break;
        }
        // Once the measured slowdown is clearly above run-to-run noise
        // (Erlang startup jitters by tens of milliseconds), aim directly
        // for the target; below that the per-iteration estimate is
        // garbage, so just grow geometrically.
        repeats = if delta >= 0.1 {
            let per_iteration = delta / repeats as f64;
            ((TARGET_SECONDS * 1.2 / per_iteration) as u64).clamp(repeats * 2, MAX_REPEATS)
        } else {
            (repeats * 16).min(MAX_REPEATS)
        };
    }

    // Timed runs at the calibrated repeat count.
    write_source(project, &with_repeats(program, repeats));
    if !warm(gleam, project, &erlang)
        || !warm(gleam, project, &node)
        || !warm(gleam, project, &native)
    {
        return None;
    }
    let erlang_many = sample(SAMPLES, || timed(gleam, project, &erlang))?;
    let node_many = sample(SAMPLES, || timed(gleam, project, &node))?;
    let jit_many = sample(SAMPLES, || timed(gleam, project, &native))?;
    if !warm(gleam, project, &export) {
        return None;
    }
    let aot_many = sample(SAMPLES, || timed(&binary, project, &[]))?;

    // Peak memory of one run each at the calibrated repeat count, with
    // the many-repeats source still in place.
    let rss = Targets {
        erlang: peak_rss(gleam, project, &erlang),
        node: peak_rss(gleam, project, &node),
        jit: peak_rss(gleam, project, &native),
        aot: peak_rss(&binary, project, &[]),
    };

    let per_iteration = |many: f64, one: f64| ((many - one) / (repeats - 1) as f64).max(0.0);
    Some(ProgramResult {
        seed,
        repeats,
        exec: Targets {
            erlang: per_iteration(erlang_many, erlang_one),
            node: per_iteration(node_many, node_one),
            jit: per_iteration(jit_many, jit_one),
            aot: per_iteration(aot_many, aot_one),
        },
        compile,
        rss,
    })
}

/// Peak resident set size in bytes of one run, child processes included
/// (so `gleam run`'s BEAM or Node.js child is what dominates), measured
/// through `/usr/bin/time -l`. macOS only; elsewhere `None`. Called only
/// for configurations whose timed runs already succeeded, so a plain
/// blocking wait is safe.
fn peak_rss(executable: &Path, project: &Path, arguments: &[&str]) -> Option<u64> {
    if !cfg!(target_os = "macos") {
        return None;
    }
    let output = Command::new("/usr/bin/time")
        .arg("-l")
        .arg(executable)
        .args(arguments)
        .current_dir(project)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    stderr.lines().find_map(|line| {
        let line = line.trim();
        line.strip_suffix("maximum resident set size")
            .and_then(|number| number.trim().parse().ok())
    })
}

/// The generated program with its `main` renamed and a new `main`
/// driving it through a tail-recursive repeat loop. The `fuzzbench_`
/// prefix avoids the smith's name pools.
fn with_repeats(program: &str, repeats: u64) -> String {
    let renamed = program.replacen("pub fn main() {", "fn fuzzbench_work() {", 1);
    format!(
        "{renamed}\nfn fuzzbench_repeat(n: Int) -> Nil {{\n  case n < 1 {{\n    True -> Nil\n    False -> {{\n      fuzzbench_work()\n      fuzzbench_repeat(n - 1)\n    }}\n  }}\n}}\n\npub fn main() {{\n  fuzzbench_repeat({repeats})\n}}\n"
    )
}

/// A persistent scratch project the generated programs are written into,
/// so warm builds stay warm between measurements.
fn scratch_project() -> PathBuf {
    let root = std::env::temp_dir().join(format!("gleam-fuzzing-bench-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("src")).expect("create scratch project");
    std::fs::write(
        root.join("gleam.toml"),
        "name = \"fuzzing_case\"\nversion = \"1.0.0\"\n",
    )
    .expect("write gleam.toml");
    root
}

fn write_source(project: &Path, source: &str) {
    std::fs::write(project.join("src").join("main.gleam"), source).expect("write program");
}

fn remove_build(project: &Path, subdirectory: &str) {
    let _ = std::fs::remove_dir_all(project.join("build").join(subdirectory));
}

/// Runs the command untimed (compiling any pending changes) and reports
/// whether it succeeded.
fn warm(executable: &Path, project: &Path, arguments: &[&str]) -> bool {
    execute(executable, project, arguments).is_some()
}

/// Runs the command and returns the wall-clock seconds it took, or
/// `None` on failure or timeout.
fn timed(executable: &Path, project: &Path, arguments: &[&str]) -> Option<f64> {
    let started = Instant::now();
    execute(executable, project, arguments)?;
    Some(started.elapsed().as_secs_f64())
}

/// The minimum of several samples; `None` if any sample fails.
fn sample(count: usize, mut run: impl FnMut() -> Option<f64>) -> Option<f64> {
    let mut best = f64::INFINITY;
    for _ in 0..count {
        best = best.min(run()?);
    }
    Some(best)
}

/// Runs a command with all output discarded, returning `Some(())` on a
/// zero exit within the timeout. The child gets its own process group so
/// a timeout can kill the whole tree.
fn execute(executable: &Path, project: &Path, arguments: &[&str]) -> Option<()> {
    let mut command = Command::new(executable);
    let _ = command
        .args(arguments)
        .current_dir(project)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        let _ = command.process_group(0);
    }
    let mut child = command.spawn().expect("spawn command");
    let deadline = Instant::now() + TIMEOUT;
    loop {
        match child.try_wait().expect("wait for command") {
            Some(status) => return status.success().then_some(()),
            None if Instant::now() > deadline => {
                #[cfg(unix)]
                {
                    let _ = Command::new("kill")
                        .args(["-KILL", &format!("-{}", child.id())])
                        .status();
                }
                let _ = child.kill();
                let _ = child.wait();
                return None;
            }
            None => std::thread::sleep(Duration::from_millis(5)),
        }
    }
}

// -- Reporting --------------------------------------------------------

fn print_report(results: &[ProgramResult], startup: &Targets<f64>) {
    println!();
    println!("Steady-state execution (per iteration; speedup is erlang time / native time,");
    println!("so above 1.0 means native is faster):");
    println!(
        "  {:>20} {:>9} {:>10} {:>10} {:>10} {:>10} {:>8} {:>8}",
        "seed", "repeats", "erlang", "node", "jit", "aot", "jit x", "aot x"
    );
    for result in results {
        println!(
            "  {:>20} {:>9} {:>10} {:>10} {:>10} {:>10} {:>8} {:>8}",
            result.seed,
            result.repeats,
            format_seconds(result.exec.erlang),
            format_seconds(result.exec.node),
            format_seconds(result.exec.jit),
            format_seconds(result.exec.aot),
            format_ratio(result.exec.erlang, result.exec.jit),
            format_ratio(result.exec.erlang, result.exec.aot),
        );
    }
    println!(
        "  {:>20} {:>9} {:>10} {:>10} {:>10} {:>10} {:>8} {:>8}",
        "geometric mean",
        "",
        "",
        "",
        "",
        "",
        format!(
            "{:.2}",
            geometric_mean(results, |result| result.exec.erlang / result.exec.jit)
        ),
        format!(
            "{:.2}",
            geometric_mean(results, |result| result.exec.erlang / result.exec.aot)
        ),
    );

    println!();
    println!("Cold compile (erlang and jit are `gleam build`; aot is `gleam export native`");
    println!("and includes linking):");
    println!(
        "  {:>20} {:>10} {:>10} {:>10} {:>10}",
        "seed", "erlang", "node", "jit", "aot"
    );
    for result in results {
        println!(
            "  {:>20} {:>9.0}ms {:>9.0}ms {:>9.0}ms {:>9.0}ms",
            result.seed,
            result.compile.erlang * 1000.0,
            result.compile.node * 1000.0,
            result.compile.jit * 1000.0,
            result.compile.aot * 1000.0,
        );
    }
    println!(
        "  {:>20} {:>9.0}ms {:>9.0}ms {:>9.0}ms {:>9.0}ms",
        "mean",
        mean(results, |result| result.compile.erlang) * 1000.0,
        mean(results, |result| result.compile.node) * 1000.0,
        mean(results, |result| result.compile.jit) * 1000.0,
        mean(results, |result| result.compile.aot) * 1000.0,
    );

    println!();
    println!("Peak RSS (one run at the calibrated repeat count, children included):");
    println!(
        "  {:>20} {:>9} {:>9} {:>9} {:>9}",
        "seed", "erlang", "node", "jit", "aot"
    );
    for result in results {
        println!(
            "  {:>20} {:>9} {:>9} {:>9} {:>9}",
            result.seed,
            format_rss(result.rss.erlang),
            format_rss(result.rss.node),
            format_rss(result.rss.jit),
            format_rss(result.rss.aot),
        );
    }

    println!();
    println!(
        "Startup (trivial program, warm): erlang {:.1}ms, node {:.1}ms, jit {:.1}ms, aot {:.1}ms",
        startup.erlang * 1000.0,
        startup.node * 1000.0,
        startup.jit * 1000.0,
        startup.aot * 1000.0
    );
}

fn save_json(results: &[ProgramResult], startup: &Targets<f64>, base_seed: u64) {
    let mut programs = String::new();
    for (index, result) in results.iter().enumerate() {
        if index > 0 {
            programs.push(',');
        }
        let rss = |bytes: Option<u64>| match bytes {
            Some(bytes) => bytes.to_string(),
            None => "null".into(),
        };
        programs.push_str(&format!(
            r#"{{"seed":{},"repeats":{},"exec_seconds":{{"erlang":{:e},"node":{:e},"jit":{:e},"aot":{:e}}},"compile_seconds":{{"erlang":{:e},"node":{:e},"jit":{:e},"aot":{:e}}},"peak_rss_bytes":{{"erlang":{},"node":{},"jit":{},"aot":{}}}}}"#,
            result.seed,
            result.repeats,
            result.exec.erlang,
            result.exec.node,
            result.exec.jit,
            result.exec.aot,
            result.compile.erlang,
            result.compile.node,
            result.compile.jit,
            result.compile.aot,
            rss(result.rss.erlang),
            rss(result.rss.node),
            rss(result.rss.jit),
            rss(result.rss.aot),
        ));
    }
    let json = format!(
        r#"{{"base_seed":{base_seed},"startup_seconds":{{"erlang":{:e},"node":{:e},"jit":{:e},"aot":{:e}}},"programs":[{programs}]}}"#,
        startup.erlang, startup.node, startup.jit, startup.aot,
    );
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("bench-results.json");
    std::fs::write(&path, json).expect("write results");
    println!("results saved to {}", path.display());
}

fn format_rss(bytes: Option<u64>) -> String {
    match bytes {
        Some(bytes) => format!("{:.0}MB", bytes as f64 / (1024.0 * 1024.0)),
        None => "?".into(),
    }
}

fn format_seconds(seconds: f64) -> String {
    if seconds >= 1e-3 {
        format!("{:.2}ms", seconds * 1000.0)
    } else {
        format!("{:.2}us", seconds * 1e6)
    }
}

fn format_ratio(erlang: f64, native: f64) -> String {
    if native <= 0.0 {
        "-".into()
    } else {
        format!("{:.2}", erlang / native)
    }
}

fn geometric_mean(results: &[ProgramResult], ratio: impl Fn(&ProgramResult) -> f64) -> f64 {
    let sum: f64 = results
        .iter()
        .map(|result| ratio(result).max(1e-12).ln())
        .sum();
    (sum / results.len() as f64).exp()
}

fn mean(results: &[ProgramResult], value: impl Fn(&ProgramResult) -> f64) -> f64 {
    results.iter().map(value).sum::<f64>() / results.len() as f64
}
