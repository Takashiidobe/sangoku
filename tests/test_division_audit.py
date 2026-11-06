import random
import subprocess
import unittest

from audit_division import (
    BOUNDARIES,
    OPERATIONS,
    VARIANTS,
    build,
    input_classes,
    quotient_remainder_size,
    run_case,
)


class DivisionAuditTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.programs = {variant: build(variant) for variant in VARIANTS}

    def test_variants_against_host_divmod(self):
        pairs = [(lhs, rhs) for lhs in BOUNDARIES for rhs in BOUNDARIES if rhs]
        rng = random.Random(1)
        pairs += [(rng.randrange(65536), rng.randrange(1, 65536)) for _ in range(128)]
        for variant, program in self.programs.items():
            for lhs, rhs in pairs:
                for operation in OPERATIONS:
                    result = run_case(program, operation, lhs, rhs)
                    self.assertTrue(
                        result["correct"], (variant, operation, lhs, rhs, result)
                    )
                result = run_case(program, "both", lhs, rhs, body=True)
                self.assertTrue(result["correct"], (variant, lhs, rhs, result))

    def test_wrappers_ignore_scratch_register_state(self):
        for variant, program in self.programs.items():
            for poison in (0, 0xA5, 255):
                for operation in ("q", "r", "both"):
                    result = run_case(
                        program, operation, 0xFEDC, 129, body=True, poison=poison
                    )
                    self.assertTrue(
                        result["correct"], (variant, poison, operation, result)
                    )

    def test_zero_divisors_trap_for_every_entry(self):
        for variant, program in self.programs.items():
            for lhs in (0, 1, 32768, 65535):
                for operation in OPERATIONS:
                    result = run_case(program, operation, lhs, 0)
                    self.assertTrue(
                        result["correct"], (variant, lhs, operation, result)
                    )
                    self.assertEqual(result["status"], "rst_10")

    def test_shared_divmod_avoids_second_division(self):
        for variant, program in self.programs.items():
            combined = run_case(program, "both", 65535, 127)
            separate = run_case(program, "separate", 65535, 127)
            self.assertEqual(combined["actual"], separate["actual"])
            self.assertLess(combined["cycles"], separate["cycles"], variant)
            self.assertLess(
                combined["peak_stack_bytes"], separate["peak_stack_bytes"], variant
            )
            self.assertEqual(program.size("WRAM0"), 4)

    def test_frozen_baseline_defects(self):
        program = build("raw")
        result = run_case(program, "q", 1, 1, body=True)
        self.assertFalse(result["correct"])
        self.assertEqual(result["actual"], 256)
        with self.assertRaises(subprocess.CalledProcessError) as failure:
            build("raw-alt")
        self.assertIn("__gbc_udiv16.divide", failure.exception.stderr)

    def test_quotient_remainder_only_link_size(self):
        for variant, program in self.programs.items():
            self.assertEqual(
                program.size("ROM0", {"Audit division"})
                - quotient_remainder_size(variant),
                14,
                variant,
            )
        self.assertEqual(quotient_remainder_size("sdcc-z80"), 95)

    def test_input_distribution_is_reproducible(self):
        first = input_classes(10, 0)
        self.assertEqual(first, input_classes(10, 0))
        self.assertNotEqual(first["wide-random"], input_classes(10, 1)["wide-random"])
        self.assertTrue(all(lhs < rhs for lhs, rhs in first["less-than"]))
        self.assertTrue(all(0 < rhs < 128 for _, rhs in first["narrow-random"]))
        self.assertTrue(all(128 <= rhs <= 65535 for _, rhs in first["wide-random"]))


if __name__ == "__main__":
    unittest.main()
