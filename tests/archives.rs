use std::{fs, path::Path, process::Command};

use object::read::archive::ArchiveFile;
use sangoku::object::{Binding, Definition, MemoryRegion, Object, RelocationKind};
use sangoku_test_support::{Result, ensure, harness, tempfile};

fn run(command: &mut Command) -> Result<String> {
    let output = command.output()?;
    ensure!(
        output.status.success(),
        "{command:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(String::from_utf8(output.stdout)?)
}

fn archive(path: &Path, metadata: bool) -> Result<Vec<Object>> {
    let bytes =
        fs::read(path).map_err(|error| format!("could not read {}: {error}", path.display()))?;
    let archive = ArchiveFile::parse(bytes.as_slice())?;
    let mut found_metadata = false;
    let mut objects = Vec::new();
    for member in archive.members() {
        let member = member?;
        let data = member.data(bytes.as_slice())?;
        if member.name() == b"lib.rmeta" {
            ensure!(!data.is_empty(), "empty Rust metadata");
            found_metadata = true;
        } else if data.starts_with(sangoku::object::MAGIC) {
            objects.push(Object::decode(data)?);
        }
    }
    ensure!(
        found_metadata == metadata,
        "wrong metadata presence for {}",
        path.display()
    );
    ensure!(
        !objects.is_empty(),
        "no native objects in {}",
        path.display()
    );
    let symbols = archive
        .symbols()?
        .ok_or("missing archive symbol index")?
        .map(|symbol| Ok(symbol?.name().to_vec()))
        .collect::<Result<Vec<_>>>()?;
    for object in &objects {
        for symbol in &object.symbols {
            if symbol.binding != Binding::Local && symbol.definition != Definition::Undefined {
                ensure!(
                    symbols.iter().any(|name| name == symbol.name.as_bytes()),
                    "{} missing from archive symbol index",
                    symbol.name
                );
            }
        }
    }
    Ok(objects)
}

fn execute_archived_answer(objects: &[Object]) -> Result<()> {
    let (object, section) = objects
        .iter()
        .find_map(|object| {
            object
                .symbols
                .iter()
                .find_map(|symbol| match (&*symbol.name, &symbol.definition) {
                    ("dependency_answer", Definition::Section { section, offset: 0 }) => {
                        Some((object, *section))
                    }
                    _ => None,
                })
        })
        .ok_or("missing dependency function")?;
    let mut code = object.sections[section.0 as usize].data.clone();
    for relocation in object
        .relocations
        .iter()
        .filter(|relocation| relocation.section == section)
    {
        let target = &object.symbols[relocation.symbol.0 as usize];
        let Definition::Section {
            section: target_section,
            offset,
        } = target.definition
        else {
            return Err("constant function has an external relocation".into());
        };
        ensure!(
            target_section == section && relocation.kind == RelocationKind::Absolute16,
            "unexpected constant function relocation"
        );
        let address = u16::try_from(0x200 + i64::from(offset) + relocation.addend)?;
        let offset = relocation.offset as usize;
        code[offset..offset + 2].copy_from_slice(&address.to_le_bytes());
    }
    let mut rom = vec![0; 32768];
    rom[0x200..0x200 + code.len()].copy_from_slice(&code);
    let mut emulator = sangoku_test_support::emu::Emulator::new(&rom, 0x200)?;
    emulator.set_sp(0xdffe);
    emulator.memory[0xdffe..0xe000].copy_from_slice(&0x100u16.to_le_bytes());
    for _ in 0..1000 {
        if emulator.pc == 0x100 {
            ensure!(
                emulator.a == 42,
                "archived function returned {}",
                emulator.a
            );
            return Ok(());
        }
        emulator.step()?;
    }
    Err("archived function did not return".into())
}

#[test]
fn cargo_archives_preserve_code_metadata_relocations_and_generics() -> Result<()> {
    let root = harness::root();
    run(Command::new("cargo").current_dir(&root).args([
        "+gbc-lr35902",
        "build",
        "--release",
        "--locked",
        "--manifest-path",
        "third_party/rustc_codegen_cranelift/Cargo.toml",
    ]))?;
    let driver = root.join("third_party/rustc_codegen_cranelift/target/release/sangoku-rustc");
    let directory = tempfile::tempdir()?;
    let project = directory.path();
    fs::create_dir(project.join("dependency"))?;
    fs::write(
        project.join("dependency/Cargo.toml"),
        "[package]\nname=\"archive-dependency\"\nversion=\"0.0.0\"\nedition=\"2024\"\n[lib]\npath=\"lib.rs\"\n",
    )?;
    fs::copy(
        root.join("tests/fixtures/archives/dependency.rs"),
        project.join("dependency/lib.rs"),
    )?;
    fs::copy(
        root.join("tests/fixtures/archives/consumer.rs"),
        project.join("lib.rs"),
    )?;
    fs::write(
        project.join("Cargo.toml"),
        "[package]\nname=\"archive-consumer\"\nversion=\"0.0.0\"\nedition=\"2024\"\n\
         [lib]\npath=\"lib.rs\"\ncrate-type=[\"rlib\",\"staticlib\"]\n\
         [dependencies]\narchive-dependency={path=\"dependency\"}\n\
         [profile.dev]\ndebug=0\nincremental=false\ncodegen-units=2\n[workspace]\n",
    )?;
    let mut cargo = Command::new("cargo");
    cargo
        .current_dir(project)
        .env("RUSTC", &driver)
        .env_remove("RUSTFLAGS")
        .env_remove("RUSTC_WRAPPER")
        .env_remove("RUSTC_WORKSPACE_WRAPPER")
        .env(
            "CARGO_ENCODED_RUSTFLAGS",
            "-Zunstable-options\x1f-Cpanic=immediate-abort",
        )
        .args([
            "+gbc-lr35902",
            "build",
            "-Zjson-target-spec",
            "--offline",
            "--message-format=json",
            "--target",
        ])
        .arg(root.join("targets/lr35902.json"));
    let messages = run(&mut cargo)?;
    let fresh = run(&mut cargo)?;
    let fresh: Vec<serde_json::Value> = fresh
        .lines()
        .filter_map(|line| serde_json::from_str(line).ok())
        .filter(|message: &serde_json::Value| message["reason"] == "compiler-artifact")
        .collect();
    ensure!(
        fresh.len() == 2 && fresh.iter().all(|message| message["fresh"] == true),
        "unchanged Cargo build did not reuse both crate artifacts"
    );
    let messages: Vec<serde_json::Value> = messages
        .lines()
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect();
    let artifact = |name: &str, extension: &str| -> Result<std::path::PathBuf> {
        messages
            .iter()
            .filter(|message| {
                message["reason"] == "compiler-artifact" && message["target"]["name"] == name
            })
            .flat_map(|message| message["filenames"].as_array().into_iter().flatten())
            .filter_map(|value| value.as_str())
            .map(std::path::PathBuf::from)
            .find(|path| path.extension().is_some_and(|suffix| suffix == extension))
            .ok_or_else(|| format!("missing {name} {extension} artifact").into())
    };
    let consumer = archive(&artifact("archive_consumer", "rlib")?, true)?;
    let combined = archive(&artifact("archive_consumer", "a")?, false)?;
    let dependency_path = artifact("archive_dependency", "rlib")?;
    let dependency = archive(&dependency_path, true)?;
    execute_archived_answer(&dependency)?;
    ensure!(dependency.len() >= 2, "dependency CGUs were lost");
    ensure!(
        combined.len() == consumer.len() + dependency.len(),
        "staticlib lost or duplicated objects"
    );
    ensure!(
        consumer.iter().any(|object| object
            .symbols
            .iter()
            .any(|symbol| symbol.name.contains("identity")
                && symbol.definition != Definition::Undefined)),
        "generic function was not instantiated from dependency metadata"
    );
    ensure!(
        consumer
            .iter()
            .any(|object| object
                .symbols
                .iter()
                .any(|symbol| symbol.name == "__gbc_mul16"
                    && symbol.definition == Definition::Undefined)),
        "runtime libcall import was lost"
    );
    ensure!(
        consumer.iter().any(
            |object| object.relocations.iter().any(|relocation| object.sections
                [relocation.section.0 as usize]
                .name
                == "call_dependency"
                && object.symbols[relocation.symbol.0 as usize].name == "dependency_answer")
        ),
        "cross-crate call relocation was lost"
    );
    for (name, region) in [
        ("POINTERS", MemoryRegion::Rom0),
        ("MUTABLE_POINTERS", MemoryRegion::Wram0),
    ] {
        let pointers = consumer
            .iter()
            .find_map(|object| {
                object
                    .symbols
                    .iter()
                    .find_map(|symbol| match (&*symbol.name, &symbol.definition) {
                        (symbol_name, Definition::Section { section, .. })
                            if symbol_name == name =>
                        {
                            Some((object, *section))
                        }
                        _ => None,
                    })
            })
            .ok_or("missing pointer table")?;
        let section = &pointers.0.sections[pointers.1.0 as usize];
        ensure!(
            section.region == region && section.data.len() == 4,
            "pointer storage was lost or assigned to the wrong region"
        );
        let targets = pointers
            .0
            .relocations
            .iter()
            .filter(|relocation| relocation.section == pointers.1)
            .map(|relocation| {
                ensure!(
                    relocation.kind == RelocationKind::Absolute16,
                    "wrong pointer relocation width"
                );
                Ok(pointers.0.symbols[relocation.symbol.0 as usize]
                    .name
                    .as_str())
            })
            .collect::<Result<Vec<_>>>()?;
        ensure!(
            targets.contains(&"BYTES") && targets.contains(&"dependency_answer"),
            "pointer targets lost: {targets:?}"
        );
    }
    ensure!(
        consumer
            .iter()
            .any(|object| object.symbols.iter().any(|symbol| {
                match (&*symbol.name, &symbol.definition) {
                    ("ZERO", Definition::Section { section, .. }) => {
                        let section = &object.sections[section.0 as usize];
                        section.region == MemoryRegion::Wram0
                            && section.data.is_empty()
                            && section.size == 8
                    }
                    _ => false,
                }
            })),
        "zero-filled RAM allocation was lost"
    );

    let saved = project.join("saved");
    fs::create_dir(&saved)?;
    let saved_dependency = saved.join("libarchive_dependency.rlib");
    fs::copy(&dependency_path, &saved_dependency)?;
    let saved_metadata = saved.join("libarchive_dependency.rmeta");
    fs::copy(artifact("archive_dependency", "rmeta")?, &saved_metadata)?;
    fs::remove_dir_all(project.join("dependency"))?;
    fs::remove_dir_all(project.join("target"))?;
    run(Command::new(&driver)
        .current_dir(&saved)
        .env("PATH", saved.join("no-programs"))
        .args(["-Zunstable-options", "--target"])
        .arg(root.join("targets/lr35902.json"))
        .args([
            "--edition=2024",
            "--crate-type=rlib",
            "--crate-name=from_saved",
            "--emit=link,metadata,obj",
            "-Cpanic=immediate-abort",
            "--extern",
        ])
        .arg(format!("archive_dependency={}", saved_dependency.display()))
        .arg("--extern")
        .arg(format!("archive_dependency={}", saved_metadata.display()))
        .arg(root.join("tests/fixtures/archives/consumer.rs"))
        .args(["--out-dir", "."]))?;
    archive(&saved.join("libfrom_saved.rlib"), true)?;
    Object::decode(&fs::read(saved.join("from_saved.o"))?)?;
    ensure!(
        saved.join("libfrom_saved.rmeta").is_file(),
        "metadata output missing"
    );
    fs::remove_file(saved.join("libfrom_saved.rmeta"))?;
    fs::remove_file(saved.join("from_saved.o"))?;
    fs::write(
        saved.join("final.rs"),
        "#![feature(no_core)]\n#![no_core]\nextern crate from_saved;\n\
         #[unsafe(no_mangle)] pub fn final_answer() -> u8 { from_saved::call_dependency() }\n",
    )?;
    run(Command::new(&driver)
        .current_dir(&saved)
        .env("PATH", saved.join("no-programs"))
        .args(["-Zunstable-options", "--target"])
        .arg(root.join("targets/lr35902.json"))
        .args([
            "--edition=2024",
            "--crate-type=rlib",
            "-Cpanic=immediate-abort",
            "--extern",
            "from_saved=libfrom_saved.rlib",
            "-Ldependency=.",
            "final.rs",
        ]))?;
    archive(&saved.join("libfinal.rlib"), true)?;
    Ok(())
}
