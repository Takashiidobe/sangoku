use sangoku_test_support::{Result, ensure, harness, random::Random};

const STACK: u16 = 0xdf00;
const REGION: usize = 0xc000;
const REGION_LEN: usize = 0x1000;
const BASE: usize = REGION + 0x400;

fn program(variant: &str) -> Result<harness::Program> {
    harness::assemble_tiny(
        &[
            (
                "stop.asm",
                "SECTION \"reserved\", ROM0[$0000]\nds $100, 0\ndone::\nnop\n".into(),
            ),
            (
                "memmove.asm",
                harness::read(format!("runtime/memmove_{variant}.asm"))?,
            ),
            (
                "memset.asm",
                harness::read(format!("runtime/memset_{variant}.asm"))?,
            ),
            (
                "memcpy.asm",
                harness::read(format!("runtime/memcpy_{variant}.asm"))?,
            ),
        ],
        None,
    )
}

fn pattern() -> Vec<u8> {
    (0..REGION_LEN)
        .map(|i| (i as u8).wrapping_mul(7).wrapping_add(3))
        .collect()
}

fn check(
    program: &harness::Program,
    name: &str,
    dst: usize,
    src: usize,
    len: usize,
) -> Result<u64> {
    let stop = program.symbols["done"];
    let mut e = program.emulator(name)?;
    let initial = pattern();
    e.memory[REGION..REGION + REGION_LEN].copy_from_slice(&initial);
    e.b = dst as u8;
    e.c = (dst >> 8) as u8;
    e.a = src as u8;
    e.set_sp(STACK);
    e.write16(STACK, stop);
    e.memory[usize::from(STACK) + 2] = (src >> 8) as u8;
    e.memory[usize::from(STACK) + 3] = len as u8;
    e.memory[usize::from(STACK) + 4] = (len >> 8) as u8;
    let arguments = e.memory[usize::from(STACK) + 2..usize::from(STACK) + 5].to_vec();
    e.run_until(stop, 5_000_000)?;
    ensure!(
        e.memory[usize::from(STACK) + 2..usize::from(STACK) + 5] == arguments,
        "{name}: stack arguments changed"
    );

    let mut expected = initial;
    let moved = expected[src - REGION..src - REGION + len].to_vec();
    expected[dst - REGION..dst - REGION + len].copy_from_slice(&moved);
    let context = format!("{name}(dst={dst:#x}, src={src:#x}, len={len})");
    ensure!(e.sp() == STACK + 2, "{context}: stack pointer not restored");
    ensure!(
        usize::from(e.a) | usize::from(e.b) << 8 == dst,
        "{context}: did not return dst"
    );
    ensure!(
        e.memory[REGION..REGION + REGION_LEN] == expected[..],
        "{context}: wrong memory"
    );
    Ok(e.cycles)
}

#[test]
fn memcpy_and_memmove_match_host_including_overlap() -> Result<()> {
    let mut cases = vec![
        (0isize, 0isize, 0usize),
        (0, 0, 1),
        (16, 0, 0),
        (0, 16, 1),
        (0, 0, 511),
    ];
    for len in [
        0usize, 1, 2, 3, 4, 5, 7, 8, 15, 255, 256, 257, 300, 511, 512, 1023, 1024,
    ] {
        for shift in [
            -5isize,
            -1,
            1,
            5,
            len as isize - 1,
            len as isize,
            len as isize + 1,
            -(len as isize),
        ] {
            cases.push((shift, 0, len));
        }
    }
    let mut random = Random::new(0x6d656d);
    for _ in 0..40 {
        let mut next = |modulus: u32| random.below(modulus) as isize;
        cases.push((next(200), next(200), next(64) as usize));
    }
    for variant in ["size", "opt"] {
        let program = program(variant)?;
        for &(dst, src, len) in &cases {
            let (dst, src) = (
                (BASE as isize + dst) as usize,
                (BASE as isize + src) as usize,
            );
            check(&program, "memmove", dst, src, len)?;
            if dst.abs_diff(src) >= len {
                check(&program, "memcpy", dst, src, len)?;
            }
        }
    }
    Ok(())
}

#[test]
fn memcpy_variants_trade_rom_for_long_copy_cycles() -> Result<()> {
    let size = program("size")?;
    let opt = program("opt")?;
    let size_bytes = size.size("ROM0", &["memcpy"]);
    let opt_bytes = opt.size("ROM0", &["memcpy"]);
    ensure!(size_bytes > 0 && size_bytes < opt_bytes);
    println!("memcpy ROM bytes: size={size_bytes}, opt={opt_bytes}");
    for len in [0, 1, 2, 3, 4, 8, 16, 64, 256] {
        let size_cycles = check(&size, "memcpy", BASE + 512, BASE, len)?;
        let opt_cycles = check(&opt, "memcpy", BASE + 512, BASE, len)?;
        println!("length={len}: size={size_cycles}, opt={opt_cycles} T-cycles (entry through RET)");
        if len >= 16 {
            ensure!(opt_cycles < size_cycles);
        }
    }
    Ok(())
}

fn check_fill(program: &harness::Program, dst: usize, value: u8, len: usize) -> Result<u64> {
    let stop = program.symbols["done"];
    let mut e = program.emulator("__gbc_memset")?;
    let mut expected = pattern();
    e.memory[REGION..REGION + REGION_LEN].copy_from_slice(&expected);
    expected[dst - REGION..dst - REGION + len].fill(value);
    e.b = dst as u8;
    e.c = (dst >> 8) as u8;
    e.a = value;
    e.set_sp(STACK);
    e.write16(STACK, stop);
    e.memory[usize::from(STACK) + 2..usize::from(STACK) + 5].copy_from_slice(&[0x12, 0x34, 0x56]);
    e.write16(STACK + 5, len as u16);
    let arguments = e.memory[usize::from(STACK) + 2..usize::from(STACK) + 7].to_vec();
    e.run_until(stop, 5_000_000)?;
    ensure!(e.sp() == STACK + 2, "memset: stack pointer not restored");
    ensure!(
        usize::from(e.a) | usize::from(e.b) << 8 == dst,
        "memset: did not return dst"
    );
    ensure!(
        e.memory[usize::from(STACK) + 2..usize::from(STACK) + 7] == arguments,
        "memset: stack arguments changed"
    );
    ensure!(
        e.memory[REGION..REGION + REGION_LEN] == expected,
        "memset(dst={dst:#x}, value={value:#x}, len={len}): wrong memory"
    );
    Ok(e.cycles)
}

#[test]
fn memset_variants_match_host_and_preserve_abi() -> Result<()> {
    for variant in ["size", "opt"] {
        let program = program(variant)?;
        for len in [0, 1, 2, 3, 4, 5, 7, 8, 255, 256, 257, 511, 1024] {
            for value in [0, 1, 0x37, 0x80, 0xfe, 0xff] {
                check_fill(&program, BASE + 255, value, len)?;
            }
            check_fill(&program, REGION + REGION_LEN - len, 0x5a, len)?;
        }
    }
    Ok(())
}

#[test]
fn move_and_fill_variants_trade_rom_for_long_operation_cycles() -> Result<()> {
    let size = program("size")?;
    let opt = program("opt")?;
    for helper in ["memmove", "memset"] {
        let size_bytes = size.size("ROM0", &[helper]);
        let opt_bytes = opt.size("ROM0", &[helper]);
        ensure!(
            size_bytes > 0 && size_bytes < opt_bytes,
            "{helper}: ROM size"
        );
        println!("{helper} ROM bytes: size={size_bytes}, opt={opt_bytes}");
    }
    for len in [0, 1, 2, 3, 4, 8, 16, 64, 256, 257, 1024] {
        for (direction, dst, src) in [("forward", BASE, BASE + 1), ("backward", BASE + 1, BASE)] {
            let size_cycles = check(&size, "memmove", dst, src, len)?;
            let opt_cycles = check(&opt, "memmove", dst, src, len)?;
            println!("memmove {direction} len={len}: size={size_cycles}, opt={opt_cycles}");
            if len >= 16 {
                ensure!(opt_cycles < size_cycles);
            }
        }
        let size_cycles = check_fill(&size, BASE + 255, 0xa5, len)?;
        let opt_cycles = check_fill(&opt, BASE + 255, 0xa5, len)?;
        println!("memset len={len}: size={size_cycles}, opt={opt_cycles}");
        if len >= 16 {
            ensure!(opt_cycles < size_cycles);
        }
    }
    Ok(())
}
