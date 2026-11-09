use sangoku_test_support::{Result, ensure, harness};
use std::process::Command;

const STACK: u16 = 0xD000;
const DATA: u16 = 0xC100;

fn program(fixture: &str) -> Result<harness::Program> {
    let path = format!(
        "{}/tests/fixtures/emulate/{fixture}.clif",
        harness::root().display()
    );
    lower_program(&path)
}

fn lower_program(path: &str) -> Result<harness::Program> {
    let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
        .args(["lower", path, "-"])
        .output()?;
    ensure!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let assembly = String::from_utf8(output.stdout)?;
    let needs_memory = assembly.contains("__gbc_mem")
        || assembly.contains("memcpy")
        || assembly.contains("memmove");
    let needs_convert = assembly.contains("__gbc_convert");
    let needs_float128 = assembly.contains("__gbc_f128");
    let needs_float64 = assembly.contains("__gbc_f64");
    let needs_float32 = assembly
        .replace("__gbc_f64", "")
        .replace("__gbc_f128", "")
        .contains("__gbc_f");
    let mut sources = vec![
        ("lowered.s", assembly),
        (
            "mul16.asm",
            harness::read("runtime/integer/mul/size/i16.asm")?,
        ),
        ("wide32.asm", harness::read("runtime/integer/all.asm")?),
        (
            "stop.s",
            "SECTION \"done\", ROM0\nEXPORT done\ndone::\n  nop\n".into(),
        ),
    ];
    if needs_float32 {
        sources.push((
            "softfloat32.asm",
            harness::read("runtime/float/all_f32.asm")?,
        ));
    }
    if needs_float64 {
        sources.push((
            "softfloat64.asm",
            harness::read("runtime/float/f64/common/dispatch.asm")?,
        ));
    }
    if needs_float128 {
        sources.push((
            "softfloat128.asm",
            harness::read("runtime/float/f128/common/dispatch.asm")?,
        ));
    }
    if needs_convert {
        sources.push((
            "convert.asm",
            harness::read("runtime/float/convert/common/dispatch.asm")?,
        ));
    }
    if needs_memory {
        for name in ["memset_opt", "memmove_opt", "memcpy_opt"] {
            sources.push((name, harness::runtime(name)?));
        }
    }
    harness::assemble(&sources, None)
}

#[test]
fn i32_load_and_store_round_trip() -> Result<()> {
    let program = program("load-store-i32")?;
    let stop = program.symbols["done"];

    let mut load = program.emulator("__rust_ld")?;
    load.memory[DATA as usize..DATA as usize + 4].copy_from_slice(&0xA1B2C3D4u32.to_le_bytes());
    load.b = DATA as u8;
    load.c = (DATA >> 8) as u8;
    load.set_sp(STACK - 2);
    load.write16(STACK - 2, stop);
    load.run_until(stop, 4096)?;
    let value = u32::from(load.a)
        | u32::from(load.b) << 8
        | u32::from(load.c) << 16
        | u32::from(load.d) << 24;
    assert_eq!(value, 0xA1B2C3D4);

    let mut store = program.emulator("__rust_st")?;
    store.memory[DATA as usize..DATA as usize + 6].fill(0xEE);
    store.b = DATA as u8;
    store.c = (DATA >> 8) as u8;
    store.set_sp(STACK - 2);
    store.write16(STACK - 2, stop);
    store.run_until(stop, 4096)?;
    assert_eq!(
        store.memory[DATA as usize..DATA as usize + 6],
        [0x78, 0x56, 0x34, 0x12, 0xEE, 0xEE]
    );
    Ok(())
}

const BASE: u16 = 0xC200;

fn call(
    program: &harness::Program,
    name: &str,
    memory: &[(u16, u8)],
) -> Result<sangoku_test_support::emu::Emulator> {
    let stop = program.symbols["done"];
    let mut emulator = program.emulator(name)?;
    for (address, byte) in memory {
        emulator.memory[*address as usize] = *byte;
    }
    emulator.b = BASE as u8;
    emulator.c = (BASE >> 8) as u8;
    emulator.set_sp(STACK - 2);
    emulator.write16(STACK - 2, stop);
    emulator.run_until(stop, 2_000_000)?;
    Ok(emulator)
}

#[test]
fn memory_offsets_preserve_live_bases_and_wrap_at_sixteen_bits() -> Result<()> {
    let offsets = [
        0i32,
        1,
        -1,
        2,
        -2,
        30,
        -30,
        300,
        -200,
        32768,
        65535,
        65536,
        i32::MIN,
        i32::MAX,
    ];
    let directory = sangoku_test_support::tempfile::tempdir()?;
    let path = directory.path().join("offsets.clif");
    let mut source = "set opt_level=none\ntarget z80\n".to_owned();
    for (i, offset) in offsets.iter().enumerate() {
        source.push_str(&format!(
            "function %offset_load_{i}(i16) -> i16 system_v {{\nblock0(v0: i16):\n v1 = load.i16 v0{offset:+}\n return v1\n}}\nfunction %offset_store_{i}(i16) -> i16 system_v {{\nblock0(v0: i16):\n v1 = iconst.i16 0x6a39\n store v1, v0{offset:+}\n return v0\n}}\n"
        ));
    }
    std::fs::write(&path, source)?;
    let program = lower_program(path.to_str().unwrap())?;
    for (i, offset) in offsets.iter().enumerate() {
        for target in [0xc0ffu16, 0xc100] {
            let base = target.wrapping_sub(*offset as u16);
            for op in ["load", "store"] {
                let mut e = program.emulator(&format!("__rust_offset_{op}_{i}"))?;
                [e.b, e.c] = base.to_le_bytes();
                e.memory[target as usize] = 0xde;
                e.memory[target as usize + 1] = 0xbc;
                e.set_sp(STACK - 2);
                e.write16(STACK - 2, program.symbols["done"]);
                e.run_until(program.symbols["done"], 4096)?;
                let expected = if op == "load" { 0xbcde } else { base };
                ensure!(
                    u16::from_le_bytes([e.a, e.b]) == expected,
                    "{op} offset {offset}, base {base:x}"
                );
                if op == "store" {
                    ensure!(e.memory[target as usize..target as usize + 2] == [0x39, 0x6a]);
                }
                ensure!(e.sp() == STACK);
            }
        }
    }
    Ok(())
}

#[test]
fn memory_offsets_beyond_a_signed_byte() -> Result<()> {
    let program = program("load-store-offsets")?;

    let e = call(&program, "__rust_ld8_300", &[(BASE + 300, 0x9c)])?;
    assert_eq!(e.a, 0x9c);

    let e = call(
        &program,
        "__rust_ld16_n200",
        &[(BASE - 200, 0x34), (BASE - 199, 0x12)],
    )?;
    assert_eq!((e.a, e.b), (0x34, 0x12));

    let bytes = [0x11u8, 0x22, 0x33, 0x44];
    let memory: Vec<_> = bytes
        .iter()
        .enumerate()
        .map(|(i, b)| (BASE + 126 + i as u16, *b))
        .collect();
    let e = call(&program, "__rust_ld32_126", &memory)?;
    assert_eq!((e.a, e.b, e.c, e.d), (0x11, 0x22, 0x33, 0x44));

    let e = call(
        &program,
        "__rust_uld16_128",
        &[(BASE + 128, 0xef), (BASE + 129, 0xbe)],
    )?;
    assert_eq!((e.a, e.b, e.c, e.d), (0xef, 0xbe, 0, 0));

    let e = call(&program, "__rust_uld8_n5", &[(BASE - 5, 0xf0)])?;
    assert_eq!((e.a, e.b), (0xf0, 0));

    let e = call(&program, "__rust_st8_300", &[])?;
    assert_eq!(e.memory[(BASE + 300) as usize], 0x5a);

    let e = call(&program, "__rust_ist16_n5", &[(BASE - 3, 0xee)])?;
    assert_eq!(
        e.memory[(BASE - 5) as usize..(BASE - 2) as usize],
        [0x78, 0x56, 0xee]
    );

    let e = call(&program, "__rust_ist8_127", &[(BASE + 128, 0xee)])?;
    assert_eq!(
        e.memory[(BASE + 127) as usize..(BASE + 129) as usize],
        [0x66, 0xee]
    );
    Ok(())
}

fn run_with(
    program: &harness::Program,
    name: &str,
    regs: (u8, u8, u8),
    memory: &[(u16, u8)],
) -> Result<sangoku_test_support::emu::Emulator> {
    let stop = program.symbols["done"];
    let mut emulator = program.emulator(name)?;
    for (address, byte) in memory {
        emulator.memory[*address as usize] = *byte;
    }
    (emulator.a, emulator.b, emulator.c) = regs;
    emulator.set_sp(STACK - 2);
    emulator.write16(STACK - 2, stop);
    emulator.run_until(stop, 2_000_000)?;
    Ok(emulator)
}

#[test]
fn sign_extension_matches_host() -> Result<()> {
    let program = program("sign-extend")?;
    let wide = |e: &sangoku_test_support::emu::Emulator| {
        u32::from(e.a) | u32::from(e.b) << 8 | u32::from(e.c) << 16 | u32::from(e.d) << 24
    };
    for value in [0i8, 1, 0x7f, -1, -128, -2] {
        let byte = value as u8;
        let e = run_with(&program, "__rust_sx8_16", (0, byte, 0), &[])?;
        assert_eq!(
            u16::from(e.a) | u16::from(e.b) << 8,
            i16::from(value) as u16
        );
        let e = run_with(&program, "__rust_sx8_32", (0, byte, 0), &[])?;
        assert_eq!(wide(&e), i32::from(value) as u32);
    }
    for value in [0i16, 1, 0x7fff, -1, -32768, 0x00ff, -256] {
        let bits = value as u16;
        let e = run_with(
            &program,
            "__rust_sx16_32",
            (0, bits as u8, (bits >> 8) as u8),
            &[],
        )?;
        assert_eq!(wide(&e), i32::from(value) as u32);
    }
    let (b, c) = (BASE as u8, (BASE >> 8) as u8);
    for value in [0u8, 0x7f, 0x80, 0xff] {
        let e = run_with(&program, "__rust_sld8_16", (0, b, c), &[(BASE, value)])?;
        assert_eq!(
            u16::from(e.a) | u16::from(e.b) << 8,
            i16::from(value as i8) as u16
        );
        let e = run_with(
            &program,
            "__rust_sld8_32",
            (0, b, c),
            &[(BASE + 300, value)],
        )?;
        assert_eq!(wide(&e), i32::from(value as i8) as u32);
    }
    for value in [0i16, 0x7fff, -1, -32768] {
        let bits = (value as u16).to_le_bytes();
        let e = run_with(
            &program,
            "__rust_sld16_32",
            (0, b, c),
            &[(BASE, bits[0]), (BASE + 1, bits[1])],
        )?;
        assert_eq!(wide(&e), i32::from(value) as u32);
    }
    Ok(())
}

fn invoke(program: &harness::Program, name: &str, data: &[u8]) -> Result<u32> {
    let memory: Vec<_> = data
        .iter()
        .enumerate()
        .map(|(i, byte)| (BASE + i as u16, *byte))
        .collect();
    let e = call(program, name, &memory)?;
    Ok(u32::from(e.a) | u32::from(e.b) << 8 | u32::from(e.c) << 16 | u32::from(e.d) << 24)
}

fn operands<const N: usize>(a: u32, b: u32) -> Vec<u8> {
    let mut data = a.to_le_bytes()[..N].to_vec();
    data.resize(4, 0);
    data.extend_from_slice(&b.to_le_bytes()[..N]);
    data
}

#[test]
fn min_max_abs_match_host() -> Result<()> {
    let program = program("min-max-abs")?;
    macro_rules! check {
        ($ty:ty, $uty:ty, $bytes:literal, $name:literal) => {{
            let edges: [$ty; 7] = [
                0,
                1,
                -1,
                <$ty>::MAX,
                <$ty>::MIN,
                <$ty>::MAX - 1,
                <$ty>::MIN + 1,
            ];
            let mut state = 0x2545_F491u32;
            let mut sampled = edges.to_vec();
            for _ in 0..12 {
                state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                sampled.push((state >> 8) as $uty as $ty);
            }
            for a in sampled.iter().copied() {
                let read = |name: &str, data: &[u8]| -> Result<$ty> {
                    Ok(invoke(&program, &format!("__rust_{name}_{}", $name), data)? as $uty as $ty)
                };
                assert_eq!(
                    read("iabs", &(a as $uty).to_le_bytes())?,
                    a.wrapping_abs(),
                    "iabs {a}"
                );
                for b in sampled.iter().copied() {
                    let data = operands::<$bytes>(a as $uty as u32, b as $uty as u32);
                    assert_eq!(read("smin", &data)?, a.min(b), "smin {a} {b}");
                    assert_eq!(read("smax", &data)?, a.max(b), "smax {a} {b}");
                    assert_eq!(
                        read("umin", &data)? as $uty,
                        (a as $uty).min(b as $uty),
                        "umin {a} {b}"
                    );
                    assert_eq!(
                        read("umax", &data)? as $uty,
                        (a as $uty).max(b as $uty),
                        "umax {a} {b}"
                    );
                }
            }
        }};
    }
    check!(i8, u8, 1, "i8");
    check!(i16, u16, 2, "i16");
    check!(i32, u32, 4, "i32");

    for value in [0u8, 0x0f, 0xff] {
        assert_eq!(
            invoke(&program, "__rust_bnot_i8", &[value])? & 0xff,
            u32::from(!value)
        );
    }
    assert_eq!(invoke(&program, "__rust_select_i8", &[7, 9, 1])? & 0xff, 7);
    assert_eq!(invoke(&program, "__rust_select_i8", &[7, 9, 0])? & 0xff, 9);
    Ok(())
}

fn samples<T: Copy>(edges: &[T], convert: impl Fn(u32) -> T) -> Vec<T> {
    let mut state = 0x1357_9BDFu32;
    let mut values = edges.to_vec();
    for _ in 0..4 {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        values.push(convert(state >> 4));
    }
    values
}

#[test]
fn wide_arithmetic_and_bit_ops_match_host() -> Result<()> {
    let program = program("wide-ops")?;
    macro_rules! check {
        ($ty:ty, $uty:ty, $bytes:literal, $name:literal) => {{
            let values = samples(&[0 as $ty, 1, 2, 3, -1, <$ty>::MAX, <$ty>::MIN], |x| {
                x as $uty as $ty
            });
            let run = |op: &str, data: &[u8]| -> Result<$uty> {
                Ok(invoke(&program, &format!("__rust_{op}_{}", $name), data)? as $uty)
            };
            for a in values.iter().copied() {
                let data = (a as $uty).to_le_bytes();
                assert_eq!(run("popcnt", &data)?, a.count_ones() as $uty, "popcnt {a}");
                assert_eq!(run("clz", &data)?, a.leading_zeros() as $uty, "clz {a}");
                assert_eq!(run("ctz", &data)?, a.trailing_zeros() as $uty, "ctz {a}");
                assert_eq!(
                    run("bitrev", &data)?,
                    a.reverse_bits() as $uty,
                    "bitrev {a}"
                );
                for b in values.iter().copied() {
                    let data = operands::<$bytes>(a as $uty as u32, b as $uty as u32);
                    assert_eq!(
                        run("imul", &data)?,
                        a.wrapping_mul(b) as $uty,
                        "imul {a} {b}"
                    );
                    if b == 0 {
                        continue;
                    }
                    let (ua, ub) = (a as $uty, b as $uty);
                    if ub != 0 {
                        assert_eq!(run("udiv", &data)?, ua / ub, "udiv {a} {b}");
                        assert_eq!(run("urem", &data)?, ua % ub, "urem {a} {b}");
                    }
                    if !(a == <$ty>::MIN && b == -1) {
                        assert_eq!(run("sdiv", &data)?, (a / b) as $uty, "sdiv {a} {b}");
                    }
                    assert_eq!(
                        run("srem", &data)?,
                        a.wrapping_rem(b) as $uty,
                        "srem {a} {b}"
                    );
                }
            }
        }};
    }
    check!(i8, u8, 1, "i8");
    check!(i16, u16, 2, "i16");
    check!(i32, u32, 4, "i32");

    for value in [0u32, 0x1234_5678, 0xA1B2_C3D4, u32::MAX] {
        assert_eq!(
            invoke(&program, "__rust_bswap_i32", &value.to_le_bytes())?,
            value.swap_bytes()
        );
    }
    Ok(())
}

#[test]
fn many_wide_arguments_and_stack_returns_round_trip() -> Result<()> {
    let program = program("abi")?;
    for seed in [1u32, 0x0102_0304, 0xFFFF_FFFF] {
        let args: Vec<u32> = (0..5u32)
            .map(|i| seed.wrapping_mul(i + 3).wrapping_add(i * 0x1111_1111))
            .collect();
        let data: Vec<u8> = args.iter().flat_map(|arg| arg.to_le_bytes()).collect();

        let expected = args.iter().fold(0u32, |sum, arg| sum.wrapping_add(*arg));
        assert_eq!(invoke(&program, "__rust_call_sum5", &data)?, expected);

        let memory: Vec<_> = data
            .iter()
            .enumerate()
            .map(|(i, byte)| (BASE + i as u16, *byte))
            .collect();
        let e = call(&program, "__rust_call_quad", &memory)?;
        let results: Vec<u32> = (0..4)
            .map(|i| {
                let at = (BASE + 32 + 4 * i) as usize;
                u32::from_le_bytes(e.memory[at..at + 4].try_into().unwrap())
            })
            .collect();
        let [a, b, c, d, e5] = args[..] else {
            unreachable!()
        };
        assert_eq!(
            results,
            [
                a.wrapping_add(b),
                c.wrapping_add(d),
                e5 ^ a,
                b.wrapping_sub(e5)
            ]
        );
    }
    Ok(())
}

#[test]
fn f32_sign_ops_select_and_const_match_host() -> Result<()> {
    let program = program("float-bits")?;
    let values = [
        0.0f32,
        -0.0,
        1.5,
        -2.25,
        f32::INFINITY,
        f32::NAN,
        1e-40,
        f32::MAX,
    ];
    for a in values {
        let data = a.to_bits().to_le_bytes();
        assert_eq!(invoke(&program, "__rust_fneg", &data)?, (-a).to_bits());
        assert_eq!(invoke(&program, "__rust_fabs", &data)?, a.abs().to_bits());
        for b in [1.0f32, -1.0] {
            let mut data = data.to_vec();
            data.extend(b.to_bits().to_le_bytes());
            assert_eq!(
                invoke(&program, "__rust_fcopysign", &data)?,
                a.copysign(b).to_bits()
            );
        }
        for flag in [0u8, 1] {
            let mut data = data.to_vec();
            data.extend([flag, 0, 0, 0]);
            let e = call(
                &program,
                "__rust_copy_select",
                &data
                    .iter()
                    .enumerate()
                    .map(|(i, b)| (BASE + i as u16, *b))
                    .collect::<Vec<_>>(),
            )?;
            let at = BASE as usize + 8;
            let got = u32::from_le_bytes(e.memory[at..at + 4].try_into().unwrap());
            assert_eq!(
                got,
                if flag == 1 {
                    a.to_bits()
                } else {
                    3.0f32.to_bits()
                }
            );
        }
    }
    Ok(())
}

fn float_samples() -> Vec<f32> {
    let mut values = vec![
        0.0f32,
        -0.0,
        1.0,
        -1.0,
        1.5,
        0.1,
        -3.75,
        16777216.0,
        f32::MAX,
        f32::MIN_POSITIVE,
        f32::from_bits(1),
        f32::from_bits(0x0040_0000),
        1e-40,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
    ];
    let mut state = 0x1234_5678u32;
    for _ in 0..6 {
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let bits = state;
        if f32::from_bits(bits).is_finite() {
            values.push(f32::from_bits(bits));
        }
    }
    values
}

fn same_float(got: u32, expected: f32, context: &str) {
    if expected.is_nan() {
        assert!(f32::from_bits(got).is_nan(), "{context}: got {got:08x}");
    } else {
        assert_eq!(got, expected.to_bits(), "{context}");
    }
}

fn host_min(a: f32, b: f32) -> f32 {
    if a.is_nan() || b.is_nan() {
        f32::NAN
    } else if a == b {
        if a.is_sign_negative() { a } else { b }
    } else if a < b {
        a
    } else {
        b
    }
}

fn host_max(a: f32, b: f32) -> f32 {
    if a.is_nan() || b.is_nan() {
        f32::NAN
    } else if a == b {
        if a.is_sign_negative() { b } else { a }
    } else if a > b {
        a
    } else {
        b
    }
}

#[test]
fn f32_arithmetic_and_compares_match_host() -> Result<()> {
    let program = program("float-arith")?;
    let values = float_samples();
    for a in values.iter().copied() {
        for b in values.iter().copied() {
            let mut data = a.to_bits().to_le_bytes().to_vec();
            data.extend(b.to_bits().to_le_bytes());
            let run = |name: &str| invoke(&program, &format!("__rust_{name}"), &data);
            let context = |op: &str| format!("{op} {:08x} {:08x}", a.to_bits(), b.to_bits());
            same_float(run("fadd")?, a + b, &context("fadd"));
            same_float(run("fsub")?, a - b, &context("fsub"));
            same_float(run("fmul")?, a * b, &context("fmul"));
            same_float(run("fmin")?, host_min(a, b), &context("fmin"));
            same_float(run("fmax")?, host_max(a, b), &context("fmax"));
            let unordered = a.is_nan() || b.is_nan();
            let checks: [(&str, bool); 14] = [
                ("eq", a == b),
                ("ne", a != b),
                ("one", !unordered && a != b),
                ("ueq", unordered || a == b),
                ("lt", a < b),
                ("le", a <= b),
                ("gt", a > b),
                ("ge", a >= b),
                ("ult", unordered || a < b),
                ("ule", unordered || a <= b),
                ("ugt", unordered || a > b),
                ("uge", unordered || a >= b),
                ("ord", !unordered),
                ("uno", unordered),
            ];
            for (cond, expected) in checks {
                let got = run(&format!("fcmp_{cond}"))? & 0xFF;
                assert_eq!(
                    got,
                    u32::from(expected),
                    "{}",
                    context(&format!("fcmp {cond}"))
                );
            }
        }
    }
    Ok(())
}

#[test]
fn f32_rounding_matches_host_on_random_operands() -> Result<()> {
    let program = program("float-arith")?;
    let mut state = 0x9E37_79B9u32;
    let mut next = || {
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        state >> 8
    };
    for index in 0..600 {
        let (ea, eb) = match index % 3 {
            0 => (100 + next() % 50, 100 + next() % 50),
            1 => (1 + next() % 40, 1 + next() % 40),
            _ => (60 + next() % 40, 100 + next() % 60),
        };
        let a = f32::from_bits((next() & 1) << 31 | ea << 23 | next() & 0x7F_FFFF);
        let b = f32::from_bits((next() & 1) << 31 | eb << 23 | next() & 0x7F_FFFF);
        let mut data = a.to_bits().to_le_bytes().to_vec();
        data.extend(b.to_bits().to_le_bytes());
        for (name, expected) in [("fadd", a + b), ("fsub", a - b), ("fmul", a * b)] {
            let got = invoke(&program, &format!("__rust_{name}"), &data)?;
            same_float(
                got,
                expected,
                &format!("{name} {:08x} {:08x}", a.to_bits(), b.to_bits()),
            );
        }
    }
    Ok(())
}

#[test]
fn f32_division_matches_host() -> Result<()> {
    let program = program("float-arith")?;
    let values = float_samples();
    let mut state = 0x2545_F491u32;
    let mut pairs: Vec<(f32, f32)> = values
        .iter()
        .flat_map(|a| values.iter().map(move |b| (*a, *b)))
        .collect();
    for index in 0..300 {
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let (ea, eb) = match index % 3 {
            0 => (100 + (state >> 8) % 50, 100 + (state >> 12) % 50),
            1 => (1 + (state >> 8) % 40, 100 + (state >> 12) % 40),
            _ => (200 + (state >> 8) % 50, 1 + (state >> 12) % 30),
        };
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let a = f32::from_bits((state & 1) << 31 | ea << 23 | (state >> 8) & 0x7F_FFFF);
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let b = f32::from_bits((state & 1) << 31 | eb << 23 | (state >> 8) & 0x7F_FFFF);
        pairs.push((a, b));
    }
    for (a, b) in pairs {
        let mut data = a.to_bits().to_le_bytes().to_vec();
        data.extend(b.to_bits().to_le_bytes());
        let got = invoke(&program, "__rust_fdiv", &data)?;
        same_float(
            got,
            a / b,
            &format!("fdiv {:08x} {:08x}", a.to_bits(), b.to_bits()),
        );
    }
    Ok(())
}

fn unary_samples() -> Vec<f32> {
    let mut values = float_samples();
    values.extend([
        0.5,
        -0.5,
        0.49999997,
        0.50000006,
        1.5,
        2.5,
        -2.5,
        3.5,
        0.75,
        -0.75,
        8388607.5,
        8388608.0,
        8388609.0,
        -8388607.5,
        123_456.79,
        -1234.5678,
        2.0,
        4.0,
        9.0,
        2.0000002,
        1.0000001,
        0.9999999,
        f32::from_bits(0x0000_0400),
        f32::from_bits(0x007F_FFFF),
        1e30,
        1e-30,
    ]);
    values
}

#[test]
fn f32_sqrt_matches_host() -> Result<()> {
    let program = program("float-arith")?;
    let mut values = unary_samples();
    let mut state = 0x6C07_8965u32;
    for _ in 0..200 {
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let exponent = 1 + (state >> 8) % 253;
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        values.push(f32::from_bits(exponent << 23 | (state >> 8) & 0x7F_FFFF));
    }
    for a in values {
        let got = invoke(&program, "__rust_fsqrt", &a.to_bits().to_le_bytes())?;
        same_float(got, a.sqrt(), &format!("fsqrt {:08x}", a.to_bits()));
    }
    Ok(())
}

#[test]
fn f32_rounding_ops_match_host() -> Result<()> {
    let program = program("float-arith")?;
    let mut values = unary_samples();
    let mut state = 0x0BAD_CAFEu32;
    for _ in 0..200 {
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let exponent = 110 + (state >> 8) % 50;
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        values.push(f32::from_bits(
            (state & 1) << 31 | exponent << 23 | (state >> 8) & 0x7F_FFFF,
        ));
    }
    for a in values {
        let data = a.to_bits().to_le_bytes();
        for (name, expected) in [
            ("ftrunc", a.trunc()),
            ("ffloor", a.floor()),
            ("fceil", a.ceil()),
            ("fnearest", a.round_ties_even()),
        ] {
            let got = invoke(&program, &format!("__rust_{name}"), &data)?;
            same_float(got, expected, &format!("{name} {:08x}", a.to_bits()));
        }
    }
    Ok(())
}

fn int_float_samples() -> Vec<f32> {
    let mut values = unary_samples();
    values.extend([
        127.9,
        -128.9,
        128.0,
        -129.0,
        255.9,
        256.0,
        -0.9,
        32767.9,
        -32768.5,
        32768.0,
        65535.9,
        65536.0,
        2147483520.0,
        2147483648.0,
        -2147483648.0,
        -2147483904.0,
        4294967040.0,
        4294967296.0,
        -1.0,
        0.99999994,
    ]);
    values
}

fn int_samples() -> Vec<i32> {
    vec![
        0,
        1,
        -1,
        2,
        127,
        -128,
        255,
        256,
        32767,
        -32768,
        65535,
        65536,
        16777216,
        16777217,
        16777219,
        0x7FFF_FFFF,
        i32::MIN,
        0x7FFF_FF80,
        0x7FFF_FFC0,
        123456789,
        -123456789,
        0x0100_0001,
    ]
}

#[test]
fn int_to_f32_matches_host() -> Result<()> {
    let program = program("float-convert")?;
    for v in int_samples() {
        let bytes = v.to_le_bytes();
        let cases: [(&str, f32); 6] = [
            ("from_sint_i8", f32::from(v as i8)),
            ("from_uint_i8", f32::from(v as u8)),
            ("from_sint_i16", f32::from(v as i16)),
            ("from_uint_i16", f32::from(v as u16)),
            ("from_sint_i32", v as f32),
            ("from_uint_i32", (v as u32) as f32),
        ];
        for (name, expected) in cases {
            let got = invoke(&program, &format!("__rust_{name}"), &bytes)?;
            same_float(got, expected, &format!("{name} {v:#x}"));
        }
    }
    Ok(())
}

#[test]
fn f32_to_int_matches_host() -> Result<()> {
    let program = program("float-convert")?;
    for a in int_float_samples() {
        let data = a.to_bits().to_le_bytes();
        let run = |name: &str| invoke(&program, &format!("__rust_{name}"), &data);
        let ctx = |name: &str| format!("{name} {:08x}", a.to_bits());
        let t = a.trunc();
        assert_eq!(
            run("to_sint_sat_i8")? & 0xFF,
            u32::from(a as i8 as u8),
            "{}",
            ctx("sat s8")
        );
        assert_eq!(
            run("to_uint_sat_i8")? & 0xFF,
            u32::from(a as u8),
            "{}",
            ctx("sat u8")
        );
        assert_eq!(
            run("to_sint_sat_i16")? & 0xFFFF,
            u32::from(a as i16 as u16),
            "{}",
            ctx("sat s16")
        );
        assert_eq!(
            run("to_uint_sat_i16")? & 0xFFFF,
            u32::from(a as u16),
            "{}",
            ctx("sat u16")
        );
        assert_eq!(
            run("to_sint_sat_i32")?,
            a as i32 as u32,
            "{}",
            ctx("sat s32")
        );
        assert_eq!(run("to_uint_sat_i32")?, a as u32, "{}", ctx("sat u32"));
        if a.is_nan() {
            continue;
        }
        if (-129.0..128.0).contains(&t) && t > -129.0 {
            assert_eq!(
                run("to_sint_i8")? & 0xFF,
                u32::from(a as i8 as u8),
                "{}",
                ctx("s8")
            );
        }
        if t > -1.0 && t < 256.0 {
            assert_eq!(
                run("to_uint_i8")? & 0xFF,
                u32::from(a as u8),
                "{}",
                ctx("u8")
            );
        }
        if (-32768.0..32768.0).contains(&t) {
            assert_eq!(
                run("to_sint_i16")? & 0xFFFF,
                u32::from(a as i16 as u16),
                "{}",
                ctx("s16")
            );
        }
        if t > -1.0 && t < 65536.0 {
            assert_eq!(
                run("to_uint_i16")? & 0xFFFF,
                u32::from(a as u16),
                "{}",
                ctx("u16")
            );
        }
        if (-2147483648.0..2147483648.0).contains(&t) {
            assert_eq!(run("to_sint_i32")?, a as i32 as u32, "{}", ctx("s32"));
        }
        if t > -1.0 && t < 4294967296.0 {
            assert_eq!(run("to_uint_i32")?, a as u32, "{}", ctx("u32"));
        }
    }
    Ok(())
}

#[test]
fn f32_to_int_traps_out_of_range() -> Result<()> {
    let program = program("float-convert")?;
    let cases: [(&str, f32); 8] = [
        ("to_sint_i8", 128.0),
        ("to_sint_i8", f32::NAN),
        ("to_uint_i8", -1.0),
        ("to_sint_i16", -32769.0),
        ("to_uint_i16", 65536.0),
        ("to_sint_i32", 2147483648.0),
        ("to_uint_i32", f32::INFINITY),
        ("to_uint_i32", -2.0),
    ];
    for (name, value) in cases {
        let mut emulator = program.emulator(&format!("__rust_{name}"))?;
        for (i, byte) in value.to_bits().to_le_bytes().iter().enumerate() {
            emulator.memory[BASE as usize + i] = *byte;
        }
        emulator.b = BASE as u8;
        emulator.c = (BASE >> 8) as u8;
        emulator.set_sp(STACK - 2);
        emulator.write16(STACK - 2, program.symbols["done"]);
        emulator.run_until(0x0010, 2_000_000)?;
    }
    Ok(())
}

fn f16_to_f32_ref(bits: u16) -> f32 {
    let sign = if bits & 0x8000 != 0 { -1.0f32 } else { 1.0 };
    let exponent = i32::from((bits >> 10) & 0x1F);
    let mantissa = f32::from(bits & 0x3FF);
    match exponent {
        0 => sign * mantissa * 2f32.powi(-24),
        31 if mantissa == 0.0 => sign * f32::INFINITY,
        31 => f32::NAN,
        _ => sign * (1024.0 + mantissa) * 2f32.powi(exponent - 25),
    }
}

fn f32_to_f16_ref(value: f32) -> u16 {
    f64_to_f16_ref(f64::from(value))
}

fn f64_to_f16_ref(value: f64) -> u16 {
    if value.is_nan() {
        return 0x7E00;
    }
    let sign = if value.is_sign_negative() { 0x8000 } else { 0 };
    let magnitude = value.abs();
    if magnitude >= 65520.0 {
        return sign | 0x7C00;
    }
    let at = |bits: u16| f64::from(f16_to_f32_ref(bits));
    let (mut lo, mut hi) = (0u16, 0x7BFFu16);
    while lo < hi {
        let mid = (lo + hi).div_ceil(2);
        if at(mid) <= magnitude {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    let below = at(lo);
    if below == magnitude || lo == 0x7BFF {
        return sign | lo;
    }
    let above = at(lo + 1);
    let pick_up = match (magnitude - below).partial_cmp(&(above - magnitude)) {
        Some(std::cmp::Ordering::Less) => false,
        Some(std::cmp::Ordering::Greater) => true,
        _ => lo & 1 == 1,
    };
    sign | if pick_up { lo + 1 } else { lo }
}

fn same_half(got: u32, expected: u16, context: &str) {
    let got = got as u16;
    let is_nan = |bits: u16| bits & 0x7C00 == 0x7C00 && bits & 0x3FF != 0;
    if is_nan(expected) {
        assert!(is_nan(got), "{context}: got {got:04x}");
    } else {
        assert_eq!(got, expected, "{context}");
    }
}

fn half_samples() -> Vec<u16> {
    let mut values = vec![
        0x0000, 0x8000, 0x0001, 0x8001, 0x03FF, 0x0400, 0x3555, 0x3C00, 0xBC00, 0x3E00, 0x4000,
        0x4248, 0x5640, 0x7BFF, 0xFBFF, 0x7C00, 0xFC00, 0x7E00, 0x7C01, 0x1234, 0xA5A5,
    ];
    let mut state = 0xC0FF_EE11u32;
    for _ in 0..6 {
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        values.push((state >> 12) as u16);
    }
    values
}

fn half_operands(a: u16, b: u16) -> Vec<u8> {
    let mut data = a.to_le_bytes().to_vec();
    data.resize(4, 0);
    data.extend(b.to_le_bytes());
    data.resize(8, 0);
    data
}

#[test]
fn f16_promote_and_demote_match_reference() -> Result<()> {
    let program = program("float-convert")?;
    for bits in half_samples() {
        let data = half_operands(bits, 0);
        let got = invoke(&program, "__rust_h_promote", &data[..4])?;
        same_float(got, f16_to_f32_ref(bits), &format!("promote {bits:04x}"));
    }
    let mut values = float_samples();
    values.extend([
        65519.99,
        65520.0,
        65504.0,
        6.1035156e-5,
        6.1e-5,
        5.9604645e-8,
        2.9802322e-8,
        2.9802325e-8,
        8.940697e-8,
        1.0009766,
        1.0004883,
        1.0004884,
        2049.0,
        2050.0,
        2051.0,
        -65520.0,
        6.097_555e-5,
        6.1005354e-5,
    ]);
    let mut state = 0x5151_7777u32;
    for _ in 0..300 {
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let exponent = 96 + (state >> 8) % 55;
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        values.push(f32::from_bits(
            (state & 1) << 31 | exponent << 23 | (state >> 8) & 0x7F_FFFF,
        ));
    }
    for v in values {
        let got = invoke(&program, "__rust_h_demote", &v.to_bits().to_le_bytes())?;
        same_half(
            got,
            f32_to_f16_ref(v),
            &format!("demote {:08x}", v.to_bits()),
        );
    }
    Ok(())
}

#[test]
fn f16_operations_match_reference() -> Result<()> {
    let program = program("float-convert")?;
    let values = half_samples();
    let demote = f32_to_f16_ref;
    for a in values.iter().copied() {
        let fa = f16_to_f32_ref(a);
        for b in values.iter().copied() {
            let fb = f16_to_f32_ref(b);
            let data = half_operands(a, b);
            let run = |name: &str| invoke(&program, &format!("__rust_h_{name}"), &data);
            let ctx = |op: &str| format!("{op} {a:04x} {b:04x}");
            same_half(run("fadd")?, demote(fa + fb), &ctx("fadd"));
            same_half(run("fsub")?, demote(fa - fb), &ctx("fsub"));
            same_half(run("fmul")?, demote(fa * fb), &ctx("fmul"));
            same_half(run("fdiv")?, demote(fa / fb), &ctx("fdiv"));
            same_half(run("fmin")?, demote(host_min(fa, fb)), &ctx("fmin"));
            same_half(run("fmax")?, demote(host_max(fa, fb)), &ctx("fmax"));
            let unordered = fa.is_nan() || fb.is_nan();
            let checks: [(&str, bool); 7] = [
                ("eq", fa == fb),
                ("ne", fa != fb),
                ("lt", fa < fb),
                ("le", fa <= fb),
                ("gt", fa > fb),
                ("ge", fa >= fb),
                ("uno", unordered),
            ];
            for (cond, expected) in checks {
                let got = run(&format!("fcmp_{cond}"))? & 0xFF;
                assert_eq!(got, u32::from(expected), "{}", ctx(cond));
            }
        }
        let data = half_operands(a, 0);
        let run = |name: &str| invoke(&program, &format!("__rust_h_{name}"), &data[..4]);
        let ctx = |op: &str| format!("{op} {a:04x}");
        same_half(run("sqrt")?, demote(fa.sqrt()), &ctx("sqrt"));
        same_half(run("trunc")?, demote(fa.trunc()), &ctx("trunc"));
        same_half(run("floor")?, demote(fa.floor()), &ctx("floor"));
        same_half(run("ceil")?, demote(fa.ceil()), &ctx("ceil"));
        same_half(
            run("nearest")?,
            demote(fa.round_ties_even()),
            &ctx("nearest"),
        );
        assert_eq!(
            run("to_sint_sat_i8")? & 0xFF,
            u32::from(fa as i8 as u8),
            "{}",
            ctx("s8")
        );
        assert_eq!(
            run("to_sint_sat_i16")? & 0xFFFF,
            u32::from(fa as i16 as u16),
            "{}",
            ctx("s16")
        );
        assert_eq!(run("to_sint_sat_i32")?, fa as i32 as u32, "{}", ctx("s32"));
    }
    for v in int_samples() {
        let bytes = v.to_le_bytes();
        let run = |name: &str| invoke(&program, &format!("__rust_h_{name}"), &bytes);
        same_half(
            run("from_sint_i8")?,
            demote(f32::from(v as i8)),
            &format!("i8 {v:#x}"),
        );
        same_half(
            run("from_sint_i16")?,
            demote(f32::from(v as i16)),
            &format!("i16 {v:#x}"),
        );
        same_half(
            run("from_sint_i32")?,
            demote(v as f32),
            &format!("i32 {v:#x}"),
        );
    }
    Ok(())
}

#[test]
fn f32_fma_matches_host() -> Result<()> {
    let program = program("float-arith")?;
    let mut triples: Vec<(f32, f32, f32)> = Vec::new();
    let edges = [
        0.0f32,
        -0.0,
        1.0,
        -1.0,
        1.5,
        3.0,
        f32::MAX,
        f32::MIN_POSITIVE,
        f32::from_bits(1),
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
        16777217.0,
        0.1,
    ];
    for a in edges {
        for b in edges {
            for c in [
                0.0f32,
                -0.0,
                1.0,
                -1.0,
                f32::INFINITY,
                f32::NAN,
                1e-30,
                f32::MAX,
            ] {
                triples.push((a, b, c));
            }
        }
    }
    let mut state = 0xD1B5_4A33u32;
    let mut next = || {
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        state >> 8
    };
    for index in 0..500 {
        let ea = 90 + next() % 80;
        let eb = 90 + next() % 80;
        let ec = match index % 4 {
            0 => (ea + eb).saturating_sub(127),
            1 => (ea + eb).saturating_sub(127) + 1 + next() % 3,
            2 => (ea + eb).saturating_sub(127) + next() % 40,
            _ => 1 + next() % 250,
        }
        .clamp(1, 254);
        let mut make = |e: u32| f32::from_bits((next() & 1) << 31 | e << 23 | next() & 0x7F_FFFF);
        triples.push((make(ea), make(eb), make(ec)));
    }
    for (a, b, c) in triples {
        let mut data = a.to_bits().to_le_bytes().to_vec();
        data.extend(b.to_bits().to_le_bytes());
        data.extend(c.to_bits().to_le_bytes());
        let got = invoke(&program, "__rust_ffma", &data)?;
        same_float(
            got,
            a.mul_add(b, c),
            &format!(
                "fma {:08x} {:08x} {:08x}",
                a.to_bits(),
                b.to_bits(),
                c.to_bits()
            ),
        );
    }
    Ok(())
}

#[test]
fn f16_fma_matches_reference() -> Result<()> {
    let program = program("float-convert")?;
    let values = half_samples();
    for a in values.iter().copied() {
        for b in values.iter().copied() {
            for c in values.iter().copied() {
                let mut data = half_operands(a, b);
                data.extend(c.to_le_bytes());
                data.resize(12, 0);
                let got = invoke(&program, "__rust_h_ffma", &data)?;
                let expected = f64::from(f16_to_f32_ref(a))
                    .mul_add(f64::from(f16_to_f32_ref(b)), f64::from(f16_to_f32_ref(c)));
                same_half(
                    got,
                    f64_to_f16_ref(expected),
                    &format!("h_fma {a:04x} {b:04x} {c:04x}"),
                );
            }
        }
    }
    Ok(())
}

#[test]
fn division_traps_reach_the_rst_vector() -> Result<()> {
    let program = program("wide-ops")?;
    let cases: [(&str, Vec<u8>); 5] = [
        ("udiv_i32", operands::<4>(5, 0)),
        ("srem_i32", operands::<4>(5, 0)),
        ("sdiv_i32", operands::<4>(0x8000_0000, 0xFFFF_FFFF)),
        ("udiv_i16", operands::<2>(5, 0)),
        ("sdiv_i8", operands::<1>(5, 0)),
    ];
    for (name, data) in cases {
        let mut emulator = program.emulator(&format!("__rust_{name}"))?;
        for (i, byte) in data.iter().enumerate() {
            emulator.memory[BASE as usize + i] = *byte;
        }
        emulator.b = BASE as u8;
        emulator.c = (BASE >> 8) as u8;
        emulator.set_sp(STACK - 2);
        emulator.write16(STACK - 2, program.symbols["done"]);
        emulator.run_until(0x0010, 2_000_000)?;
    }
    Ok(())
}

#[test]
fn i8_constant_shifts_and_rotates_match_host() -> Result<()> {
    let program = program("shift-i8")?;
    let values = [0u8, 1, 0x80, 0xff, 0x5a, 0xa5, 0x7f];
    for value in values {
        for n in [0u32, 1, 3, 7] {
            let run = |op: &str| -> Result<u8> {
                Ok(invoke(&program, &format!("__rust_{op}{n}_i8"), &[value])? as u8)
            };
            assert_eq!(run("ishl")?, value << n, "ishl {value:#x} {n}");
            assert_eq!(run("ushr")?, value >> n, "ushr {value:#x} {n}");
            assert_eq!(
                run("sshr")?,
                ((value as i8) >> n) as u8,
                "sshr {value:#x} {n}"
            );
            assert_eq!(run("rotl")?, value.rotate_left(n), "rotl {value:#x} {n}");
            assert_eq!(run("rotr")?, value.rotate_right(n), "rotr {value:#x} {n}");
        }
    }
    for value in values {
        for n in [0u8, 1, 3, 7, 8, 9, 255] {
            let run = |op: &str| -> Result<u8> {
                Ok(invoke(&program, &format!("__rust_{op}var_i8"), &[value, n])? as u8)
            };
            let m = u32::from(n & 7);
            assert_eq!(run("ishl")?, value << m, "ishl var {value:#x} {n}");
            assert_eq!(run("ushr")?, value >> m, "ushr var {value:#x} {n}");
            assert_eq!(
                run("sshr")?,
                ((value as i8) >> m) as u8,
                "sshr var {value:#x} {n}"
            );
            assert_eq!(
                run("rotl")?,
                value.rotate_left(m),
                "rotl var {value:#x} {n}"
            );
            assert_eq!(
                run("rotr")?,
                value.rotate_right(m),
                "rotr var {value:#x} {n}"
            );
        }
    }
    Ok(())
}

#[test]
fn i8_overflow_ops_match_host() -> Result<()> {
    let program = program("overflow-i8")?;
    let edges = [0u8, 1, 2, 0x0f, 0x10, 0x7e, 0x7f, 0x80, 0x81, 0xfe, 0xff];
    let run = |op: &str, a: u8, b: u8| -> Result<(u8, bool)> {
        let r = invoke(&program, &format!("__rust_{op}_overflow_i8"), &[a, b])?;
        Ok((r as u8, (r >> 8) as u8 != 0))
    };
    for a in edges {
        for b in edges {
            let (sa, sb) = (a as i8, b as i8);
            assert_eq!(run("uadd", a, b)?, a.overflowing_add(b), "uadd {a} {b}");
            assert_eq!(run("usub", a, b)?, a.overflowing_sub(b), "usub {a} {b}");
            assert_eq!(run("umul", a, b)?, a.overflowing_mul(b), "umul {a} {b}");
            let signed = |(v, o): (i8, bool)| (v as u8, o);
            assert_eq!(
                run("sadd", a, b)?,
                signed(sa.overflowing_add(sb)),
                "sadd {sa} {sb}"
            );
            assert_eq!(
                run("ssub", a, b)?,
                signed(sa.overflowing_sub(sb)),
                "ssub {sa} {sb}"
            );
            assert_eq!(
                run("smul", a, b)?,
                signed(sa.overflowing_mul(sb)),
                "smul {sa} {sb}"
            );
        }
    }
    Ok(())
}

#[test]
fn i8_cls_and_mulhi_match_host() -> Result<()> {
    let program = program("trap-cls-mulhi-i8")?;
    let edges = [
        0u8, 1, 2, 0x0f, 0x3f, 0x40, 0x7f, 0x80, 0x81, 0xc0, 0xfe, 0xff,
    ];
    for a in edges {
        let got = invoke(&program, "__rust_cls_i8", &[a])? as u8;
        let expected = ((a ^ ((a as i8) >> 1) as u8).leading_zeros() - 1) as u8;
        assert_eq!(got, expected, "cls {a:#x}");
        for b in edges {
            let unsigned = ((u16::from(a) * u16::from(b)) >> 8) as u8;
            let signed = ((i16::from(a as i8) * i16::from(b as i8)) >> 8) as u8;
            let run = |op: &str| -> Result<u8> {
                Ok(invoke(&program, &format!("__rust_{op}_i8"), &[a, b])? as u8)
            };
            assert_eq!(run("umulhi")?, unsigned, "umulhi {a:#x} {b:#x}");
            assert_eq!(run("smulhi")?, signed, "smulhi {a:#x} {b:#x}");
        }
    }
    Ok(())
}

#[test]
fn i8_trapnz_and_trapz_trap_only_on_their_condition() -> Result<()> {
    let program = program("trap-cls-mulhi-i8")?;
    for (name, value, traps) in [
        ("trapnz_i8", 0u8, false),
        ("trapnz_i8", 1, true),
        ("trapnz_i8", 0x80, true),
        ("trapz_i8", 0, true),
        ("trapz_i8", 1, false),
        ("trapz_i8", 0x80, false),
    ] {
        let mut emulator = program.emulator(&format!("__rust_{name}"))?;
        emulator.memory[BASE as usize] = value;
        emulator.b = BASE as u8;
        emulator.c = (BASE >> 8) as u8;
        emulator.set_sp(STACK - 2);
        let done = program.symbols["done"];
        emulator.write16(STACK - 2, done);
        let target = if traps { 0x0010 } else { done };
        emulator.run_until(target, 2_000_000)?;
        if !traps {
            assert_eq!(emulator.a, value, "{name} {value:#x} result");
        }
    }
    Ok(())
}

const EDGES16: [u16; 12] = [
    0, 1, 2, 0xff, 0x100, 0x1234, 0x7fff, 0x8000, 0x8001, 0xa5c3, 0xfeff, 0xffff,
];

#[test]
fn i16_shifts_and_rotates_match_host() -> Result<()> {
    let program = program("ops-i16")?;
    for value in EDGES16 {
        let bytes = value.to_le_bytes();
        for n in [0u32, 1, 3, 8, 9, 15] {
            let run = |op: &str| -> Result<u16> {
                Ok(invoke(&program, &format!("__rust_{op}{n}_i16"), &bytes)? as u16)
            };
            assert_eq!(run("ishl")?, value << n, "ishl {value:#x} {n}");
            assert_eq!(run("ushr")?, value >> n, "ushr {value:#x} {n}");
            assert_eq!(
                run("sshr")?,
                ((value as i16) >> n) as u16,
                "sshr {value:#x} {n}"
            );
            assert_eq!(run("rotl")?, value.rotate_left(n), "rotl {value:#x} {n}");
            assert_eq!(run("rotr")?, value.rotate_right(n), "rotr {value:#x} {n}");
        }
        for n in [0u8, 1, 7, 8, 15, 16, 17, 255] {
            let m = u32::from(n & 15);
            let data = [bytes[0], bytes[1], n];
            let run = |op: &str| -> Result<u16> {
                Ok(invoke(&program, &format!("__rust_{op}var_i16"), &data)? as u16)
            };
            assert_eq!(run("ishl")?, value << m, "ishl var {value:#x} {n}");
            assert_eq!(run("ushr")?, value >> m, "ushr var {value:#x} {n}");
            assert_eq!(
                run("sshr")?,
                ((value as i16) >> m) as u16,
                "sshr var {value:#x} {n}"
            );
            assert_eq!(
                run("rotl")?,
                value.rotate_left(m),
                "rotl var {value:#x} {n}"
            );
            assert_eq!(
                run("rotr")?,
                value.rotate_right(m),
                "rotr var {value:#x} {n}"
            );
        }
    }
    Ok(())
}

#[test]
fn i16_cls_mulhi_and_overflow_match_host() -> Result<()> {
    let program = program("ops-i16")?;
    for a in EDGES16 {
        let got = invoke(&program, "__rust_cls_i16", &a.to_le_bytes())? as u16;
        let expected = (a ^ ((a as i16) >> 1) as u16).leading_zeros() as u16 - 1;
        assert_eq!(got, expected, "cls {a:#x}");
        for b in EDGES16 {
            let data = operands::<2>(u32::from(a), u32::from(b));
            let data = [&data[..2], &data[4..6]].concat();
            let run = |op: &str| -> Result<(u16, bool)> {
                let r = invoke(&program, &format!("__rust_{op}_i16"), &data)?;
                Ok((r as u16, (r >> 16) as u8 != 0))
            };
            let mulhi = |op: &str| -> Result<u16> {
                Ok(invoke(&program, &format!("__rust_{op}_i16"), &data)? as u16)
            };
            assert_eq!(
                mulhi("umulhi")?,
                ((u32::from(a) * u32::from(b)) >> 16) as u16,
                "umulhi {a:#x} {b:#x}"
            );
            assert_eq!(
                mulhi("smulhi")?,
                ((i32::from(a as i16) * i32::from(b as i16)) >> 16) as u16,
                "smulhi {a:#x} {b:#x}"
            );
            let (sa, sb) = (a as i16, b as i16);
            let signed = |(v, o): (i16, bool)| (v as u16, o);
            assert_eq!(
                run("uadd_overflow")?,
                a.overflowing_add(b),
                "uadd {a:#x} {b:#x}"
            );
            assert_eq!(
                run("usub_overflow")?,
                a.overflowing_sub(b),
                "usub {a:#x} {b:#x}"
            );
            assert_eq!(
                run("umul_overflow")?,
                a.overflowing_mul(b),
                "umul {a:#x} {b:#x}"
            );
            assert_eq!(
                run("sadd_overflow")?,
                signed(sa.overflowing_add(sb)),
                "sadd {a:#x} {b:#x}"
            );
            assert_eq!(
                run("ssub_overflow")?,
                signed(sa.overflowing_sub(sb)),
                "ssub {a:#x} {b:#x}"
            );
            assert_eq!(
                run("smul_overflow")?,
                signed(sa.overflowing_mul(sb)),
                "smul {a:#x} {b:#x}"
            );
        }
    }
    Ok(())
}

#[test]
fn split_i64_values_match_host() -> Result<()> {
    for level in ["none", "speed"] {
        check_split_i64(level)?;
    }
    Ok(())
}

fn check_split_i64(level: &str) -> Result<()> {
    let source = include_str!("fixtures/emulate/split-i64.clif");
    let functions: Vec<_> = source.split("\nfunction ").skip(1).collect();
    let mut programs = std::collections::BTreeMap::new();
    for function in &functions {
        let name = function
            .strip_prefix('%')
            .unwrap()
            .split('(')
            .next()
            .unwrap();
        if name == "sum64" {
            continue;
        }
        let mut text = format!(
            "set enable_multi_ret_implicit_sret=true\nset opt_level={level}\ntarget z80\nfunction {function}"
        );
        if name.starts_with("call_") {
            let callee = functions.iter().find(|f| f.starts_with("%sum64(")).unwrap();
            text.push_str(&format!("\nfunction {callee}"));
        }
        let directory = sangoku_test_support::tempfile::tempdir()?;
        let path = directory.path().join(format!("{level}-{name}.clif"));
        std::fs::write(&path, text)?;
        programs.insert(name.to_owned(), lower_program(path.to_str().unwrap())?);
    }
    let inputs = [
        0,
        1,
        0xffff_ffff,
        0x1_0000_0000,
        1u64 << 63,
        u64::MAX,
        0x0123_4567_89ab_cdef,
    ];
    let run = |name: &str, a: u64, b: u64| -> Result<_> {
        let bytes: Vec<_> = a
            .to_le_bytes()
            .into_iter()
            .chain(b.to_le_bytes())
            .enumerate()
            .map(|(i, byte)| (BASE + i as u16, byte))
            .collect();
        call(&programs[name], &format!("__rust_{name}"), &bytes)
    };
    let value = |e: &sangoku_test_support::emu::Emulator, offset: usize| {
        u64::from_le_bytes(
            e.memory[BASE as usize + offset..BASE as usize + offset + 8]
                .try_into()
                .unwrap(),
        )
    };
    for (a, b) in [
        (0u64, 0u64),
        (u64::MAX, 1),
        (0xffff_ffff, 1),
        (0, 1),
        (i64::MAX as u64, 1),
        (1 << 63, u64::MAX),
        (0x0123_4567_89ab_cdef, 0xfedc_ba98_7654_3210),
    ] {
        for (op, expected) in [
            ("iadd", a.wrapping_add(b)),
            ("isub", a.wrapping_sub(b)),
            ("imul", a.wrapping_mul(b)),
            ("band", a & b),
            ("bor", a | b),
            ("bxor", a ^ b),
            ("umin", a.min(b)),
            ("umax", a.max(b)),
            ("smin", (a as i64).min(b as i64) as u64),
            ("smax", (a as i64).max(b as i64) as u64),
        ] {
            let e = run(&format!("{op}64"), a, b)?;
            assert_eq!(value(&e, 16), expected, "{op} {a:#x} {b:#x}");
        }
        for (cc, expected) in [
            ("eq", a == b),
            ("ne", a != b),
            ("ult", a < b),
            ("ule", a <= b),
            ("ugt", a > b),
            ("uge", a >= b),
            ("slt", (a as i64) < b as i64),
            ("sle", (a as i64) <= b as i64),
            ("sgt", (a as i64) > b as i64),
            ("sge", (a as i64) >= b as i64),
        ] {
            let e = run(&format!("cmp_{cc}64"), a, b)?;
            assert_eq!(
                e.memory[BASE as usize + 24],
                expected as u8,
                "{cc} {a:#x} {b:#x}"
            );
        }
        for (op, (expected, overflow)) in [
            ("uadd_overflow", a.overflowing_add(b)),
            ("usub_overflow", a.overflowing_sub(b)),
            ("sadd_overflow", {
                let (x, o) = (a as i64).overflowing_add(b as i64);
                (x as u64, o)
            }),
            ("ssub_overflow", {
                let (x, o) = (a as i64).overflowing_sub(b as i64);
                (x as u64, o)
            }),
        ] {
            let e = run(&format!("{op}64"), a, b)?;
            assert_eq!(
                (value(&e, 16), e.memory[BASE as usize + 24] != 0),
                (expected, overflow),
                "{op} {a:#x} {b:#x}"
            );
        }
        for name in ["call_sum64", "call_indirect_sum64"] {
            let e = run(name, a, b)?;
            assert_eq!(value(&e, 16), a.wrapping_add(b), "{name}");
        }
        let e = run("call_indirect_twice64", a, b)?;
        assert_eq!(value(&e, 16), a.wrapping_add(b).wrapping_add(b));
        let e = run("constant_mul64", a, b)?;
        assert_eq!(value(&e, 16), a.wrapping_mul(65536));
        let e = run("narrow_shift_wide_count64", a, b)?;
        assert_eq!(value(&e, 16) as u32, (a as u32).wrapping_shl(b as u32));
        let e = run("loop64", a, b)?;
        assert_eq!(value(&e, 16), a.wrapping_add(b & 3));
        let e = run("select64", a, b)?;
        assert_eq!(value(&e, 16), if a != 0 { a } else { b });
        let e = run("big_endian64", a, b)?;
        assert_eq!(value(&e, 16), a.swap_bytes());
        assert_eq!(value(&e, 24), b.swap_bytes());
        let e = run("flow64", a, b)?;
        assert_eq!(value(&e, 16), (if a != 0 { a } else { b }).wrapping_add(1));
    }
    for a in inputs {
        for (op, expected) in [
            ("bnot", !a),
            ("ineg", a.wrapping_neg()),
            ("iabs", (a as i64).wrapping_abs() as u64),
            ("clz", a.leading_zeros() as u64),
            ("ctz", a.trailing_zeros() as u64),
            ("popcnt", a.count_ones() as u64),
            ("bswap", a.swap_bytes()),
            ("bitrev", a.reverse_bits()),
        ] {
            let e = run(&format!("{op}64"), a, 0)?;
            assert_eq!(value(&e, 16), expected, "{op} {a:#x}");
        }
        for count in [0u32, 1, 31, 32, 33, 63, 64, 65, 127] {
            for (op, expected) in [
                ("ishl", a.wrapping_shl(count)),
                ("ushr", a.wrapping_shr(count)),
                ("sshr", (a as i64).wrapping_shr(count) as u64),
                ("rotl", a.rotate_left(count)),
                ("rotr", a.rotate_right(count)),
            ] {
                let e = run(&format!("{op}64"), a, count as u64)?;
                assert_eq!(value(&e, 16), expected, "{op} {a:#x} {count}");
            }
        }
        let e = run("conversions64", a, a)?;
        assert_eq!(value(&e, 16), a as i16 as i64 as u64);
        assert_eq!(value(&e, 24), a as u16 as u64);
        assert_eq!(value(&e, 32), a.rotate_left(32));
        assert_eq!(e.read16(BASE + 40), a as u16);
    }
    Ok(())
}

#[test]
fn rust_u64_memory_add_lowers_and_executes() -> Result<()> {
    let source = harness::root().join("tests/fixtures/core/programs/u64.rs");
    let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
        .args(["clif", source.to_str().unwrap(), "-"])
        .output()?;
    ensure!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let directory = sangoku_test_support::tempfile::tempdir()?;
    let path = directory.path().join("rust-u64.clif");
    std::fs::write(&path, output.stdout)?;
    let program = lower_program(path.to_str().unwrap())?;
    let stop = program.symbols["done"];
    for (a, b) in [
        (0xffff_ffffu64, 1u64),
        (u64::MAX, 1),
        (0x0123_4567_89ab_cdef, 0x7654_3210_fedc_ba98),
    ] {
        let mut e = program.emulator("__rust_sum_to_memory")?;
        e.memory[BASE as usize..BASE as usize + 8].copy_from_slice(&a.to_le_bytes());
        e.memory[BASE as usize + 8..BASE as usize + 16].copy_from_slice(&b.to_le_bytes());
        e.b = BASE as u8;
        e.c = (BASE >> 8) as u8;
        e.a = (BASE + 8) as u8;
        e.memory[STACK as usize] = ((BASE + 8) >> 8) as u8;
        e.write16(STACK + 1, BASE + 16);
        e.set_sp(STACK - 2);
        e.write16(STACK - 2, stop);
        e.run_until(stop, 65536)?;
        let result = u64::from_le_bytes(
            e.memory[BASE as usize + 16..BASE as usize + 24]
                .try_into()
                .unwrap(),
        );
        assert_eq!(result, a.wrapping_add(b));
    }
    Ok(())
}

#[test]
fn split_i128_values_match_host() -> Result<()> {
    for level in ["none", "speed"] {
        check_split_i128(level)?;
    }
    Ok(())
}

fn check_split_i128(level: &str) -> Result<()> {
    let source = include_str!("fixtures/emulate/split-i128.clif");
    let functions: Vec<_> = source.split("\nfunction ").skip(1).collect();
    let mut programs = std::collections::BTreeMap::new();
    for function in &functions {
        let name = function
            .strip_prefix('%')
            .unwrap()
            .split('(')
            .next()
            .unwrap();
        if name == "sum128" {
            continue;
        }
        let mut text = format!(
            "set enable_multi_ret_implicit_sret=true\nset opt_level={level}\ntarget z80\nfunction {function}"
        );
        if name.starts_with("call_") {
            let callee = functions
                .iter()
                .find(|f| f.starts_with("%sum128("))
                .unwrap();
            text.push_str(&format!("\nfunction {callee}"));
        }
        let directory = sangoku_test_support::tempfile::tempdir()?;
        let path = directory.path().join(format!("{level}-{name}.clif"));
        std::fs::write(&path, text)?;
        programs.insert(name.to_owned(), lower_program(path.to_str().unwrap())?);
    }
    let run = |name: &str, a: u128, b: u128| -> Result<_> {
        let bytes: Vec<_> = a
            .to_le_bytes()
            .into_iter()
            .chain(b.to_le_bytes())
            .enumerate()
            .map(|(i, v)| (BASE + i as u16, v))
            .collect();
        call(&programs[name], &format!("__rust_{name}"), &bytes)
    };
    let value = |e: &sangoku_test_support::emu::Emulator, offset: usize| {
        u128::from_le_bytes(
            e.memory[BASE as usize + offset..BASE as usize + offset + 16]
                .try_into()
                .unwrap(),
        )
    };
    for (a, b) in [
        (0u128, 0u128),
        (0, 1),
        (1 << 127, 1),
        (u128::MAX, 1),
        ((1 << 32) - 1, 1),
        ((1 << 64) - 1, 1),
        ((1 << 96) - 1, 1),
        (i128::MAX as u128, 1),
        (1 << 127, u128::MAX),
        (
            0x0123456789abcdef_fedcba9876543210,
            0xfedcba9876543210_0123456789abcdef,
        ),
    ] {
        for (op, expected) in [
            ("iadd", a.wrapping_add(b)),
            ("isub", a.wrapping_sub(b)),
            ("imul", a.wrapping_mul(b)),
            ("band", a & b),
            ("bor", a | b),
            ("bxor", a ^ b),
            ("umin", a.min(b)),
            ("umax", a.max(b)),
            ("smin", (a as i128).min(b as i128) as u128),
            ("smax", (a as i128).max(b as i128) as u128),
        ] {
            let e = run(&format!("{op}128"), a, b)?;
            assert_eq!(value(&e, 32), expected, "{level} {op} {a:#x} {b:#x}");
        }
        for (cc, expected) in [
            ("eq", a == b),
            ("ne", a != b),
            ("ult", a < b),
            ("ule", a <= b),
            ("ugt", a > b),
            ("uge", a >= b),
            ("slt", (a as i128) < b as i128),
            ("sle", (a as i128) <= b as i128),
            ("sgt", (a as i128) > b as i128),
            ("sge", (a as i128) >= b as i128),
        ] {
            let e = run(&format!("cmp_{cc}128"), a, b)?;
            assert_eq!(
                e.memory[BASE as usize + 48],
                u8::from(expected),
                "{level} {cc} {a:#x} {b:#x}"
            );
        }
        for (op, (expected, flag)) in [
            ("uadd_overflow", a.overflowing_add(b)),
            ("usub_overflow", a.overflowing_sub(b)),
            ("sadd_overflow", {
                let (v, f) = (a as i128).overflowing_add(b as i128);
                (v as u128, f)
            }),
            ("ssub_overflow", {
                let (v, f) = (a as i128).overflowing_sub(b as i128);
                (v as u128, f)
            }),
        ] {
            let e = run(&format!("{op}128"), a, b)?;
            assert_eq!(
                (value(&e, 32), e.memory[BASE as usize + 48] != 0),
                (expected, flag),
                "{level} {op}"
            );
        }
        let e = run("call_sum128", a, b)?;
        assert_eq!(value(&e, 32), a.wrapping_add(b));
        let e = run("call_indirect_twice128", a, b)?;
        assert_eq!(value(&e, 32), a.wrapping_add(b).wrapping_add(b));
        let e = run("select128", a, b)?;
        assert_eq!(value(&e, 32), if a != 0 { a } else { b });
        let e = run("loop128", a, b)?;
        assert_eq!(value(&e, 32), a.wrapping_add(b & 3));
        let e = run("big_endian128", a, b)?;
        assert_eq!(value(&e, 32), a.swap_bytes());
        assert_eq!(value(&e, 48), b.swap_bytes());
        let e = run("conversions128", a, b)?;
        assert_eq!(value(&e, 32), a as i64 as i128 as u128);
        assert_eq!(value(&e, 48), a as u64 as u128);
        assert_eq!(value(&e, 64), b.rotate_left(64));
        assert_eq!(value(&e, 80) as u64, b as u64);
        let e = run("flow128", a, b)?;
        assert_eq!(value(&e, 32), (if a != 0 { a } else { b }).wrapping_sub(1));
    }
    for a in [
        0u128,
        1,
        1 << 127,
        u128::MAX,
        0x0123456789abcdef_fedcba9876543210,
    ] {
        for (op, expected) in [
            ("bnot", !a),
            ("ineg", a.wrapping_neg()),
            ("iabs", (a as i128).wrapping_abs() as u128),
            ("clz", u128::from(a.leading_zeros())),
            ("ctz", u128::from(a.trailing_zeros())),
            ("popcnt", u128::from(a.count_ones())),
            ("bswap", a.swap_bytes()),
            ("bitrev", a.reverse_bits()),
        ] {
            let e = run(&format!("{op}128"), a, 0)?;
            assert_eq!(value(&e, 32), expected, "{level} {op}");
        }
        for count in [0u32, 1, 31, 32, 33, 63, 64, 65, 95, 96, 97, 127, 128, 129] {
            for (op, expected) in [
                ("ishl", a.wrapping_shl(count)),
                ("ushr", a.wrapping_shr(count)),
                ("sshr", (a as i128).wrapping_shr(count) as u128),
                ("rotl", a.rotate_left(count)),
                ("rotr", a.rotate_right(count)),
            ] {
                let e = run(&format!("{op}128"), a, u128::from(count))?;
                assert_eq!(value(&e, 32), expected, "{level} {op} {a:#x} {count}");
            }
        }
    }
    Ok(())
}

#[test]
fn rust_u128_memory_add_lowers_and_executes() -> Result<()> {
    let source = harness::root().join("tests/fixtures/core/programs/u128.rs");
    let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
        .args(["clif", source.to_str().unwrap(), "-"])
        .output()?;
    ensure!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let directory = sangoku_test_support::tempfile::tempdir()?;
    let path = directory.path().join("rust-u128.clif");
    std::fs::write(&path, output.stdout)?;
    let program = lower_program(path.to_str().unwrap())?;
    let stop = program.symbols["done"];
    for (a, b) in [
        (((1u128 << 96) - 1), 1u128),
        (u128::MAX, 1),
        (0x0123_4567_89ab_cdef, 0x7654_3210_fedc_ba98),
    ] {
        let mut e = program.emulator("__rust_sum_to_memory")?;
        e.memory[BASE as usize..BASE as usize + 16].copy_from_slice(&a.to_le_bytes());
        e.memory[BASE as usize + 16..BASE as usize + 32].copy_from_slice(&b.to_le_bytes());
        e.b = BASE as u8;
        e.c = (BASE >> 8) as u8;
        e.a = (BASE + 16) as u8;
        e.memory[STACK as usize] = ((BASE + 16) >> 8) as u8;
        e.write16(STACK + 1, BASE + 32);
        e.set_sp(STACK - 2);
        e.write16(STACK - 2, stop);
        e.run_until(stop, 65536)?;
        let result = u128::from_le_bytes(
            e.memory[BASE as usize + 32..BASE as usize + 48]
                .try_into()
                .unwrap(),
        );
        assert_eq!(result, a.wrapping_add(b));
    }
    Ok(())
}

#[test]
fn split_f64_preserves_bits() -> Result<()> {
    let source = include_str!("fixtures/emulate/split-f64.clif");
    for level in ["none", "speed"] {
        let directory = sangoku_test_support::tempfile::tempdir()?;
        let path = directory.path().join("f64.clif");
        std::fs::write(&path, format!("set opt_level={level}\n{source}"))?;
        let program = lower_program(path.to_str().unwrap())?;
        for (a, b) in [
            (0u64, 1u64 << 63),
            (1u64 << 63, 0),
            (0x7ff0_0000_0000_0000, 0xfff0_0000_0000_0000),
            (0xfff8_1234_5678_9abc, 0x7ff0_0000_0000_0001),
            (1, 0x8000_0000_0000_0001),
            (0x3ff0_0000_0000_0001, 0xbff0_0000_0000_0001),
        ] {
            for (name, expected) in [
                ("copy", a),
                ("bits", a),
                ("neg", a ^ (1 << 63)),
                ("abs", a & !(1 << 63)),
                ("sign", (a & !(1 << 63)) | (b & (1 << 63))),
                ("select", a),
                (
                    "constant",
                    (-0x123456789abcde_i64 as f64 / 4294967296.0).to_bits(),
                ),
                ("transport", a),
            ] {
                let memory: Vec<_> = a
                    .to_le_bytes()
                    .into_iter()
                    .chain(b.to_le_bytes())
                    .enumerate()
                    .map(|(i, byte)| (BASE + i as u16, byte))
                    .chain([(BASE + 24, 1)])
                    .collect();
                let e = call(&program, &format!("__rust_{name}"), &memory)?;
                let actual = u64::from_le_bytes(
                    e.memory[BASE as usize + 16..BASE as usize + 24]
                        .try_into()
                        .unwrap(),
                );
                assert_eq!(actual, expected, "{level} {name} {a:016x}");
            }
        }
    }
    Ok(())
}

#[test]
fn split_f128_preserves_bits() -> Result<()> {
    let source = include_str!("fixtures/emulate/split-f128.clif");
    for level in ["none", "speed"] {
        let directory = sangoku_test_support::tempfile::tempdir()?;
        let functions: Vec<_> = source.split("\nfunction ").skip(1).collect();
        let mut programs = std::collections::BTreeMap::new();
        for function in &functions {
            let name = function
                .strip_prefix('%')
                .unwrap()
                .split('(')
                .next()
                .unwrap();
            if name == "identity" {
                continue;
            }
            let mut text = format!(
                "set enable_multi_ret_implicit_sret=true\nset opt_level={level}\ntarget z80\nfunction {function}"
            );
            if name == "transport" {
                let callee = functions
                    .iter()
                    .find(|f| f.starts_with("%identity("))
                    .unwrap();
                text.push_str(&format!("\nfunction {callee}"));
            }
            let path = directory.path().join(format!("{name}.clif"));
            std::fs::write(&path, text)?;
            programs.insert(name, lower_program(path.to_str().unwrap())?);
        }
        for (a, b) in [
            (0u128, 1u128 << 127),
            (1u128 << 127, 0),
            (
                0x7fff_0000_0000_0000_0000_0000_0000_0000,
                0xffff_0000_0000_0000_0000_0000_0000_0000,
            ),
            (
                0xffff_8123_4567_89ab_cdef_0123_4567_89ab,
                0x7fff_0000_0000_0000_0000_0000_0000_0001,
            ),
            (1, 0x8000_0000_0000_0000_0000_0000_0000_0001),
            (
                0x3fff_0000_0000_0000_0000_0000_0000_0001,
                0xbfff_0000_0000_0000_0000_0000_0000_0001,
            ),
        ] {
            for (name, expected) in [
                ("copy", a),
                ("bits", a),
                ("neg", a ^ (1 << 127)),
                ("abs", a & !(1 << 127)),
                ("sign", (a & !(1 << 127)) | (b & (1 << 127))),
                ("select", if a != 0 { a } else { b }),
                ("constant", 0xc013_2345_6789_abcd_ef01_2345_6789_abcd),
                ("transport", a),
                ("big_endian", a.swap_bytes()),
            ] {
                let memory: Vec<_> = a
                    .to_le_bytes()
                    .into_iter()
                    .chain(b.to_le_bytes())
                    .enumerate()
                    .map(|(i, byte)| (BASE + i as u16, byte))
                    .chain([(BASE + 48, u8::from(a != 0))])
                    .collect();
                let e = call(&programs[name], &format!("__rust_{name}"), &memory)?;
                let actual = u128::from_le_bytes(
                    e.memory[BASE as usize + 32..BASE as usize + 48]
                        .try_into()
                        .unwrap(),
                );
                assert_eq!(actual, expected, "{level} {name} {a:032x}");
                if name == "big_endian" {
                    assert_eq!(
                        &e.memory[BASE as usize + 48..BASE as usize + 64],
                        &b.to_be_bytes()
                    );
                }
            }
        }
    }
    Ok(())
}

#[test]
fn rust_f128_memory_lowers_and_executes() -> Result<()> {
    let source = harness::root().join("tests/fixtures/core/programs/f128.rs");
    let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
        .args(["clif", source.to_str().unwrap(), "-"])
        .output()?;
    ensure!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let directory = sangoku_test_support::tempfile::tempdir()?;
    let path = directory.path().join("rust-f128.clif");
    std::fs::write(&path, output.stdout)?;
    let program = lower_program(path.to_str().unwrap())?;
    let stop = program.symbols["done"];
    for bits in [
        0u128,
        1 << 127,
        1,
        0xffff_8123_4567_89ab_cdef_0123_4567_89ab,
    ] {
        let mut e = program.emulator("__rust_copy_to_memory")?;
        e.memory[BASE as usize..BASE as usize + 16].copy_from_slice(&bits.to_le_bytes());
        e.b = BASE as u8;
        e.c = (BASE >> 8) as u8;
        e.a = (BASE + 16) as u8;
        e.memory[STACK as usize] = ((BASE + 16) >> 8) as u8;
        e.set_sp(STACK - 2);
        e.write16(STACK - 2, stop);
        e.run_until(stop, 65536)?;
        assert_eq!(
            &e.memory[BASE as usize + 16..BASE as usize + 32],
            &bits.to_le_bytes()
        );
    }
    let e = call(&program, "__rust_constant_to_memory", &[])?;
    assert_eq!(
        &e.memory[BASE as usize..BASE as usize + 16],
        &0x3fff_0000_0000_0000_0000_0000_0000_0000u128.to_le_bytes()
    );
    Ok(())
}

#[test]
fn split_i64_remaining_arithmetic_matches_host() -> Result<()> {
    let source = include_str!("fixtures/emulate/split-i64-arithmetic.clif");
    for level in ["none", "speed"] {
        let directory = sangoku_test_support::tempfile::tempdir()?;
        let mut programs = std::collections::BTreeMap::new();
        for function in source.split("\nfunction ").skip(1) {
            let name = function
                .strip_prefix('%')
                .unwrap()
                .split('(')
                .next()
                .unwrap();
            let text = format!(
                "set enable_multi_ret_implicit_sret=true\nset opt_level={level}\ntarget z80\nfunction {function}"
            );
            let path = directory.path().join(format!("{name}.clif"));
            std::fs::write(&path, text)?;
            programs.insert(name, lower_program(path.to_str().unwrap())?);
        }
        for (a, b) in [
            (0u64, 1u64),
            (1, 1),
            (u64::MAX, 1),
            (u64::MAX, u64::MAX),
            (1 << 63, u64::MAX),
            (1 << 63, 1),
            (1 << 63, 2),
            (i64::MAX as u64, 2),
            (1 << 32, 1 << 32),
            (0xffff_ffff, 0x1_0000_0001),
            (0x8000_0000_0000_0001, 0x8000_0000_0000_0000),
            (0x0123_4567_89ab_cdef, 0xfedc_ba98_7654_3210),
            ((-17i64) as u64, 5),
            (17, (-5i64) as u64),
            ((-17i64) as u64, (-5i64) as u64),
        ] {
            let memory: Vec<_> = a
                .to_le_bytes()
                .into_iter()
                .chain(b.to_le_bytes())
                .enumerate()
                .map(|(i, byte)| (BASE + i as u16, byte))
                .collect();
            let product = u128::from(a) * u128::from(b);
            let signed_product = i128::from(a as i64) * i128::from(b as i64);
            let normalized = if (a as i64) < 0 { !a } else { a };
            let mut cases = vec![
                ("udiv64", a / b, None),
                ("urem64", a % b, None),
                ("srem64", (a as i64).wrapping_rem(b as i64) as u64, None),
                ("umulhi64", (product >> 64) as u64, None),
                ("smulhi64", (signed_product >> 64) as u64, None),
                (
                    "umul_overflow64",
                    product as u64,
                    Some(product > u128::from(u64::MAX)),
                ),
                (
                    "smul_overflow64",
                    signed_product as u64,
                    Some(i64::try_from(signed_product).is_err()),
                ),
                ("cls64", u64::from(normalized.leading_zeros() - 1), None),
            ];
            if (a, b) != (1 << 63, u64::MAX) {
                cases.push(("sdiv64", ((a as i64) / (b as i64)) as u64, None));
            }
            for (name, expected, overflow) in cases {
                let e = call(&programs[name], &format!("__rust_{name}"), &memory)?;
                let actual = u64::from_le_bytes(
                    e.memory[BASE as usize + 16..BASE as usize + 24]
                        .try_into()
                        .unwrap(),
                );
                assert_eq!(actual, expected, "{level} {name} {a:016x} {b:016x}");
                if let Some(overflow) = overflow {
                    assert_eq!(
                        e.memory[BASE as usize + 24] != 0,
                        overflow,
                        "{level} {name} {a:016x} {b:016x}"
                    );
                }
            }
        }
        for (name, a, b) in [
            ("udiv64", 1u64, 0u64),
            ("urem64", 1, 0),
            ("sdiv64", 1, 0),
            ("srem64", 1, 0),
            ("sdiv64", 1 << 63, u64::MAX),
        ] {
            let program = &programs[name];
            let mut e = program.emulator(&format!("__rust_{name}"))?;
            e.memory[BASE as usize..BASE as usize + 8].copy_from_slice(&a.to_le_bytes());
            e.memory[BASE as usize + 8..BASE as usize + 16].copy_from_slice(&b.to_le_bytes());
            e.b = BASE as u8;
            e.c = (BASE >> 8) as u8;
            e.set_sp(STACK - 2);
            e.write16(STACK - 2, program.symbols["done"]);
            e.run_until(0x0010, 2_000_000)?;
        }
    }
    Ok(())
}

#[test]
fn scalar_integer_gaps_match_host() -> Result<()> {
    let source = include_str!("fixtures/emulate/scalar-integer-gaps.clif");
    for level in ["none", "speed"] {
        let directory = sangoku_test_support::tempfile::tempdir()?;
        let mut programs = std::collections::BTreeMap::new();
        for function in source.split("\nfunction ").skip(1) {
            let name = function
                .strip_prefix('%')
                .unwrap()
                .split('(')
                .next()
                .unwrap();
            let path = directory.path().join(format!("{name}.clif"));
            std::fs::write(
                &path,
                format!(
                    "set enable_multi_ret_implicit_sret=true\nset opt_level={level}\ntarget z80\nfunction {function}"
                ),
            )?;
            programs.insert(name, lower_program(path.to_str().unwrap())?);
        }
        for (a, b) in [
            (0u32, 0u32),
            (u32::MAX, 1),
            (0x8000_0000, u32::MAX),
            (i32::MAX as u32, 1),
            (0x10000, 0x10000),
            (0xffff, 0x10001),
            (0x8000_0000, 1),
            (17, (-5i32) as u32),
            (u32::MAX, u32::MAX),
        ] {
            let product = u64::from(a) * u64::from(b);
            let signed_product = i64::from(a as i32) * i64::from(b as i32);
            let memory: Vec<_> = a
                .to_le_bytes()
                .into_iter()
                .chain(b.to_le_bytes())
                .enumerate()
                .map(|(i, v)| (BASE + i as u16, v))
                .collect();
            for (op, value, overflow) in [
                (
                    "uadd_overflow",
                    a.overflowing_add(b).0,
                    Some(a.overflowing_add(b).1),
                ),
                (
                    "usub_overflow",
                    a.overflowing_sub(b).0,
                    Some(a.overflowing_sub(b).1),
                ),
                (
                    "sadd_overflow",
                    (a as i32).overflowing_add(b as i32).0 as u32,
                    Some((a as i32).overflowing_add(b as i32).1),
                ),
                (
                    "ssub_overflow",
                    (a as i32).overflowing_sub(b as i32).0 as u32,
                    Some((a as i32).overflowing_sub(b as i32).1),
                ),
                (
                    "umul_overflow",
                    product as u32,
                    Some(product > u64::from(u32::MAX)),
                ),
                (
                    "smul_overflow",
                    signed_product as u32,
                    Some(i32::try_from(signed_product).is_err()),
                ),
                ("umulhi", (product >> 32) as u32, None),
                ("smulhi", (signed_product >> 32) as u32, None),
                (
                    "cls",
                    (if (a as i32) < 0 { !a } else { a }).leading_zeros() - 1,
                    None,
                ),
            ] {
                let name = format!("{op}32");
                let e = call(&programs[name.as_str()], &format!("__rust_{name}"), &memory)?;
                assert_eq!(
                    u32::from_le_bytes(
                        e.memory[BASE as usize + 8..BASE as usize + 12]
                            .try_into()
                            .unwrap()
                    ),
                    value,
                    "{level} {name} {a:x} {b:x}"
                );
                if let Some(flag) = overflow {
                    assert_eq!(
                        e.memory[BASE as usize + 12] != 0,
                        flag,
                        "{level} {name} {a:x} {b:x}"
                    );
                }
            }
        }
        for bits in [
            0u64,
            u64::MAX,
            0x0123_4567_89ab_cdef,
            0x8000_0000,
            0x8000,
            0x80,
        ] {
            for width in [8u32, 16, 32] {
                for big in [false, true] {
                    if width == 8 && big {
                        continue;
                    }
                    let suffix = format!("{width}{}", if big { "_big" } else { "" });
                    let memory: Vec<_> = bits
                        .to_le_bytes()
                        .into_iter()
                        .enumerate()
                        .map(|(i, b)| (BASE + i as u16, b))
                        .chain((0..9).map(|i| (BASE + 16 + i, 0xa5)))
                        .collect();
                    for op in ["istore", "uload", "sload"] {
                        let name = format!("{op}{suffix}");
                        let e = call(&programs[name.as_str()], &format!("__rust_{name}"), &memory)?;
                        let bytes = (width / 8) as usize;
                        if op == "istore" {
                            let little = bits.to_le_bytes();
                            let mut expected = little[..bytes].to_vec();
                            if big {
                                expected.reverse();
                            }
                            assert_eq!(
                                &e.memory[BASE as usize + 16..BASE as usize + 16 + bytes],
                                &expected,
                                "{level} {name}"
                            );
                            assert_eq!(e.memory[BASE as usize + 16 + bytes], 0xa5);
                        } else {
                            let mut input = bits.to_le_bytes()[..bytes].to_vec();
                            if big {
                                input.reverse();
                            }
                            let mut value = 0u64;
                            for (i, b) in input.into_iter().enumerate() {
                                value |= u64::from(b) << (i * 8);
                            }
                            if op == "sload" {
                                value = ((value << (64 - width)) as i64 >> (64 - width)) as u64;
                            }
                            assert_eq!(
                                u64::from_le_bytes(
                                    e.memory[BASE as usize + 16..BASE as usize + 24]
                                        .try_into()
                                        .unwrap()
                                ),
                                value,
                                "{level} {name} {bits:x}"
                            );
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

#[test]
fn split_i128_remaining_arithmetic_matches_host() -> Result<()> {
    let source = include_str!("fixtures/emulate/split-i128-arithmetic.clif");
    let high_product = |a: u128, b: u128| {
        let al = a as u64 as u128;
        let ah = a >> 64;
        let bl = b as u64 as u128;
        let bh = b >> 64;
        let ll = al * bl;
        let lh = al * bh;
        let hl = ah * bl;
        let hh = ah * bh;
        let middle = (ll >> 64) + (lh as u64 as u128) + (hl as u64 as u128);
        hh + (lh >> 64) + (hl >> 64) + (middle >> 64)
    };
    for level in ["none", "speed"] {
        let directory = sangoku_test_support::tempfile::tempdir()?;
        let mut programs = std::collections::BTreeMap::new();
        for function in source.split("\nfunction ").skip(1) {
            let name = function
                .strip_prefix('%')
                .unwrap()
                .split('(')
                .next()
                .unwrap();
            let path = directory.path().join(format!("{name}.clif"));
            std::fs::write(
                &path,
                format!(
                    "set enable_multi_ret_implicit_sret=true\nset opt_level={level}\ntarget z80\nfunction {function}"
                ),
            )?;
            let program = lower_program(path.to_str().unwrap())?;
            let code_size: usize = program
                .sections
                .values()
                .filter(|s| s.name.starts_with("__rust_cgu_"))
                .map(|s| s.size)
                .sum();
            ensure!(
                code_size <= 8192,
                "{level} {name} uses {code_size} bytes; wide arithmetic must leave at least half a ROM bank for callers"
            );
            programs.insert(name, program);
        }
        for (a, b) in [
            (0u128, 1u128),
            (1, 1),
            (u128::MAX, 1),
            (u128::MAX, u128::MAX),
            (1 << 127, u128::MAX),
            (1 << 127, 1),
            (1 << 127, 2),
            (i128::MAX as u128, 2),
            (1 << 64, 1 << 64),
            ((1 << 96) - 1, (1 << 32) + 1),
            (0x8000_0000_0000_0000_0000_0000_0000_0001, 1 << 127),
            (
                0x0123_4567_89ab_cdef_fedc_ba98_7654_3210,
                0xfedc_ba98_7654_3210_0123_4567_89ab_cdef,
            ),
            ((-17i128) as u128, 5),
            (17, (-5i128) as u128),
            ((-17i128) as u128, (-5i128) as u128),
        ] {
            let am = if (a as i128) < 0 { a.wrapping_neg() } else { a };
            let bm = if (b as i128) < 0 { b.wrapping_neg() } else { b };
            let magnitude_high = high_product(am, bm);
            let signed_high = if ((a as i128) < 0) != ((b as i128) < 0) {
                (!magnitude_high).wrapping_add(u128::from(am.wrapping_mul(bm) == 0))
            } else {
                magnitude_high
            };
            let normalized = if (a as i128) < 0 { !a } else { a };
            let memory: Vec<_> = a
                .to_le_bytes()
                .into_iter()
                .chain(b.to_le_bytes())
                .enumerate()
                .map(|(i, v)| (BASE + i as u16, v))
                .collect();
            let mut cases = vec![
                ("udiv128", a / b),
                ("urem128", a % b),
                ("srem128", (a as i128).wrapping_rem(b as i128) as u128),
                ("umulhi128", high_product(a, b)),
                ("smulhi128", signed_high),
                ("cls128", u128::from(normalized.leading_zeros() - 1)),
            ];
            if (a, b) != (1 << 127, u128::MAX) {
                cases.push(("sdiv128", ((a as i128) / (b as i128)) as u128));
            }
            for (name, expected) in cases {
                let e = call(&programs[name], &format!("__rust_{name}"), &memory)?;
                let actual = u128::from_le_bytes(
                    e.memory[BASE as usize + 32..BASE as usize + 48]
                        .try_into()
                        .unwrap(),
                );
                assert_eq!(actual, expected, "{level} {name} {a:032x} {b:032x}");
            }
        }
        for (name, a, b) in [
            ("udiv128", 1u128, 0u128),
            ("urem128", 1, 0),
            ("sdiv128", 1, 0),
            ("srem128", 1, 0),
            ("sdiv128", 1 << 127, u128::MAX),
        ] {
            let program = &programs[name];
            let mut e = program.emulator(&format!("__rust_{name}"))?;
            e.memory[BASE as usize..BASE as usize + 16].copy_from_slice(&a.to_le_bytes());
            e.memory[BASE as usize + 16..BASE as usize + 32].copy_from_slice(&b.to_le_bytes());
            e.b = BASE as u8;
            e.c = (BASE >> 8) as u8;
            e.set_sp(STACK - 2);
            e.write16(STACK - 2, program.symbols["done"]);
            e.run_until(0x0010, 2_000_000)?;
        }
    }
    Ok(())
}

#[test]
fn f64_shared_runtime_matches_host() -> Result<()> {
    let source = include_str!("fixtures/emulate/softfloat-f64.clif");
    let mut input = harness::read("runtime/float/f64/common/implementation.c")?.into_bytes();
    input.extend(harness::read("tools/generate_softfloat64.py")?.into_bytes());
    let header = format!(
        "; generated from softfloat64.c, sha256 {}",
        harness::hash(&input)
    );
    ensure!(
        harness::read("runtime/float/f64/common/dispatch.asm")?
            .lines()
            .next()
            == Some(header.as_str()),
        "stale f64 runtime; run tools/generate_softfloat64.py"
    );
    for level in ["none", "speed"] {
        let directory = sangoku_test_support::tempfile::tempdir()?;
        let mut programs = std::collections::BTreeMap::new();
        for function in source.split("\nfunction ").skip(1) {
            let name = function
                .strip_prefix('%')
                .unwrap()
                .split('(')
                .next()
                .unwrap();
            let path = directory.path().join(format!("{name}.clif"));
            std::fs::write(
                &path,
                format!(
                    "set enable_multi_ret_implicit_sret=true\nset opt_level={level}\ntarget z80\nfunction {function}"
                ),
            )?;
            let program = lower_program(path.to_str().unwrap())?;
            ensure!(
                program.sections["softfloat64"].size <= 8192,
                "f64 runtime exceeds half a ROM bank"
            );
            let caller: usize = program
                .sections
                .values()
                .filter(|s| s.name.starts_with("__rust_cgu_"))
                .map(|s| s.size)
                .sum();
            ensure!(
                caller <= 4096,
                "{level} {name}: f64 caller grew to {caller} bytes"
            );
            ensure!(
                !program.sections.contains_key("softfloat32"),
                "f64-only caller pulled in f32 runtime"
            );
            programs.insert(name, program);
        }
        let invoke = |name: &str, a: f64, b: f64, c: f64| -> Result<_> {
            let memory: Vec<_> = a
                .to_bits()
                .to_le_bytes()
                .into_iter()
                .chain(b.to_bits().to_le_bytes())
                .chain(c.to_bits().to_le_bytes())
                .enumerate()
                .map(|(i, v)| (BASE + i as u16, v))
                .collect();
            call(&programs[name], &format!("__rust_{name}"), &memory)
        };
        let check = |name: &str, a: f64, b: f64, c: f64, expected: f64| -> Result<()> {
            let e = invoke(name, a, b, c)?;
            let got = u64::from_le_bytes(
                e.memory[BASE as usize + 24..BASE as usize + 32]
                    .try_into()
                    .unwrap(),
            );
            if expected.is_nan() {
                assert!(
                    f64::from_bits(got).is_nan(),
                    "{level} {name} {a:?} {b:?} {c:?}: {got:016x}"
                );
            } else {
                assert_eq!(got, expected.to_bits(), "{level} {name} {a:?} {b:?} {c:?}");
            }
            Ok(())
        };
        for (a, b) in [
            (0.0, -0.0),
            (1.0, 2.0),
            (1.0, 2.0f64.powi(-53)),
            (f64::from_bits(0x3ff0000000000001), 2.0f64.powi(-53)),
            (f64::MIN_POSITIVE, -f64::from_bits(1)),
            (f64::from_bits(1), f64::from_bits(1)),
            (f64::MAX, 2.0),
            (1e308, 1e-308),
            (-17.0, 5.0),
            (f64::INFINITY, f64::NEG_INFINITY),
            (f64::NAN, 1.0),
            (f64::from_bits(0x7ff0000000000001), 0.0),
        ] {
            for (name, expected) in [
                ("fadd64", a + b),
                ("fsub64", a - b),
                ("fmul64", a * b),
                ("fdiv64", a / b),
                (
                    "fmin64",
                    if a.is_nan() || b.is_nan() {
                        f64::NAN
                    } else if a == b {
                        if a.is_sign_negative() { a } else { b }
                    } else {
                        a.min(b)
                    },
                ),
                (
                    "fmax64",
                    if a.is_nan() || b.is_nan() {
                        f64::NAN
                    } else if a == b {
                        if a.is_sign_negative() { b } else { a }
                    } else {
                        a.max(b)
                    },
                ),
            ] {
                check(name, a, b, 0.0, expected)?;
            }
            let unordered = a.is_nan() || b.is_nan();
            for (cc, expected) in [
                ("eq", a == b),
                ("ne", a != b),
                ("one", !unordered && a != b),
                ("ueq", unordered || a == b),
                ("lt", a < b),
                ("le", a <= b),
                ("gt", a > b),
                ("ge", a >= b),
                ("ult", unordered || a < b),
                ("ule", unordered || a <= b),
                ("ugt", unordered || a > b),
                ("uge", unordered || a >= b),
                ("ord", !unordered),
                ("uno", unordered),
            ] {
                let name = format!("cmp_{cc}64");
                let e = invoke(&name, a, b, 0.0)?;
                assert_eq!(
                    e.memory[BASE as usize + 24] != 0,
                    expected,
                    "{level} {name} {a:?} {b:?}"
                );
            }
        }
        for a in [
            0.0,
            -0.0,
            0.5,
            -0.5,
            f64::from_bits(0x3fe0000000000001),
            1.5,
            2.5,
            -2.5,
            f64::from_bits(1),
            f64::MIN_POSITIVE,
            2.0,
            f64::MAX,
            f64::INFINITY,
            -1.0,
            f64::NAN,
        ] {
            for (name, expected) in [
                ("sqrt64", a.sqrt()),
                ("trunc64", a.trunc()),
                ("floor64", a.floor()),
                ("ceil64", a.ceil()),
                ("nearest64", a.round_ties_even()),
            ] {
                check(name, a, 0.0, 0.0, expected)?;
            }
        }
        for (a, b, c) in [
            (1.0, 2.0, 3.0),
            (
                f64::from_bits(0x3ff0000000000001),
                f64::from_bits(0x3feffffffffffffe),
                -1.0,
            ),
            (f64::MAX, 2.0, -f64::MAX),
            (1e308, 1e-308, -1.0),
            (f64::from_bits(1), 0.5, f64::from_bits(1)),
            (f64::INFINITY, 0.0, 1.0),
            (-0.0, 2.0, -0.0),
            (f64::MAX, f64::MAX, f64::NEG_INFINITY),
        ] {
            check("fma64", a, b, c, a.mul_add(b, c))?;
        }
    }
    Ok(())
}

#[test]
fn rust_f64_memory_add_lowers_and_executes() -> Result<()> {
    let source = harness::root().join("tests/fixtures/core/programs/f64.rs");
    let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
        .args(["clif", source.to_str().unwrap(), "-"])
        .output()?;
    ensure!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let directory = sangoku_test_support::tempfile::tempdir()?;
    let path = directory.path().join("rust-f64.clif");
    std::fs::write(&path, output.stdout)?;
    let program = lower_program(path.to_str().unwrap())?;
    ensure!(program.sections.contains_key("softfloat64"));
    ensure!(!program.sections.contains_key("softfloat32"));
    let stop = program.symbols["done"];
    for (a, b) in [
        (1.0, 2.0f64.powi(-52)),
        (f64::from_bits(1), f64::from_bits(1)),
        (f64::MAX, f64::MAX),
        (f64::NAN, 1.0),
    ] {
        let mut e = program.emulator("__rust_sum_to_memory")?;
        e.memory[BASE as usize..BASE as usize + 8].copy_from_slice(&a.to_bits().to_le_bytes());
        e.memory[BASE as usize + 8..BASE as usize + 16].copy_from_slice(&b.to_bits().to_le_bytes());
        e.b = BASE as u8;
        e.c = (BASE >> 8) as u8;
        e.a = (BASE + 8) as u8;
        e.memory[STACK as usize] = ((BASE + 8) >> 8) as u8;
        e.write16(STACK + 1, BASE + 16);
        e.set_sp(STACK - 2);
        e.write16(STACK - 2, stop);
        e.run_until(stop, 2_000_000)?;
        let got = u64::from_le_bytes(
            e.memory[BASE as usize + 16..BASE as usize + 24]
                .try_into()
                .unwrap(),
        );
        let expected = a + b;
        if expected.is_nan() {
            assert!(f64::from_bits(got).is_nan());
        } else {
            assert_eq!(got, expected.to_bits(), "{a:?} + {b:?}");
        }
    }
    Ok(())
}

#[test]
fn f128_shared_runtime_matches_reference() -> Result<()> {
    use sangoku_test_support::{Value, serde_json};
    let mut input = harness::read("runtime/float/f128/common/implementation.c")?.into_bytes();
    input.extend(harness::read("tools/generate_softfloat64.py")?.into_bytes());
    let header = format!(
        "; generated from softfloat128.c, sha256 {}",
        harness::hash(&input)
    );
    ensure!(
        harness::read("runtime/float/f128/common/dispatch.asm")?
            .lines()
            .next()
            == Some(header.as_str()),
        "stale f128 runtime; regenerate with --width 128"
    );
    let reference: Value =
        serde_json::from_str(include_str!("fixtures/emulate/softfloat-f128.json"))?;
    ensure!(
        reference["generator_sha256"]
            == harness::hash(harness::read("tools/generate_softfloat128_cases.py")?.as_bytes()),
        "stale binary128 reference cases"
    );
    let source = include_str!("fixtures/emulate/softfloat-f128.clif");
    for level in ["none", "speed"] {
        let directory = sangoku_test_support::tempfile::tempdir()?;
        let mut programs = std::collections::BTreeMap::new();
        let mut metrics = std::collections::BTreeMap::new();
        for function in source.split("\nfunction ").skip(1) {
            let name = function
                .strip_prefix('%')
                .unwrap()
                .split('(')
                .next()
                .unwrap();
            let path = directory.path().join(format!("{name}.clif"));
            std::fs::write(
                &path,
                format!(
                    "set enable_multi_ret_implicit_sret=true\nset opt_level={level}\ntarget z80\nfunction {function}"
                ),
            )?;
            let program = lower_program(path.to_str().unwrap())?;
            ensure!(
                program.sections["softfloat128"].size <= 8192,
                "f128 runtime exceeds half a ROM bank"
            );
            ensure!(
                !program.sections.contains_key("softfloat64")
                    && !program.sections.contains_key("softfloat32"),
                "f128 caller pulled in another float runtime"
            );
            let caller: usize = program
                .sections
                .values()
                .filter(|s| s.name.starts_with("__rust_cgu_"))
                .map(|s| s.size)
                .sum();
            ensure!(caller <= 8192, "{level} {name}: caller uses {caller} bytes");
            metrics.insert(name.to_owned(), (caller, 0i32));
            programs.insert(name.to_owned(), program);
        }
        for case in reference["cases"].as_array().unwrap() {
            let name = format!("{}128", case["op"].as_str().unwrap());
            let parse = |key: &str| u128::from_str_radix(case[key].as_str().unwrap(), 16).unwrap();
            let program = &programs[&name];
            let mut e = program.emulator(&format!("__rust_{name}"))?;
            for (i, key) in ["a", "b", "c"].iter().enumerate() {
                e.memory[BASE as usize + 16 * i..BASE as usize + 16 * (i + 1)]
                    .copy_from_slice(&parse(key).to_le_bytes());
            }
            e.b = BASE as u8;
            e.c = (BASE >> 8) as u8;
            e.set_sp(STACK - 2);
            e.write16(STACK - 2, program.symbols["done"]);
            e.reset_metrics();
            e.run_until(program.symbols["done"], 12_000_000)?;
            let got = if name.starts_with("cmp_") {
                u128::from(e.memory[BASE as usize + 48])
            } else {
                u128::from_le_bytes(
                    e.memory[BASE as usize + 48..BASE as usize + 64]
                        .try_into()
                        .unwrap(),
                )
            };
            let expected = parse("result");
            let nan = |v: u128| ((v >> 112) & 32767) == 32767 && v & ((1u128 << 112) - 1) != 0;
            assert!(
                got == expected || nan(expected) && nan(got),
                "{level} {name} {case}: got {got:032x}"
            );
            let peak = e.peak_stack_bytes + 2;
            ensure!(peak <= 512, "{level} {name} uses {peak} stack bytes");
            metrics.get_mut(&name).unwrap().1 = metrics[&name].1.max(peak);
        }
        if std::env::var_os("SANGOKU_REPORT_SF128").is_some() {
            for (name, (caller, stack)) in metrics {
                eprintln!("f128-size {level} {name} caller={caller} stack={stack}");
            }
        }
    }
    Ok(())
}

#[test]
fn rust_f128_arithmetic_lowers_and_executes() -> Result<()> {
    let reference: sangoku_test_support::Value = sangoku_test_support::serde_json::from_str(
        include_str!("fixtures/emulate/softfloat-f128.json"),
    )?;
    for (example, op, entry) in [
        ("f128-add.rs", "fadd", "sum_to_memory"),
        ("f128-fma.rs", "fma", "fma_to_memory"),
    ] {
        let source = harness::root()
            .join("tests/fixtures/core/programs")
            .join(example);
        let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
            .args(["clif", source.to_str().unwrap(), "-"])
            .output()?;
        ensure!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let directory = sangoku_test_support::tempfile::tempdir()?;
        let path = directory.path().join("rust-f128.clif");
        std::fs::write(&path, output.stdout)?;
        let program = lower_program(path.to_str().unwrap())?;
        ensure!(program.sections.contains_key("softfloat128"));
        ensure!(
            !program.sections.contains_key("softfloat64")
                && !program.sections.contains_key("softfloat32")
        );
        for case in reference["cases"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|c| c["op"] == op)
            .take(8)
        {
            let parse = |key: &str| u128::from_str_radix(case[key].as_str().unwrap(), 16).unwrap();
            let mut e = program.emulator(&format!("__rust_{entry}"))?;
            for (i, key) in ["a", "b", "c"].iter().enumerate() {
                e.memory[BASE as usize + 16 * i..BASE as usize + 16 * (i + 1)]
                    .copy_from_slice(&parse(key).to_le_bytes());
            }
            e.b = BASE as u8;
            e.c = (BASE >> 8) as u8;
            e.a = (BASE + 16) as u8;
            e.memory[STACK as usize] = ((BASE + 16) >> 8) as u8;
            if op == "fma" {
                e.write16(STACK + 1, BASE + 32);
                e.write16(STACK + 3, BASE + 48);
            } else {
                e.write16(STACK + 1, BASE + 48);
            }
            e.set_sp(STACK - 2);
            e.write16(STACK - 2, program.symbols["done"]);
            e.reset_metrics();
            e.run_until(program.symbols["done"], 12_000_000)?;
            let got = u128::from_le_bytes(
                e.memory[BASE as usize + 48..BASE as usize + 64]
                    .try_into()
                    .unwrap(),
            );
            let expected = parse("result");
            let nan = |v: u128| ((v >> 112) & 32767) == 32767 && v & ((1u128 << 112) - 1) != 0;
            assert!(
                got == expected || nan(expected) && nan(got),
                "{example} {case}: {got:032x}"
            );
            ensure!(
                e.peak_stack_bytes + 2 <= 512,
                "{example}: stack grew to {}",
                e.peak_stack_bytes + 2
            );
        }
    }
    Ok(())
}

#[test]
fn wide_numeric_conversions_match_reference() -> Result<()> {
    use sangoku_test_support::{Value, serde_json};
    let mut input = harness::read("runtime/float/convert/common/implementation.c")?.into_bytes();
    input.extend(harness::read("tools/generate_convert.py")?.into_bytes());
    let header = format!(
        "; generated from convert.c, sha256 {}",
        harness::hash(&input)
    );
    ensure!(
        harness::read("runtime/float/convert/common/dispatch.asm")?
            .lines()
            .next()
            == Some(header.as_str()),
        "stale conversion runtime; run tools/generate_convert.py"
    );
    let reference: Value =
        serde_json::from_str(include_str!("fixtures/emulate/wide-convert.json"))?;
    ensure!(
        reference["generator_sha256"]
            == harness::hash(harness::read("tools/generate_conversion_cases.py")?.as_bytes()),
        "stale conversion reference cases"
    );
    let mut groups = std::collections::BTreeMap::<String, String>::new();
    for function in include_str!("fixtures/emulate/wide-convert.clif")
        .split("\nfunction ")
        .skip(1)
    {
        let name = function
            .strip_prefix('%')
            .unwrap()
            .split('(')
            .next()
            .unwrap();
        let group = name.rsplit_once('_').unwrap().0;
        groups
            .entry(group.into())
            .or_default()
            .push_str(&format!("\nfunction {function}"));
    }
    for level in ["none", "speed"] {
        let directory = sangoku_test_support::tempfile::tempdir()?;
        for (group, functions) in &groups {
            let path = directory.path().join(format!("{group}.clif"));
            std::fs::write(
                &path,
                format!(
                    "set enable_multi_ret_implicit_sret=true\nset opt_level={level}\ntarget z80\n{functions}"
                ),
            )?;
            let program = lower_program(path.to_str().unwrap())?;
            ensure!(
                program.sections["convert"].size <= 8192,
                "conversion runtime exceeds half a ROM bank"
            );
            for case in reference["cases"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|case| case["name"].as_str().unwrap().rsplit_once('_').unwrap().0 == group)
            {
                let name = case["name"].as_str().unwrap();
                let destination = name.rsplit('_').next().unwrap();
                let bytes: usize = destination[1..].parse::<usize>()? / 8;
                let parse =
                    |key: &str| u128::from_str_radix(case[key].as_str().unwrap(), 16).unwrap();
                let mut e = program.emulator(&format!("__rust_{name}"))?;
                e.memory[BASE as usize..BASE as usize + 16]
                    .copy_from_slice(&parse("source").to_le_bytes());
                e.memory[BASE as usize + 16..BASE as usize + 32].fill(0xA5);
                e.b = BASE as u8;
                e.c = (BASE >> 8) as u8;
                e.set_sp(STACK - 2);
                e.write16(STACK - 2, program.symbols["done"]);
                let trap = case["trap"].as_bool().unwrap();
                let stop = if trap {
                    0x0010
                } else {
                    program.symbols["done"]
                };
                e.run_until(stop, 1_000_000)?;
                let output = &e.memory[BASE as usize + 16..BASE as usize + 16 + bytes];
                if trap {
                    ensure!(
                        output.iter().all(|&b| b == 0xA5),
                        "{level} {case}: wrote result before trapping"
                    );
                } else {
                    let mut raw = [0u8; 16];
                    raw[..bytes].copy_from_slice(output);
                    let got = u128::from_le_bytes(raw);
                    ensure!(got == parse("result"), "{level} {case}: got {got:x}");
                }
                ensure!(
                    e.memory[BASE as usize + 16 + bytes..BASE as usize + 32]
                        .iter()
                        .all(|&b| b == 0xA5),
                    "{level} {case}: overwrote destination"
                );
            }
        }
    }
    Ok(())
}

#[test]
fn rust_numeric_conversions_lower_and_execute() -> Result<()> {
    use sangoku_test_support::{Value, serde_json};
    let reference: Value =
        serde_json::from_str(include_str!("fixtures/emulate/wide-convert.json"))?;
    let source = harness::root().join("tests/fixtures/core/programs/conversions.rs");
    for opt in ["0", "2"] {
        let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
            .args(["clif", source.to_str().unwrap(), "-", "--opt-level", opt])
            .output()?;
        ensure!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let directory = sangoku_test_support::tempfile::tempdir()?;
        let path = directory.path().join("rust-conversions.clif");
        std::fs::write(&path, output.stdout)?;
        let program = lower_program(path.to_str().unwrap())?;
        ensure!(program.sections.contains_key("convert"));
        for (entry, name, bytes) in [
            ("i64_to_f16", "fcvt_from_sint_i64_f16", 2),
            ("u64_to_f32", "fcvt_from_uint_i64_f32", 4),
            ("i128_to_f64", "fcvt_from_sint_i128_f64", 8),
            ("u128_to_f128", "fcvt_from_uint_i128_f128", 16),
            ("f64_to_i8", "fcvt_to_sint_sat_f64_i8", 1),
            ("f128_to_u128", "fcvt_to_uint_sat_f128_i128", 16),
            ("f64_to_f16", "fdemote_f64_f16", 2),
            ("f16_to_f128", "fpromote_f16_f128", 16),
            ("f128_to_f32", "fdemote_f128_f32", 4),
        ] {
            for case in reference["cases"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|case| case["name"] == name)
                .take(12)
            {
                let parse =
                    |key: &str| u128::from_str_radix(case[key].as_str().unwrap(), 16).unwrap();
                let data = parse("source").to_le_bytes();
                let memory: Vec<_> = data
                    .iter()
                    .enumerate()
                    .map(|(i, &byte)| (BASE + i as u16, byte))
                    .collect();
                let e = call(&program, &format!("__rust_{entry}"), &memory)?;
                let mut raw = [0u8; 16];
                raw[..bytes].copy_from_slice(&e.memory[BASE as usize..BASE as usize + bytes]);
                let got = u128::from_le_bytes(raw);
                ensure!(
                    got == parse("result"),
                    "opt {opt} {entry} {case}: got {got:x}"
                );
            }
        }
    }
    Ok(())
}

#[test]
fn signed_division_overflow_and_remainder_follow_clif() -> Result<()> {
    let functions: String = include_str!("fixtures/emulate/wide-ops.clif")
        .split("\nfunction ")
        .skip(1)
        .filter(|function| function.starts_with("%sdiv_") || function.starts_with("%srem_"))
        .map(|function| format!("\nfunction {function}"))
        .collect();
    for level in ["none", "speed"] {
        let directory = sangoku_test_support::tempfile::tempdir()?;
        let path = directory.path().join("signed-division.clif");
        std::fs::write(
            &path,
            format!("set opt_level={level}\ntarget z80\n{functions}"),
        )?;
        let program = lower_program(path.to_str().unwrap())?;
        for bits in [8, 16, 32] {
            let minimum = -(1i64 << (bits - 1));
            let maximum = -minimum - 1;
            for (a, b) in [
                (minimum, -1),
                (minimum, 1),
                (minimum, -2),
                (minimum + 1, -1),
                (maximum, -1),
                (-7, 3),
                (7, -3),
                (-7, -3),
                (7, 3),
                (-1, -1),
                (0, -1),
                (1, 0),
            ] {
                for op in ["sdiv", "srem"] {
                    let name = format!("__rust_{op}_i{bits}");
                    let mut e = program.emulator(&name)?;
                    e.memory[BASE as usize..BASE as usize + 4]
                        .copy_from_slice(&(a as u32).to_le_bytes());
                    e.memory[BASE as usize + 4..BASE as usize + 8]
                        .copy_from_slice(&(b as u32).to_le_bytes());
                    e.b = BASE as u8;
                    e.c = (BASE >> 8) as u8;
                    e.set_sp(STACK - 2);
                    e.write16(STACK - 2, program.symbols["done"]);
                    let trap = b == 0 || op == "sdiv" && a == minimum && b == -1;
                    let stop = if trap {
                        0x0010
                    } else {
                        program.symbols["done"]
                    };
                    e.run_until(stop, 65536)?;
                    if !trap {
                        let result = if op == "sdiv" { a / b } else { a % b };
                        let mask = (1u64 << bits) - 1;
                        let got = u64::from(e.a)
                            | u64::from(e.b) << 8
                            | u64::from(e.c) << 16
                            | u64::from(e.d) << 24;
                        ensure!(
                            got & mask == result as u64 & mask,
                            "{level} {name} {a} {b}: got {got:x}"
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

#[test]
fn branch_tables_handle_full_width_indices() -> Result<()> {
    for level in ["none", "speed"] {
        let directory = sangoku_test_support::tempfile::tempdir()?;
        let path = directory.path().join("table.clif");
        std::fs::write(
            &path,
            format!(
                "set opt_level={level}\ntarget z80\nfunction %table(i16) -> i8 system_v {{\nblock0(v0: i16):\n v1 = load.i32 v0\n br_table v1, block3, [block1, block2]\nblock1:\n v2 = iconst.i8 11\n return v2\nblock2:\n v3 = iconst.i8 22\n return v3\nblock3:\n v4 = iconst.i8 33\n return v4\n}}\n"
            ),
        )?;
        let program = lower_program(path.to_str().unwrap())?;
        for (index, expected) in [
            (0u32, 11),
            (1, 22),
            (2, 33),
            (255, 33),
            (256, 33),
            (65536, 33),
            (1 << 24, 33),
            (u32::MAX, 33),
        ] {
            let got = invoke(&program, "__rust_table", &index.to_le_bytes())? & 255;
            ensure!(got == expected, "{level} index {index}: got {got}");
        }
    }
    Ok(())
}

#[test]
fn memset_runtime_obeys_split_abi_and_bounds() -> Result<()> {
    let program = harness::assemble(
        &[
            (
                "memset.asm",
                harness::read("runtime/memory/memset/speed/pointer.asm")?,
            ),
            ("stop.asm", "SECTION \"done\", ROM0\ndone::\nnop\n".into()),
        ],
        None,
    )?;
    for len in [0u16, 1, 17, 257] {
        let mut e = program.emulator("__gbc_memset")?;
        e.memory[0xc100..0xc220].fill(0xa5);
        e.b = 1;
        e.c = 0xc1;
        e.a = 0x37;
        e.memory[0xd000..0xd003].copy_from_slice(&[0x12, 0x34, 0x56]);
        e.write16(0xd003, len);
        e.set_sp(0xcffe);
        e.write16(0xcffe, program.symbols["done"]);
        e.run_until(program.symbols["done"], 10000)?;
        ensure!((e.a, e.b) == (1, 0xc1));
        ensure!(e.memory[0xc100] == 0xa5);
        ensure!(
            e.memory[0xc101..0xc101 + usize::from(len)]
                .iter()
                .all(|&b| b == 0x37),
            "len {len}: bytes {:?}, d={:x}",
            &e.memory[0xc100..0xc108],
            e.d
        );
        ensure!(
            e.memory[0xc101 + usize::from(len)..0xc220]
                .iter()
                .all(|&b| b == 0xa5)
        );
        ensure!(e.sp() == 0xd000);
    }
    Ok(())
}

#[test]
fn rust_usize_and_pointer_offsets_use_16_bits() -> Result<()> {
    let source = harness::root().join("tests/fixtures/core/programs/usize.rs");
    for level in ["0", "2"] {
        let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
            .args(["clif", source.to_str().unwrap(), "-", "--opt-level", level])
            .output()?;
        ensure!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let directory = sangoku_test_support::tempfile::tempdir()?;
        let path = directory.path().join("usize.clif");
        std::fs::write(&path, output.stdout)?;
        let program = lower_program(path.to_str().unwrap())?;
        let stop = program.symbols["done"];
        for (a, b) in [
            (0u16, 0u16),
            (32767, 1),
            (32768, 32768),
            (65535, 1),
            (65535, 65535),
        ] {
            let mut e = program.emulator("__rust_sum_to_memory")?;
            e.memory[BASE as usize..BASE as usize + 2].copy_from_slice(&a.to_le_bytes());
            e.memory[BASE as usize + 2..BASE as usize + 4].copy_from_slice(&b.to_le_bytes());
            e.b = BASE as u8;
            e.c = (BASE >> 8) as u8;
            e.a = (BASE + 2) as u8;
            e.memory[STACK as usize] = ((BASE + 2) >> 8) as u8;
            e.write16(STACK + 1, BASE + 16);
            e.set_sp(STACK - 2);
            e.write16(STACK - 2, stop);
            e.run_until(stop, 4096)?;
            ensure!(
                e.read16(BASE + 16) == a.wrapping_add(b),
                "level {level}: {a} + {b}"
            );
        }
        for (base, offset) in [(0u16, 1i16), (65535, 1), (1, -2), (0x8000, i16::MIN)] {
            let mut e = program.emulator("__rust_offset_to_memory")?;
            e.b = base as u8;
            e.c = (base >> 8) as u8;
            e.a = offset as u8;
            e.memory[STACK as usize] = ((offset as u16) >> 8) as u8;
            e.write16(STACK + 1, BASE + 16);
            e.set_sp(STACK - 2);
            e.write16(STACK - 2, stop);
            e.run_until(stop, 4096)?;
            ensure!(
                e.read16(BASE + 16) == base.wrapping_add(offset as u16),
                "level {level}: pointer {base} offset {offset}"
            );
        }
    }
    Ok(())
}

#[test]
fn memory_libcalls_follow_the_lowered_calling_convention() -> Result<()> {
    let program = program("memory-libcalls")?;
    let source: Vec<u8> = (0..37u8)
        .map(|i| i.wrapping_mul(5).wrapping_add(1))
        .collect();
    for (name, dst) in [
        ("__rust_copy_to", 0xC500u16),
        ("__rust_move_to", 0xC500),
        ("__rust_move_to", 0xC410),
        ("__rust_move_to", 0xC3F0),
    ] {
        let stop = program.symbols["done"];
        let mut e = program.emulator(name)?;
        e.memory[0xC400..0xC400 + source.len()].copy_from_slice(&source);
        e.b = dst as u8;
        e.c = (dst >> 8) as u8;
        e.set_sp(STACK - 2);
        e.write16(STACK - 2, stop);
        e.run_until(stop, 2_000_000)?;
        let at = usize::from(dst);
        ensure!(
            e.memory[at..at + source.len()] == source[..],
            "{name} to {dst:#x} copied the wrong bytes"
        );
        ensure!(
            u16::from(e.a) | u16::from(e.b) << 8 == dst,
            "{name} to {dst:#x} returned the wrong pointer"
        );
    }
    Ok(())
}
