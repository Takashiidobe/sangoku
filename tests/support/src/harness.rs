use crate::emu::Emulator;
use crate::{Result, Value, ensure, json};
use regex::Regex;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Debug)]
pub struct CommandFailure {
    pub command: Vec<String>,
    pub stderr: String,
}
impl std::fmt::Display for CommandFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "command {:?} failed: {}", self.command, self.stderr)
    }
}
impl std::error::Error for CommandFailure {}

#[derive(Clone, Debug)]
pub struct Section {
    pub name: String,
    pub area: String,
    pub address: usize,
    pub size: usize,
}
#[derive(Clone, Debug)]
pub struct Program {
    pub rom: Vec<u8>,
    pub symbols: BTreeMap<String, u16>,
    pub sections: BTreeMap<String, Section>,
    pub provenance: Value,
}
impl Program {
    pub fn emulator(&self, entry: &str) -> Result<Emulator> {
        let pc = *self
            .symbols
            .get(entry)
            .ok_or_else(|| anyhow::anyhow!("missing symbol {entry}"))?;
        Emulator::new(&self.rom, pc)
    }
    pub fn size(&self, area: &str, names: &[&str]) -> usize {
        self.sections
            .values()
            .filter(|s| s.area == area && (names.is_empty() || names.contains(&s.name.as_str())))
            .map(|s| s.size)
            .sum()
    }
}
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}
pub fn read(path: impl AsRef<Path>) -> Result<String> {
    Ok(fs::read_to_string(root().join(path))?)
}
pub fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub fn write(path: &Path, contents: impl AsRef<[u8]>) -> Result<()> {
    if let Some(p) = path.parent() {
        fs::create_dir_all(p)?;
    }
    fs::write(path, contents)?;
    Ok(())
}
pub fn write_json(path: &Path, value: &Value) -> Result<()> {
    write(path, serde_json::to_string_pretty(value)? + "\n")
}
pub fn command(args: &[String]) -> Result<String> {
    let out = Command::new(&args[0])
        .args(&args[1..])
        .current_dir(root())
        .output()?;
    ensure!(
        out.status.success(),
        "command {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    Ok(String::from_utf8(out.stdout)?)
}
pub fn assemble(sources: &[(&str, String)], artifacts: Option<&Path>) -> Result<Program> {
    assemble_mode(sources, artifacts, false)
}

pub fn assemble_tiny(sources: &[(&str, String)], artifacts: Option<&Path>) -> Result<Program> {
    assemble_mode(sources, artifacts, true)
}

fn assemble_mode(
    sources: &[(&str, String)],
    artifacts: Option<&Path>,
    tiny: bool,
) -> Result<Program> {
    let temp = if artifacts.is_none() {
        Some(tempfile::tempdir()?)
    } else {
        None
    };
    let directory = if let Some(path) = artifacts {
        root().join(path)
    } else {
        temp.as_ref().unwrap().path().to_path_buf()
    };
    fs::create_dir_all(&directory)?;
    let mut commands = Vec::new();
    let mut objects = Vec::new();
    let mut run = |args: Vec<String>| -> Result<()> {
        commands.push(args.clone());
        let out = Command::new(&args[0]).args(&args[1..]).output()?;
        if !out.status.success() {
            let stderr = String::from_utf8_lossy(&out.stderr);
            write(
                &directory.join("build-error.txt"),
                [out.stdout.as_slice(), out.stderr.as_slice()].concat(),
            )?;
            return Err(CommandFailure {
                command: args,
                stderr: stderr.into_owned(),
            }
            .into());
        }
        Ok(())
    };
    let mut hashes = serde_json::Map::new();
    for (index, (name, source)) in sources.iter().enumerate() {
        ensure!(
            Path::new(name).file_name().and_then(|x| x.to_str()) == Some(*name),
            "assembly source names must be plain filenames"
        );
        let path = directory.join(name);
        fs::write(&path, source)?;
        let obj = directory.join(format!("{index}.o"));
        run(vec![
            "rgbasm".into(),
            "-I".into(),
            format!("{}/", root().display()),
            "-o".into(),
            obj.display().to_string(),
            path.display().to_string(),
        ])?;
        objects.push(obj.display().to_string());
        hashes.insert((*name).into(), json!(hash(source.as_bytes())));
    }
    let mut cmd = vec![
        "rgblink".into(),
        "-o".into(),
        directory.join("runtime.gb").display().to_string(),
        "-n".into(),
        directory.join("runtime.sym").display().to_string(),
        "-m".into(),
        directory.join("runtime.map").display().to_string(),
    ];
    if tiny {
        cmd.push("-t".into());
    }
    cmd.extend(objects);
    run(cmd)?;
    let sym_re = Regex::new(r"^([0-9a-fA-F]+):([0-9a-fA-F]+) (.+)$")?;
    let mut symbols = BTreeMap::new();
    for line in fs::read_to_string(directory.join("runtime.sym"))?.lines() {
        if let Some(c) = sym_re.captures(line) {
            ensure!(
                u16::from_str_radix(&c[1], 16)? == 0,
                "banked symbols are unsupported by the flat-memory harness"
            );
            symbols.insert(c[3].into(), u16::from_str_radix(&c[2], 16)?);
        }
    }
    let heading = Regex::new(r"^(\w+) bank #(\d+):$")?;
    let section = Regex::new(
        r#"SECTION: \$([0-9a-fA-F]+)(?:-\$[0-9a-fA-F]+)? \(\$([0-9a-fA-F]+) bytes?\) \["(.+)"\]"#,
    )?;
    let mut area = String::new();
    let mut sections = BTreeMap::new();
    for line in fs::read_to_string(directory.join("runtime.map"))?.lines() {
        if let Some(c) = heading.captures(line) {
            area = c[1].into();
            ensure!(
                area == "ROM0" || area == "WRAM0",
                "unsupported memory region {area}"
            );
        }
        if let Some(c) = section.captures(line) {
            let name = c[3].to_owned();
            ensure!(
                !sections.contains_key(&name),
                "duplicate section name {name}"
            );
            sections.insert(
                name.clone(),
                Section {
                    name,
                    area: area.clone(),
                    address: usize::from_str_radix(&c[1], 16)?,
                    size: usize::from_str_radix(&c[2], 16)?,
                },
            );
        }
    }
    ensure!(
        !sections.is_empty(),
        "linker map contains no recognized sections"
    );
    let mut tools = serde_json::Map::new();
    for tool in ["rgbasm", "rgblink"] {
        tools.insert(
            tool.into(),
            json!(command(&[tool.into(), "--version".into()])?.trim()),
        );
    }
    Ok(Program {
        rom: fs::read(directory.join("runtime.gb"))?,
        symbols,
        sections,
        provenance: json!({"source_sha256":hashes,"tools":tools,"commands":commands,"artifacts":if artifacts.is_some(){Some(directory)}else{None}}),
    })
}
