import random
import subprocess
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def run(code, lhs, rhs):
    r = [lhs & 255, lhs >> 8, 0x55, 0xAA, 0x12, 0x34, 0, rhs & 255]
    pc, sp, carry, zero, ticks = 0x100, 0xD000, 0, False, 0
    memory = bytearray(65536)
    memory[sp + 2] = rhs >> 8

    def hl():
        return r[4] * 256 + r[5]

    def sethl(v):
        r[4], r[5] = (v >> 8) & 255, v & 255

    for _ in range(1000):
        op = code[pc]
        pc += 1
        if op == 0xC9:
            return r[7] | (r[0] << 8), ticks + 16
        if 0x40 <= op <= 0x7F:
            dst, src = (op >> 3) & 7, op & 7
            v = memory[hl()] if src == 6 else r[src]
            r[dst] = v
            ticks += 8 if src == 6 else 4
        elif op == 0xF8:
            sethl(sp + code[pc])
            pc += 1
            carry = 0
            zero = False
            ticks += 12
        elif op == 0x21:
            sethl(code[pc] | code[pc + 1] << 8)
            pc += 2
            ticks += 12
        elif op in (0x3E, 0x06):
            r[7 if op == 0x3E else 0] = code[pc]
            pc += 1
            ticks += 8
        elif op in (0x09, 0x19, 0x29):
            v = (
                hl()
                + ({0x09: r[0] * 256 + r[1], 0x19: r[2] * 256 + r[3], 0x29: hl()}[op])
            )
            carry = int(v > 65535)
            sethl(v)
            ticks += 8
        elif op == 0xCB:
            sub = code[pc]
            pc += 1
            reg = sub & 7
            old = r[reg]
            if sub >> 3 == 7:
                r[reg], carry = old >> 1, old & 1
            elif sub >> 3 == 3:
                r[reg], carry = (old >> 1) | (carry << 7), old & 1
            elif sub >> 3 == 4:
                r[reg], carry = (old << 1) & 255, old >> 7
            elif sub >> 3 == 2:
                r[reg], carry = ((old << 1) | carry) & 255, old >> 7
            else:
                raise ValueError(hex(sub))
            zero = r[reg] == 0
            ticks += 8
        elif op == 0x17:
            old = r[7]
            r[7], carry = ((old << 1) | carry) & 255, old >> 7
            zero = False
            ticks += 4
        elif op == 0xB7:
            zero = r[7] == 0
            carry = 0
            ticks += 4
        elif op in (0x05, 0x3D):
            reg = 0 if op == 0x05 else 7
            r[reg] = (r[reg] - 1) & 255
            zero = r[reg] == 0
            ticks += 4
        elif op in (0x20, 0x30):
            disp = code[pc]
            pc += 1
            taken = not (zero if op == 0x20 else carry)
            if taken:
                pc += disp if disp < 128 else disp - 256
            ticks += 12 if taken else 8
        else:
            raise ValueError(hex(op))
    raise RuntimeError("loop")


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

    def check_product(self, lhs, rhs):
        result, _ = run(self.code, lhs & 0xFFFF, rhs & 0xFFFF)
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
                result, ticks = run(self.code, 0x1234, rhs)
                self.assertEqual(result, (0x1234 * rhs) & 0xFFFF)
                base = 480 if rhs < 256 else 848
                self.assertEqual(ticks, base + 4 * rhs.bit_count())


if __name__ == "__main__":
    unittest.main()
