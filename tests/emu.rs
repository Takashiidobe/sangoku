use sangoku_test_support::emu::{CARRY, Emulator, ZERO};
fn emu(code: &[u8]) -> Emulator {
    Emulator::new(code, 0).unwrap()
}
fn step(e: &mut Emulator, cost: u32) {
    assert_eq!(e.step().unwrap(), cost);
}
#[test]
fn return_pops_address_and_wraps_stack() {
    let mut e = emu(&[]);
    e.pc = 0x100;
    e.memory[0x100] = 0xC9;
    e.set_sp(65535);
    e.write16(e.sp(), 0x1234);
    step(&mut e, 16);
    assert_eq!((e.pc, e.sp(), e.cycles), (0x1234, 1, 16));
}
#[test]
fn decrement_flags_preserve_carry() {
    for (v, result, f) in [(0, 255, 0x70), (1, 0, 0xD0), (16, 15, 0x70)] {
        let mut e = emu(&[5]);
        e.b = v;
        e.f = 0x10;
        step(&mut e, 4);
        assert_eq!((e.b, e.f), (result, f));
    }
}
#[test]
fn add_hl_preserves_zero_and_sets_carries() {
    let mut e = emu(&[9]);
    e.set_hl(65535);
    e.c = 1;
    e.f = 0xC0;
    step(&mut e, 8);
    assert_eq!((e.hl(), e.f), (0, 0xB0));
}
#[test]
fn signed_stack_offset_and_flags() {
    for (sp, off, result, flags) in [(1, 255, 0, 0x30), (0, 255, 65535, 0), (65535, 1, 0, 0x30)] {
        let mut e = emu(&[0xF8, off]);
        e.set_sp(sp);
        e.f = 0xF0;
        step(&mut e, 12);
        assert_eq!((e.hl(), e.f, e.sp()), (result, flags, sp));
    }
}
#[test]
fn memory_load_and_store() {
    let mut e = emu(&[0x77, 0x7E]);
    e.set_hl(0xC000);
    e.a = 0xAB;
    e.f = 0xF0;
    step(&mut e, 8);
    assert_eq!(e.memory[0xC000], 0xAB);
    e.a = 0;
    step(&mut e, 8);
    assert_eq!((e.a, e.f), (0xAB, 0xF0));
}
#[test]
fn rotate_carry_and_zero() {
    let mut e = emu(&[0xCB, 0x11, 0x17]);
    e.c = 0x80;
    e.a = 0x80;
    step(&mut e, 8);
    assert_eq!((e.c, e.f), (0, 0x90));
    step(&mut e, 4);
    assert_eq!((e.a, e.f), (1, 0x10));
}
#[test]
fn conditional_branch_timing() {
    for (flags, pc, cost) in [(0, 4, 12), (0x80, 2, 8)] {
        let mut e = emu(&[0x20, 2]);
        e.f = flags;
        step(&mut e, cost);
        assert_eq!(e.pc, pc);
    }
}
#[test]
fn run_until_returns_incremental_cost() {
    let mut e = emu(&[0, 0]);
    assert_eq!(e.run_until(1, 1).unwrap(), 4);
    assert_eq!(e.run_until(2, 1).unwrap(), 4);
    assert_eq!(e.run_until(2, 0).unwrap(), 0);
    assert_eq!(e.cycles, 8);
}
#[test]
fn run_until_limits_execution() {
    let mut e = emu(&[0x18, 0xFE]);
    assert!(e.run_until(2, 3).is_err());
    assert_eq!((e.pc, e.cycles), (0, 36));
}
#[test]
fn call_return_and_peak_stack() {
    let mut e = emu(&[0xCD, 8, 0, 0, 0, 0, 0, 0, 0xC9]);
    e.set_sp(0xD000);
    e.reset_metrics();
    step(&mut e, 24);
    assert_eq!((e.pc, e.sp(), e.read16(e.sp())), (8, 0xCFFE, 3));
    assert_eq!(e.peak_stack_bytes, 2);
    step(&mut e, 16);
    assert_eq!((e.pc, e.sp(), e.cycles), (3, 0xD000, 40));
    assert_eq!(
        e.written_addresses.into_iter().collect::<Vec<_>>(),
        [0xCFFE, 0xCFFF]
    );
}
#[test]
fn stack_pairs_and_af_mask() {
    for pair in 0..4 {
        let mut e = emu(&[0xC5 + pair * 16, 0xC1 + pair * 16]);
        e.set_sp(0xD000);
        e.b = 0x12;
        e.c = 0x34;
        e.d = 0x56;
        e.e = 0x78;
        e.set_hl(0x9ABC);
        e.a = 0xDE;
        e.f = 0xF0;
        e.reset_metrics();
        step(&mut e, 16);
        assert_eq!(
            e.read16(e.sp()),
            [0x1234, 0x5678, 0x9ABC, 0xDEF0][pair as usize]
        );
        e.write16(e.sp(), 0xABCD);
        step(&mut e, 12);
        let actual = if pair == 3 {
            (e.a as u16) << 8 | e.f as u16
        } else {
            e.read_pair(pair)
        };
        assert_eq!(actual, if pair == 3 { 0xABC0 } else { 0xABCD });
        assert_eq!((e.sp(), e.peak_stack_bytes), (0xD000, 2));
    }
}
#[test]
fn stack_accounting_wrap_and_reset() {
    let mut e = emu(&[0xC5, 0xC1]);
    e.set_sp(1);
    e.reset_metrics();
    e.step().unwrap();
    assert_eq!((e.sp(), e.peak_stack_bytes), (65535, 2));
    e.step().unwrap();
    assert_eq!((e.sp(), e.peak_stack_bytes), (1, 2));
    e.reset_metrics();
    assert_eq!(
        (e.cycles, e.peak_stack_bytes, e.written_addresses.len()),
        (0, 0, 0)
    );
}
#[test]
fn conditional_calls_returns_and_jumps() {
    for condition in 0..4 {
        let flag = if condition < 2 { ZERO } else { CARRY };
        for taken in [false, true] {
            let flags = if taken == (condition & 1 != 0) {
                flag
            } else {
                0
            };
            for (base, yes, no) in [(0xC4, 24, 12), (0xC0, 20, 8), (0xC2, 16, 12)] {
                let mut e = emu(&[base + condition * 8, 0x20, 0]);
                e.set_sp(0xD000);
                e.f = flags;
                e.write16(e.sp(), 0x20);
                e.reset_metrics();
                step(&mut e, if taken { yes } else { no });
                assert_eq!(
                    e.pc,
                    if taken {
                        0x20
                    } else if base == 0xC0 {
                        1
                    } else {
                        3
                    }
                );
                let sp = if taken {
                    match base {
                        0xC0 => 0xD002,
                        0xC4 => 0xCFFE,
                        _ => 0xD000,
                    }
                } else {
                    0xD000
                };
                assert_eq!(e.sp(), sp);
            }
        }
    }
}
#[test]
fn restart_vectors() {
    for v in (0..64).step_by(8) {
        let mut e = emu(&[0xC7 + v]);
        e.set_sp(0xD000);
        e.reset_metrics();
        step(&mut e, 16);
        assert_eq!((e.pc, e.sp(), e.read16(e.sp())), (v as u16, 0xCFFE, 1));
    }
}
#[test]
fn subtract_borrow_and_compare_exhaustively() {
    for op in [0x90, 0x98, 0xB8] {
        let mut e = emu(&[op]);
        for carry in 0..=u8::from(op == 0x98) {
            for lhs in 0..=255u16 {
                for rhs in 0..=255u16 {
                    e.pc = 0;
                    e.a = lhs as u8;
                    e.b = rhs as u8;
                    e.f = carry << 4;
                    step(&mut e, 4);
                    let d = lhs as i32 - rhs as i32 - carry as i32;
                    let result = d as u8;
                    let flags = 0x40
                        | if result == 0 { 0x80 } else { 0 }
                        | if (lhs as i32 ^ rhs as i32 ^ d) & 16 != 0 {
                            0x20
                        } else {
                            0
                        }
                        | if d < 0 { 0x10 } else { 0 };
                    assert_eq!(
                        (e.a, e.f),
                        (if op == 0xB8 { lhs as u8 } else { result }, flags),
                        "{op:X} {lhs} {rhs} {carry}"
                    );
                }
            }
        }
    }
}
#[test]
fn alu_operand_forms_and_flags() {
    for (op, lhs, rhs, flags, result, expected) in [
        (0, 255, 1, 0, 0, 0xB0),
        (1, 254, 1, 0x10, 0, 0xB0),
        (2, 0, 1, 0, 255, 0x70),
        (3, 0, 0, 0x10, 255, 0x70),
        (4, 0xF0, 15, 0, 0, 0xA0),
        (5, 255, 255, 0, 0, 0x80),
        (6, 0, 0, 0xF0, 0, 0x80),
        (7, 0, 1, 0, 0, 0x70),
    ] {
        for form in 0..3 {
            let opcode = if form == 2 {
                0xC6 + op * 8
            } else {
                0x80 + op * 8 + if form == 1 { 6 } else { 0 }
            };
            let mut e = emu(&[opcode, rhs]);
            e.set_hl(0xC000);
            e.memory[0xC000] = rhs;
            e.a = lhs;
            e.b = rhs;
            e.f = flags;
            step(&mut e, if form == 0 { 4 } else { 8 });
            assert_eq!((e.a, e.f), (if op == 7 { lhs } else { result }, expected));
        }
    }
}
#[test]
fn increment_and_pair_arithmetic() {
    for flags in [0, 0x10] {
        for v in 0..=255u16 {
            let mut e = emu(&[0x0C]);
            e.c = v as u8;
            e.f = flags;
            step(&mut e, 4);
            let result = (v + 1) as u8;
            let expected =
                flags | if result == 0 { 0x80 } else { 0 } | if v & 15 == 15 { 0x20 } else { 0 };
            assert_eq!((e.c, e.f), (result, expected));
        }
    }
    let mut e = emu(&[0x33, 0x3B, 0xE8, 0xFE]);
    e.set_sp(65535);
    e.f = 0xF0;
    e.reset_metrics();
    step(&mut e, 8);
    assert_eq!((e.sp(), e.f), (0, 0xF0));
    step(&mut e, 8);
    assert_eq!((e.sp(), e.f), (65535, 0xF0));
    step(&mut e, 16);
    assert_eq!((e.sp(), e.f), (0xFFFD, 0x30));
}
#[test]
fn bit_test_and_carry_instructions() {
    for (v, flags) in [(0, 0xB0), (0x80, 0x30)] {
        let mut e = emu(&[0xCB, 0x78, 0x3F, 0x37]);
        e.b = v;
        e.f = 0x50;
        step(&mut e, 8);
        assert_eq!(e.f, flags);
        step(&mut e, 4);
        assert_eq!(e.f, flags & 0x80);
        step(&mut e, 4);
        assert_eq!(e.f, flags & 0x80 | 0x10);
    }
}
#[test]
fn absolute_memory_and_accumulator_rotates() {
    let mut e = emu(&[0xEA, 0, 0xC0, 0xFA, 0, 0xC0]);
    e.a = 0x42;
    e.f = 0xF0;
    step(&mut e, 16);
    e.a = 0;
    step(&mut e, 16);
    assert_eq!((e.a, e.f), (0x42, 0xF0));
    assert_eq!(
        e.written_addresses.into_iter().collect::<Vec<_>>(),
        [0xC000]
    );
    for (op, v, carry, result, flags) in [
        (7, 0x80, 0, 1, 0x10),
        (15, 1, 0, 0x80, 0x10),
        (31, 0, 1, 0x80, 0),
        (23, 0x80, 1, 1, 0x10),
    ] {
        let mut e = emu(&[op]);
        e.a = v;
        e.f = 0xE0 | carry << 4;
        step(&mut e, 4);
        assert_eq!((e.a, e.f), (result, flags));
    }
}
#[test]
fn memory_bit_operations_and_circular_rotates() {
    for (op, v, result) in [(0, 0x80, 1), (8, 1, 0x80)] {
        let mut e = emu(&[0xCB, op]);
        e.b = v;
        e.f = 0xF0;
        step(&mut e, 8);
        assert_eq!((e.b, e.f), (result, 0x10));
    }
    let mut e = emu(&[0xCB, 0x7E, 0x34]);
    e.set_hl(0xC000);
    e.f = 0xF0;
    e.memory[0xC000] = 255;
    step(&mut e, 12);
    assert_eq!(e.f, 0x30);
    step(&mut e, 12);
    assert_eq!((e.memory[0xC000], e.f), (0, 0xB0));
    assert_eq!(
        e.written_addresses.into_iter().collect::<Vec<_>>(),
        [0xC000]
    );
}
#[test]
fn stack_pointer_load_indirect_jump_and_complement() {
    let mut e = emu(&[0x2F, 0xF9, 0xE9]);
    e.a = 0x55;
    e.f = 0x90;
    e.set_hl(0xCFFE);
    e.set_sp(0xD000);
    e.reset_metrics();
    step(&mut e, 4);
    assert_eq!((e.a, e.f), (0xAA, 0xF0));
    step(&mut e, 8);
    assert_eq!((e.sp(), e.peak_stack_bytes), (0xCFFE, 2));
    step(&mut e, 4);
    assert_eq!(e.pc, 0xCFFE);
}
#[test]
fn swap_flags_and_cost_for_all_bytes() {
    for r in [0, 6, 7] {
        let mut e = emu(&[0xCB, 0x30 | r]);
        e.set_hl(0xC000);
        for v in 0..=255u16 {
            e.pc = 0;
            e.f = 0xF0;
            e.write_reg(r, v as u8);
            step(&mut e, if r == 6 { 16 } else { 8 });
            assert_eq!(e.read8(r), (v as u8).rotate_left(4));
            assert_eq!(e.f, if v == 0 { 0x80 } else { 0 });
        }
    }
}
#[test]
fn unsupported_instruction_fails() {
    let mut e = emu(&[0xD3]);
    assert!(
        e.step()
            .unwrap_err()
            .to_string()
            .contains("opcode D3 at 0000")
    );
}

#[test]
fn arithmetic_shift_right_all_registers_bytes_flags_and_costs() {
    for register in 0..8 {
        let mut e = emu(&[0xCB, 0x28 | register]);
        for value in 0..=255u16 {
            for flags in (0..=0xF0).step_by(16) {
                e.pc = 0;
                e.set_hl(0xC000);
                e.write_reg(register, value as u8);
                e.f = flags;
                step(&mut e, if register == 6 { 16 } else { 8 });
                let expected = ((value as u8 as i8) >> 1) as u8;
                assert_eq!(e.read8(register), expected);
                assert_eq!(
                    e.f,
                    if expected == 0 { 0x80 } else { 0 } | if value & 1 != 0 { 0x10 } else { 0 }
                );
            }
        }
    }
}

#[test]
fn auto_indexed_hl_load_store_preserve_flags_and_wrap() {
    for op in [0x22, 0x2A, 0x32, 0x3A] {
        for address in [0u16, 0xC000, 0xFFFF] {
            let mut e = emu(&[]);
            e.memory[0x100] = op;
            e.pc = 0x100;
            e.set_hl(address);
            e.a = 0xA5;
            e.f = 0xF0;
            e.memory[address as usize] = 0x5A;
            e.reset_metrics();
            step(&mut e, 8);
            assert_eq!(
                e.hl(),
                address.wrapping_add(if op & 0x10 == 0 { 1 } else { u16::MAX })
            );
            assert_eq!(e.f, 0xF0);
            assert_eq!(e.a, if op & 8 == 0 { 0xA5 } else { 0x5A });
            assert_eq!(
                e.memory[address as usize],
                if op & 8 == 0 { 0xA5 } else { 0x5A }
            );
            assert_eq!(e.written_addresses.len(), usize::from(op & 8 == 0));
        }
    }
}
#[test]
fn set_and_res_change_one_bit_and_keep_flags() {
    let mut e = emu(&[0xCB, 0xC0, 0xCB, 0x80]);
    e.b = 0x0A;
    e.f = 0xF0;
    step(&mut e, 8);
    assert_eq!((e.b, e.f), (0x0B, 0xF0));
    step(&mut e, 8);
    assert_eq!((e.b, e.f), (0x0A, 0xF0));
    let mut e = emu(&[0xCB, 0xFE]);
    e.set_hl(0xC000);
    step(&mut e, 16);
    assert_eq!(e.memory[0xC000], 0x80);
}
#[test]
fn accumulator_transfers_through_bc_and_de() {
    let mut e = emu(&[0x1A, 0x02, 0x0A, 0x12]);
    e.set_hl(0);
    e.d = 0xC0;
    e.e = 0x10;
    e.b = 0xC0;
    e.c = 0x20;
    e.memory[0xC010] = 0x5A;
    step(&mut e, 8);
    assert_eq!(e.a, 0x5A);
    step(&mut e, 8);
    assert_eq!(e.memory[0xC020], 0x5A);
    e.memory[0xC020] = 0x77;
    step(&mut e, 8);
    assert_eq!(e.a, 0x77);
    step(&mut e, 8);
    assert_eq!(e.memory[0xC010], 0x77);
}
