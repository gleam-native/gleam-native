// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! Benchmark mode: uses the fuzzer's generated programs as workloads and
//! compares the Erlang target, the native JIT, and the AOT executable
//! (`gleam export native`) on three axes:
//!
//! - Steady-state execution: each program's `main` is renamed and driven
//!   by a tail-recursive repeat loop; a warm `gleam run` is timed at
//!   repeat counts 1 and N (N calibrated on Erlang so the run takes
//!   around a second) and the per-iteration time is the slope
//!   `(t_N - t_1) / (N - 1)`, which cancels VM startup, build-freshness
//!   checks, and JIT compilation. The workload includes the programs'
//!   `echo` formatting and writes (output goes to the null device).
//! - Compile time: a cold `gleam build` per target (the target's build
//!   directory is deleted before every timed build), plus
//!   `gleam export native`, which is inherently cold and includes
//!   linking.
//! - Startup time: a warm run of a trivial `pub fn main() { Nil }`
//!   program per target, and the AOT binary of the same.
//!
//! Timed samples use the minimum over several runs. Results are printed
//! as tables and saved as JSON next to this crate.

use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Timed runs per measurement point; the minimum is used.
const SAMPLES: usize = 3;
/// Startup is cheap to sample, so take a few more.
const STARTUP_SAMPLES: usize = 7;
/// Calibration grows the repeat count until the Erlang run is this much
/// slower than the single-repeat run.
const TARGET_SECONDS: f64 = 1.0;
const MAX_REPEATS: u64 = 1 << 20;
const TIMEOUT: Duration = Duration::from_secs(300);

pub struct Options {
    pub programs: u64,
    pub base_seed: u64,
}

struct ProgramResult {
    seed: u64,
    repeats: u64,
    /// Per-iteration execution time in seconds, per target.
    exec: Targets<f64>,
    /// Cold compile time in seconds, per target.
    compile: Targets<f64>,
}

struct Targets<T> {
    erlang: T,
    jit: T,
    aot: T,
}

pub fn main(options: &Options, gleam: &Path, project: &Path) {
    println!("base seed: {}", options.base_seed);
    println!(
        "benchmarking {} program(s): erlang vs native JIT vs native AOT",
        options.programs
    );

    let startup = measure_startup(gleam, project);
    println!(
        "startup (trivial program, warm): erlang {:.1}ms, jit {:.1}ms, aot {:.1}ms",
        startup.erlang * 1000.0,
        startup.jit * 1000.0,
        startup.aot * 1000.0
    );

    let mut results: Vec<ProgramResult> = Vec::new();
    let started = Instant::now();
    for index in 0..options.programs {
        let seed = if index == 0 {
            options.base_seed
        } else {
            crate::generate::Rng::mix(options.base_seed, index)
        };
        let program = crate::generate::program(seed);
        match measure_program(gleam, project, &program, seed) {
            Some(result) => {
                println!(
                    "  [{}/{}] seed {seed}: repeats {}, per-iteration erlang {}, jit {}, aot {} ({:.0}s elapsed)",
                    index + 1,
                    options.programs,
                    result.repeats,
                    format_seconds(result.exec.erlang),
                    format_seconds(result.exec.jit),
                    format_seconds(result.exec.aot),
                    started.elapsed().as_secs_f64()
                );
                results.push(result);
            }
            None => println!(
                "  [{}/{}] seed {seed}: SKIPPED (a run failed or timed out; check with the fuzzer: cargo run -p test-fuzz -- --seed {seed})",
                index + 1,
                options.programs
            ),
        }
    }

    if results.is_empty() {
        println!("no programs completed; nothing to report");
        std::process::exit(1);
    }

    print_report(&results, &startup);
    save_json(&results, &startup, options.base_seed);
}

// -- Measurement ------------------------------------------------------

/// Warm-run startup times for a trivial program on each target.
fn measure_startup(gleam: &Path, project: &Path) -> Targets<f64> {
    write_source(project, "pub fn main() {\n  Nil\n}\n");
    let erlang = ["run", "--target", "erlang"];
    let native = ["run", "--target", "native"];
    assert!(warm(gleam, project, &erlang), "trivial erlang run failed");
    assert!(warm(gleam, project, &native), "trivial native run failed");
    assert!(warm(gleam, project, &["export", "native"]), "trivial export failed");
    let binary = project.join("fuzz");
    Targets {
        erlang: sample(STARTUP_SAMPLES, || timed(gleam, project, &erlang))
            .expect("trivial erlang run failed"),
        jit: sample(STARTUP_SAMPLES, || timed(gleam, project, &native))
            .expect("trivial native run failed"),
        aot: sample(STARTUP_SAMPLES, || timed(&binary, project, &[]))
            .expect("trivial aot run failed"),
    }
}

/// All measurements for one generated program, or `None` if any step
/// fails (the fuzzer is the tool for diagnosing those seeds).
fn measure_program(
    gleam: &Path,
    project: &Path,
    program: &str,
    seed: u64,
) -> Option<ProgramResult> {
    let erlang = ["run", "--target", "erlang"];
    let native = ["run", "--target", "native"];
    let export = ["export", "native"];
    let binary = project.join("fuzz");

    // Single-repeat baselines.
    write_source(project, &with_repeats(program, 1));
    if !warm(gleam, project, &erlang) || !warm(gleam, project, &native) {
        return None;
    }
    let erlang_one = sample(SAMPLES, || timed(gleam, project, &erlang))?;
    let jit_one = sample(SAMPLES, || timed(gleam, project, &native))?;

    // Cold compile times, while the single-repeat source is in place.
    let compile = Targets {
        erlang: sample(SAMPLES, || {
            remove_build(project, "dev/erlang");
            timed(gleam, project, &["build", "--target", "erlang"])
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
    if !warm(gleam, project, &erlang) || !warm(gleam, project, &native) {
        return None;
    }
    let erlang_many = sample(SAMPLES, || timed(gleam, project, &erlang))?;
    let jit_many = sample(SAMPLES, || timed(gleam, project, &native))?;
    if !warm(gleam, project, &export) {
        return None;
    }
    let aot_many = sample(SAMPLES, || timed(&binary, project, &[]))?;

    let per_iteration = |many: f64, one: f64| ((many - one) / (repeats - 1) as f64).max(0.0);
    Some(ProgramResult {
        seed,
        repeats,
        exec: Targets {
            erlang: per_iteration(erlang_many, erlang_one),
            jit: per_iteration(jit_many, jit_one),
            aot: per_iteration(aot_many, aot_one),
        },
        compile,
    })
}

/// The generated program with its `main` renamed to `bench_work` and a
/// new `main` driving it through a tail-recursive repeat loop.
fn with_repeats(program: &str, repeats: u64) -> String {
    let renamed = program.replacen("pub fn main() {", "fn bench_work() {", 1);
    format!(
        "{renamed}\nfn bench_repeat(n: Int) -> Nil {{\n  case n < 1 {{\n    True -> Nil\n    False -> {{\n      bench_work()\n      bench_repeat(n - 1)\n    }}\n  }}\n}}\n\npub fn main() {{\n  bench_repeat({repeats})\n}}\n"
    )
}

fn write_source(project: &Path, source: &str) {
    std::fs::write(project.join("src").join("fuzz.gleam"), source).expect("write program");
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
/// a timeout can kill the whole tree (see the fuzzer's runner).
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
        "  {:>20} {:>9} {:>10} {:>10} {:>10} {:>8} {:>8}",
        "seed", "repeats", "erlang", "jit", "aot", "jit x", "aot x"
    );
    for result in results {
        println!(
            "  {:>20} {:>9} {:>10} {:>10} {:>10} {:>8} {:>8}",
            result.seed,
            result.repeats,
            format_seconds(result.exec.erlang),
            format_seconds(result.exec.jit),
            format_seconds(result.exec.aot),
            format_ratio(result.exec.erlang, result.exec.jit),
            format_ratio(result.exec.erlang, result.exec.aot),
        );
    }
    println!(
        "  {:>20} {:>9} {:>10} {:>10} {:>10} {:>8} {:>8}",
        "geometric mean",
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
        "  {:>20} {:>10} {:>10} {:>10}",
        "seed", "erlang", "jit", "aot"
    );
    for result in results {
        println!(
            "  {:>20} {:>9.0}ms {:>9.0}ms {:>9.0}ms",
            result.seed,
            result.compile.erlang * 1000.0,
            result.compile.jit * 1000.0,
            result.compile.aot * 1000.0,
        );
    }
    println!(
        "  {:>20} {:>9.0}ms {:>9.0}ms {:>9.0}ms",
        "mean",
        mean(results, |result| result.compile.erlang) * 1000.0,
        mean(results, |result| result.compile.jit) * 1000.0,
        mean(results, |result| result.compile.aot) * 1000.0,
    );

    println!();
    println!(
        "Startup (trivial program, warm): erlang {:.1}ms, jit {:.1}ms, aot {:.1}ms",
        startup.erlang * 1000.0,
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
        programs.push_str(&format!(
            r#"{{"seed":{},"repeats":{},"exec_seconds":{{"erlang":{:e},"jit":{:e},"aot":{:e}}},"compile_seconds":{{"erlang":{:e},"jit":{:e},"aot":{:e}}}}}"#,
            result.seed,
            result.repeats,
            result.exec.erlang,
            result.exec.jit,
            result.exec.aot,
            result.compile.erlang,
            result.compile.jit,
            result.compile.aot,
        ));
    }
    let json = format!(
        r#"{{"base_seed":{base_seed},"startup_seconds":{{"erlang":{:e},"jit":{:e},"aot":{:e}}},"programs":[{programs}]}}"#,
        startup.erlang, startup.jit, startup.aot,
    );
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("bench-results.json");
    std::fs::write(&path, json).expect("write results");
    println!("results saved to {}", path.display());
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
