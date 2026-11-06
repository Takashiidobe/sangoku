use crate::{Result, Value, ensure, json};
use crate::{
    asm,
    harness::{self, Program},
    random::Random,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};
pub const VARIANTS: &[&str] = &[
    "repaired",
    "shift",
    "byte-loop",
    "unrolled",
    "clear",
    "swar",
    "swar-shared",
    "nibble",
    "nibble-page",
    "byte",
    "sdcc",
];
pub const INLINE: &[&str] = &[
    "repaired",
    "shift",
    "byte-loop",
    "unrolled",
    "clear",
    "swar",
    "nibble",
    "nibble-page",
    "byte",
];
pub const PRESERVES_HL: &[&str] = &[
    "shift",
    "byte-loop",
    "unrolled",
    "clear",
    "swar",
    "swar-shared",
];
fn fixture(name: &str) -> Result<String> {
    harness::read(format!("tests/fixtures/popcount/{name}"))
}
pub fn source_for(variant: &str) -> Result<String> {
    ensure!(
        VARIANTS.contains(&variant) || variant == "raw",
        "unknown popcount variant {variant}"
    );
    if variant == "raw" || variant == "repaired" {
        let re = regex::Regex::new(r#"SECTION "[^"]+", ROM0"#)?;
        let mut source = re
            .replacen(
                &fixture("baseline.asm")?,
                1,
                regex::NoExpand("SECTION \"Audit helper\", ROM0[$400]"),
            )
            .to_string()
            .replace("__rust_audit_popcount", "audit_popcount");
        if variant == "repaired" {
            let store = "ld hl, sp + 0\nld [hl], a";
            let load = "ld hl, sp + 0\nld a, [hl]";
            ensure!(
                source.matches(store).count() == 1 && source.matches(load).count() == 1,
                "unexpected frozen spill/reload sequences"
            );
            source = source
                .replace(store, "push hl\nld hl, sp + 2\nld [hl], a\npop hl")
                .replace(load, "push hl\nld hl, sp + 2\nld a, [hl]\npop hl");
        }
        return Ok(source + "\naudit_popcount_end::\n");
    }
    let mut source = if variant == "sdcc" {
        let re = regex::Regex::new(r"\b(\d+)\$")?;
        let lines: Vec<_> = fixture("sdcc-native.s")?
            .lines()
            .filter_map(|line| {
                let line = line.split(';').next().unwrap().trim();
                if line.is_empty() || line.starts_with('.') {
                    None
                } else {
                    Some(
                        re.replace_all(line, "sdcc_$1")
                            .replace('#', "")
                            .replace("_audit_popcount::", "sdcc_count:"),
                    )
                }
            })
            .collect();
        format!(
            "audit_popcount::\n    ld e, b\n    ld d, c\n    call sdcc_count\n    ld a, c\n    ret\naudit_popcount_end::\n{}\n",
            lines.join("\n")
        )
    } else {
        fixture(&format!("{}.asm", variant.trim_end_matches("-page")))?
    };
    if variant == "nibble-page" {
        source = source
            .replace("    add LOW(audit_table)\n", "")
            .replace("ROM0[$810], ALIGN[4]", "ROM0[$800], ALIGN[8]");
    }
    Ok("SECTION \"Audit helper\", ROM0[$400]\n".to_owned() + &source)
}
pub fn modes_for(variant: &str) -> &'static [&'static str] {
    if INLINE.contains(&variant) {
        &["body", "call", "call-live", "inline", "inline-live"]
    } else {
        &["body", "call", "call-live"]
    }
}
pub fn build(variant: &str, artifacts: Option<&Path>) -> Result<Program> {
    let source = source_for(variant)?;
    let mut caller = "SECTION \"Audit callers\", ROM0[$100]\n".to_owned();
    if INLINE.contains(&variant) {
        let body = source
            .split_once("audit_popcount::\n")
            .unwrap()
            .1
            .split_once("audit_popcount_end::")
            .unwrap()
            .0
            .trim();
        ensure!(body.ends_with("ret"), "inline fixture must end in RET");
        caller += &format!("MACRO POPCOUNT_INLINE\n{}\nENDM\n", &body[..body.len() - 3]);
    }
    for &mode in &modes_for(variant)[1..] {
        let key = mode.replace('-', "_");
        caller += &format!("audit_{key}_start::\n{}", asm::LOAD_INPUT);
        let live = mode.ends_with("live");
        let registers = live && mode.starts_with("inline") && PRESERVES_HL.contains(&variant);
        if live {
            caller += if registers {
                "    ld l, b\n    ld h, c\n"
            } else {
                "    push bc\n"
            };
        }
        caller += if mode.starts_with("inline") {
            "    POPCOUNT_INLINE\n"
        } else {
            "    call audit_popcount\n"
        };
        if live {
            caller += if registers {
                asm::ADD_REGISTER_INPUT
            } else {
                asm::ADD_STACK_INPUT
            };
        }
        caller += &format!("audit_{key}_done::\n");
    }
    caller += "SECTION \"Audit input\", WRAM0[$c100]\naudit_input::\n    ds 2\n";
    harness::assemble(
        &[("helper.asm", source), ("callers.asm", caller)],
        artifacts,
    )
}
pub fn run_case(
    program: &Program,
    variant: &str,
    value: u16,
    mode: &str,
    poison: u8,
    stack_base: u16,
) -> Result<Value> {
    let key = mode.replace('-', "_");
    let body = mode == "body";
    let stop = program.symbols[if body {
        "audit_call_done".to_owned()
    } else {
        format!("audit_{key}_done")
    }
    .as_str()];
    let mut emu = program.emulator(&if body {
        "audit_popcount".to_owned()
    } else {
        format!("audit_{key}_start")
    })?;
    emu.d = poison;
    emu.e = poison ^ 255;
    emu.set_hl(u16::from_be_bytes([poison, poison ^ 255]));
    emu.f = 0xF0;
    emu.set_sp(stack_base);
    emu.memory[0xC100..0xC102].copy_from_slice(&value.to_le_bytes());
    if body {
        emu.b = value as u8;
        emu.c = (value >> 8) as u8;
        emu.set_sp(emu.sp().wrapping_sub(2));
        emu.write16(emu.sp(), stop);
    }
    let initial: Vec<_> = "abcdefhl".chars().map(|r| (r, emu.reg(r))).collect();
    let hl = emu.hl();
    emu.reset_metrics();
    emu.run_until(stop, 2048)?;
    let actual = emu.a as u16 | (emu.b as u16) << 8;
    let expected = if mode.ends_with("live") {
        value.wrapping_add(value.count_ones() as u16)
    } else {
        value.count_ones() as u16
    };
    let entry = if body { 2 } else { 0 };
    let peak = entry + emu.peak_stack_bytes;
    let writes = emu
        .written_addresses
        .iter()
        .all(|&a| stack_base as i32 - peak <= a as i32 && a < stack_base);
    let preserved = !body || !PRESERVES_HL.contains(&variant) || emu.hl() == hl;
    let restored = emu.sp() == stack_base;
    Ok(json!({
        "actual":actual,
        "expected":expected,
        "correct":actual==expected&&writes&&restored&&preserved,
        "cycles":emu.cycles,
        "entry_stack_bytes":entry,
        "peak_stack_bytes":peak,
        "writes_in_stack":writes,
        "stack_restored":restored,
        "observed_changes":initial.into_iter().filter(|(r,v)|emu.reg(*r)!=*v).map(|(r,_)|r.to_string()).collect::<Vec<_>>()
    }))
}
pub fn input_classes(samples: usize, seed: u64) -> Vec<(String, Vec<u16>)> {
    let mut rng = Random::new(seed);
    let sparse: BTreeSet<_> = (0..16)
        .flat_map(|l| (l..16).map(move |r| (1u16 << l) | (1u16 << r)))
        .collect();
    let dense: BTreeSet<_> = sparse.iter().map(|v| 65535 ^ v).collect();
    vec![
        ("all-8-bit".into(), (0..256).collect()),
        ("zero".into(), vec![0]),
        ("single-bit".into(), (0..16).map(|b| 1 << b).collect()),
        ("sparse".into(), sparse.into_iter().collect()),
        ("dense".into(), dense.into_iter().collect()),
        (
            "boundary".into(),
            vec![
                0, 1, 127, 128, 255, 256, 257, 32767, 32768, 65534, 65535, 0x5555, 0xAAAA, 0x8001,
                0xFF00,
            ],
        ),
        (
            "uniform-random".into(),
            (0..samples).map(|_| rng.range(0, 65536)).collect(),
        ),
    ]
}
pub fn audit(
    variants: &[String],
    samples: usize,
    seed: u64,
    exhaustive16: bool,
    artifacts: &Path,
) -> Result<Value> {
    let classes = input_classes(samples, seed);
    let inputs: serde_json::Map<_, _> =
        classes.iter().map(|(k, v)| (k.clone(), json!(v))).collect();
    let raw = build("raw", Some(&artifacts.join("raw")))?;
    let mut report = json!({
        "timing_unit":"CPU T-cycles",
        "seed":seed,
        "samples":samples,
        "input_classes":inputs,
        "baseline":[],
        "results":[],
        "limitations":["flat-memory instruction subset, no interrupts/banking/peripheral timing","synthetic callers load input from two harness-owned RAM bytes","call-live conservatively saves input on stack across the normal call ABI","inline-live preserves input in free HL where the audited body preserves that pair","table alignment padding and actual future allocator spills are separate costs"],
        "baseline_code_bytes":raw.size("ROM0",&["Audit helper"]),
        "baseline_provenance":raw.provenance
    });
    for value in [0, 1, 255, 256, 0x1234, 65535] {
        for stack in [0xD000, 0xD002] {
            report["baseline"].as_array_mut().unwrap().push(json!({"value":value,"stack_base":stack,"result":run_case(&raw,"raw",value,"body",0x55,stack)?}));
        }
    }
    for variant in variants {
        println!("auditing {variant}");
        let p = build(variant, Some(&artifacts.join(variant)))?;
        let mut groups: BTreeMap<(String, String), Vec<Value>> = BTreeMap::new();
        let mut order = vec![];
        let mut changes: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        let mut errors = vec![];
        let mut executions = 0;
        for (label, values) in &classes {
            for &value in values {
                for &mode in modes_for(variant) {
                    let r = run_case(&p, variant, value, mode, 0x55, 0xD000)?;
                    executions += 1;
                    changes.entry(mode.into()).or_default().extend(
                        r["observed_changes"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|x| x.as_str().unwrap().to_owned()),
                    );
                    if r["correct"] != true {
                        errors.push(json!({"class":label,"value":value,"mode":mode,"result":r}));
                    }
                    let key = (label.clone(), mode.into());
                    if !groups.contains_key(&key) {
                        order.push(key.clone());
                    }
                    groups.entry(key).or_default().push(r);
                }
            }
        }
        let (mut count, mut correct) = (0, true);
        if exhaustive16 && ["swar", "nibble", "nibble-page", "byte"].contains(&variant.as_str()) {
            for value in 0..=65535 {
                let r = run_case(&p, variant, value, "body", 0x55, 0xD000)?;
                count += 1;
                correct &= r["correct"] == true;
            }
        }
        let mut summary = vec![];
        for (label, mode) in order {
            let mut row = crate::cli::statistics(&groups[&(label.clone(), mode.clone())]);
            row["class"] = json!(label);
            row["mode"] = json!(mode);
            summary.push(row);
        }
        let helper = p.size("ROM0", &["Audit helper"]);
        let table = p.size("ROM0", &["Audit table"]);
        let mut caller = serde_json::Map::new();
        let mut totals = serde_json::Map::new();
        for &mode in &modes_for(variant)[1..] {
            let key = mode.replace('-', "_");
            let size =
                p.symbols[&format!("audit_{key}_done")] - p.symbols[&format!("audit_{key}_start")];
            caller.insert(mode.into(), json!(size));
            totals.insert(
                mode.into(),
                json!(size as usize + if mode.starts_with("call") { helper } else { 0 } + table),
            );
        }
        let result = json!({
            "variant":variant,
            "code_bytes":helper,
            "table_bytes":table,
            "table_alignment":if variant=="nibble"{16}else if variant=="nibble-page"||variant=="byte"{256}else{1},
            "scratch_ram_bytes":0,
            "harness_input_bytes":2,
            "caller_bytes":caller,
            "caller_total_rom_bytes":totals,
            "body_observed_changes":changes.get("body"),
            "observed_changes_by_mode":changes,
            "provenance":p.provenance,
            "validated_executions":executions,
            "errors":errors.iter().take(8).collect::<Vec<_>>(),
            "error_count":errors.len(),"exhaustive16":{"executed":count,
            "correct":correct},
            "summary":summary
        });
        report["results"].as_array_mut().unwrap().push(result);
    }
    Ok(report)
}
