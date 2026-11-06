import unittest

from emu import Emulator


class EmulatorTest(unittest.TestCase):
    def test_return_pops_address_and_wraps_stack(self):
        emu = Emulator(pc=0x100)
        emu.memory[0x100] = 0xC9
        emu.sp = 0xFFFF
        emu.write16(emu.sp, 0x1234)
        self.assertEqual(emu.step(), 16)
        self.assertEqual((emu.pc, emu.sp, emu.cycles), (0x1234, 1, 16))

    def test_decrement_flags_preserve_carry(self):
        for value, result, flags in ((0, 255, 0x70), (1, 0, 0xD0), (16, 15, 0x70)):
            with self.subTest(value=value):
                emu = Emulator(bytes([0x05]))
                emu.b, emu.f = value, 0x10
                self.assertEqual(emu.step(), 4)
                self.assertEqual((emu.b, emu.f), (result, flags))

    def test_add_hl_preserves_zero_and_sets_carries(self):
        emu = Emulator(bytes([0x09]))
        emu.hl, emu.b, emu.c, emu.f = 0xFFFF, 0, 1, 0xC0
        self.assertEqual(emu.step(), 8)
        self.assertEqual((emu.hl, emu.f), (0, 0xB0))

    def test_signed_stack_offset_and_flags(self):
        for sp, offset, result, flags in (
            (1, 0xFF, 0, 0x30),
            (0, 0xFF, 0xFFFF, 0),
            (0xFFFF, 1, 0, 0x30),
        ):
            with self.subTest(sp=sp, offset=offset):
                emu = Emulator(bytes([0xF8, offset]))
                emu.sp, emu.f = sp, 0xF0
                self.assertEqual(emu.step(), 12)
                self.assertEqual((emu.hl, emu.f, emu.sp), (result, flags, sp))

    def test_memory_load_and_store(self):
        emu = Emulator(bytes([0x77, 0x7E]))
        emu.hl, emu.a, emu.f = 0xC000, 0xAB, 0xF0
        self.assertEqual(emu.step(), 8)
        self.assertEqual(emu.memory[0xC000], 0xAB)
        emu.a = 0
        self.assertEqual(emu.step(), 8)
        self.assertEqual((emu.a, emu.f), (0xAB, 0xF0))

    def test_rotate_carry_and_zero(self):
        emu = Emulator(bytes([0xCB, 0x11, 0x17]))
        emu.c, emu.a, emu.f = 0x80, 0x80, 0
        self.assertEqual(emu.step(), 8)
        self.assertEqual((emu.c, emu.f), (0, 0x90))
        self.assertEqual(emu.step(), 4)
        self.assertEqual((emu.a, emu.f), (1, 0x10))

    def test_conditional_branch_timing(self):
        for flags, pc, cost in ((0, 4, 12), (0x80, 2, 8)):
            with self.subTest(flags=flags):
                emu = Emulator(bytes([0x20, 2]))
                emu.f = flags
                self.assertEqual(emu.step(), cost)
                self.assertEqual(emu.pc, pc)

    def test_run_until_returns_incremental_cost(self):
        emu = Emulator(bytes([0, 0]))
        self.assertEqual(emu.run_until(1, max_instructions=1), 4)
        self.assertEqual(emu.run_until(2, max_instructions=1), 4)
        self.assertEqual(emu.run_until(2, max_instructions=0), 0)
        self.assertEqual(emu.cycles, 8)

    def test_run_until_limits_execution(self):
        emu = Emulator(bytes([0x18, 0xFE]))
        with self.assertRaises(TimeoutError):
            emu.run_until(2, max_instructions=3)
        self.assertEqual((emu.pc, emu.cycles), (0, 36))

    def test_call_return_and_peak_stack(self):
        emu = Emulator(bytes([0xCD, 8, 0, 0, 0, 0, 0, 0, 0xC9]))
        emu.sp = 0xD000
        emu.reset_metrics()
        self.assertEqual(emu.step(), 24)
        self.assertEqual((emu.pc, emu.sp, emu.read16(emu.sp)), (8, 0xCFFE, 3))
        self.assertEqual(emu.peak_stack_bytes, 2)
        self.assertEqual(emu.step(), 16)
        self.assertEqual((emu.pc, emu.sp, emu.cycles), (3, 0xD000, 40))
        self.assertEqual(emu.written_addresses, {0xCFFE, 0xCFFF})

    def test_stack_pairs_and_af_mask(self):
        for pair in range(4):
            with self.subTest(pair=pair):
                emu = Emulator(bytes([0xC5 + pair * 16, 0xC1 + pair * 16]))
                emu.sp = 0xD000
                emu.b, emu.c, emu.d, emu.e, emu.hl = 0x12, 0x34, 0x56, 0x78, 0x9ABC
                emu.a, emu.f = 0xDE, 0xF0
                expected = [0x1234, 0x5678, 0x9ABC, 0xDEF0][pair]
                emu.reset_metrics()
                self.assertEqual(emu.step(), 16)
                self.assertEqual(emu.read16(emu.sp), expected)
                emu.write16(emu.sp, 0xABCD)
                self.assertEqual(emu.step(), 12)
                actual = emu.a << 8 | emu.f if pair == 3 else emu._read_pair(pair)
                self.assertEqual(actual, 0xABC0 if pair == 3 else 0xABCD)
                self.assertEqual((emu.sp, emu.peak_stack_bytes), (0xD000, 2))

    def test_stack_accounting_wrap_and_reset(self):
        emu = Emulator(bytes([0xC5, 0xC1]))
        emu.sp = 1
        emu.reset_metrics()
        emu.step()
        self.assertEqual((emu.sp, emu.peak_stack_bytes), (0xFFFF, 2))
        emu.step()
        self.assertEqual((emu.sp, emu.peak_stack_bytes), (1, 2))
        emu.reset_metrics()
        self.assertEqual(
            (emu.cycles, emu.peak_stack_bytes, emu.written_addresses), (0, 0, set())
        )

    def test_conditional_calls_returns_and_jumps(self):
        for condition in range(4):
            flag = Emulator.ZERO if condition < 2 else Emulator.CARRY
            for taken in (False, True):
                flags = flag if taken == bool(condition & 1) else 0
                for base, yes_cost, no_cost in (
                    (0xC4, 24, 12),
                    (0xC0, 20, 8),
                    (0xC2, 16, 12),
                ):
                    with self.subTest(condition=condition, taken=taken, base=base):
                        emu = Emulator(bytes([base + condition * 8, 0x20, 0]))
                        emu.sp, emu.f = 0xD000, flags
                        emu.write16(emu.sp, 0x20)
                        emu.reset_metrics()
                        self.assertEqual(emu.step(), yes_cost if taken else no_cost)
                        sequential = 1 if base == 0xC0 else 3
                        self.assertEqual(emu.pc, 0x20 if taken else sequential)
                        expected_sp = (
                            0xD000 + (2 if base == 0xC0 else -2 if base == 0xC4 else 0)
                            if taken
                            else 0xD000
                        )
                        self.assertEqual(emu.sp, expected_sp)

    def test_restart_vectors(self):
        for vector in range(0, 64, 8):
            emu = Emulator(bytes([0xC7 + vector]))
            emu.sp = 0xD000
            emu.reset_metrics()
            self.assertEqual(emu.step(), 16)
            self.assertEqual((emu.pc, emu.sp, emu.read16(emu.sp)), (vector, 0xCFFE, 1))

    def test_subtract_borrow_and_compare_exhaustively(self):
        for opcode in (0x90, 0x98, 0xB8):
            emu = Emulator(bytes([opcode]))
            for carry in (0, 1) if opcode == 0x98 else (0,):
                for lhs in range(256):
                    for rhs in range(256):
                        emu.pc, emu.a, emu.b, emu.f = 0, lhs, rhs, carry << 4
                        self.assertEqual(emu.step(), 4)
                        difference = lhs - rhs - carry
                        result = difference & 255
                        flags = (
                            0x40
                            | (0x80 if result == 0 else 0)
                            | (0x20 if (lhs ^ rhs ^ difference) & 16 else 0)
                            | (0x10 if difference < 0 else 0)
                        )
                        self.assertEqual(
                            (emu.a, emu.f),
                            (lhs if opcode == 0xB8 else result, flags),
                            (opcode, lhs, rhs, carry),
                        )

    def test_alu_operand_forms_and_flags(self):
        cases = [
            (0, 0xFF, 1, 0, 0, 0xB0),
            (1, 0xFE, 1, 0x10, 0, 0xB0),
            (2, 0, 1, 0, 0xFF, 0x70),
            (3, 0, 0, 0x10, 0xFF, 0x70),
            (4, 0xF0, 0x0F, 0, 0, 0xA0),
            (5, 0xFF, 0xFF, 0, 0, 0x80),
            (6, 0, 0, 0xF0, 0, 0x80),
            (7, 0, 1, 0, 0, 0x70),
        ]
        for operation, lhs, rhs, flags, result, expected_flags in cases:
            for form in ("register", "memory", "immediate"):
                with self.subTest(operation=operation, form=form):
                    opcode = (
                        0xC6 + operation * 8
                        if form == "immediate"
                        else (0x80 + operation * 8 + (6 if form == "memory" else 0))
                    )
                    emu = Emulator(bytes([opcode, rhs]))
                    emu.hl = 0xC000
                    emu.memory[emu.hl] = rhs
                    emu.a, emu.b, emu.f = lhs, rhs, flags
                    self.assertEqual(emu.step(), 4 if form == "register" else 8)
                    self.assertEqual((emu.a, emu.f), (result, expected_flags))

    def test_increment_and_pair_arithmetic(self):
        for flags in (0, 0x10):
            for value in range(256):
                emu = Emulator(bytes([0x0C]))
                emu.c, emu.f = value, flags
                self.assertEqual(emu.step(), 4)
                result = (value + 1) & 255
                expected_flags = (
                    flags
                    | (0x80 if result == 0 else 0)
                    | (0x20 if value & 15 == 15 else 0)
                )
                self.assertEqual((emu.c, emu.f), (result, expected_flags))
        emu = Emulator(bytes([0x33, 0x3B, 0xE8, 0xFE]))
        emu.sp, emu.f = 0xFFFF, 0xF0
        emu.reset_metrics()
        self.assertEqual(emu.step(), 8)
        self.assertEqual((emu.sp, emu.f), (0, 0xF0))
        self.assertEqual(emu.step(), 8)
        self.assertEqual((emu.sp, emu.f), (0xFFFF, 0xF0))
        self.assertEqual(emu.step(), 16)
        self.assertEqual((emu.sp, emu.f), (0xFFFD, 0x30))

    def test_bit_test_and_carry_instructions(self):
        for value, flags in ((0, 0xB0), (0x80, 0x30)):
            emu = Emulator(bytes([0xCB, 0x78, 0x3F, 0x37]))
            emu.b, emu.f = value, 0x50
            self.assertEqual(emu.step(), 8)
            self.assertEqual(emu.f, flags)
            self.assertEqual(emu.step(), 4)
            self.assertEqual(emu.f, flags & 0x80)
            self.assertEqual(emu.step(), 4)
            self.assertEqual(emu.f, flags & 0x80 | 0x10)

    def test_absolute_memory_and_accumulator_rotates(self):
        emu = Emulator(bytes([0xEA, 0, 0xC0, 0xFA, 0, 0xC0]))
        emu.a, emu.f = 0x42, 0xF0
        self.assertEqual(emu.step(), 16)
        emu.a = 0
        self.assertEqual(emu.step(), 16)
        self.assertEqual((emu.a, emu.f), (0x42, 0xF0))
        self.assertEqual(emu.written_addresses, {0xC000})
        for opcode, value, carry, result, flags in (
            (0x07, 0x80, 0, 1, 0x10),
            (0x0F, 1, 0, 0x80, 0x10),
            (0x1F, 0, 1, 0x80, 0),
            (0x17, 0x80, 1, 1, 0x10),
        ):
            emu = Emulator(bytes([opcode]))
            emu.a, emu.f = value, 0xE0 | carry << 4
            self.assertEqual(emu.step(), 4)
            self.assertEqual((emu.a, emu.f), (result, flags))

    def test_memory_bit_operations_and_circular_rotates(self):
        for opcode, value, result in ((0x00, 0x80, 1), (0x08, 1, 0x80)):
            emu = Emulator(bytes([0xCB, opcode]))
            emu.b, emu.f = value, 0xF0
            self.assertEqual(emu.step(), 8)
            self.assertEqual((emu.b, emu.f), (result, 0x10))
        emu = Emulator(bytes([0xCB, 0x7E, 0x34]))
        emu.hl, emu.f = 0xC000, 0xF0
        emu.memory[emu.hl] = 0xFF
        self.assertEqual(emu.step(), 12)
        self.assertEqual(emu.f, 0x30)
        self.assertEqual(emu.step(), 12)
        self.assertEqual((emu.memory[emu.hl], emu.f), (0, 0xB0))
        self.assertEqual(emu.written_addresses, {0xC000})

    def test_stack_pointer_load_indirect_jump_and_complement(self):
        emu = Emulator(bytes([0x2F, 0xF9, 0xE9]))
        emu.a, emu.f, emu.hl, emu.sp = 0x55, 0x90, 0xCFFE, 0xD000
        emu.reset_metrics()
        self.assertEqual(emu.step(), 4)
        self.assertEqual((emu.a, emu.f), (0xAA, 0xF0))
        self.assertEqual(emu.step(), 8)
        self.assertEqual((emu.sp, emu.peak_stack_bytes), (0xCFFE, 2))
        self.assertEqual(emu.step(), 4)
        self.assertEqual(emu.pc, 0xCFFE)

    def test_unsupported_instruction_fails(self):
        emu = Emulator(bytes([0xD3]))
        with self.assertRaisesRegex(NotImplementedError, "opcode D3 at 0000"):
            emu.step()


if __name__ == "__main__":
    unittest.main()
