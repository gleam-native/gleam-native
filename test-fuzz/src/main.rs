// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! Differential fuzzer for the native target: generates seeded, total,
//! well-typed Gleam programs (no standard library) and runs each on
//! Erlang (the semantic reference), the native JIT, and the native JIT
//! under `GLEAM_DEBUG_RC=1` (checked reference counting). The standard
//! output, filtered standard error, and exit codes of all three runs must
//! agree; any mismatch, crash, or timeout is saved as a reproducer.
//!
//! ```text
//! cargo run -p test-fuzz --release -- --iterations 200
//! cargo run -p test-fuzz --release -- --seed 12345      # reproduce one
//! cargo run -p test-fuzz --release -- --emit 12345      # print program
//! ```

mod bench;
mod generate;

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant, SystemTime};

struct RunOutput {
    stdout: String,
    stderr: String,
    code: Option<i32>,
    timed_out: bool,
}

fn main() {
    let mut iterations: Option<u64> = None;
    let mut base_seed: Option<u64> = None;
    let mut emit: Option<u64> = None;
    let mut bench = false;
    let mut arguments = std::env::args().skip(1);
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--bench" => bench = true,
            "--iterations" => {
                iterations = Some(
                    arguments
                        .next()
                        .and_then(|value| value.parse().ok())
                        .expect("--iterations takes a number"),
                );
            }
            "--seed" => {
                base_seed = Some(
                    arguments
                        .next()
                        .and_then(|value| value.parse().ok())
                        .expect("--seed takes a number"),
                );
            }
            "--emit" => {
                emit = Some(
                    arguments
                        .next()
                        .and_then(|value| value.parse().ok())
                        .expect("--emit takes a number"),
                );
            }
            other => panic!("unknown argument: {other}"),
        }
    }

    if let Some(seed) = emit {
        print!("{}", generate::program(seed));
        return;
    }

    // With an explicit seed the default is a single reproduction run.
    let iterations = iterations.unwrap_or(match (base_seed.is_some(), bench) {
        (true, _) => 1,
        (false, true) => 10,
        (false, false) => 100,
    });
    let base_seed = base_seed.unwrap_or_else(|| {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("clock")
            .as_nanos() as u64
    });

    let workspace = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .to_path_buf();
    let gleam = build_gleam(&workspace);
    let project = scratch_project();

    if bench {
        // `--iterations` is the number of benchmarked programs here.
        bench::main(
            &bench::Options {
                programs: iterations,
                base_seed,
            },
            &gleam,
            &project,
        );
        return;
    }
    println!("base seed: {base_seed}");

    let mut failures: Vec<u64> = Vec::new();
    let started = Instant::now();
    for index in 0..iterations {
        // The first iteration uses the base seed directly, so a failure's
        // printed seed reproduces its exact program with `--seed`; later
        // iterations mix the index in non-linearly.
        let seed = if index == 0 {
            base_seed
        } else {
            generate::Rng::mix(base_seed, index)
        };
        let program = generate::program(seed);
        std::fs::write(project.join("src").join("fuzz.gleam"), &program).expect("write program");

        let erlang = run(&gleam, &project, "erlang", false);
        let native = run(&gleam, &project, "native", false);
        let checked = run(&gleam, &project, "native", true);

        let mut problems = Vec::new();
        for (name, output) in [
            ("erlang", &erlang),
            ("native", &native),
            ("checked", &checked),
        ] {
            if output.timed_out {
                problems.push(format!("{name}: timed out"));
            } else if output.code != Some(0) {
                problems.push(format!("{name}: exit code {:?}", output.code));
            }
        }
        if native.stdout != erlang.stdout || native.stderr != erlang.stderr {
            problems.push("native output differs from erlang".to_string());
        }
        if checked.stdout != erlang.stdout || checked.stderr != erlang.stderr {
            problems.push("checked-RC output differs from erlang".to_string());
        }

        if problems.is_empty() {
            if (index + 1) % 10 == 0 {
                println!(
                    "  {}/{iterations} ok ({:.1}s)",
                    index + 1,
                    started.elapsed().as_secs_f64()
                );
            }
            continue;
        }

        failures.push(seed);
        let directory = workspace
            .join("test-fuzz")
            .join("failures")
            .join(seed.to_string());
        std::fs::create_dir_all(&directory).expect("create failure directory");
        std::fs::write(directory.join("fuzz.gleam"), &program).expect("save program");
        for (name, output) in [
            ("erlang", &erlang),
            ("native", &native),
            ("checked", &checked),
        ] {
            std::fs::write(directory.join(format!("{name}.stdout")), &output.stdout)
                .expect("save stdout");
            std::fs::write(directory.join(format!("{name}.stderr")), &output.stderr)
                .expect("save stderr");
        }
        std::fs::write(directory.join("problems.txt"), problems.join("\n")).expect("save problems");
        println!("FAILURE seed {seed}: {}", problems.join("; "));
        println!("  saved to {}", directory.display());
    }

    println!(
        "ran {iterations} programs in {:.1}s: {} failure(s)",
        started.elapsed().as_secs_f64(),
        failures.len()
    );
    if !failures.is_empty() {
        println!("reproduce with: cargo run -p test-fuzz -- --seed <seed>");
        std::process::exit(1);
    }
}

/// Builds the workspace's `gleam` binary (same profile as the fuzzer) and
/// returns its path.
fn build_gleam(workspace: &Path) -> PathBuf {
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    let mut command = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into()));
    let _ = command.args(["build", "-p", "gleam"]);
    if !cfg!(debug_assertions) {
        let _ = command.arg("--release");
    }
    let status = command
        .current_dir(workspace)
        .status()
        .expect("run cargo build");
    assert!(status.success(), "building the gleam binary failed");
    workspace.join("target").join(profile).join("gleam")
}

/// A scratch project the generated programs are written into.
fn scratch_project() -> PathBuf {
    let root = std::env::temp_dir().join(format!("gleam-native-fuzz-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("src")).expect("create scratch project");
    std::fs::write(
        root.join("gleam.toml"),
        "name = \"fuzz\"\nversion = \"1.0.0\"\n",
    )
    .expect("write gleam.toml");
    root
}

/// Runs the current program on one target, with a timeout, normalizing
/// away the build tool's progress lines (their timings differ run to
/// run).
fn run(gleam: &Path, project: &Path, target: &str, checked_rc: bool) -> RunOutput {
    let mut command = Command::new(gleam);
    let _ = command
        .args(["run", "--target", target])
        .current_dir(project)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    if checked_rc {
        let _ = command.env("GLEAM_DEBUG_RC", "1");
    }
    // A process group of its own, so a timeout can kill the whole tree:
    // killing only `gleam` would orphan a hanging `escript`, which keeps
    // holding the project's build lock and stalls every following run.
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        let _ = command.process_group(0);
    }
    let mut child = command.spawn().expect("spawn gleam");

    // Drain both pipes concurrently: a program writing more than the pipe
    // buffer would otherwise block forever and read as a timeout.
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

    let deadline = Instant::now() + Duration::from_secs(60);
    let status = loop {
        match child.try_wait().expect("wait for gleam") {
            Some(status) => break Some(status),
            None if Instant::now() > deadline => {
                // The child is its own process group leader; signal the
                // whole group.
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
            None => std::thread::sleep(Duration::from_millis(20)),
        }
    };
    let stdout = stdout_reader.join().expect("collect stdout");
    let stderr = stderr_reader.join().expect("collect stderr");
    RunOutput {
        stdout: String::from_utf8_lossy(&stdout).into_owned(),
        stderr: filter_progress(&String::from_utf8_lossy(&stderr)),
        code: status.and_then(|status| status.code()),
        timed_out: status.is_none(),
    }
}

fn filter_progress(stderr: &str) -> String {
    stderr
        .lines()
        .filter(|line| {
            let line = line.trim_start();
            ![
                "Compiling",
                "Compiled",
                "Resolving",
                "Downloading",
                "Downloaded",
                "Running",
            ]
            .iter()
            .any(|prefix| line.starts_with(prefix))
        })
        .collect::<Vec<_>>()
        .join("\n")
}
