// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2025 The Gleam contributors

use std::{io::Read, process::Stdio};

use camino::{Utf8Path, Utf8PathBuf};
use gleam_core::{
    build::{Runtime, Target},
    io::Command,
    paths::ProjectPaths,
};

use gleam_cli::{
    fs,
    run::{self, Which},
};

fn run_and_produce_pretty_snapshot(
    target: Option<Target>,
    runtime: Option<Runtime>,
    project_directory: Utf8PathBuf,
) -> String {
    let project_root = fs::get_project_root(project_directory).expect("project root");

    // Node has a bug with paths on Windows at the moment, so we have to
    // edit the path to work around it.
    // https://github.com/nodejs/node/issues/60435
    #[cfg(windows)]
    let project_root = project_root
        .to_string()
        .strip_prefix(r"\\?\")
        .map(Utf8PathBuf::from)
        .unwrap_or(project_root);
    let paths = ProjectPaths::new(project_root);

    let output = run_and_capture_output(&paths, "main", target, runtime)
        // Since the echo output's contains a path we will replace the `\` with a `/`
        // so that the snapshot doesn't fail on Windows in CI.
        .replace("src\\", "src/");

    let main_module_content =
        fs::read(paths.src_directory().join("main.gleam")).expect("read main module");

    format!(
        "--- main.gleam ----------------------
{main_module_content}

--- gleam run output ----------------
{output}
"
    )
}

/// Builds the `native-runtime-static` library that native compilation links
/// executables against, returning its path in the workspace target
/// directory (a test binary's own directory is `target/{profile}/deps`, so
/// the sibling lookup would not find it).
fn ensure_runtime_static_library() -> Utf8PathBuf {
    static ONCE: std::sync::Once = std::sync::Once::new();
    let workspace = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .to_path_buf();
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    ONCE.call_once(|| {
        let mut command =
            std::process::Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into()));
        let _ = command.args(["build", "-p", "native-runtime-static"]);
        if !cfg!(debug_assertions) {
            let _ = command.arg("--release");
        }
        let output = command
            .current_dir(&workspace)
            .output()
            .expect("build native-runtime-static");
        assert!(
            output.status.success(),
            "building native-runtime-static failed.\nstderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    });
    workspace
        .join("target")
        .join(profile)
        .join("libnative_runtime_static.a")
}

/// Compiles the case ahead of time for the native target and runs the
/// resulting executable. The AOT and JIT paths share their code generation
/// and runtime, so this is the native execution the suite verifies.
fn run_native_case(paths: &ProjectPaths, main_module: &str) -> std::process::Output {
    let runtime_library = ensure_runtime_static_library();
    let executable = paths.build_directory().join("test-executable");
    gleam_cli::export::native_executable_for_tests(
        paths,
        main_module,
        &executable,
        Some(runtime_library),
    )
    .expect("compile the case to a native executable");
    std::process::Command::new(&executable)
        .stdin(Stdio::null())
        .current_dir(paths.root())
        .output()
        .expect("run the native executable")
}

fn run_and_capture_output(
    paths: &ProjectPaths,
    main_module: &str,
    target: Option<Target>,
    runtime: Option<Runtime>,
) -> String {
    fs::delete_directory(&paths.build_directory()).expect("delete build directory content");

    if target == Some(Target::Native) {
        let output = run_native_case(paths, main_module);
        return String::from_utf8_lossy(&output.stderr).into_owned();
    }

    let Command {
        program,
        args,
        env,
        cwd: _,
        stdio: _,
    } = run::setup(
        paths,
        vec![],
        target,
        runtime,
        Some(main_module.into()),
        Which::Src,
        true,
    )
    .expect("run setup");

    let mut process = std::process::Command::new(&program)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .envs(env.iter().map(|pair| (&pair.0, &pair.1)))
        .current_dir(paths.root())
        .spawn()
        .unwrap_or_else(|error| panic!("Failed to spawn process '{}': {}", &program, &error));

    let mut stderr = process.stderr.take().expect("take stderr");
    let mut output = String::new();
    let _ = stderr.read_to_string(&mut output).expect("read stderr");
    let _ = process.wait().expect("run with no errors");
    output
}

macro_rules! assert_output {
    ($project_name: expr) => {
        let snapshot_name = snapshot_name(None, None, $project_name);
        insta::allow_duplicates! {
            assert_output!(&snapshot_name, Some(Target::Erlang), None, $project_name);
            assert_output!(&snapshot_name, Some(Target::JavaScript), Some(Runtime::Bun), $project_name);
            assert_output!(&snapshot_name, Some(Target::JavaScript), Some(Runtime::Deno), $project_name);
            assert_output!(&snapshot_name, Some(Target::JavaScript), Some(Runtime::NodeJs), $project_name);
            // Native compilation needs a Unix C toolchain.
            if !cfg!(windows) {
                assert_output!(&snapshot_name, Some(Target::Native), None, $project_name);
            }
        }
    };

    // For behavior JavaScript legitimately computes differently (its
    // integers are 64-bit floats): the Erlang and native targets share the
    // snapshot.
    (no_javascript: $project_name: expr) => {
        let snapshot_name = snapshot_name(None, None, $project_name);
        insta::allow_duplicates! {
            assert_output!(&snapshot_name, Some(Target::Erlang), None, $project_name);
            // Native compilation needs a Unix C toolchain.
            if !cfg!(windows) {
                assert_output!(&snapshot_name, Some(Target::Native), None, $project_name);
            }
        }
    };

    // For cases every target but native runs: conformance cases test the
    // compiler and must not depend on the standard library, so cases that
    // need it are excluded from the native row by design.
    (no_native: $project_name: expr) => {
        let snapshot_name = snapshot_name(None, None, $project_name);
        insta::allow_duplicates! {
            assert_output!(&snapshot_name, Some(Target::Erlang), None, $project_name);
            assert_output!(&snapshot_name, Some(Target::JavaScript), Some(Runtime::Bun), $project_name);
            assert_output!(&snapshot_name, Some(Target::JavaScript), Some(Runtime::Deno), $project_name);
            assert_output!(&snapshot_name, Some(Target::JavaScript), Some(Runtime::NodeJs), $project_name);
        }
    };

    ($target: expr, $project_name: expr) => {
        let snapshot_name = snapshot_name(Some($target), None, $project_name);
        match $target {
            Target::JavaScript => insta::allow_duplicates! {
                assert_output!(&snapshot_name, Some($target), Some(Runtime::Bun), $project_name);
                assert_output!(&snapshot_name, Some($target), Some(Runtime::Deno), $project_name);
                assert_output!(&snapshot_name, Some($target), Some(Runtime::NodeJs), $project_name);
            },
            Target::Erlang => {
                assert_output!(&snapshot_name, Some($target), None, $project_name);
            },
            Target::Native => {
                // Native compilation needs a Unix C toolchain.
                if !cfg!(windows) {
                    assert_output!(&snapshot_name, Some($target), None, $project_name);
                }
            },
        }
    };

    ($snapshot_name: expr, $target: expr, $runtime: expr, $project_name: expr) => {
        let path = fs::canonicalise(&Utf8Path::new("../test-output/cases").join($project_name))
            .expect("canonicalise path");
        let output = run_and_produce_pretty_snapshot($target, $runtime, path);
        insta::assert_snapshot!($snapshot_name.to_string(), output);
    };
}

/// The pretty snapshot for a native-only case: these programs print to
/// both standard streams and may exit non-zero, so the whole observable
/// outcome is captured.
fn run_and_produce_native_case_snapshot(project_directory: Utf8PathBuf) -> String {
    let project_root = fs::get_project_root(project_directory).expect("project root");
    let paths = ProjectPaths::new(project_root);
    fs::delete_directory(&paths.build_directory()).expect("delete build directory content");
    let output = run_native_case(&paths, "main");
    let main_module_content =
        fs::read(paths.src_directory().join("main.gleam")).expect("read main module");
    let status = match output.status.code() {
        Some(code) => code.to_string(),
        None => "killed by a signal".into(),
    };
    format!(
        "--- main.gleam ----------------------
{main_module_content}

--- exit status ---------------------
{status}

--- standard output -----------------
{stdout}
--- standard error ------------------
{stderr}",
        stdout = String::from_utf8_lossy(&output.stdout),
        stderr = String::from_utf8_lossy(&output.stderr),
    )
}

/// A test over a native-only case: its program uses native externals, so no
/// other target can build it, and its snapshot captures both output streams
/// and the exit status.
macro_rules! assert_native_case {
    ($project_name: expr) => {
        // Native compilation needs a Unix C toolchain.
        if !cfg!(windows) {
            let path = fs::canonicalise(&Utf8Path::new("../test-output/cases").join($project_name))
                .expect("canonicalise path");
            let output = run_and_produce_native_case_snapshot(path);
            insta::assert_snapshot!($project_name.to_string(), output);
        }
    };
}

fn snapshot_name(target: Option<Target>, runtime: Option<Runtime>, suffix: &str) -> String {
    let show_target = |target: Target| match target {
        Target::Erlang => "erlang",
        Target::JavaScript => "javascript",
        Target::Native => "native",
    };
    let show_runtime = |runtime: Runtime| match runtime {
        Runtime::NodeJs => "nodejs",
        Runtime::Deno => "deno",
        Runtime::Bun => "bun",
    };
    let prefix = match (target, runtime) {
        (None, None) => "".into(),
        (None, Some(runtime)) => format!("{}-", show_runtime(runtime)),
        (Some(target), None) => format!("{}-", show_target(target)),
        (Some(target), Some(runtime)) => {
            format!("{}-{}-", show_target(target), show_runtime(runtime))
        }
    };
    format!("{prefix}{suffix}")
}

#[test]
fn echo_bitarray() {
    assert_output!(Target::JavaScript, "echo_bitarray");
    assert_output!(Target::Erlang, "echo_bitarray");
    assert_output!(Target::Native, "echo_bitarray");
}

#[test]
fn echo_bool() {
    assert_output!("echo_bool");
}

#[test]
fn echo_charlist() {
    assert_output!("echo_charlist");
}

#[test]
fn echo_custom_type() {
    assert_output!(Target::Erlang, "echo_custom_type");
    assert_output!(Target::JavaScript, "echo_custom_type");
    assert_output!(Target::Native, "echo_custom_type");
}

#[test]
fn echo_dict() {
    // The native target cannot build the standard library yet.
    assert_output!(no_native: "echo_dict");
}

#[test]
fn echo_float() {
    assert_output!(Target::Erlang, "echo_float");
    assert_output!(Target::JavaScript, "echo_float");
    assert_output!(Target::Native, "echo_float");
}

#[test]
fn echo_nan_infinity() {
    assert_output!(Target::JavaScript, "echo_nan_infinity");
}

#[test]
fn echo_function() {
    assert_output!("echo_function");
}

#[test]
fn echo_importing_module_named_inspect() {
    assert_output!("echo_importing_module_named_inspect");
}

#[test]
fn echo_int() {
    assert_output!("echo_int");
}

#[test]
fn echo_list() {
    assert_output!("echo_list");
}

#[test]
fn echo_nil() {
    assert_output!("echo_nil");
}

#[test]
fn echo_string() {
    assert_output!("echo_string");
}

#[test]
fn echo_tuple() {
    assert_output!("echo_tuple");
}

#[test]
fn echo_non_record_atom_tag() {
    assert_output!(Target::Erlang, "echo_non_record_atom_tag");
}

#[test]
fn echo_circular_reference() {
    assert_output!(Target::JavaScript, "echo_circular_reference");
}

#[test]
fn echo_singleton() {
    assert_output!("echo_singleton");
}

#[test]
fn echo_with_message() {
    assert_output!("echo_with_message");
}

#[test]
fn linked_process_exit() {
    assert_output!(Target::Erlang, "linked_process_exit");
}

#[test]
fn stack_trace() {
    assert_output!(Target::Erlang, "panic_stack_trace");
    assert_output!(Target::Native, "panic_stack_trace");
}

// The cases below were migrated from the end-to-end tests in
// `gleam-bin/tests/native_target.rs`. Most test general Gleam behavior and
// run on every target through `echo`'s shared output format; the
// remaining `native_*` cases use native-only runtime externals or test
// native-specific runtime behavior.

#[test]
fn tuples() {
    assert_output!("tuples");
}

#[test]
fn lists() {
    assert_output!("lists");
}

#[test]
fn bit_arrays() {
    assert_output!("bit_arrays");
}

#[test]
fn custom_types() {
    assert_output!("custom_types");
}

#[test]
fn closures_and_pipes() {
    assert_output!("closures_and_pipes");
}

#[test]
fn case_matching() {
    assert_output!("case_matching");
}

#[test]
fn arithmetic() {
    assert_output!("arithmetic");
}

#[test]
fn big_integers() {
    assert_output!(no_javascript: "big_integers");
}

#[test]
fn bit_arrays_wide() {
    assert_output!(no_javascript: "bit_arrays_wide");
}

#[test]
fn destructuring() {
    assert_output!("destructuring");
}

#[test]
fn fizzbuzz() {
    assert_output!("fizzbuzz");
}

#[test]
fn negation_and_constant_forms() {
    assert_output!("negation_and_constant_forms");
}

#[test]
fn tail_recursion() {
    assert_output!("tail_recursion");
}

// Runtime failures are general Gleam behavior, but every target formats
// its report differently, so each target documents its own output. The
// JavaScript runtimes are skipped: their reports embed machine-specific
// absolute paths in stack traces.

#[test]
fn panic_with_message() {
    assert_output!(Target::Erlang, "panic_with_message");
    assert_native_case!("panic_with_message");
}

#[test]
fn todo_unimplemented() {
    assert_output!(Target::Erlang, "todo_unimplemented");
    assert_native_case!("todo_unimplemented");
}

#[test]
fn let_assert_failure() {
    assert_output!(Target::Erlang, "let_assert_failure");
    assert_native_case!("let_assert_failure");
}

#[test]
fn assert_failure() {
    assert_output!(Target::Erlang, "assert_failure");
    assert_native_case!("assert_failure");
}

// Native-only cases: `native_strings` exercises the native runtime's
// string externals directly, and the stack cases test the native
// runtime's dedicated program thread.

#[test]
fn native_strings() {
    assert_native_case!("native_strings");
}

#[test]
fn native_stack_overflow() {
    assert_native_case!("native_stack_overflow");
}

#[test]
fn native_configured_stack_size() {
    assert_native_case!("native_configured_stack_size");
}
