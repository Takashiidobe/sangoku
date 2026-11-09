use sangoku_test_support::{Result, ensure, harness, json};
use std::{env, path::PathBuf};

const LENGTHS: [usize; 5] = [1, 2, 4, 8, 16];

fn source() -> String {
    let mut text =
        "set enable_multi_ret_implicit_sret=true\nset opt_level=speed_and_size\ntarget z80\n"
            .to_owned();
    for operation in ["copy", "fill"] {
        for len in LENGTHS {
            for strategy in ["inline", "helper"] {
                text += &format!("\nfunction %{operation}_{strategy}_{len}(i16) system_v {{\n");
                if strategy == "helper" {
                    let (symbol, value_type) = if operation == "copy" {
                        ("Memcpy", "i16")
                    } else {
                        ("Memset", "i32")
                    };
                    text += &format!(
                        "    sig0 = (i16, {value_type}, i16) -> i16 system_v\n    fn0 = %{symbol} sig0\n"
                    );
                }
                text += "block0(v0: i16):\n";
                if strategy == "helper" {
                    let (ty, value) = if operation == "copy" {
                        ("i16", 0xc400)
                    } else {
                        ("i32", 0xa5)
                    };
                    text += &format!(
                        "    v1 = iconst.{ty} {value}\n    v2 = iconst.i16 {len}\n    v3 = call fn0(v0, v1, v2)\n"
                    );
                } else {
                    let ty = if len == 1 { "i8" } else { "i16" };
                    let width = if len == 1 { 1 } else { 2 };
                    if operation == "copy" {
                        text += "    v1 = iconst.i16 0xc400\n";
                    } else {
                        let value = if len == 1 { 0xa5 } else { 0xa5a5 };
                        text += &format!("    v1 = iconst.{ty} {value}\n");
                    }
                    for (index, offset) in (0..len).step_by(width).enumerate() {
                        if operation == "copy" {
                            text += &format!(
                                "    v{} = load.{ty} v1+{offset}\n    store v{}, v0+{offset}\n",
                                index + 2,
                                index + 2
                            );
                        } else {
                            text += &format!("    store v1, v0+{offset}\n");
                        }
                    }
                }
                text += "    return\n}\n";
            }
        }
    }
    text
}

fn main() -> Result<()> {
    let directory = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("build/memory-inline"));
    let directory = harness::root().join(directory);
    let input = directory.join("callers.clif");
    harness::write(&input, source())?;
    let assembly = harness::command(&[
        "cargo".into(),
        "run".into(),
        "-q".into(),
        "-p".into(),
        "sangoku-lower".into(),
        "--".into(),
        input.display().to_string(),
        "-".into(),
    ])?;
    let mut rows = vec![];
    let mut provenance = serde_json::Map::new();
    for variant in ["size", "opt"] {
        let artifact_dir = directory.join(variant);
        let program = harness::assemble_tiny(
            &[
                ("callers.asm", assembly.clone()),
                (
                    "memcpy.asm",
                    harness::runtime(&format!("memcpy_{variant}"))?,
                ),
                (
                    "memset.asm",
                    harness::runtime(&format!("memset_{variant}"))?,
                ),
                (
                    "stop.asm",
                    "SECTION \"reserved\", ROM0[$0000]\nds $100, 0\ndone::\nnop\n".into(),
                ),
            ],
            Some(&artifact_dir),
        )?;
        provenance.insert(variant.into(), program.provenance.clone());
        for operation in ["copy", "fill"] {
            for len in LENGTHS {
                for strategy in ["inline", "helper"] {
                    let name = format!("__rust_{operation}_{strategy}_{len}");
                    let code_bytes = program
                        .sections
                        .values()
                        .find(|s| s.address == usize::from(program.symbols[&name]))
                        .ok_or("missing caller section")?
                        .size;
                    let helper_bytes = if strategy == "helper" {
                        program.size(
                            "ROM0",
                            &[if operation == "copy" {
                                "memcpy"
                            } else {
                                "memset"
                            }],
                        )
                    } else {
                        0
                    };
                    for dst in [0xc100usize, 0xc1ff] {
                        let mut e = program.emulator(&name)?;
                        let mut expected: Vec<u8> = (0..0x800)
                            .map(|i| (i as u8).wrapping_mul(7).wrapping_add(3))
                            .collect();
                        e.memory[0xc000..0xc800].copy_from_slice(&expected);
                        if operation == "copy" {
                            let bytes = expected[0x400..0x400 + len].to_vec();
                            expected[dst - 0xc000..dst - 0xc000 + len].copy_from_slice(&bytes);
                        } else {
                            expected[dst - 0xc000..dst - 0xc000 + len].fill(0xa5);
                        }
                        e.b = dst as u8;
                        e.c = (dst >> 8) as u8;
                        e.set_sp(0xde00);
                        e.write16(0xde00, program.symbols["done"]);
                        e.reset_metrics();
                        e.run_until(program.symbols["done"], 100_000)?;
                        ensure!(
                            e.memory[0xc000..0xc800] == expected,
                            "{name}: memory mismatch"
                        );
                        ensure!(e.sp() == 0xde02, "{name}: stack not restored");
                        let stack_bytes = e.peak_stack_bytes + 2;
                        rows.push(json!({"operation":operation,"variant":variant,"length":len,"strategy":strategy,
                            "destination":dst,"cycles":e.cycles,"caller_bytes":code_bytes,"helper_bytes":helper_bytes,
                            "peak_stack_bytes":stack_bytes,"correct":true}));
                        if dst == 0xc100 {
                            println!(
                                "{operation} {variant} {len:2} {strategy:6}: {} T, {code_bytes}+{helper_bytes} bytes, stack {stack_bytes}",
                                e.cycles
                            );
                        }
                    }
                }
            }
        }
    }
    harness::write_json(
        &directory.join("results.json"),
        &json!({
            "scope":"optimized CLIF, i16 chunks, dynamic destination, constant source/value, no unrelated live values; entry through RET",
            "rows":rows,
            "assembler_provenance":provenance,
            "cg_clif_revision":harness::command(&["git".into(),"-C".into(),"third_party/rustc_codegen_cranelift".into(),"rev-parse".into(),"HEAD".into()])?.trim(),
            "backend_revision":harness::command(&["git".into(),"-C".into(),"third_party/wasmtime".into(),"rev-parse".into(),"HEAD".into()])?.trim(),
            "host_rustc":harness::command(&["rustc".into(),"--version".into()])?.trim(),
        }),
    )?;
    Ok(())
}
