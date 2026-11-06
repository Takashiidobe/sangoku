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

    def test_unsupported_instruction_fails(self):
        emu = Emulator(bytes([0xD3]))
        with self.assertRaisesRegex(NotImplementedError, "opcode D3 at 0000"):
            emu.step()


if __name__ == "__main__":
    unittest.main()
