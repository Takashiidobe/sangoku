use crate::{Result, Value, ensure, json, number};
use crate::{
    cli::Args,
    emu::Emulator,
    harness::{self, Program},
    popcount,
    random::Random,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const LEVELS: &[&str] = &["0", "1", "2", "3", "s", "z"];
const PROBES: &[&str] = &[
    "popcount",
    "popcount_wide",
    "popcount_live",
    "popcount_pressure",
];
fn strings(args: &[&str]) -> Vec<String> {
    args.iter().map(|s| (*s).to_owned()).collect()
}
struct Runner {
    directory: PathBuf,
    commands: Vec<Vec<String>>,
    env: BTreeMap<String, String>,
}
impl Runner {
    fn new(directory: PathBuf) -> Result<Self> {
        let directory = harness::root().join(directory);
        fs::create_dir_all(&directory)?;
        Ok(Self {
            directory,
            commands: vec![],
            env: BTreeMap::new(),
        })
    }
    fn run(&mut self, name: &str, args: Vec<String>, input: Option<&[u8]>) -> Result<String> {
        let mut cmd = Command::new(&args[0]);
        cmd.args(&args[1..])
            .current_dir(harness::root())
            .envs(&self.env)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let out = if let Some(input) = input {
            let mut child = cmd.stdin(Stdio::piped()).spawn()?;
            child.stdin.take().unwrap().write_all(input)?;
            child.wait_with_output()?
        } else {
            cmd.output()?
        };
        self.commands.push(args.clone());
        harness::write(
            &self.directory.join(format!("{name}.log")),
            [out.stdout.as_slice(), out.stderr.as_slice()].concat(),
        )?;
        ensure!(
            out.status.success(),
            "command {args:?} failed; see {}",
            self.directory.join(format!("{name}.log")).display()
        );
        Ok(String::from_utf8(out.stdout)?)
    }
    fn backend(&mut self) -> Result<()> {
        self.run(
            "backend-build",
            strings(&[
                "cargo",
                "+gbc-lr35902",
                "build",
                "--release",
                "--locked",
                "--manifest-path",
                "third_party/rustc_codegen_cranelift/Cargo.toml",
            ]),
            None,
        )?;
        Ok(())
    }
    fn compile(&mut self, name: &str, source: &str, level: &str, file: &str) -> Result<()> {
        let mut args = strings(&[
            "rustc",
            "+gbc-lr35902",
            "-Zunstable-options",
            "-Zcodegen-backend=third_party/rustc_codegen_cranelift/target/release/librustc_codegen_cranelift.so",
            "--target",
            "targets/lr35902.json",
            "--emit=asm",
            "--crate-type=lib",
            "-Cpanic=immediate-abort",
        ]);
        args.extend([
            format!("-Copt-level={level}"),
            source.into(),
            "-o".into(),
            self.directory.join(file).display().to_string(),
        ]);
        self.run(name, args, None)?;
        Ok(())
    }
    fn checks(&mut self, pop: bool) -> Result<()> {
        self.env.insert(
            "SANGOKU_SPILL_ARTIFACTS".into(),
            self.directory.join("instructions").display().to_string(),
        );
        if pop {
            self.env
                .insert("CG_CLIF_ENABLE_VERIFIER".into(), "1".into());
            self.env.insert(
                "SANGOKU_POPCOUNT_ARTIFACTS".into(),
                self.directory.join("kernels").display().to_string(),
            );
        }
        self.run(
            "codegen-tests",
            strings(&[
                "cargo",
                "+gbc-lr35902",
                "test",
                "--locked",
                "--manifest-path",
                "third_party/wasmtime/cranelift/codegen/Cargo.toml",
                "--no-default-features",
                "--features",
                "std,lr35902,x86",
                "--lib",
                "isa::lr35902::",
            ]),
            None,
        )?;
        self.backend()?;
        if pop {
            for level in LEVELS {
                self.compile(
                    &format!("compile-{level}"),
                    "tests/fixtures/popcount/replacement.rs",
                    level,
                    &format!("level-{level}.asm"),
                )?;
            }
        } else {
            for (name, source) in [
                ("popcount", "tests/fixtures/popcount/baseline.rs"),
                ("live", "tests/fixtures/spills/live.rs"),
            ] {
                self.compile(name, source, "2", &format!("{name}.asm"))?;
            }
        }
        Ok(())
    }
}
fn rust_sources(directory: &Path) -> Result<Vec<PathBuf>> {
    let mut files = vec![];
    for entry in fs::read_dir(directory)? {
        let p = entry?.path();
        if p.is_dir() {
            files.extend(rust_sources(&p)?);
        } else if p.extension().is_some_and(|s| s == "rs") {
            files.push(p);
        }
    }
    files.sort();
    Ok(files)
}
fn hashes(paths: Vec<PathBuf>) -> Result<Value> {
    let mut result = serde_json::Map::new();
    for path in paths {
        let key = path
            .strip_prefix(harness::root())
            .unwrap_or(&path)
            .display()
            .to_string();
        result.insert(key, json!(harness::hash(&fs::read(path)?)));
    }
    Ok(Value::Object(result))
}
fn source_hashes() -> Result<Value> {
    let root = harness::root();
    let mut files = rust_sources(&root.join("tests/support/src"))?;
    files.extend(rust_sources(
        &root.join("third_party/wasmtime/cranelift/codegen/src/isa/lr35902"),
    )?);
    files.extend([
        root.join("Cargo.lock"),
        root.join("third_party/rustc_codegen_cranelift/src/lib.rs"),
        root.join("third_party/wasmtime/cranelift/codegen/meta/src/isa/lr35902.rs"),
        root.join("tests/fixtures/spills/live.rs"),
        root.join("tests/fixtures/popcount/replacement.rs"),
    ]);
    hashes(files)
}
pub fn check_accesses(directory: &Path) -> Result<usize> {
    let mut paths: Vec<_> = fs::read_dir(directory.join("instructions"))?
        .map(|e| e.map(|x| x.path()))
        .collect::<std::result::Result<_, _>>()?;
    paths.sort();
    let mut executions = 0;
    for path in paths {
        let stem = path.file_stem().unwrap().to_string_lossy();
        if path.extension().is_none_or(|x| x != "bin") || !stem.contains('_') {
            continue;
        }
        let parts: Vec<_> = stem.split('_').collect();
        ensure!(
            parts.len() == 3,
            "invalid access fixture {}",
            path.display()
        );
        let register = "bcdehl_a".as_bytes()[parts[1].parse::<usize>()?] as char;
        let offset: u16 = parts[2].parse()?;
        let code = fs::read(&path)?;
        let source = fs::read_to_string(path.with_extension("asm"))?;
        let p = harness::assemble(
            &[(
                "access.asm",
                "SECTION \"Access\", ROM0[0]\n".to_owned() + &source,
            )],
            Some(&directory.join("assembled").join(stem.as_ref())),
        )?;
        ensure!(
            p.rom[..code.len()] == code && p.size("ROM0", &[]) == code.len(),
            "native/RGBDS mismatch {}",
            path.display()
        );
        for stack in [0xC800u16, 0xD400] {
            for seed in [0u16, 0x55, 0xFF] {
                for flags in (0..256u16).step_by(16) {
                    let mut e = Emulator::new(&code, 0)?;
                    for (i, r) in "abcdehl".chars().enumerate() {
                        e.set_reg(r, (seed + i as u16 * 31) as u8);
                    }
                    e.f = flags as u8;
                    e.set_sp(stack);
                    let address = stack + offset;
                    let mut value = (seed * 73 + flags + offset) as u8;
                    e.memory[address as usize] = value;
                    let mut expected: BTreeMap<_, _> =
                        "abcdefhl".chars().map(|r| (r, e.reg(r))).collect();
                    if parts[0] == "load" {
                        expected.insert(register, value);
                    } else {
                        value = expected[&register];
                    }
                    e.reset_metrics();
                    e.run_until(code.len() as u16, 1024)?;
                    let actual: BTreeMap<_, _> =
                        "abcdefhl".chars().map(|r| (r, e.reg(r))).collect();
                    ensure!(
                        actual == expected,
                        "register preservation failure {} stack={stack:X} seed={seed} flags={flags}",
                        path.display()
                    );
                    ensure!(
                        e.sp() == stack && e.peak_stack_bytes == 4,
                        "access stack accounting failure"
                    );
                    ensure!(
                        e.memory[address as usize] == value,
                        "access memory result failure"
                    );
                    ensure!(
                        e.written_addresses
                            .iter()
                            .all(|&a| (stack - 4..stack).contains(&a)
                                || (parts[0] == "store" && a == address)),
                        "access wrote outside stack"
                    );
                    executions += 1;
                }
            }
        }
    }
    ensure!(
        executions == 12096,
        "expected 12096 access executions, got {executions}"
    );
    Ok(executions)
}
pub fn check_pressure(directory: &Path) -> Result<usize> {
    let mut n = 0;
    for name in ["pressure", "incoming"] {
        let code = fs::read(directory.join("instructions").join(format!("{name}.bin")))?;
        for stack in [0xC800u16, 0xD400] {
            for value in 0..256u16 {
                let mut e = Emulator::new(&code, 0)?;
                for (i, r) in "abcdehl".chars().enumerate() {
                    e.set_reg(r, (value + i as u16 * 31) as u8);
                }
                e.f = value as u8 & 0xF0;
                e.set_sp(stack - 2);
                e.write16(e.sp(), 0x800);
                e.memory[stack as usize..stack as usize + 4].fill(value as u8 ^ 255);
                e.memory[stack as usize + 4] = value as u8;
                e.reset_metrics();
                e.run_until(0x800, 2048)?;
                ensure!(
                    e.a == if name == "pressure" { 78 } else { value as u8 },
                    "{name} returned wrong value"
                );
                ensure!(
                    e.sp() == stack
                        && e.read16(stack - 2) == 0x800
                        && e.memory[stack as usize + 4] == value as u8,
                    "{name} corrupted its frame"
                );
                ensure!(
                    e.written_addresses
                        .iter()
                        .all(
                            |&a| stack as i32 - 2 - e.peak_stack_bytes <= a as i32 && a < stack - 2
                        ),
                    "{name} wrote outside stack"
                );
                n += 1;
            }
        }
    }
    Ok(n)
}
fn normalize(source: &str, section: &str) -> Result<String> {
    Ok(regex::Regex::new(r#"SECTION "[^"]+", ROM0"#)?
        .replacen(
            source,
            1,
            regex::NoExpand(&format!("SECTION \"{section}\", ROM0[$400]")),
        )
        .to_string())
}
fn spill_rust(directory: &Path, exhaustive: bool) -> Result<Value> {
    let mut rng = Random::new(7);
    let mut values: BTreeSet<u16> = (0..256)
        .chain((0..256).map(|v| v << 8))
        .chain([65535, 0x5555, 0xAAAA, 0x8001, 0x1234])
        .collect();
    values.extend((0..1024).map(|_| rng.range(0, 65536)));
    let mut summary = serde_json::Map::new();
    for (name, symbol) in [
        ("popcount", "__rust_audit_popcount"),
        ("live", "__rust_audit_popcount_live"),
    ] {
        let source = normalize(
            &fs::read_to_string(directory.join(format!("{name}.asm")))?,
            "Audit helper",
        )?
        .replace(symbol, "audit_popcount");
        let p = harness::assemble(
            &[
                ("function.asm", source),
                (
                    "stop.asm",
                    "SECTION \"Stop\", ROM0[$100]\naudit_call_done::\n nop\n".into(),
                ),
            ],
            Some(&directory.join(name)),
        )?;
        let (mut n, mut max_cycles, mut max_stack) = (0, 0, 0);
        let mut run = |value: u16, stack, poison| -> Result<()> {
            let r = popcount::run_case(&p, name, value, "body", poison, stack)?;
            let expected = if name == "live" {
                value.wrapping_add(value.count_ones() as u16)
            } else {
                value.count_ones() as u16
            };
            ensure!(
                r["actual"] == expected
                    && r["writes_in_stack"] == true
                    && r["stack_restored"] == true,
                "{name} failed for {value}: {r}"
            );
            n += 1;
            max_cycles = max_cycles.max(number(&r["cycles"]));
            max_stack = max_stack.max(number(&r["peak_stack_bytes"]));
            Ok(())
        };
        for stack in [0xC800, 0xD000, 0xD002] {
            for poison in [0, 0xA5, 0xFF] {
                for &value in &values {
                    run(value, stack, poison)?;
                }
            }
        }
        if exhaustive {
            for value in 0..=65535 {
                run(value, 0xD000, 0x55)?;
            }
        }
        summary.insert(
            name.into(),
            json!({
                "executions":n,
                "code_bytes":p.size("ROM0",&["Audit helper"]),
                "max_cycles":max_cycles,
                "peak_stack_bytes":max_stack,
                "provenance":p.provenance
            }),
        );
    }
    Ok(Value::Object(summary))
}
fn kernels(directory: &Path) -> Result<BTreeMap<&'static str, Vec<u8>>> {
    let mut result = BTreeMap::new();
    for name in ["shift", "swar"] {
        let p = popcount::build(name, None)?;
        let begin = p.symbols["audit_popcount"] as usize;
        let end = p.symbols["audit_popcount_end"] as usize;
        result.insert(name, p.rom[begin..end - 1].to_vec());
    }
    for opt in ["none", "speed", "speed_and_size"] {
        for size in [false, true] {
            let actual = fs::read(directory.join("kernels").join(format!("{opt}-{size}.bin")))?;
            let mut expected = result[if size || opt == "none" {
                "shift"
            } else {
                "swar"
            }]
            .clone();
            expected.push(0xC9);
            ensure!(actual == expected, "kernel differs for {opt}/{size}");
        }
    }
    Ok(result)
}
fn execute_probe(
    p: &Program,
    probe: &str,
    value: u16,
    other: u16,
    stack: u16,
    poison: u8,
) -> Result<(u64, i32)> {
    let mut e = p.emulator(&format!("__rust_audit_{probe}"))?;
    for (i, r) in "adehl".chars().enumerate() {
        e.set_reg(r, poison.wrapping_add((i * 31) as u8));
    }
    e.b = value as u8;
    e.c = (value >> 8) as u8;
    e.a = if probe == "popcount_pressure" {
        other as u8
    } else {
        poison
    };
    e.f = poison & 0xF0;
    e.set_sp(stack - 2);
    e.write16(e.sp(), p.symbols["audit_stop"]);
    e.memory[stack as usize] = (other >> 8) as u8;
    e.reset_metrics();
    e.run_until(p.symbols["audit_stop"], 4096)?;
    let mut actual = e.a as u32 | (e.b as u32) << 8;
    let mut expected = value.count_ones();
    match probe {
        "popcount_wide" => actual |= (e.c as u32) << 16 | (e.d as u32) << 24,
        "popcount_live" => expected = (value as u32 + expected) & 65535,
        "popcount_pressure" => {
            expected = (value as u32 + other as u32 + expected + other.count_ones()) & 65535
        }
        _ => {}
    }
    ensure!(
        actual == expected,
        "{probe} failed value={value} other={other} stack={stack:X} poison={poison:X}: {actual} != {expected}"
    );
    ensure!(
        e.sp() == stack
            && e.memory[stack as usize] == (other >> 8) as u8
            && e.read16(stack - 2) == p.symbols["audit_stop"],
        "{probe} corrupted its frame"
    );
    let peak = 2 + e.peak_stack_bytes;
    ensure!(
        e.written_addresses
            .iter()
            .all(|&a| stack as i32 - peak <= a as i32 && a < stack - 2),
        "{probe} wrote outside stack"
    );
    Ok((e.cycles, peak))
}
fn check_level(
    directory: &Path,
    level: &str,
    reference: &BTreeMap<&str, Vec<u8>>,
    exhaustive: bool,
) -> Result<Value> {
    let source = normalize(
        &fs::read_to_string(directory.join(format!("level-{level}.asm")))?,
        "Probes",
    )? + "\naudit_program_end::\n";
    let p = harness::assemble(
        &[
            ("probes.asm", source),
            (
                "stop.asm",
                "SECTION \"Stop\", ROM0[$100]\naudit_stop::\n nop\n".into(),
            ),
        ],
        Some(&directory.join(format!("level-{level}"))),
    )?;
    let mut entries: Vec<_> = PROBES
        .iter()
        .map(|probe| p.symbols[&format!("__rust_audit_{probe}")])
        .collect();
    entries.sort();
    entries.push(p.symbols["audit_program_end"]);
    let mut rng = Random::new(11);
    let mut cases: BTreeSet<(u16, u16)> = (0..256)
        .map(|v| (v, v ^ 65535))
        .chain((0..256).map(|v| (v << 8, v)))
        .chain([(65535, 65535), (0x5555, 0xAAAA), (0x8001, 0x1234)])
        .collect();
    cases.extend((0..1024).map(|_| (rng.range(0, 65536), rng.range(0, 65536))));
    let kernel = &reference[if ["1", "2", "3"].contains(&level) {
        "swar"
    } else {
        "shift"
    }];
    let mut results = serde_json::Map::new();
    for &probe in PROBES {
        println!("checking opt={level} {probe}");
        let start = p.symbols[&format!("__rust_audit_{probe}")];
        let end = entries[entries.iter().position(|&v| v == start).unwrap() + 1];
        let body = &p.rom[start as usize..end as usize];
        ensure!(
            body.windows(kernel.len())
                .filter(|w| *w == kernel.as_slice())
                .count()
                == if probe.ends_with("pressure") { 2 } else { 1 },
            "wrong kernel for {level}/{probe}"
        );
        let (mut n, mut min, mut max, mut peak) = (0, u64::MAX, 0, 0);
        let mut run = |value, other, base, poison| -> Result<()> {
            let (cost, stack) = execute_probe(&p, probe, value, other, base, poison)?;
            min = min.min(cost);
            max = max.max(cost);
            peak = peak.max(stack);
            n += 1;
            Ok(())
        };
        for base in [0xC800, 0xD000, 0xD002] {
            for poison in [0, 0xA5, 0xFF] {
                for &(value, other) in &cases {
                    run(value, other, base, poison)?;
                }
            }
        }
        if exhaustive {
            for value in 0..=65535u16 {
                run(
                    value,
                    value.wrapping_mul(40503).wrapping_add(12345),
                    0xD000,
                    0x55,
                )?;
            }
        }
        results.insert(probe.into(),json!({"executions":n,"code_bytes":end-start,"min_cycles":min,"max_cycles":max,"peak_stack_bytes":peak}));
    }
    Ok(json!({"probes":results,"provenance":p.provenance}))
}
fn generate(directory: PathBuf, sdcc: PathBuf) -> Result<()> {
    let mut runner = Runner::new(directory)?;
    runner.backend()?;
    let directory = runner.directory.clone();
    let sdcc = harness::root().join(sdcc).canonicalize()?;
    runner.compile(
        "rustc",
        "tests/fixtures/popcount/baseline.rs",
        "2",
        "baseline.asm",
    )?;
    let c_source = harness::root().join("tests/fixtures/popcount/sdcc-loop.c");
    let preprocessed = format!(
        "#line 1 {}\n{}",
        serde_json::to_string(&c_source.display().to_string())?,
        fs::read_to_string(&c_source)?
    );
    harness::write(&directory.join("sdcc-loop.i"), &preprocessed)?;
    runner.run(
        "sdcc",
        vec![
            sdcc.display().to_string(),
            "--c1mode".into(),
            "-msm83".into(),
            "--std-c23".into(),
            "--opt-code-speed".into(),
            "-o".into(),
            directory.join("sdcc-native.s").display().to_string(),
        ],
        Some(preprocessed.as_bytes()),
    )?;
    let rustc_version = runner.run(
        "rustc-version",
        strings(&["rustc", "+gbc-lr35902", "-vV"]),
        None,
    )?;
    let sdcc_version = runner.run(
        "sdcc-version",
        vec![sdcc.display().to_string(), "--version".into()],
        None,
    )?;
    let mut revisions = serde_json::Map::new();
    for name in [
        "third_party/rust",
        "third_party/rustc_codegen_cranelift",
        "third_party/wasmtime",
    ] {
        let revision = runner.run(
            &format!("{}-revision", name.replace('/', "-")),
            vec![
                "git".into(),
                "-C".into(),
                harness::root().join(name).display().to_string(),
                "rev-parse".into(),
                "HEAD".into(),
            ],
            None,
        )?;
        revisions.insert(name.into(), json!(revision.trim()));
    }
    let mut paths = vec![
        c_source,
        directory.join("baseline.asm"),
        directory.join("sdcc-native.s"),
    ];
    for name in [
        "tests/fixtures/popcount/baseline.rs",
        "third_party/rustc_codegen_cranelift/target/release/librustc_codegen_cranelift.so",
        "third_party/wasmtime/cranelift/codegen/src/isa/lr35902/lower.rs",
        "third_party/wasmtime/cranelift/codegen/src/isa/lr35902/inst/emit.rs",
        "third_party/wasmtime/cranelift/codegen/src/isa/lr35902/inst/regs.rs",
        "third_party/wasmtime/cranelift/codegen/src/machinst/vcode.rs",
    ] {
        paths.push(harness::root().join(name));
    }
    harness::write_json(
        &directory.join("provenance.json"),
        &json!({"commands":runner.commands,"rustc_version":rustc_version,"sdcc_version":sdcc_version,"sha256":hashes(paths)?,"git_revisions":revisions}),
    )
}
pub fn run(tool: &str, args: &[String]) -> Result<()> {
    if tool == "generate-popcount-baselines" {
        let a = Args::parse(args, &["output", "sdcc"], &[])?;
        let sdcc = a
            .get("sdcc")
            .ok_or_else(|| crate::error!("--sdcc is required"))?;
        return generate(
            a.path("output", "build/popcount-audit/regenerated"),
            PathBuf::from(sdcc),
        );
    }
    let a = Args::parse(args, &["output"], &["skip-build", "exhaustive16"])?;
    let pop = tool == "check-popcount";
    ensure!(
        pop || tool == "check-spills",
        "unknown compiler check {tool}"
    );
    let mut runner = Runner::new(a.path(
        "output",
        if pop {
            "build/popcount-replacement"
        } else {
            "build/spill-repair"
        },
    ))?;
    if !a.has("skip-build") {
        runner.checks(pop)?;
    }
    let directory = runner.directory.clone();
    let access = check_accesses(&directory)?;
    let pressure = check_pressure(&directory)?;
    let mut result = json!({"commands":runner.commands,"access_executions":access,"pressure_executions":pressure});
    if pop {
        let reference = kernels(&directory)?;
        let mut levels = serde_json::Map::new();
        for level in LEVELS {
            levels.insert(
                (*level).into(),
                check_level(&directory, level, &reference, a.has("exhaustive16"))?,
            );
        }
        result["verifier_enabled"] = json!(true);
        result["seed"] = json!(11);
        result["exhaustive16"] = json!(a.has("exhaustive16"));
        result["levels"] = Value::Object(levels);
    } else {
        result["rust"] = spill_rust(&directory, a.has("exhaustive16"))?;
    }
    result["sha256"] = source_hashes()?;
    harness::write_json(&directory.join("results.json"), &result)?;
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}
