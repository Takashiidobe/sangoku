use sangoku_test_support::{Result, ensure, harness};
use std::process::Command;

const STACK: u16 = 0xD000;
const DATA: u16 = 0xC100;

fn program(fixture: &str) -> Result<harness::Program> {
    let path = format!(
        "{}/tests/fixtures/emulate/{fixture}.clif",
        harness::root().display()
    );
    let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
        .args(["lower", &path, "-"])
        .output()?;
    ensure!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    harness::assemble(
        &[
            ("lowered.s", String::from_utf8(output.stdout)?),
            ("udiv16.asm", harness::read("runtime/udiv16.asm")?),
            ("mul16.asm", harness::read("runtime/mul16.asm")?),
            ("wide32.asm", harness::read("runtime/wide32.asm")?),
            ("softfloat32.asm", harness::read("runtime/softfloat32.asm")?),
            (
                "stop.s",
                "SECTION \"done\", ROM0\nEXPORT done\ndone::\n  nop\n".into(),
            ),
        ],
        None,
    )
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
