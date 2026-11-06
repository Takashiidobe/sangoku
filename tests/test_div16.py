import random
import unittest

from runtime_harness import ROOT, assemble


class Div16ExecutionTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        source = (
            (ROOT / "runtime/udiv16.asm").read_text().replace("ROM0", "ROM0[$200]", 1)
        )
        cls.program = assemble({"division.asm": source})

    def test_division_core_with_explicit_register_contract(self):
        values = [0, 1, 2, 127, 128, 255, 256, 257, 32767, 32768, 65535]
        pairs = [(lhs, rhs) for lhs in values for rhs in values if rhs]
        rng = random.Random(0)
        pairs += [(rng.randrange(65536), rng.randrange(1, 65536)) for _ in range(512)]
        for lhs, rhs in pairs:
            with self.subTest(lhs=lhs, rhs=rhs):
                emu = self.program.emulator("__gbc_udiv16.divide")
                emu.b, emu.c = lhs >> 8, lhs & 255
                emu.d, emu.e = rhs & 255, rhs >> 8
                emu.sp = 0xD000
                emu.write16(emu.sp, 0x100)
                emu.reset_metrics()
                emu.run_until(0x100)
                self.assertEqual(emu.a | emu.b << 8, lhs // rhs)
                self.assertEqual(emu.hl, lhs % rhs)
                self.assertEqual(emu.sp, 0xD002)
                self.assertEqual(emu.peak_stack_bytes, 2)

    def test_zero_divisor_reaches_rst_vector(self):
        emu = self.program.emulator("__gbc_udiv16")
        emu.sp = 0xD000
        emu.write16(emu.sp, 0x100)
        emu.reset_metrics()
        emu.run_until(0x10)
        self.assertEqual(emu.sp, 0xCFFE)
        self.assertEqual(emu.peak_stack_bytes, 2)
        self.assertEqual(
            emu.read16(emu.sp), self.program.symbols["__gbc_udiv16.divide"]
        )


if __name__ == "__main__":
    unittest.main()
