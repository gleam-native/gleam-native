// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! End-to-end tests for the native target's tooling: `gleam run` argument
//! passing, the built-in test runner, ahead-of-time export and
//! cross-compilation, and build artifact hygiene — everything that drives
//! the real `gleam` binary in ways the conformance suite cannot.
//!
//! Language-semantics execution tests live in `test-output`'s conformance
//! suite (the `native_*` cases and the native rows of the shared cases).

use std::path::PathBuf;
use std::process::Command;

/// A project directory in the system temp dir, removed on drop.
struct TestProject {
    root: PathBuf,
}

impl TestProject {
    fn new(name: &str, source: &str) -> Self {
        Self::with_config_extras(name, source, "")
    }

    fn with_config_extras(name: &str, source: &str, config_extras: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "gleam-native-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("src")).expect("create project directories");
        std::fs::write(
            root.join("gleam.toml"),
            format!(
                "name = \"{name}\"\nversion = \"1.0.0\"\ntarget = \"native\"\n{config_extras}"
            ),
        )
        .expect("write gleam.toml");
        std::fs::write(root.join("src").join(format!("{name}.gleam")), source)
            .expect("write source");
        Self { root }
    }

    fn run(&self) -> std::process::Output {
        self.run_with_arguments(&[])
    }

    /// A project with an additional `test/{name}_test.gleam` module.
    fn with_test_module(name: &str, source: &str, test_source: &str) -> Self {
        let project = Self::new(name, source);
        std::fs::create_dir_all(project.root.join("test")).expect("create test directory");
        std::fs::write(
            project.root.join("test").join(format!("{name}_test.gleam")),
            test_source,
        )
        .expect("write test source");
        project
    }

    fn run_with_arguments(&self, arguments: &[&str]) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_gleam"))
            .arg("run")
            .args(arguments)
            .current_dir(&self.root)
            .output()
            .expect("run the gleam binary")
    }

    fn test(&self) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_gleam"))
            .arg("test")
            .current_dir(&self.root)
            .output()
            .expect("run the gleam binary")
    }

    /// Runs `gleam export native` with extra arguments and environment
    /// variables, returning the process output.
    fn export_native_command(
        &self,
        arguments: &[&str],
        environment: &[(&str, &str)],
    ) -> std::process::Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_gleam"));
        let _ = command
            .args(["export", "native"])
            .args(arguments)
            .current_dir(&self.root);
        for (name, value) in environment {
            let _ = command.env(name, value);
        }
        command.output().expect("run the gleam binary")
    }

    /// Compiles the project ahead of time with `gleam export native` and
    /// returns the path of the generated executable.
    fn export_native(&self, name: &str) -> PathBuf {
        ensure_runtime_static_library();
        let output = self.export_native_command(&[], &[]);
        assert!(
            output.status.success(),
            "gleam export native failed.\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let executable = self.root.join(name);
        assert!(executable.is_file(), "no executable at {executable:?}");
        executable
    }
}

/// Builds the `native-runtime-static` library that `gleam export native`
/// links executables against, so it sits next to the `gleam` binary under
/// test whichever packages this test run happened to build.
fn ensure_runtime_static_library() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("workspace root")
            .to_path_buf();
        let mut command = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into()));
        let _ = command.args(["build", "-p", "native-runtime-static"]);
        if !cfg!(debug_assertions) {
            let _ = command.arg("--release");
        }
        let output = command
            .current_dir(workspace)
            .output()
            .expect("build native-runtime-static");
        assert!(
            output.status.success(),
            "building native-runtime-static failed.\nstderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    });
}

impl Drop for TestProject {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

#[test]
fn arguments_and_exit_codes() {
    let project = TestProject::new(
        "arguments",
        r#"@external(native, "runtime", "println")
pub fn println(text: String) -> Nil

@external(native, "runtime", "gleam_native_start_arguments")
fn start_arguments() -> List(String)

@external(native, "runtime", "gleam_native_exit")
fn exit(code: Int) -> Nil

fn print_each(arguments: List(String)) -> Int {
  case arguments {
    [] -> 0
    [argument, ..rest] -> {
      println(argument)
      1 + print_each(rest)
    }
  }
}

pub fn main() -> Nil {
  let count = print_each(start_arguments())
  // Exit with the number of arguments as the code.
  exit(count)
  println("unreachable")
}
"#,
    );

    let output = project.run_with_arguments(&["alpha", "beta", "gamma"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        output.status.code(),
        Some(3),
        "expected exit code 3.
stdout: {stdout}
stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        stdout.contains("alpha
beta
gamma
"),
        "unexpected output.
stdout: {stdout}"
    );
    assert!(!stdout.contains("unreachable"));

    // Without arguments the program exits zero.
    let output = project.run();
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn outdated_artifacts_are_regenerated() {
    // An artifact from an older compiler (a format version bump, or plain
    // corruption) must make the build recompile the module and regenerate
    // it — never leave the project failing at run time until a manual
    // clean build.
    let project = TestProject::new(
        "outdated_artifact",
        r#"pub fn main() -> Nil {
  echo "healed"
  Nil
}
"#,
    );
    let output = project.run();
    assert!(
        output.status.success(),
        "the first build should run.
stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Overwrite the compiled artifact with bytes that are not a current
    // artifact, standing in for any older format.
    let artifact = project
        .root
        .join("build/dev/native/outdated_artifact/_gleam_artefacts/outdated_artifact.nir");
    assert!(artifact.is_file(), "no artifact at {artifact:?}");
    std::fs::write(&artifact, b"stale-old-format").expect("overwrite artifact");

    let output = project.run();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "the rebuild should regenerate the outdated artifact and run.
stderr: {stderr}"
    );
    assert!(
        stderr.contains("\"healed\""),
        "unexpected output.
stderr: {stderr}"
    );
}

#[test]
fn removed_modules_do_not_leave_stale_artifacts() {
    // Deleting a module must also drop its compiled `.nir` artifact on the
    // next build: the native runner loads every artifact in the build
    // directory, so a stale one referencing since-removed functions would
    // otherwise break `gleam run` until a clean build.
    let project = TestProject::new(
        "stale_artifacts",
        r#"@external(native, "runtime", "println")
pub fn println(text: String) -> Nil

pub fn shout() -> Nil {
  println("shout")
}

pub fn main() -> Nil {
  println("first version")
}
"#,
    );
    let extra_path = project.root.join("src").join("extra.gleam");
    std::fs::write(
        &extra_path,
        r#"import stale_artifacts

pub fn noisy() -> Nil {
  stale_artifacts.shout()
}
"#,
    )
    .expect("write extra module");

    let output = project.run();
    assert!(
        output.status.success(),
        "the first build should run.
stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Remove the extra module and the function it referenced. The sleep
    // keeps the rewritten source's modification time clearly newer than
    // the cached one.
    std::thread::sleep(std::time::Duration::from_millis(1100));
    std::fs::remove_file(&extra_path).expect("delete extra module");
    std::fs::write(
        project.root.join("src").join("stale_artifacts.gleam"),
        r#"@external(native, "runtime", "println")
pub fn println(text: String) -> Nil

pub fn main() -> Nil {
  println("second version")
}
"#,
    )
    .expect("rewrite source");

    let output = project.run();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "the rebuild should prune the removed module's artifact and run.
stdout: {stdout}
stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        stdout.contains("second version"),
        "unexpected output.
stdout: {stdout}"
    );
}

#[test]
fn test_runner_passing_suite() {
    // `gleam test` discovers public zero-argument `*_test` functions in the
    // test directory and runs each; the test module imports the code under
    // test with module-qualified calls, constants, and constructors.
    let project = TestProject::with_test_module(
        "runner_pass",
        r#"pub const answer = 42

pub type Shape {
  Circle(radius: Float)
  Point
}

pub type Config {
  Config(host: String, port: Int)
}

pub const default = Config("localhost", 80)

pub fn add(a: Int, b: Int) -> Int {
  a + b
}
"#,
        r#"import runner_pass

pub fn add_test() {
  assert runner_pass.add(40, runner_pass.answer - 40) == 42
}

pub fn qualified_update_test() {
  let config = runner_pass.Config(..runner_pass.default, port: 443)
  assert config.port == 443
  assert config.host == "localhost"
}

pub fn shapes_test() {
  let shape = runner_pass.Circle(2.0)
  assert shape != runner_pass.Point
  let make = runner_pass.Circle
  assert make(2.0) == shape
}

fn helper() -> Int {
  1
}

pub fn not_a_test(value: Int) -> Int {
  value + helper()
}
"#,
    );

    let output = project.test();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        output.status.code(),
        Some(0),
        "a passing suite should exit 0.
stdout: {stdout}
stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        stdout.contains("  PASS runner_pass_test.add_test\n")
            && stdout.contains("  PASS runner_pass_test.shapes_test\n"),
        "expected a PASS line per test.
stdout: {stdout}"
    );
    assert!(
        stdout.contains("Ran 3 tests, 0 failures"),
        "helper, non-test, and private functions must not be discovered.
stdout: {stdout}"
    );
}

#[test]
fn test_runner_continues_after_a_failure() {
    // A failing test is reported and the remaining tests still run; the
    // process exits 1 with a summary counting the failure.
    let project = TestProject::with_test_module(
        "runner_fail",
        r#"pub fn add(a: Int, b: Int) -> Int {
  a + b
}
"#,
        r#"import runner_fail

pub fn a_first_test() {
  assert runner_fail.add(1, 1) == 2
}

pub fn b_broken_test() {
  assert runner_fail.add(2, 2) == 5 as "two and two is not five"
}

pub fn c_last_test() {
  assert runner_fail.add(3, 3) == 6
}
"#,
    );

    let output = project.test();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        output.status.code(),
        Some(1),
        "a failing suite should exit 1.
stdout: {stdout}
stderr: {stderr}"
    );
    assert!(
        stdout.contains("  PASS runner_fail_test.a_first_test\n")
            && stdout.contains("  FAIL runner_fail_test.b_broken_test\n")
            && stdout.contains("  PASS runner_fail_test.c_last_test\n"),
        "the test after the failure must still run.
stdout: {stdout}"
    );
    assert!(
        stdout.contains("Ran 3 tests, 1 failure"),
        "unexpected summary.
stdout: {stdout}"
    );
    assert!(
        stderr.contains("runtime error: assert") && stderr.contains("two and two is not five"),
        "the failure report should appear on stderr.
stderr: {stderr}"
    );
}

#[test]
fn test_runner_empty_suite() {
    // A project without tests runs cleanly and reports zero tests.
    let project = TestProject::new(
        "runner_empty",
        r#"pub fn main() -> Nil {
  Nil
}
"#,
    );
    let output = project.test();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        output.status.code(),
        Some(0),
        "an empty suite should exit 0.
stdout: {stdout}
stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        stdout.contains("Ran 0 tests, 0 failures"),
        "unexpected summary.
stdout: {stdout}"
    );
}

#[test]
fn export_native_executable() {
    // Ahead-of-time compilation: the exported executable must behave like
    // the JIT-run program, including runtime slow paths (big integers,
    // string building), the constructor names `echo` reads from the
    // embedded program data, command line arguments, and the exit code.
    let project = TestProject::new(
        "export_native",
        r#"@external(native, "runtime", "println")
pub fn println(text: String) -> Nil

@external(native, "runtime", "print_int")
pub fn print_int(value: Int) -> Nil

@external(native, "runtime", "gleam_native_start_arguments")
fn start_arguments() -> List(String)

@external(native, "runtime", "gleam_native_exit")
fn exit(code: Int) -> Nil

pub type Fruit {
  Apple(count: Int)
  Banana
}

fn print_each(arguments: List(String)) -> Int {
  case arguments {
    [] -> 0
    [argument, ..rest] -> {
      println(argument)
      1 + print_each(rest)
    }
  }
}

pub fn main() -> Nil {
  println("Hello from " <> "ahead of time!")
  print_int(4611686018427387903 * 4)
  let _ = echo Apple(3)
  exit(print_each(start_arguments()))
  println("unreachable")
}
"#,
    );

    let executable = project.export_native("export_native");
    let output = Command::new(&executable)
        .args(["alpha", "beta"])
        .current_dir(&project.root)
        .output()
        .expect("run the exported executable");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        output.status.code(),
        Some(2),
        "expected the argument count as the exit code.
stdout: {stdout}
stderr: {stderr}"
    );
    let expected = "Hello from ahead of time!
18446744073709551612
alpha
beta
";
    assert!(
        stdout.contains(expected),
        "unexpected output.
stdout: {stdout}"
    );
    assert!(!stdout.contains("unreachable"));
    assert!(
        stderr.contains("Apple(3)"),
        "echo should print the interned constructor name.
stderr: {stderr}"
    );
}

#[test]
fn export_native_cross_compiles_for_the_other_macos_architecture() {
    // Cross-compilation within macOS: an arm64 host produces an x86_64
    // executable and vice versa, linked with the host's C compiler. Skipped
    // off macOS, and when the runtime library cannot be cross-built (the
    // rustup target is not installed).
    if !cfg!(target_os = "macos") {
        eprintln!("skipped: cross-architecture link test requires macOS");
        return;
    }
    let (platform, triple, cpu_type) = if cfg!(target_arch = "aarch64") {
        ("macos-x64", "x86_64-apple-darwin", 0x0100_0007u32)
    } else {
        ("macos-arm64", "aarch64-apple-darwin", 0x0100_000C)
    };

    // Cross-build the runtime library into the workspace target directory,
    // where the export's development-layout lookup finds it.
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .to_path_buf();
    let mut command = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into()));
    let _ = command.args(["build", "-p", "native-runtime-static", "--target", triple]);
    if !cfg!(debug_assertions) {
        let _ = command.arg("--release");
    }
    let output = command
        .current_dir(workspace)
        .output()
        .expect("run cargo build");
    if !output.status.success() {
        eprintln!(
            "skipped: could not cross-build the runtime library for {triple} \
(is the rustup target installed?)"
        );
        return;
    }

    let project = TestProject::new(
        "export_cross",
        r#"@external(native, "runtime", "println")
pub fn println(text: String) -> Nil

pub fn main() -> Nil {
  println("crossed " <> "over")
}
"#,
    );
    let output = project.export_native_command(&["--platform", platform], &[]);
    assert!(
        output.status.success(),
        "gleam export native --platform {platform} failed.\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let executable = project.root.join("export_cross");
    let bytes = std::fs::read(&executable).expect("read the executable");
    assert_eq!(&bytes[..4], &0xFEED_FACFu32.to_le_bytes(), "not Mach-O 64");
    assert_eq!(
        u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
        cpu_type,
        "wrong architecture"
    );

    // Run it when the host can (arm64 Macs run x86_64 through Rosetta when
    // it is installed; Intel Macs cannot run arm64).
    if let Ok(output) = Command::new(&executable).current_dir(&project.root).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("crossed over"),
            "unexpected output.\nstdout: {stdout}"
        );
    } else {
        eprintln!("skipped execution: host cannot run {platform} binaries");
    }
}

#[test]
fn export_native_reports_a_missing_runtime_library() {
    let project = TestProject::new(
        "export_no_runtime",
        r#"pub fn main() -> Nil {
  Nil
}
"#,
    );
    let output = project.export_native_command(
        &[],
        &[("GLEAM_NATIVE_RUNTIME_LIB", "/nonexistent/libruntime.a")],
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        output.status.code(),
        Some(1),
        "expected failure.\nstderr: {stderr}"
    );
    assert!(
        stderr.contains("GLEAM_NATIVE_RUNTIME_LIB") && stderr.contains("does not exist"),
        "stderr: {stderr}"
    );
}

#[test]
fn export_native_configured_stack_size() {
    // The configured stack size travels in the executable's embedded
    // program data: a depth that fits in the default gigabyte must
    // overflow, and be reported, with a small configured stack.
    let project = TestProject::with_config_extras(
        "export_native_stack",
        r#"@external(native, "runtime", "print_int")
pub fn print_int(value: Int) -> Nil

fn deep(n: Int) -> Int {
  case n {
    0 -> 0
    _ -> 1 + deep(n - 1)
  }
}

pub fn main() -> Nil {
  print_int(deep(200_000))
}
"#,
        "\n[native]\nstack_size_megabytes = 4\n",
    );
    let executable = project.export_native("export_native_stack");
    let output = Command::new(&executable)
        .current_dir(&project.root)
        .output()
        .expect("run the exported executable");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        output.status.code(),
        Some(1),
        "a 4 MB stack should overflow.
stderr: {stderr}"
    );
    assert!(
        stderr.contains("runtime error: stack overflow"),
        "stderr: {stderr}"
    );
}

