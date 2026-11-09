use sangoku_test_support::{Result, ensure, harness, json};
use std::{env, path::PathBuf};

const LENGTHS: [usize; 5] = [0, 1, 4, 16, 256];

fn callers() -> String {
    let mut text =
        "set enable_multi_ret_implicit_sret=true\nset opt_level=speed_and_size\ntarget z80\n"
            .to_owned();
    for len in LENGTHS {
        for operation in ["compare", "equal", "move"] {
            let result = if operation == "equal" {
                "i8"
            } else if operation == "move" {
                "i16"
            } else {
                "i32"
            };
            let callee = if operation == "move" {
                "Memmove"
            } else {
                "Memcmp"
            };
            let callee_result = if operation == "move" { "i16" } else { "i32" };
            text += &format!(
                "\nfunction %{operation}_{len}(i16) -> {result} system_v {{\n    sig0 = (i16, i16, i16) -> {callee_result} system_v\n    fn0 = %{callee} sig0\nblock0(v0: i16):\n    v1 = iconst.i16 0xc400\n    v2 = iconst.i16 {len}\n    v3 = call fn0(v0, v1, v2)\n"
            );
            if operation == "equal" {
                text += "    v4 = iconst.i32 0\n    v5 = icmp eq v3, v4\n    return v5\n}\n";
            } else {
                text += "    return v3\n}\n";
            }
        }
        if len > 16 {
            continue;
        }
        text += &format!(
            "\nfunction %inline_equal_{len}(i16) -> i8 system_v {{\nblock0(v0: i16):\n    v1 = iconst.i16 0xc400\n    v2 = iconst.i8 1\n"
        );
        let mut next = 3;
        let mut previous = 2;
        for offset in (0..len).step_by(2) {
            let ty = if len == 1 { "i8" } else { "i16" };
            text += &format!(
                "    v{next} = load.{ty} v0+{offset}\n    v{} = load.{ty} v1+{offset}\n    v{} = icmp eq v{next}, v{}\n    v{} = band v{previous}, v{}\n",
                next + 1,
                next + 2,
                next + 1,
                next + 3,
                next + 2
            );
            previous = next + 3;
            next += 4;
        }
        text += &format!("    return v{previous}\n}}\n");
    }
    text
}

fn main() -> Result<()> {
    let directory = harness::root().join(
        env::args()
            .nth(1)
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("build/move-compare")),
    );
    harness::write(&directory.join("callers.clif"), callers())?;
    harness::command(&[
        "cargo".into(),
        "run".into(),
        "-q".into(),
        "--".into(),
        "build".into(),
        "examples/hello.rs".into(),
        directory.join("hello.gb").display().to_string(),
        "--opt-level".into(),
        "z".into(),
    ])?;
    let assembly = harness::command(&[
        "cargo".into(),
        "run".into(),
        "-q".into(),
        "-p".into(),
        "sangoku-lower".into(),
        "--".into(),
        directory.join("callers.clif").display().to_string(),
        "-".into(),
    ])?;
    let builtins = harness::read("build/core/immediate-abort/opt-z/compiler_builtins.s")?;
    let start = builtins
        .find("SECTION \"__rust_memcmp\"")
        .ok_or("missing builtins memcmp")?;
    let end = builtins[start + 1..]
        .find("\nSECTION ")
        .map_or(builtins.len(), |n| start + 1 + n);
    let mut rows = vec![];
    let mut provenance = serde_json::Map::new();
    for variant in ["builtins", "size", "opt"] {
        let source = if variant == "builtins" {
            builtins[start..end].to_owned()
        } else {
            harness::read(format!("tests/fixtures/audit/memcmp_{variant}.asm"))?
        };
        let move_variant = if variant == "size" { "size" } else { "opt" };
        let program = harness::assemble_tiny(
            &[
                ("callers.asm", assembly.clone()),
                ("compare.asm", source),
                (
                    "move.asm",
                    harness::runtime(&format!("memmove_{move_variant}"))?,
                ),
                (
                    "stop.asm",
                    "SECTION \"reserved\", ROM0[$0000]\nds $100, 0\ndone::\nnop\n".into(),
                ),
            ],
            Some(&directory.join(variant)),
        )?;
        provenance.insert(variant.into(), program.provenance.clone());
        for len in LENGTHS {
            let mut cases = vec![("equal".to_owned(), None, 0u8, 0u8)];
            if len > 0 {
                for (position, index) in [("first", 0), ("middle", len / 2), ("last", len - 1)] {
                    for (left, right) in [(0u8, 255u8), (255, 0), (127, 128), (128, 127)] {
                        cases.push((
                            format!("{position}_{left}_{right}"),
                            Some(index),
                            left,
                            right,
                        ));
                    }
                }
            }
            if len > 1 {
                cases.push(("first_then_opposite".to_owned(), Some(0), 127, 128));
            }
            for (case, position, left, right) in cases {
                for operation in ["body", "compare", "equal", "inline_equal"] {
                    if operation == "inline_equal" && (len > 16 || variant != "size") {
                        continue;
                    }
                    let name = if operation == "body" {
                        "__rust_memcmp".to_owned()
                    } else {
                        format!("__rust_{operation}_{len}")
                    };
                    let mut e = program.emulator(&name)?;
                    let mut initial: Vec<u8> = (0..0x800)
                        .map(|n| (n as u8).wrapping_mul(7).wrapping_add(3))
                        .collect();
                    let bytes = initial[0x400..0x400 + len].to_vec();
                    initial[0x100..0x100 + len].copy_from_slice(&bytes);
                    if let Some(index) = position {
                        initial[0x100 + index] = left;
                        initial[0x400 + index] = right;
                    }
                    if case == "first_then_opposite" {
                        initial[0x100 + len - 1] = right;
                        initial[0x400 + len - 1] = left;
                    }
                    e.memory[0xc000..0xc800].copy_from_slice(&initial);
                    e.b = 0;
                    e.c = 0xc1;
                    e.set_sp(0xde00);
                    e.write16(0xde00, program.symbols["done"]);
                    if operation == "body" {
                        e.a = 0;
                        e.memory[0xde02] = 0xc4;
                        e.write16(0xde03, len as u16);
                    }
                    let arguments = e.memory[0xde02..0xde05].to_vec();
                    e.reset_metrics();
                    e.run_until(program.symbols["done"], 1_000_000)?;
                    let actual = i32::from_le_bytes([e.a, e.b, e.c, e.d]);
                    let expected = if position.is_some() {
                        i32::from(left) - i32::from(right)
                    } else {
                        0
                    };
                    let correct = if operation.ends_with("equal") {
                        e.a == u8::from(expected == 0)
                    } else {
                        actual.signum() == expected.signum()
                    };
                    ensure!(
                        e.memory[0xc000..0xc800] == initial
                            && e.sp() == 0xde02
                            && e.memory[0xde02..0xde05] == arguments,
                        "{variant} {name}: memory/stack"
                    );
                    let caller_bytes = if operation == "body" {
                        0
                    } else {
                        program.sections[&name].size
                    };
                    let helper_bytes = if operation == "inline_equal" {
                        0
                    } else {
                        program
                            .sections
                            .values()
                            .find(|s| s.address == usize::from(program.symbols["__rust_memcmp"]))
                            .ok_or("missing compare section")?
                            .size
                    };
                    rows.push(json!({"variant":variant,"operation":operation,"length":len,"case":case,"cycles":e.cycles,"caller_bytes":caller_bytes,"helper_bytes":helper_bytes,"peak_stack_bytes":e.peak_stack_bytes+2+if operation=="body"{3}else{0},"actual":actual,"expected_sign":expected.signum(),"correct":correct}));
                    if !correct {
                        eprintln!(
                            "FAIL {variant} {operation} n={len} {case}: {actual}, expected sign {}",
                            expected.signum()
                        );
                    }
                }
            }
            if variant == "builtins" {
                continue;
            }
            for (case, dst) in [
                ("same", 0xc400usize),
                ("forward", 0xc3ff),
                ("backward", 0xc401),
                ("disjoint", 0xc100),
            ] {
                let name = format!("__rust_move_{len}");
                let mut e = program.emulator(&name)?;
                let mut expected: Vec<u8> = (0..0x800)
                    .map(|n| (n as u8).wrapping_mul(7).wrapping_add(3))
                    .collect();
                e.memory[0xc000..0xc800].copy_from_slice(&expected);
                let bytes = expected[0x400..0x400 + len].to_vec();
                expected[dst - 0xc000..dst - 0xc000 + len].copy_from_slice(&bytes);
                e.b = dst as u8;
                e.c = (dst >> 8) as u8;
                e.set_sp(0xde00);
                e.write16(0xde00, program.symbols["done"]);
                e.reset_metrics();
                e.run_until(program.symbols["done"], 1_000_000)?;
                ensure!(
                    e.memory[0xc000..0xc800] == expected
                        && e.sp() == 0xde02
                        && u16::from_le_bytes([e.a, e.b]) == dst as u16,
                    "{variant} {name} {case}"
                );
                rows.push(json!({"variant":variant,"operation":"move","length":len,"case":case,"cycles":e.cycles,"caller_bytes":program.sections[&name].size,"helper_bytes":program.size("ROM0",&["memmove"]),"peak_stack_bytes":e.peak_stack_bytes+2,"correct":true}));
            }
        }
    }
    let failed = rows.iter().filter(|r| r["correct"] == false).count();
    let references = [
        "device/lib/memcmp.c",
        "device/lib/memmove.c",
        "device/lib/z80-common/memmove.s",
    ]
    .into_iter()
    .map(|name| {
        let path = PathBuf::from("/home/takashi/sdcc").join(name);
        (
            name,
            std::fs::read(path).ok().map(|bytes| harness::hash(&bytes)),
        )
    })
    .collect::<std::collections::BTreeMap<_, _>>();
    let backend = harness::command(&[
        "git".into(),
        "-C".into(),
        "third_party/wasmtime".into(),
        "rev-parse".into(),
        "HEAD".into(),
    ])?;
    let cg_clif = harness::command(&[
        "git".into(),
        "-C".into(),
        "third_party/rustc_codegen_cranelift".into(),
        "rev-parse".into(),
        "HEAD".into(),
    ])?;
    harness::write_json(
        &directory.join("results.json"),
        &json!({"rows":rows,"provenance":provenance,"failed":failed,"reference_sha256":references,"backend_revision":backend.trim(),"cg_clif_revision":cg_clif.trim()}),
    )?;
    println!(
        "{} cases, {failed} incorrect; {}",
        rows.len(),
        directory.join("results.json").display()
    );
    ensure!(
        failed == 0,
        "comparison candidate failures recorded in results.json"
    );
    Ok(())
}
