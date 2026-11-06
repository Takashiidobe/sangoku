import random
import subprocess
import tempfile
import unittest
from pathlib import Path

from emu import Emulator

ROOT = Path(__file__).resolve().parents[1]


class Mul16Test(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        with tempfile.TemporaryDirectory() as directory:
            directory = Path(directory)
            source = (ROOT / "runtime/mul16.asm").read_text()
            (directory / "mul16.asm").write_text(
                source.replace("ROM0", "ROM0[$100]", 1)
            )
            subprocess.run(
                [
                    "rgbasm",
                    "-o",
                    str(directory / "mul16.o"),
                    str(directory / "mul16.asm"),
                ],
                check=True,
            )
            subprocess.run(
                [
                    "rgblink",
                    "-o",
                    str(directory / "mul16.gb"),
                    str(directory / "mul16.o"),
                ],
                check=True,
            )
            cls.code = (directory / "mul16.gb").read_bytes()

    def run_product(self, lhs, rhs):
        emu = Emulator(self.code, pc=0x100)
        emu.b, emu.c = lhs & 0xFF, lhs >> 8
        emu.a = rhs & 0xFF
        emu.d, emu.e, emu.hl = 0x55, 0xAA, 0x1234
        emu.f = 0xF0
        emu.sp = 0xD000
        emu.write16(emu.sp, 0x150)
        emu.memory[emu.sp + 2] = rhs >> 8
        ticks = emu.run_until(0x150)
        self.assertEqual(emu.sp, 0xD002)
        self.assertEqual(emu.memory[0xD002], rhs >> 8)
        return emu.a | emu.b << 8, ticks

    def check_product(self, lhs, rhs):
        result, _ = self.run_product(lhs & 0xFFFF, rhs & 0xFFFF)
        self.assertEqual(result, (lhs * rhs) & 0xFFFF, (lhs, rhs))

    def test_boundaries_and_byte_order(self):
        values = [
            0,
            1,
            2,
            0x7F,
            0x80,
            0xFF,
            0x100,
            0x101,
            0x1234,
            0x5678,
            0x7FFF,
            0x8000,
            0xFFFF,
            -1,
            -2,
            -32768,
        ]
        for lhs in values:
            for rhs in values:
                with self.subTest(lhs=lhs, rhs=rhs):
                    self.check_product(lhs, rhs)

    def test_random_products(self):
        rng = random.Random(0)
        for _ in range(10000):
            self.check_product(rng.randrange(65536), rng.randrange(65536))

    def test_shortcut_timing(self):
        for rhs in (0, 1, 0xFF, 0x100, 0xFFFF):
            with self.subTest(rhs=rhs):
                result, ticks = self.run_product(0x1234, rhs)
                self.assertEqual(result, (0x1234 * rhs) & 0xFFFF)
                base = 480 if rhs < 256 else 848
                self.assertEqual(ticks, base + 4 * rhs.bit_count())


if __name__ == "__main__":
    unittest.main()
