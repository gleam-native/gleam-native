// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2022 The Gleam contributors

use crate::fs::{self, ZipArchive};
use camino::Utf8PathBuf;
use gleam_core::{
    Result,
    analyse::TargetSupport,
    build::{Codegen, Compile, ErlangOutput, Mode, Options, Target},
    error::{Error, ShellCommandFailureReason},
    paths::ProjectPaths,
    type_::ModuleFunction,
};
use std::io::Cursor;

static ENTRYPOINT_FILENAME_POWERSHELL: &str = "entrypoint.ps1";
static ENTRYPOINT_FILENAME_POSIX_SHELL: &str = "entrypoint.sh";

static ENTRYPOINT_TEMPLATE_POWERSHELL: &str =
    include_str!("../templates/erlang-shipment-entrypoint.ps1");
static ENTRYPOINT_TEMPLATE_POSIX_SHELL: &str =
    include_str!("../templates/erlang-shipment-entrypoint.sh");

/// Generate a single file of precompiled Erlang, suitable for CLIs.
///
pub fn escript(paths: &ProjectPaths) -> Result<()> {
    let target = Target::Erlang;
    let mode = Mode::Prod;
    let build = paths.build_directory_for_target(mode, target);

    // Reset the directories to ensure we have a clean slate and no old code
    fs::delete_directory(&build)?;

    let manifest = crate::build::download_dependencies(paths, crate::cli::Reporter::new())?;

    // Build project in production mode
    let build_options = Options {
        root_target_support: TargetSupport::Enforced,
        warnings_as_errors: false,
        codegen: Codegen::All,
        compile: Compile::All,
        mode,
        target: Some(target),
        no_print_progress: false,
        erlang_output: ErlangOutput::Binary,
    };
    let built = crate::build::main(paths, build_options, manifest)?;
    let package_name = &built.root_package.config.name;

    // The main function must exist for the escript to call. This will return an
    // error if it could not be found.
    let _: ModuleFunction = built.get_main_function(package_name, target)?;

    // Create the zip archive for the code
    let mut zip = ZipArchive::new(Cursor::new(Vec::new()));

    for entry in fs::read_dir(&build)?.filter_map(Result::ok) {
        let ebin = entry.path().join("ebin");

        // We want the ebin code directories for each package
        if !ebin.is_dir() {
            continue;
        }

        for entry in fs::read_dir(&ebin)?.filter_map(Result::ok) {
            let path = entry.path();
            let extension = path.extension().unwrap_or_default();

            let Some(name) = path.file_name() else {
                continue;
            };

            if !path.is_file() {
                continue;
            }

            // We want to copy compiled BEAM bytecode and app configuration files
            if extension != "beam" && extension != "app" {
                continue;
            }

            zip.add_file_from_disc(path, name)?;
        }
    }

    let zip = zip.finish()?.into_inner();

    let escript_path = paths.root().join(package_name.as_str());
    let mut file = fs::open_file(&escript_path)?;

    // The -escript flag in the header instructs the BEAM `escript` program
    // to run the regular Gleam entrypoint module when running this escript.
    let header = format!(
        "#!/usr/bin/env escript
%%
%%!-escript main {package_name}@@main
"
    );

    fs::write_to_open_file(&mut file, &escript_path, header)?;
    fs::write_to_open_file(&mut file, &escript_path, zip)?;
    fs::make_executable(&escript_path)?;

    // Windows shells largely do not use shebangs, so for the escript to be
    // directly executable a .cmd wrapper script is provided.
    if cfg!(windows) {
        let cmd_path = escript_path.with_extension("cmd");
        fs::write(&cmd_path, "@echo off\r\nescript.exe \"%~dpn0\" %*\r\n")?;
    }

    println!(
        "
Your escript has been generated to {escript_path}.
",
    );

    Ok(())
}

/// Generate a directory of precompiled Erlang along with a start script.
/// Suitable for deployment to a server.
///
/// For each Erlang application (aka package) directory these directories are
/// copied across:
/// - ebin
/// - include
/// - priv
pub(crate) fn erlang_shipment(paths: &ProjectPaths) -> Result<()> {
    let target = Target::Erlang;
    let mode = Mode::Prod;
    let build = paths.build_directory_for_target(mode, target);
    let out = paths.erlang_shipment_directory();

    fs::mkdir(&out)?;

    // Reset the directories to ensure we have a clean slate and no old code
    fs::delete_directory(&build)?;
    fs::delete_directory(&out)?;

    // Build project in production mode
    let built = crate::build::main(
        paths,
        Options {
            root_target_support: TargetSupport::Enforced,
            warnings_as_errors: false,
            codegen: Codegen::All,
            compile: Compile::All,
            mode,
            target: Some(target),
            no_print_progress: false,
            erlang_output: ErlangOutput::Binary,
        },
        crate::build::download_dependencies(paths, crate::cli::Reporter::new())?,
    )?;

    for entry in fs::read_dir(&build)?.filter_map(Result::ok) {
        let path = entry.path();

        // We are only interested in package directories
        if !path.is_dir() {
            continue;
        }

        let name = path.file_name().expect("Directory name");
        let build = build.join(name);
        let out = out.join(name);
        fs::mkdir(&out)?;

        // Copy desired package subdirectories
        for subdirectory in ["ebin", "priv", "include"] {
            let source = build.join(subdirectory);
            if source.is_dir() {
                let source = fs::canonicalise(&source)?;
                let out = out.join(subdirectory);
                fs::copy_dir(source, &out)?;
            }
        }
    }

    // PowerShell entry point script.
    write_entrypoint_script(
        &out.join(ENTRYPOINT_FILENAME_POWERSHELL),
        ENTRYPOINT_TEMPLATE_POWERSHELL,
        &built.root_package.config.name,
    )?;

    // POSIX Shell entry point script.
    write_entrypoint_script(
        &out.join(ENTRYPOINT_FILENAME_POSIX_SHELL),
        ENTRYPOINT_TEMPLATE_POSIX_SHELL,
        &built.root_package.config.name,
    )?;

    crate::cli::print_exported(&built.root_package.config.name);

    println!(
        "
Your Erlang shipment has been generated to {out}.

It can be copied to a compatible server with Erlang installed and run with
one of the following scripts:
    - {ENTRYPOINT_FILENAME_POWERSHELL} (PowerShell script)
    - {ENTRYPOINT_FILENAME_POSIX_SHELL} (POSIX Shell script)
",
    );

    Ok(())
}

fn write_entrypoint_script(
    entrypoint_output_path: &Utf8PathBuf,
    entrypoint_template_path: &str,
    package_name: &str,
) -> Result<()> {
    let text = entrypoint_template_path.replace("$PACKAGE_NAME_FROM_GLEAM", package_name);
    fs::write(entrypoint_output_path, &text)?;
    fs::make_executable(entrypoint_output_path)?;
    Ok(())
}

/// A platform `gleam export native` can compile for. Every platform is a
/// 64-bit little-endian Unix; Windows needs a runtime port (signal-based
/// stack overflow handling, at least) before it can join.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum NativePlatform {
    /// 64-bit ARM Linux, statically linked against musl libc
    LinuxArm64,
    /// 64-bit x86 Linux, statically linked against musl libc
    LinuxX64,
    /// 64-bit ARM Linux, dynamically linked against glibc
    LinuxArm64Gnu,
    /// 64-bit x86 Linux, dynamically linked against glibc
    LinuxX64Gnu,
    /// Apple silicon macOS
    MacosArm64,
    /// Intel macOS
    MacosX64,
}

impl NativePlatform {
    /// The command line name, as clap renders it.
    fn name(self) -> &'static str {
        match self {
            Self::LinuxArm64 => "linux-arm64",
            Self::LinuxX64 => "linux-x64",
            Self::LinuxArm64Gnu => "linux-arm64-gnu",
            Self::LinuxX64Gnu => "linux-x64-gnu",
            Self::MacosArm64 => "macos-arm64",
            Self::MacosX64 => "macos-x64",
        }
    }

    /// The Rust/LLVM target triple: what Cranelift compiles for, and what
    /// `cargo build --target` expects when building the runtime library.
    pub fn triple(self) -> &'static str {
        match self {
            Self::LinuxArm64 => "aarch64-unknown-linux-musl",
            Self::LinuxX64 => "x86_64-unknown-linux-musl",
            Self::LinuxArm64Gnu => "aarch64-unknown-linux-gnu",
            Self::LinuxX64Gnu => "x86_64-unknown-linux-gnu",
            Self::MacosArm64 => "aarch64-apple-darwin",
            Self::MacosX64 => "x86_64-apple-darwin",
        }
    }

    /// The equivalent `zig cc -target` triple.
    fn zig_target(self) -> &'static str {
        match self {
            Self::LinuxArm64 => "aarch64-linux-musl",
            Self::LinuxX64 => "x86_64-linux-musl",
            Self::LinuxArm64Gnu => "aarch64-linux-gnu",
            Self::LinuxX64Gnu => "x86_64-linux-gnu",
            Self::MacosArm64 => "aarch64-macos",
            Self::MacosX64 => "x86_64-macos",
        }
    }

    /// The architecture as Rust's `std::env::consts::ARCH` names it.
    fn architecture(self) -> &'static str {
        match self {
            Self::LinuxArm64 | Self::LinuxArm64Gnu | Self::MacosArm64 => "aarch64",
            Self::LinuxX64 | Self::LinuxX64Gnu | Self::MacosX64 => "x86_64",
        }
    }

    fn is_linux(self) -> bool {
        matches!(
            self,
            Self::LinuxArm64 | Self::LinuxX64 | Self::LinuxArm64Gnu | Self::LinuxX64Gnu
        )
    }

    fn is_musl(self) -> bool {
        matches!(self, Self::LinuxArm64 | Self::LinuxX64)
    }

    fn is_macos(self) -> bool {
        matches!(self, Self::MacosArm64 | Self::MacosX64)
    }

    /// The `cc -arch` name for macOS platforms.
    fn macos_architecture(self) -> &'static str {
        match self {
            Self::MacosArm64 => "arm64",
            _ => "x86_64",
        }
    }

    /// Whether this platform is exactly the machine gleam is running on, so
    /// host-native artifacts (the plain runtime library) are usable.
    fn matches_host(self) -> bool {
        if self.architecture() != std::env::consts::ARCH {
            return false;
        }
        if self.is_macos() {
            return cfg!(target_os = "macos");
        }
        cfg!(target_os = "linux") && self.is_musl() == cfg!(target_env = "musl")
    }
}

/// Compile the project ahead of time into a native executable, written to
/// the project root and named after the package. Compiles for the machine
/// gleam is running on unless a platform is given.
///
/// The Cranelift-generated object file is linked against the
/// `native-runtime-static` library (which provides the runtime and the C
/// `main`). The host's C compiler driver links host-compatible platforms;
/// other platforms use `zig cc` or an explicit `--linker` command.
pub(crate) fn native(
    paths: &ProjectPaths,
    platform: Option<NativePlatform>,
    runtime_lib: Option<Utf8PathBuf>,
    linker: Option<String>,
) -> Result<()> {
    let target = Target::Native;
    let mode = Mode::Prod;
    let build = paths.build_directory_for_target(mode, target);

    // Reset the directories to ensure we have a clean slate and no old code
    fs::delete_directory(&build)?;

    // Build project in production mode
    let built = crate::build::main(
        paths,
        Options {
            root_target_support: TargetSupport::Enforced,
            warnings_as_errors: false,
            codegen: Codegen::All,
            compile: Compile::All,
            mode,
            target: Some(target),
            no_print_progress: false,
            erlang_output: ErlangOutput::Binary,
        },
        crate::build::download_dependencies(paths, crate::cli::Reporter::new())?,
    )?;
    let package_name = &built.root_package.config.name;

    // The main function must exist for the executable to call. This will
    // return an error if it could not be found.
    let _: ModuleFunction = built.get_main_function(package_name, target)?;

    let fail = |error: String| Error::NativeExecutableGeneration { error };

    let modules = crate::run::load_native_modules(&build).map_err(fail)?;
    let object = native_generation::object::compile(
        &modules,
        package_name,
        built.root_package.config.native.stack_size_megabytes,
        platform.map(NativePlatform::triple),
    )
    .map_err(fail)?;
    let object_path = build.join(format!("{package_name}.o"));
    fs::write_bytes(&object_path, &object)?;

    let runtime_library = runtime_static_library(platform, runtime_lib).map_err(fail)?;
    let executable_path = paths.root().join(package_name.as_str());
    let linker_command = select_linker(platform, linker).map_err(|error| {
        let (mut example, example_program) = match platform {
            Some(platform) => (format!("zig cc -target {}", platform.zig_target()), "zig"),
            None => ("cc".into(), "cc"),
        };
        example.push_str(&format!(
            " {object_path} {runtime_library} -o {executable_path}"
        ));
        for flag in link_flags(platform, example_program) {
            example.push(' ');
            example.push_str(flag);
        }
        fail(format!(
            "{error}

The compiled object file has been kept at:

    {object_path}

and the matching runtime library is at:

    {runtime_library}

so once a linker is available the manual link is:

    {example}"
        ))
    })?;
    link_executable(
        &linker_command,
        platform,
        &object_path,
        &runtime_library,
        &executable_path,
    )?;

    crate::cli::print_exported(package_name);
    match platform {
        Some(platform) => println!(
            "
Your {} executable has been generated to {executable_path}.
",
            platform.name()
        ),
        None => println!(
            "
Your native executable has been generated to {executable_path}.
",
        ),
    }

    Ok(())
}

/// The extra flags the link needs: the libraries the Rust runtime expects
/// on Linux, fully static linking on musl, and — when the linker is zig,
/// which has no implicit libgcc — zig's bundled libunwind for the
/// `_Unwind_*` symbols Rust's panic machinery references.
fn link_flags(platform: Option<NativePlatform>, linker_program: &str) -> Vec<&'static str> {
    let mut flags = vec![];
    let linux = platform.map_or(cfg!(target_os = "linux"), NativePlatform::is_linux);
    if linux {
        flags.extend(["-lpthread", "-ldl", "-lm"]);
    }
    if platform.is_some_and(NativePlatform::is_musl) {
        flags.push("-static");
    }
    let zig = std::path::Path::new(linker_program)
        .file_name()
        .is_some_and(|name| name.to_string_lossy().starts_with("zig"));
    if linux && zig {
        flags.push("-lunwind");
    }
    flags
}

/// Chooses the linker command: an explicit `--linker` or
/// `GLEAM_NATIVE_LINKER` override, the host's C compiler driver where it can
/// link the platform (its own, and on macOS the other architecture via
/// `-arch`), and `zig cc` for everything else.
fn select_linker(
    platform: Option<NativePlatform>,
    linker: Option<String>,
) -> Result<Vec<String>, String> {
    let split = |command: String, source: &str| {
        let words: Vec<String> = command.split_whitespace().map(String::from).collect();
        if words.is_empty() {
            return Err(format!("{source} is empty"));
        }
        Ok(words)
    };
    if let Some(command) = linker {
        return split(command, "--linker");
    }
    if let Ok(command) = std::env::var("GLEAM_NATIVE_LINKER") {
        return split(command, "GLEAM_NATIVE_LINKER");
    }
    let Some(platform) = platform else {
        return Ok(vec!["cc".into()]);
    };
    if platform.is_macos() && cfg!(target_os = "macos") {
        return Ok(vec![
            "cc".into(),
            "-arch".into(),
            platform.macos_architecture().into(),
        ]);
    }
    if platform.matches_host() {
        return Ok(vec!["cc".into()]);
    }
    if zig_is_available() {
        return Ok(vec![
            "zig".into(),
            "cc".into(),
            "-target".into(),
            platform.zig_target().into(),
        ]);
    }
    Err(format!(
        "there is no linker available for {}: this machine's C compiler \
cannot link for it, and `zig` was not found on PATH.

Install zig (https://ziglang.org) to cross-link executables, or pass a \
suitable cross linker with --linker (also settable as GLEAM_NATIVE_LINKER).",
        platform.name()
    ))
}

/// Whether `zig` can be run, making `zig cc` available as a cross linker.
fn zig_is_available() -> bool {
    std::process::Command::new("zig")
        .arg("version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

/// Links the object file and the runtime library into an executable with the
/// given C-driver-style linker command.
fn link_executable(
    linker: &[String],
    platform: Option<NativePlatform>,
    object: &Utf8PathBuf,
    runtime_library: &Utf8PathBuf,
    executable: &Utf8PathBuf,
) -> Result<()> {
    let (program, leading_arguments) = linker.split_first().expect("linker command is non-empty");
    let mut command = std::process::Command::new(program);
    let _ = command
        .args(leading_arguments)
        .arg(object)
        .arg(runtime_library)
        .arg("-o")
        .arg(executable)
        .args(link_flags(platform, program));
    let output = command.output().map_err(|error| match error.kind() {
        std::io::ErrorKind::NotFound => Error::ShellProgramNotFound {
            program: program.into(),
            os: fs::get_os(),
        },
        kind => Error::ShellCommand {
            program: program.into(),
            reason: ShellCommandFailureReason::IoError(kind),
        },
    })?;
    if !output.status.success() {
        return Err(Error::ShellCommand {
            program: program.into(),
            reason: ShellCommandFailureReason::ShellCommandError(
                String::from_utf8_lossy(&output.stderr).into_owned(),
            ),
        });
    }
    Ok(())
}

/// Locates the `native-runtime-static` library to link against, compiled
/// for the given platform (or the host). The search order is the
/// `--runtime-lib` flag, the `GLEAM_NATIVE_RUNTIME_LIB` environment
/// variable, a `libnative_runtime_static[-<triple>].a` next to the running
/// `gleam` binary, and — for development builds running from a cargo target
/// directory — cargo's `<triple>/<profile>` cross-compilation layout.
fn runtime_static_library(
    platform: Option<NativePlatform>,
    flag: Option<Utf8PathBuf>,
) -> Result<Utf8PathBuf, String> {
    if let Some(path) = flag {
        if path.is_file() {
            return Ok(path);
        }
        return Err(format!("--runtime-lib is {path}, which does not exist"));
    }
    if let Ok(path) = std::env::var("GLEAM_NATIVE_RUNTIME_LIB") {
        let path = Utf8PathBuf::from(path);
        if path.is_file() {
            return Ok(path);
        }
        return Err(format!(
            "GLEAM_NATIVE_RUNTIME_LIB is set to {path}, which does not exist"
        ));
    }
    let executable = std::env::current_exe()
        .map_err(|error| format!("could not locate the gleam executable: {error}"))?;
    let directory = executable
        .parent()
        .ok_or("could not locate the gleam executable's directory")?;

    let mut candidates = vec![];
    match platform {
        None => candidates.push(directory.join("libnative_runtime_static.a")),
        Some(platform) => {
            let triple = platform.triple();
            candidates.push(directory.join(format!("libnative_runtime_static-{triple}.a")));
            // A development build running from cargo's target directory:
            // cross-compiled artifacts live in target/<triple>/<profile>/
            // next to this binary's target/<profile>/.
            if let (Some(target_directory), Some(profile)) =
                (directory.parent(), directory.file_name())
            {
                candidates.push(
                    target_directory
                        .join(triple)
                        .join(profile)
                        .join("libnative_runtime_static.a"),
                );
            }
            // The platform is this very machine, so the host library fits.
            if platform.matches_host() {
                candidates.push(directory.join("libnative_runtime_static.a"));
            }
        }
    }
    for candidate in &candidates {
        if candidate.is_file() {
            return Utf8PathBuf::from_path_buf(candidate.clone())
                .map_err(|path| format!("non UTF-8 path: {}", path.display()));
        }
    }

    let searched: Vec<String> = candidates
        .iter()
        .map(|candidate| format!("    {}", candidate.display()))
        .collect();
    let build_command = match platform {
        Some(platform) => format!(
            "rustup target add {triple}
    cargo build -p native-runtime-static --target {triple}",
            triple = platform.triple()
        ),
        None => "cargo build -p native-runtime-static".into(),
    };
    Err(format!(
        "could not find the native runtime library. Searched:

{}

Build it in the gleam source tree with:

    {build_command}

or point at an existing one with --runtime-lib (also settable as \
GLEAM_NATIVE_RUNTIME_LIB).",
        searched.join("\n")
    ))
}

pub fn hex_tarball(paths: &ProjectPaths) -> Result<()> {
    let mut config = crate::config::root_config(paths)?;
    let data: Vec<u8> = crate::publish::build_hex_tarball(paths, &mut config)?;

    let path = paths.build_export_hex_tarball(&config.name, &config.version.to_string());
    fs::write_bytes(&path, &data)?;
    println!(
        "
Your hex tarball has been generated in {}.
",
        path
    );
    Ok(())
}

pub fn javascript_prelude() -> Result<()> {
    print!("{}", gleam_core::javascript::PRELUDE);
    Ok(())
}

pub fn typescript_prelude() -> Result<()> {
    print!("{}", gleam_core::javascript::PRELUDE_TS_DEF);
    Ok(())
}

pub fn package_interface(paths: &ProjectPaths, out: Utf8PathBuf) -> Result<()> {
    // Build the project
    let mut built = crate::build::main(
        paths,
        Options {
            mode: Mode::Prod,
            target: None,
            codegen: Codegen::None,
            compile: Compile::All,
            warnings_as_errors: false,
            root_target_support: TargetSupport::Enforced,
            no_print_progress: false,
            erlang_output: ErlangOutput::Binary,
        },
        crate::build::download_dependencies(paths, crate::cli::Reporter::new())?,
    )?;
    built.root_package.attach_doc_and_module_comments();

    let out = gleam_core::docs::generate_json_package_interface(
        out,
        &built.root_package,
        &built.module_interfaces,
    );
    fs::write_outputs_under(&[out], paths.root())?;
    Ok(())
}

pub fn package_information(paths: &ProjectPaths, out: Utf8PathBuf) -> Result<()> {
    let config = crate::config::root_config(paths)?;
    let out = gleam_core::docs::generate_json_package_information(out, config);
    fs::write_outputs_under(&[out], paths.root())?;
    Ok(())
}
