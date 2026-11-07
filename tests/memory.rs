use sangoku_test_support::{Result, ensure, harness, random::Random};

const STACK: u16 = 0xdf00;
const REGION: usize = 0xc000;
const REGION_LEN: usize = 0x1000;
const BASE: usize = REGION + 0x400;

fn program() -> Result<harness::Program> {
    harness::assemble_tiny(
        &[
            (
                "stop.asm",
                "SECTION \"reserved\", ROM0[$0000]\nds $100, 0\ndone::\nnop\n".into(),
            ),
            ("memory.asm", harness::read("runtime/memory.asm")?),
        ],
        None,
    )
}

fn pattern() -> Vec<u8> {
    (0..REGION_LEN)
        .map(|i| (i as u8).wrapping_mul(7).wrapping_add(3))
        .collect()
}

fn check(program: &harness::Program, name: &str, dst: usize, src: usize, len: usize) -> Result<()> {
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
    e.run_until(stop, 5_000_000)?;

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
    Ok(())
}

#[test]
fn memcpy_and_memmove_match_host_including_overlap() -> Result<()> {
    let program = program()?;
    let mut cases = vec![(0isize, 0isize, 0usize), (0, 0, 1), (16, 0, 0), (0, 16, 1)];
    for len in [1usize, 2, 3, 7, 255, 256, 257, 300] {
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
    for (dst, src, len) in cases {
        let (dst, src) = (
            (BASE as isize + dst) as usize,
            (BASE as isize + src) as usize,
        );
        check(&program, "memmove", dst, src, len)?;
        if dst.abs_diff(src) >= len {
            check(&program, "memcpy", dst, src, len)?;
        }
    }
    Ok(())
}
