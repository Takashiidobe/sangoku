use crate::{Result, Value, emu::Emulator, harness, json};
use regex::Regex;
use std::{collections::BTreeMap, fs, path::Path, process::Command};

pub const CASES: &[&str] = &[
    "plane",
    "upper",
    "lower",
    "iterator",
    "range_search",
    "expansion_search",
];

fn copy_artifacts(from: &Path, to: &Path) -> Result<()> {
    if from.is_dir() {
        fs::create_dir_all(to)?;
        for file in fs::read_dir(from)? {
            let file = file?;
            copy_artifacts(&file.path(), &to.join(file.file_name()))?;
        }
    } else {
        fs::copy(from, to)?;
    }
    Ok(())
}

fn source(case: &str) -> Result<String> {
    ensure!(CASES.contains(&case), "unknown optimization case {case}");
    Ok(if case == "plane" {
        "tests/core_rom/unicode_plane.rs".into()
    } else {
        format!("tests/fixtures/optimization/{case}.rs")
    })
}

fn inputs(case: &str) -> Vec<u32> {
    match case {
        "upper" | "lower" | "iterator" => {
            ['\0', 'a', 'Ä', 'ß', 'ǅ', '🙂', 'µ', 'ÿ', '×', '÷', 'İ', 'ΐ']
                .into_iter()
                .map(u32::from)
                .collect()
        }
        "range_search" => vec![
            0, 0xff, 0x100, 0x103, 0x104, 0x200, 0x201, 0x204, 0x205, 0x300, 0xff07, 0xffff,
        ],
        "expansion_search" => vec![0, 0xde, 0xdf, 0xe0, 0x390, 0x391, 0xfb03, 0xffff],
        _ => vec![0, 0xffff, 0x10000, 0x10ffff, 0x12345678, u32::MAX],
    }
}

fn expected(case: &str, input: u32) -> (u8, Vec<u8>) {
    match case {
        "upper" | "lower" | "iterator" => {
            let c = char::from_u32(input).unwrap();
            let mut chars: Vec<char> = if case != "lower" {
                c.to_uppercase().collect()
            } else {
                c.to_lowercase().collect()
            };
            if case == "iterator" {
                chars.reverse();
            }
            (
                chars.len() as u8,
                chars
                    .into_iter()
                    .flat_map(|c| (c as u32).to_le_bytes())
                    .collect(),
            )
        }
        "range_search" => {
            let index = [(0x100, 3), (0x200, 4), (0x300, 0), (0xff00, 7)]
                .iter()
                .position(|&(start, len)| (start..=start + len).contains(&input));
            (
                index
                    .filter(|&i| i != 1 || input & 1 == 0)
                    .map_or(0, |i| i as u8 + 1),
                vec![],
            )
        }
        "expansion_search" => {
            let index = [0xdf, 0x390, 0xfb03].iter().position(|&key| key == input);
            (index.map_or(0, |i| i as u8 + 1), vec![])
        }
        _ => (
            0,
            [(input >> 16) as u16, input as u16]
                .into_iter()
                .flat_map(u16::to_le_bytes)
                .collect(),
        ),
    }
}

fn execute(case: &str, rom: &[u8], entry: u16) -> Result<Vec<Value>> {
    inputs(case)
        .into_iter()
        .map(|input| {
            let mut e = Emulator::new(rom, entry)?;
            e.memory[0xc000..0xc004].copy_from_slice(&input.to_le_bytes());
            e.memory[0xc004..0xc020].fill(0xa5);
            if case == "iterator" {
                let mut chars = [0u32; 3];
                for (index, c) in char::from_u32(input).unwrap().to_uppercase().enumerate() {
                    chars[index] = c as u32;
                }
                for (index, c) in chars.into_iter().enumerate() {
                    e.memory[0xc000 + index * 4..0xc004 + index * 4]
                        .copy_from_slice(&c.to_le_bytes());
                }
            }
            e.set_sp(0xdffe);
            e.write16(0xdffe, 0xffff);
            e.reset_metrics();
            e.run_until(0xffff, 2_000_000)?;
            let (result, bytes) = expected(case, input);
            let address = if case == "plane" { 0xc004 } else { 0xc010 };
            ensure!(
                e.a == result && e.memory[address..address + bytes.len()] == bytes,
                "{case}({input:#x}) returned {} with {:?}, expected {result} with {bytes:?}",
                e.a,
                &e.memory[address..address + bytes.len()]
            );
            ensure!(e.sp() == 0xe000, "{case}({input:#x}) corrupted the stack");
            Ok(json!({"input":input,"cycles":e.cycles,"peak_stack_bytes":e.peak_stack_bytes}))
        })
        .collect()
}

fn sizes(map: &str, assembly: &str) -> Result<(usize, usize, BTreeMap<String, usize>)> {
    let pattern = Regex::new(r#"SECTION:.*?\(\$([0-9a-fA-F]+) bytes?\) \["([^"]+)"\]"#)?;
    let mut data_sections = Vec::new();
    for section in assembly.split("SECTION ").skip(1) {
        let (header, body) = section.split_once('\n').ok_or("section without body")?;
        let name = header.split('"').nth(1).ok_or("section without name")?;
        if body
            .lines()
            .map(str::trim)
            .filter(|s| !s.is_empty() && !s.ends_with(':') && !s.starts_with("EXPORT "))
            .all(|s| {
                ["db ", "dw ", "dl ", "ds ", "ALIGN "]
                    .iter()
                    .any(|prefix| s.starts_with(prefix))
            })
        {
            data_sections.push(name);
        }
    }
    let mut total = 0;
    let mut data = 0;
    let mut symbols = BTreeMap::new();
    for c in pattern.captures_iter(map) {
        let size = usize::from_str_radix(&c[1], 16)?;
        let raw = &c[2];
        total += size;
        if data_sections.contains(&raw) {
            data += size;
        }
        let name = if raw.contains("_2Edata") {
            let owner = raw
                .strip_prefix("__rust_")
                .unwrap_or(raw)
                .split("_2E")
                .next()
                .unwrap();
            format!("anonymous data::{owner}")
        } else {
            format!(
                "{:#}",
                rustc_demangle::demangle(raw.strip_prefix("__rust_").unwrap_or(raw))
            )
        };
        *symbols.entry(name).or_default() += size;
    }
    ensure!(total > 0, "map contains no sized sections");
    Ok((total, data, symbols))
}

fn git(path: &str, args: &[&str]) -> Result<String> {
    harness::command(
        &[
            vec!["git".into(), "-C".into(), path.into()],
            args.iter().map(|s| (*s).into()).collect(),
        ]
        .concat(),
    )
}

fn provenance() -> Result<Value> {
    let mut repos = BTreeMap::new();
    for path in [
        ".",
        "third_party/wasmtime",
        "third_party/rustc_codegen_cranelift",
        "third_party/rust",
    ] {
        repos.insert(
            path,
            json!({"head":git(path, &["rev-parse", "HEAD"])?.trim(),
            "diff_sha256":harness::hash(git(path, &["diff", "HEAD"])?.as_bytes())}),
        );
    }
    Ok(
        json!({"repositories":repos,"rustc":harness::command(&["rustc".into(),"+gbc-lr35902".into(),"-vV".into()])?,
        "rustc_options":harness::read("build/core/immediate-abort/opt-z/rustc-options")?,
        "backend_sha256":backend_hash()?}),
    )
}

fn backend_hash() -> Result<String> {
    Ok(harness::hash(&fs::read(harness::root().join(
        "third_party/rustc_codegen_cranelift/target/release/librustc_codegen_cranelift.so",
    ))?))
}

pub fn compare(before: &Value, after: &Value) -> Result<Value> {
    ensure!(
        before["schema"] == 1 && after["schema"] == 1 && before["opt_level"] == after["opt_level"],
        "incompatible optimization reports"
    );
    let mut changes = BTreeMap::new();
    for (name, new) in after["cases"].as_object().ok_or("missing cases")? {
        let old = &before["cases"][name];
        ensure!(
            old.is_object() && old["source_sha256"] == new["source_sha256"],
            "baseline missing case {name} or its source changed"
        );
        let old_symbols: BTreeMap<String, i64> =
            crate::serde_json::from_value(old["symbols"].clone())?;
        let new_symbols: BTreeMap<String, i64> =
            crate::serde_json::from_value(new["symbols"].clone())?;
        let mut symbols = old_symbols.clone();
        symbols.extend(new_symbols.clone());
        let deltas: BTreeMap<_, _> = symbols
            .keys()
            .filter_map(|s| {
                let delta = new_symbols.get(s).unwrap_or(&0) - old_symbols.get(s).unwrap_or(&0);
                (delta != 0).then_some((s, delta))
            })
            .collect();
        let old_runs = old["runs"].as_array().ok_or("missing baseline runs")?;
        let new_runs = new["runs"].as_array().ok_or("missing runs")?;
        ensure!(
            old_runs.len() == new_runs.len(),
            "{name} input sets changed"
        );
        let mut runs = vec![];
        for (old, new) in old_runs.iter().zip(new_runs) {
            ensure!(old["input"] == new["input"], "{name} input sets changed");
            runs.push(json!({"input":new["input"],
                "cycles_delta":new["cycles"].as_i64().ok_or("missing cycles")? - old["cycles"].as_i64().ok_or("missing cycles")?,
                "peak_stack_delta":new["peak_stack_bytes"].as_i64().ok_or("missing stack")? - old["peak_stack_bytes"].as_i64().ok_or("missing stack")?}));
        }
        changes.insert(name, json!({"section_bytes_delta":new["section_bytes"].as_i64().ok_or("missing size")? - old["section_bytes"].as_i64().ok_or("missing size")?,
            "data_bytes_delta":new["data_bytes"].as_i64().ok_or("missing data size")? - old["data_bytes"].as_i64().ok_or("missing data size")?,
            "symbols":deltas,"runs":runs}));
    }
    Ok(json!({"cases":changes}))
}

pub fn run(output: &Path, baseline: Option<&Path>, selected: &[String]) -> Result<Value> {
    let output = harness::root().join(output);
    fs::create_dir_all(&output)?;
    ensure!(!selected.is_empty(), "select at least one case");
    for case in selected {
        source(case)?;
    }
    let baseline = baseline
        .map(|p| {
            let p = harness::root().join(p);
            let p = if p.is_dir() { p.join("report.json") } else { p };
            ensure!(
                p.canonicalize()?
                    != output
                        .join("report.json")
                        .canonicalize()
                        .unwrap_or(output.join("report.json")),
                "output would overwrite the baseline"
            );
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(
                crate::serde_json::from_slice::<Value>(&fs::read(p)?)?,
            )
        })
        .transpose()?;
    let mut cases = BTreeMap::new();
    let mut fingerprint = None;
    for case in selected {
        let source = source(case)?;
        let directory = output.join(case);
        fs::create_dir_all(&directory)?;
        let source_bytes = fs::read(harness::root().join(&source))?;
        harness::write(&directory.join("source.rs"), &source_bytes)?;
        let rom_path = directory.join("game.gb");
        let args = ["run", "-q", "--", "build", &source];
        let result = Command::new("cargo")
            .current_dir(harness::root())
            .args(args)
            .arg(&rom_path)
            .args(["--opt-level", "z"])
            .output()?;
        harness::write(
            &directory.join("build.log"),
            [result.stdout.as_slice(), result.stderr.as_slice()].concat(),
        )?;
        let stderr = String::from_utf8(result.stderr)?;
        ensure!(result.status.success(), "{case} build failed: {stderr}");
        ensure!(
            fs::read(harness::root().join(&source))? == source_bytes,
            "{case} source changed during snapshot"
        );
        let hash = backend_hash()?;
        ensure!(
            fingerprint
                .as_ref()
                .is_none_or(|previous| previous == &hash),
            "backend changed during snapshot; rebuild it first and rerun"
        );
        fingerprint = Some(hash);
        let artifacts = stderr
            .lines()
            .find_map(|s| s.strip_prefix("Build artifacts: "))
            .ok_or("missing artifacts")?;
        for file in [
            "game.mir",
            "game.clif",
            "game.asm",
            "probe.asm",
            "game.map",
            "probe.map",
            "game.sym",
        ] {
            copy_artifacts(&Path::new(artifacts).join(file), &directory.join(file))?;
        }
        let sym = fs::read_to_string(directory.join("game.sym"))?;
        let entry = sym
            .lines()
            .find_map(|s| {
                let (address, name) = s.split_once(' ')?;
                (name == "__rust_gbc_main").then_some(address)
            })
            .ok_or("missing entry symbol")?;
        let (bank, address) = entry.split_once(':').ok_or("missing bank")?;
        ensure!(
            u16::from_str_radix(bank, 16)? <= 1,
            "banked ROM unsupported"
        );
        let runs = execute(
            case,
            &fs::read(&rom_path)?,
            u16::from_str_radix(address, 16)?,
        )?;
        let (total, data, symbols) = sizes(
            &fs::read_to_string(directory.join("probe.map"))?,
            &fs::read_to_string(directory.join("probe.asm"))?,
        )?;
        println!(
            "{case}: {total} section bytes ({data} data), {} inputs passed",
            runs.len()
        );
        cases.insert(case, json!({"source":source,"source_sha256":harness::hash(&source_bytes),
            "section_bytes":total,"data_bytes":data,"code_and_runtime_bytes":total-data,"symbols":symbols,"runs":runs}));
    }
    let provenance = provenance()?;
    ensure!(
        fingerprint.as_deref() == provenance["backend_sha256"].as_str(),
        "backend changed during snapshot"
    );
    let report = json!({"schema":1,"opt_level":"z","provenance":provenance,"cases":cases});
    if let Some(before) = baseline {
        let delta = compare(&before, &report)?;
        harness::write_json(&output.join("comparison.json"), &delta)?;
        for (name, case) in delta["cases"].as_object().unwrap() {
            println!(
                "{name}: {} section bytes vs baseline",
                case["section_bytes_delta"]
            );
            for (symbol, delta) in case["symbols"].as_object().unwrap() {
                println!("  {delta:>6} {symbol}");
            }
        }
    }
    harness::write_json(&output.join("report.json"), &report)?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_sizes_include_named_data_and_merge_anonymous_data() -> Result<()> {
        let map = "SECTION: $4000 ($0003 bytes) [\"__rust_gbc_main\"]\nSECTION: $4003 ($0002 bytes) [\"TABLE\"]\nSECTION: $4005 ($0001 byte) [\"__rust_case_2Ehash_2Edata0\"]\n";
        let asm = "SECTION \"__rust_gbc_main\", ROMX\n__rust_gbc_main::\n  db $00\n  ret\nSECTION \"TABLE\", ROMX\nTABLE::\n  dw $1234\nSECTION \"__rust_case_2Ehash_2Edata0\", ROMX\n  db 0\n";
        let (total, data, symbols) = sizes(map, asm)?;
        assert_eq!((total, data), (6, 3));
        assert_eq!(symbols["anonymous data::case"], 1);
        Ok(())
    }

    #[test]
    fn comparisons_report_removed_symbols_and_reject_changed_inputs() -> Result<()> {
        let before = json!({"schema":1,"opt_level":"z","cases":{"upper":{
            "source_sha256":"same","section_bytes":100,"data_bytes":20,
            "symbols":{"kept":70,"removed":10},"runs":[{"input":0,"cycles":100,"peak_stack_bytes":20}]}}});
        let mut after = before.clone();
        after["cases"]["upper"]["symbols"] = json!({"kept":65,"added":2});
        after["cases"]["upper"]["section_bytes"] = json!(87);
        after["cases"]["upper"]["runs"][0]["cycles"] = json!(90);
        let delta = compare(&before, &after)?;
        assert_eq!(delta["cases"]["upper"]["symbols"]["removed"], -10);
        assert_eq!(delta["cases"]["upper"]["symbols"]["added"], 2);
        assert_eq!(delta["cases"]["upper"]["section_bytes_delta"], -13);
        assert_eq!(delta["cases"]["upper"]["runs"][0]["cycles_delta"], -10);
        after["cases"]["upper"]["source_sha256"] = json!("different");
        assert!(compare(&before, &after).is_err());
        after["cases"]["upper"]["source_sha256"] = json!("same");
        after["cases"]["upper"]["runs"][0]["input"] = json!(1);
        assert!(compare(&before, &after).is_err());
        Ok(())
    }
}
