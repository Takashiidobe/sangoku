use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use super::{CoreLibraries, TOOLCHAIN, modified, run_command, rustc_command};
use sangoku::{
    bundle,
    error::{Error, Result},
};

pub const FILES: &[&str] = &[
    "ufmt_write.s",
    "libufmt_write.rmeta",
    "ufmt.s",
    "libufmt.rmeta",
    "sangoku_console.s",
    "libsangoku_console.rmeta",
    "sangoku_audio.s",
    "libsangoku_audio.rmeta",
    "sangoku_vec.s",
    "libsangoku_vec.rmeta",
    "libufmt_macros.so",
];

pub struct Libraries(PathBuf);

impl Libraries {
    pub fn externs(&self) -> Vec<OsString> {
        let mut args = vec![
            "-L".into(),
            format!("dependency={}", self.0.display()).into(),
        ];
        for (name, file) in [
            ("ufmt_write", "libufmt_write.rmeta"),
            ("ufmt", "libufmt.rmeta"),
            ("sangoku_console", "libsangoku_console.rmeta"),
            ("sangoku_audio", "libsangoku_audio.rmeta"),
            ("sangoku_vec", "libsangoku_vec.rmeta"),
            ("ufmt_macros", "libufmt_macros.so"),
        ] {
            args.push("--extern".into());
            args.push(format!("{name}={}", self.0.join(file).display()).into());
        }
        args
    }

    pub fn assembly(&self) -> impl Iterator<Item = PathBuf> + '_ {
        [
            "ufmt_write.s",
            "ufmt.s",
            "sangoku_console.s",
            "sangoku_audio.s",
            "sangoku_vec.s",
        ]
        .into_iter()
        .map(|file| self.0.join(file))
    }
}

fn output(command: &mut Command) -> Result<Vec<u8>> {
    let output = command
        .stderr(Stdio::inherit())
        .output()
        .map_err(|error| Error::io(format!("could not run {command:?}"), error))?;
    if !output.status.success() {
        return Err(Error::CommandFailed {
            command: format!("{command:?}"),
            status: output.status,
        });
    }
    Ok(output.stdout)
}

pub fn libraries(
    root: &Path,
    compiler: &Path,
    opt_level: &str,
    core: &CoreLibraries,
) -> Result<Libraries> {
    let libraries = Libraries(core.0.join("console"));
    if bundle::is_bundled(root) {
        for file in FILES {
            if !libraries.0.join(file).is_file() {
                return Err(Error::Bundle(format!(
                    "missing prebuilt library {}",
                    libraries.0.join(file).display()
                )));
            }
        }
        return Ok(libraries);
    }
    fs::create_dir_all(&libraries.0)
        .map_err(|error| Error::io(format!("could not create {}", libraries.0.display()), error))?;
    let lock = fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(libraries.0.join("build.lock"))
        .map_err(|error| Error::io("could not open console build lock", error))?;
    lock.lock()
        .map_err(|error| Error::io("could not lock console cache", error))?;
    let options = format!(
        "{:?}",
        rustc_command(root, opt_level)
            .get_args()
            .collect::<Vec<_>>()
    );
    let stamp = libraries.0.join("rustc-options");
    let newest_input = [
        compiler.to_path_buf(),
        core.0.join("libcore.rmeta"),
        root.join("Cargo.lock"),
        root.join("src/console.rs"),
        root.join("runtime/console/Cargo.toml"),
        root.join("runtime/console/src/lib.rs"),
        root.join("runtime/audio/Cargo.toml"),
        root.join("runtime/audio/src/lib.rs"),
        root.join("runtime/vec/Cargo.toml"),
        root.join("runtime/vec/src/lib.rs"),
    ]
    .iter()
    .filter_map(|path| modified(path))
    .max();
    if fs::read_to_string(&stamp).is_ok_and(|saved| saved == options)
        && FILES.iter().all(|file| {
            modified(&libraries.0.join(file)).is_some_and(|time| Some(time) >= newest_input)
        })
    {
        return Ok(libraries);
    }
    fs::create_dir_all(&libraries.0)
        .map_err(|error| Error::io(format!("could not create {}", libraries.0.display()), error))?;
    fs::write(&stamp, "")
        .map_err(|error| Error::io("could not invalidate console cache", error))?;
    let metadata = output(Command::new("cargo").current_dir(root).args([
        "metadata",
        "--locked",
        "--format-version",
        "1",
    ]))?;
    let metadata: serde_json::Value = serde_json::from_slice(&metadata)
        .map_err(|error| Error::Message(format!("invalid Cargo metadata: {error}")))?;
    let packages = metadata["packages"]
        .as_array()
        .ok_or("Cargo metadata has no packages")?;
    let source = |name: &str| -> Result<(PathBuf, String)> {
        let package = packages
            .iter()
            .find(|package| package["name"] == name)
            .ok_or_else(|| Error::Message(format!("Cargo metadata has no {name} package")))?;
        let manifest = package["manifest_path"]
            .as_str()
            .ok_or("package has no manifest path")?;
        let edition = package["edition"]
            .as_str()
            .ok_or("package has no edition")?;
        Ok((
            Path::new(manifest)
                .parent()
                .ok_or("package manifest has no parent")?
                .join("src/lib.rs"),
            edition.to_owned(),
        ))
    };
    let messages = output(
        Command::new("cargo")
            .current_dir(root)
            .arg(format!("+{TOOLCHAIN}"))
            .args([
                "build",
                "--release",
                "--locked",
                "-p",
                "ufmt",
                "--message-format=json",
            ]),
    )?;
    let macro_library = String::from_utf8_lossy(&messages)
        .lines()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .find(|message| {
            message["reason"] == "compiler-artifact" && message["target"]["name"] == "ufmt_macros"
        })
        .and_then(|message| {
            message["filenames"]
                .as_array()?
                .iter()
                .filter_map(|file| file.as_str())
                .find(|file| file.ends_with(".so"))
                .map(PathBuf::from)
        })
        .ok_or("Cargo did not produce the ufmt procedural macro library")?;
    let macro_target = libraries.0.join("libufmt_macros.so");
    let macro_temporary = libraries.0.join("libufmt_macros.so.tmp");
    fs::copy(&macro_library, &macro_temporary)
        .map_err(|error| Error::io("could not copy ufmt procedural macros", error))?;
    fs::rename(&macro_temporary, &macro_target)
        .map_err(|error| Error::io("could not install ufmt procedural macros", error))?;
    for (name, (source, edition)) in [
        ("ufmt_write", source("ufmt-write")?),
        ("ufmt", source("ufmt")?),
        (
            "sangoku_console",
            (root.join("runtime/console/src/lib.rs"), "2024".to_owned()),
        ),
        (
            "sangoku_audio",
            (root.join("runtime/audio/src/lib.rs"), "2024".to_owned()),
        ),
        (
            "sangoku_vec",
            (root.join("runtime/vec/src/lib.rs"), "2024".to_owned()),
        ),
    ] {
        let mut command = rustc_command(root, opt_level);
        command
            .arg(format!("--edition={edition}"))
            .arg(format!("-Cmetadata=sangoku-{name}"))
            .args(["-Awarnings", "--emit=asm,metadata", "--crate-name", name])
            .args(core.externs(2))
            .arg("-L")
            .arg(format!("dependency={}", libraries.0.display()));
        if matches!(name, "ufmt_write" | "ufmt") {
            command.args(["--cap-lints", "allow"]);
        }
        if matches!(name, "ufmt" | "sangoku_console") {
            command.arg("--extern").arg(format!(
                "ufmt_write={}",
                libraries.0.join("libufmt_write.rmeta").display()
            ));
            command
                .arg("--extern")
                .arg(format!("ufmt_macros={}", macro_target.display()));
        }
        if name == "sangoku_console" {
            command.arg("--extern").arg(format!(
                "ufmt={}",
                libraries.0.join("libufmt.rmeta").display()
            ));
        }
        run_command(
            command
                .arg(source)
                .arg("-o")
                .arg(libraries.0.join(format!("{name}.s"))),
        )?;
    }
    fs::write(&stamp, options)
        .map_err(|error| Error::io("could not record console compiler options", error))?;
    Ok(libraries)
}
