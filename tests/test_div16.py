import unittest

from audit_division import build, input_classes, run_case


class Div16ExecutionTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.program = build("runtime")

    def test_public_entries_against_host_divmod(self):
        for label, pairs in input_classes(128, seed=2).items():
            for lhs, rhs in pairs:
                for operation in ("q", "r"):
                    for body in (True, False):
                        result = run_case(self.program, operation, lhs, rhs, body=body)
                        self.assertTrue(
                            result["correct"],
                            (label, operation, lhs, rhs, body, result),
                        )
                        self.assertEqual(result["peak_stack_bytes"], 6)
                result = run_case(self.program, "separate", lhs, rhs)
                self.assertTrue(result["correct"], (label, lhs, rhs, result))
                self.assertEqual(result["peak_stack_bytes"], 8)

    def test_entries_ignore_scratch_state(self):
        for poison in (0, 0xA5, 255):
            for lhs, rhs in (
                (1, 1),
                (0, 65535),
                (65535, 127),
                (65535, 128),
                (65535, 32768),
            ):
                for operation in ("q", "r"):
                    result = run_case(
                        self.program, operation, lhs, rhs, body=True, poison=poison
                    )
                    self.assertTrue(
                        result["correct"], (poison, lhs, rhs, operation, result)
                    )

    def test_zero_divisors_reach_rst_vector(self):
        for lhs in (0, 1, 32768, 65535):
            for operation in ("q", "r"):
                for body in (True, False):
                    result = run_case(self.program, operation, lhs, 0, body=body)
                    self.assertTrue(result["correct"])
                    self.assertEqual(result["status"], "rst_10")
                    self.assertEqual(result["peak_stack_bytes"], 8)

    def test_runtime_costs(self):
        self.assertEqual(self.program.size("ROM0", {"Audit division"}), 95)
        self.assertNotIn("audit_both", self.program.symbols)
        for operation, cycles in (("q", 1236), ("r", 1240)):
            result = run_case(self.program, operation, 1, 1)
            self.assertTrue(result["correct"])
            self.assertEqual(result["cycles"], cycles)


if __name__ == "__main__":
    unittest.main()
