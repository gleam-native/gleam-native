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

/// Compile the project ahead of time into a native executable for this
/// machine, written to the project root and named after the package.
///
/// The Cranelift-generated object file is linked against the
/// `native-runtime-static` library (which provides the runtime and the C
/// `main`) with the system C compiler driver, which supplies the platform
/// startup files and default libraries.
pub(crate) fn native(paths: &ProjectPaths) -> Result<()> {
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
    )
    .map_err(fail)?;
    let object_path = build.join(format!("{package_name}.o"));
    fs::write_bytes(&object_path, &object)?;

    let runtime_library = runtime_static_library().map_err(fail)?;
    let executable_path = paths.root().join(package_name.as_str());
    link_executable(&object_path, &runtime_library, &executable_path)?;

    crate::cli::print_exported(package_name);
    println!(
        "
Your native executable has been generated to {executable_path}.
",
    );

    Ok(())
}

/// Links the object file and the runtime library into an executable with the
/// system C compiler driver.
fn link_executable(
    object: &Utf8PathBuf,
    runtime_library: &Utf8PathBuf,
    executable: &Utf8PathBuf,
) -> Result<()> {
    let program = "cc";
    let mut command = std::process::Command::new(program);
    let _ = command
        .arg(object)
        .arg(runtime_library)
        .arg("-o")
        .arg(executable);
    if cfg!(target_os = "linux") {
        let _ = command.args(["-lpthread", "-ldl", "-lm"]);
    }
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

/// Locates the `native-runtime-static` library executables are linked
/// against: the path in the `GLEAM_NATIVE_RUNTIME_LIB` environment variable
/// if set, otherwise `libnative_runtime_static.a` next to the running
/// `gleam` binary.
fn runtime_static_library() -> Result<Utf8PathBuf, String> {
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
    match executable
        .parent()
        .map(|directory| directory.join("libnative_runtime_static.a"))
    {
        Some(path) if path.is_file() => Utf8PathBuf::from_path_buf(path)
            .map_err(|path| format!("non UTF-8 path: {}", path.display())),
        _ => Err("could not find libnative_runtime_static.a next to the gleam \
executable. Build it with `cargo build -p native-runtime-static` or set \
GLEAM_NATIVE_RUNTIME_LIB to its path."
            .into()),
    }
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
