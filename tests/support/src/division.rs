use crate::{Result, Value, ensure, json, number};
use crate::{
    harness::{self, Program},
    random::Random,
    runtime,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};
pub const VARIANTS: &[&str] = &[
    "rolled",
    "unroll4",
    "unroll16",
    "early",
    "sdcc-sm83",
    "sdcc-z80",
    "sdcc-z80-early",
];
pub const OPERATIONS: &[&str] = &["q", "r", "both", "separate"];
pub const BOUNDARIES: &[u16] = &[
    0, 1, 2, 3, 126, 127, 128, 129, 254, 255, 256, 257, 32767, 32768, 32769, 65534, 65535,
];
fn fixture(name: &str) -> Result<String> {
    harness::read(format!("tests/fixtures/division/{name}"))
}
pub fn source_for(variant: &str) -> Result<String> {
    if variant == "runtime" {
        return Ok(harness::read("runtime/udiv16.asm")?
            .replacen(
                "SECTION \"Unsigned 16-bit division\", ROM0",
                "SECTION \"Audit division\", ROM0[$400]",
                1,
            )
            .replace("__gbc_udiv16::", "audit_q::\n__gbc_udiv16::")
            .replace("__gbc_urem16::", "audit_r::\n__gbc_urem16::")
            .replace("div16_core:", "audit_core::\ndiv16_core:"));
    }
    if variant == "raw" || variant == "raw-alt" {
        return Ok(fixture(if variant == "raw" {
            "baseline.asm"
        } else {
            "baseline-alt.asm"
        })?
        .replacen("ROM0", "ROM0[$400]", 1)
        .replace("__gbc_udiv16::", "audit_q::\n__gbc_udiv16::")
        .replace("__gbc_urem16::", "audit_r::\n__gbc_urem16::"));
    }
    ensure!(
        VARIANTS.contains(&variant),
        "unknown division variant {variant}"
    );
    let source = if variant.starts_with("sdcc-") {
        format!(
            "{}{}",
            if variant.ends_with("-early") {
                "DEF AUDIT_EARLY EQU 1\n"
            } else {
                ""
            },
            fixture(&format!("{}.asm", variant.trim_end_matches("-early")))?
        )
    } else {
        let unroll = match variant {
            "unroll4" => 4,
            "unroll16" => 16,
            _ => 1,
        };
        format!(
            "DEF AUDIT_UNROLL EQU {unroll}\n{}{}",
            if variant == "early" {
                "DEF AUDIT_EARLY EQU 1\n"
            } else {
                ""
            },
            fixture("restoring.asm")?
        )
    };
    Ok(source
        .replace(
            "INCLUDE \"tests/fixtures/division/wrappers.asm\"",
            &fixture("wrappers.asm")?,
        )
        .replace(
            "INCLUDE \"tests/fixtures/division/early.asm\"",
            &fixture("early.asm")?,
        ))
}
pub fn quotient_remainder_size(variant: &str, artifacts: Option<&Path>) -> Result<usize> {
    let wrappers = fixture("wrappers.asm")?;
    let combined = &wrappers[wrappers.find("audit_both::").unwrap()..];
    let source = source_for(variant)?.replace(combined, "");
    Ok(
        harness::assemble(&[("division.asm", source)], artifacts)?
            .size("ROM0", &["Audit division"]),
    )
}
fn setup_call(entry: &str) -> String {
    format!(
        "    ld hl, audit_inputs\n    ld b, [hl]\n    inc hl\n    ld c, [hl]\n    inc hl\n    ld a, [hl]\n    ld e, a\n    inc hl\n    ld a, [hl]\n    add sp, -2\n    ld hl, sp + 0\n    ld [hl], a\n    ld a, e\n    call {entry}\n    add sp, 2\n"
    )
}
pub fn build(variant: &str, artifacts: Option<&Path>) -> Result<Program> {
    let operations: &[&str] = if variant == "runtime" {
        &["q", "r", "separate"]
    } else if variant == "raw" || variant == "raw-alt" {
        &["q", "r"]
    } else {
        OPERATIONS
    };
    let mut caller = "SECTION \"Audit callers\", ROM0[$100]\n".to_owned();
    for op in operations {
        caller += &format!("audit_{op}_start::\n");
        if *op == "separate" {
            caller += &setup_call("audit_q");
            caller += "    ld c, a\n    push bc\n";
            caller += &setup_call("audit_r");
            caller += "    ld c, a\n    ld d, b\n    pop hl\n    ld a, l\n    ld b, h\n";
        } else {
            caller += &setup_call(&format!("audit_{op}"));
        }
        caller += &format!("audit_{op}_done::\n");
    }
    caller += "SECTION \"Audit inputs\", WRAM0[$c100]\naudit_inputs::\n    ds 4\n";
    harness::assemble(
        &[
            ("division.asm", source_for(variant)?),
            ("callers.asm", caller),
        ],
        artifacts,
    )
}
pub fn run_case(
    program: &Program,
    op: &str,
    lhs: u16,
    rhs: u16,
    body: bool,
    poison: u8,
) -> Result<Value> {
    ensure!(
        !(body && op == "separate"),
        "separate calls have no single helper body"
    );
    let entry = if body {
        format!("audit_{op}")
    } else {
        format!("audit_{op}_start")
    };
    let mut r = runtime::execute(
        program,
        &entry,
        lhs,
        rhs,
        body,
        &format!("audit_{op}_done"),
        |e| {
            let a = program.symbols["audit_inputs"] as usize;
            e.memory[a..a + 2].copy_from_slice(&lhs.to_le_bytes());
            e.memory[a + 2..a + 4].copy_from_slice(&rhs.to_le_bytes());
            e.d = poison;
            e.e = poison ^ 255;
            e.set_hl(u16::from_be_bytes([poison, poison ^ 255]));
        },
    )?;
    let regs = &r["registers"];
    let value = number(&regs["a"]) | number(&regs["b"]) << 8;
    let actual = if op == "both" || op == "separate" {
        json!([value, number(&regs["c"]) | number(&regs["d"]) << 8])
    } else {
        json!(value)
    };
    let expected = match lhs.checked_div(rhs) {
        None => Value::Null,
        Some(quotient) => match op {
            "q" => json!(quotient),
            "r" => json!(lhs % rhs),
            _ => json!([quotient, lhs % rhs]),
        },
    };
    let base = 0xD000 - number(&r["peak_stack_bytes"]);
    let writes = r["written_addresses"]
        .as_array()
        .unwrap()
        .iter()
        .all(|a| base <= number(a) && number(a) < 0xD000);
    let correct = if rhs == 0 {
        r["status"] == "rst_10"
    } else {
        r["status"] == "returned" && r["stack_restored"] == true && actual == expected && writes
    };
    r["actual"] = actual;
    r["expected"] = expected;
    r["correct"] = json!(correct);
    r["writes_in_stack"] = json!(writes);
    Ok(r)
}
pub fn input_classes(samples: usize, seed: u64) -> Vec<(String, Vec<(u16, u16)>)> {
    let mut rng = Random::new(seed);
    let mut classes = vec![
        (
            "boundary".into(),
            BOUNDARIES
                .iter()
                .flat_map(|&a| BOUNDARIES.iter().filter(|&&b| b != 0).map(move |&b| (a, b)))
                .collect(),
        ),
        (
            "divisor-one".into(),
            BOUNDARIES.iter().map(|&a| (a, 1)).collect(),
        ),
        (
            "equal".into(),
            BOUNDARIES
                .iter()
                .filter(|&&a| a != 0)
                .map(|&a| (a, a))
                .collect(),
        ),
        (
            "power-of-two".into(),
            BOUNDARIES
                .iter()
                .flat_map(|&a| (0..16).map(move |bit| (a, 1 << bit)))
                .collect(),
        ),
    ];
    let rhs: Vec<_> = (0..samples).map(|_| rng.range(1, 65536)).collect();
    classes.push((
        "less-than".into(),
        rhs.into_iter()
            .map(|b| (rng.range(0, b as u32), b))
            .collect(),
    ));
    classes.push((
        "narrow-random".into(),
        (0..samples)
            .map(|_| (rng.range(0, 65536), rng.range(1, 128)))
            .collect(),
    ));
    classes.push((
        "wide-random".into(),
        (0..samples)
            .map(|_| (rng.range(0, 65536), rng.range(128, 65536)))
            .collect(),
    ));
    classes
}
pub fn audit(
    variants: &[String],
    samples: usize,
    seed: u64,
    exhaustive8: bool,
    artifacts: &Path,
) -> Result<Value> {
    let classes = input_classes(samples, seed);
    let inputs: serde_json::Map<_, _> =
        classes.iter().map(|(k, v)| (k.clone(), json!(v))).collect();
    let mut report = json!({
        "timing_unit":"CPU T-cycles",
        "seed":seed,
        "samples_per_random_class":samples,
        "input_classes":inputs,
        "upstream":serde_json::from_str::<Value>(&fixture("provenance.json")?)?,
        "results":[],
        "baselines":{},
        "limitations":["flat-memory SM83 subset, no interrupts/peripherals/bank switching/bus timing","hand-written callers reload operands from a 4-byte harness-owned WRAM input block","code sizes include available quotient/remainder/combined wrappers and the core","combined result quotient A/B and remainder C/D; separate calls preserve quotient on stack","body measurements include RET and a preexisting 4-byte caller frame","no 32-bit core reproducer: generic SDCC 32-bit C inspected but not benchmarked"]
    });
    for variant in ["raw", "raw-alt"] {
        let dir = artifacts.join(variant);
        match build(variant, Some(&dir)) {
            Ok(p) => {
                let mut cases = vec![];
                for op in ["q", "r"] {
                    for (a, b) in [(1, 1), (0x1234, 16), (65535, 0), (0, 65535)] {
                        for poison in [0, 0x55] {
                            let r = run_case(&p, op, a, b, true, poison)?;
                            cases.push(json!({
                                "operation":op,
                                "lhs":a,
                                "rhs":b,
                                "poison":poison,
                                "actual":r["actual"],
                                "expected":r["expected"],
                                "correct":r["correct"],
                                "cycles":r["cycles"]
                            }));
                        }
                    }
                }
                report["baselines"][variant] = json!({"code_bytes":p.size("ROM0",&["Unsigned 16-bit division"]),"cases":cases,"provenance":p.provenance});
            }
            Err(e) => {
                let Some(failure) = e.downcast_ref::<harness::CommandFailure>() else {
                    return Err(e);
                };
                report["baselines"][variant] =
                    json!({"build_error":e.to_string(),"stderr":failure.stderr,"artifacts":dir});
            }
        }
    }
    for variant in variants {
        println!("auditing {variant}");
        let p = build(variant, Some(&artifacts.join(variant)))?;
        let operations: Vec<_> = OPERATIONS
            .iter()
            .copied()
            .filter(|op| p.symbols.contains_key(&format!("audit_{op}_start")))
            .collect();
        let mut groups: BTreeMap<(String, String, String), Vec<Value>> = BTreeMap::new();
        let mut order = vec![];
        let mut errors = vec![];
        let mut executions = 0;
        let mut changed = BTreeSet::new();
        for (label, pairs) in &classes {
            for &(a, b) in pairs {
                for &op in &operations {
                    for &body in if op == "separate" {
                        &[false][..]
                    } else {
                        &[true, false][..]
                    } {
                        let r = run_case(&p, op, a, b, body, 0x55)?;
                        executions += 1;
                        for v in r["observed_register_changes"].as_array().unwrap() {
                            changed.insert(v.as_str().unwrap().to_owned());
                        }
                        if r["correct"] != true {
                            errors.push(json!({"class":label,"operation":op,"body":body,"lhs":a,"rhs":b,"result":r}));
                        }
                        let key = (
                            label.clone(),
                            op.to_owned(),
                            if body { "body" } else { "caller" }.to_owned(),
                        );
                        if !groups.contains_key(&key) {
                            order.push(key.clone());
                        }
                        groups.entry(key).or_default().push(r);
                    }
                }
            }
        }
        let mut traps = vec![];
        for a in [0, 1, 32768, 65535] {
            for &op in &operations {
                let r = run_case(&p, op, a, 0, false, 0x55)?;
                traps.push(json!({"lhs":a,"operation":op,"correct":r["correct"],"cycles":r["cycles"],"peak_stack_bytes":r["peak_stack_bytes"]}));
            }
        }
        let (mut count, mut correct) = (0, true);
        if exhaustive8 && ["sdcc-z80", "sdcc-z80-early", "runtime"].contains(&variant.as_str()) {
            for a in 0..256 {
                for b in 0..256 {
                    for &op in if variant == "runtime" {
                        &["q", "r"][..]
                    } else {
                        &["both"][..]
                    } {
                        let r = run_case(&p, op, a, b, true, 0x55)?;
                        count += 1;
                        correct &= r["correct"] == true;
                    }
                }
            }
        }
        let mut summary = vec![];
        for (label, op, mode) in order {
            let rows = &groups[&(label.clone(), op.clone(), mode.clone())];
            let mut row = crate::cli::statistics(rows);
            row["class"] = json!(label);
            row["operation"] = json!(op);
            row["mode"] = json!(mode);
            summary.push(row);
        }
        let caller: serde_json::Map<_, _> = operations
            .iter()
            .map(|op| {
                (
                    (*op).to_owned(),
                    json!(
                        p.symbols[&format!("audit_{op}_done")]
                            - p.symbols[&format!("audit_{op}_start")]
                    ),
                )
            })
            .collect();
        let section = &p.sections["Audit division"];
        let result = json!({
            "variant":variant,
            "code_bytes":p.size("ROM0",&["Audit division"]),
            "quotient_remainder_code_bytes":quotient_remainder_size(variant,Some(&artifacts.join(variant).join("q-r-only")))?,
            "core_bytes":section.address+section.size-p.symbols["audit_core"]as usize,
            "table_bytes":0,
            "scratch_ram_bytes":0,
            "harness_input_bytes":4,
            "caller_bytes":caller,
            "allowed_clobbers":["a","b","c","d","e","f","h","l"],
            "observed_changes":changed,
            "provenance":p.provenance,
            "validated_executions":executions,
            "errors":errors.iter().take(8).collect::<Vec<_>>(),
            "error_count":errors.len(),
            "zero_divisor":traps,"exhaustive8":{"executed":count,
            "correct":correct},
            "summary":summary
        });
        report["results"].as_array_mut().unwrap().push(result);
    }
    Ok(report)
}
