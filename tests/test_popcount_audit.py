import random
import unittest

from audit_popcount import VARIANTS, build, input_classes, modes_for, run_case


class PopcountAuditTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.programs = {variant: build(variant) for variant in VARIANTS}

    def test_candidates_against_host_bit_count(self):
        rng = random.Random(1)
        values = sorted(
            set(range(256))
            | {value << 8 for value in range(256)}
            | {0xFFFF, 0x5555, 0xAAAA, 0x8001, 0x1234}
            | {rng.randrange(65536) for _ in range(128)}
        )
        for variant, program in self.programs.items():
            for value in values:
                for mode in modes_for(variant):
                    result = run_case(program, variant, value, mode)
                    self.assertTrue(result["correct"], (variant, value, mode, result))

    def test_frozen_compiler_spill_defect_depends_on_stack(self):
        program = build("raw")
        first = run_case(program, "raw", 0, stack_base=0xD000)
        second = run_case(program, "raw", 0, stack_base=0xD002)
        self.assertEqual(program.size("ROM0", {"Audit helper"}), 207)
        self.assertFalse(first["correct"])
        self.assertFalse(second["correct"])
        self.assertNotEqual(first["actual"], second["actual"])

    def test_scratch_and_stack_state_independence(self):
        for variant, program in self.programs.items():
            for poison in (0, 0xA5, 255):
                for stack_base in (0xC900, 0xD002):
                    result = run_case(
                        program, variant, 0x8001, poison=poison, stack_base=stack_base
                    )
                    self.assertTrue(
                        result["correct"], (variant, poison, stack_base, result)
                    )

    def test_tables_and_alignment(self):
        for variant, size, alignment in [
            ("nibble", 16, 16),
            ("nibble-page", 16, 256),
            ("byte", 256, 256),
        ]:
            program = self.programs[variant]
            address = program.symbols["audit_table"]
            self.assertEqual(address % alignment, 0)
            self.assertEqual(program.size("ROM0", {"Audit table"}), size)
            self.assertEqual(
                list(program.rom[address : address + size]),
                [value.bit_count() for value in range(size)],
            )

    def test_inline_and_call_overhead(self):
        for variant, program in self.programs.items():
            if "inline" not in modes_for(variant):
                continue
            called = run_case(program, variant, 0x1234, "call")
            inlined = run_case(program, variant, 0x1234, "inline")
            self.assertEqual(called["actual"], inlined["actual"])
            self.assertEqual(called["cycles"] - inlined["cycles"], 40)
        called = run_case(self.programs["swar"], "swar", 0xFFFF, "call-live")
        inlined = run_case(self.programs["swar"], "swar", 0xFFFF, "inline-live")
        self.assertEqual(called["actual"], 15)
        self.assertEqual(inlined["actual"], 15)
        self.assertEqual(
            (called["peak_stack_bytes"], inlined["peak_stack_bytes"]), (4, 0)
        )

    def test_fixed_cost_candidates(self):
        for variant, code_bytes, cycles in [
            ("swar", 51, 216),
            ("nibble-page", 34, 164),
            ("byte", 10, 60),
        ]:
            program = self.programs[variant]
            self.assertEqual(program.size("ROM0", {"Audit helper"}), code_bytes)
            for value in (0, 1, 0xFFFF):
                self.assertEqual(run_case(program, variant, value)["cycles"], cycles)

    def test_input_distributions(self):
        classes = input_classes(128, 0)
        self.assertEqual(classes, input_classes(128, 0))
        self.assertNotEqual(
            classes["uniform-random"], input_classes(128, 1)["uniform-random"]
        )
        self.assertEqual(classes["all-8-bit"], list(range(256)))
        self.assertTrue(all(value.bit_count() <= 2 for value in classes["sparse"]))
        self.assertTrue(all(value.bit_count() >= 14 for value in classes["dense"]))


if __name__ == "__main__":
    unittest.main()
