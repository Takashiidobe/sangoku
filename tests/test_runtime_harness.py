import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

from audit_runtime import execute, main, measure
from unittest.mock import patch
from runtime_harness import ROOT, assemble


class RuntimeHarnessTest(unittest.TestCase):
    def test_map_sizes_include_zero_table_bytes_and_exclude_padding(self):
        program = assemble(
            {
                "sizes.asm": """SECTION "code", ROM0[$100]
entry::
    ret
SECTION "table", ROM0[$400]
    db 0, 0
SECTION "scratch", WRAM0[$c000]
    ds 7
"""
            }
        )
        self.assertEqual(program.symbols["entry"], 0x100)
        self.assertEqual(program.size("ROM0"), 3)
        self.assertEqual(program.size("ROM0", {"table"}), 2)
        self.assertEqual(program.size("WRAM0"), 7)
        self.assertIsNone(program.provenance["artifacts"])

    def test_mul16_body_and_complete_caller(self):
        report = measure("mul16", 1, 1)
        self.assertEqual(
            report["sizes"],
            {
                "helper_code_bytes": 30,
                "table_rom_bytes": 0,
                "caller_code_bytes": 18,
                "scratch_ram_bytes": 0,
            },
        )
        for mode, cycles, entry_stack, peak_extra in [
            ("body", 484, 4, 0),
            ("caller", 592, 0, 4),
        ]:
            result = report["measurements"][mode]
            self.assertTrue(result["correct"])
            self.assertEqual(result["cycles"], cycles)
            self.assertEqual(result["entry_stack_bytes"], entry_stack)
            self.assertEqual(result["peak_additional_stack_bytes"], peak_extra)
            self.assertEqual(result["peak_stack_bytes"], 4)
            self.assertEqual(result["scratch_written_bytes"], 0)

    def test_nested_stack_fixture_and_scratch_accounting(self):
        report = measure("stack-fixture", 1, 1)
        self.assertEqual(
            report["sizes"],
            {
                "helper_code_bytes": 24,
                "table_rom_bytes": 2,
                "caller_code_bytes": 18,
                "scratch_ram_bytes": 4,
            },
        )
        for mode, cycles, entry_stack, peak_extra in [
            ("body", 180, 4, 6),
            ("caller", 288, 0, 10),
        ]:
            result = report["measurements"][mode]
            self.assertTrue(result["correct"])
            self.assertEqual(result["cycles"], cycles)
            self.assertEqual(result["entry_stack_bytes"], entry_stack)
            self.assertEqual(result["peak_additional_stack_bytes"], peak_extra)
            self.assertEqual(result["peak_stack_bytes"], 10)
            self.assertEqual(result["scratch_written_bytes"], 1)
            self.assertEqual(result["registers"]["a"], 42)

    def test_division_zero_trap_report(self):
        report = measure("udiv16", 1, 0)
        for result in report["measurements"].values():
            self.assertTrue(result["correct"])
            self.assertEqual(result["status"], "rst_10")
            self.assertEqual(result["peak_stack_bytes"], 8)

    def test_retained_artifacts_and_incorrect_stack(self):
        with tempfile.TemporaryDirectory() as directory:
            directory = Path(directory)
            result = subprocess.run(
                [
                    sys.executable,
                    "-B",
                    str(ROOT / "tests/audit_runtime.py"),
                    "--helper",
                    "mul16",
                    "--lhs",
                    "1",
                    "--rhs",
                    "1",
                    "--output",
                    str(directory / "report.json"),
                    "--artifacts",
                    str(directory / "artifacts"),
                ],
                check=True,
            )
            self.assertEqual(result.returncode, 0)
            report = json.loads((directory / "report.json").read_text())["reports"][0]
            artifacts = Path(report["provenance"]["artifacts"])
            for name in (
                "helper.asm",
                "caller.asm",
                "runtime.gb",
                "runtime.map",
                "runtime.sym",
            ):
                self.assertTrue((artifacts / name).is_file(), name)
            self.assertEqual(
                set(report["provenance"]["source_sha256"]), {"helper.asm", "caller.asm"}
            )
            program = assemble(
                {
                    "incorrect.asm": """SECTION "Audit helper", ROM0[$200]
bad::
    inc sp
    ret
SECTION "Audit caller", ROM0[$100]
audit_done::
    nop
"""
                }
            )
            result = execute(program, "bad", lhs=1, rhs=1, body=True)
            self.assertFalse(result["stack_restored"])

    def test_cli_fails_on_incorrect_output(self):
        def corrupt(*args, **kwargs):
            result = execute(*args, **kwargs)
            result["registers"]["a"] ^= 1
            return result

        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory) / "report.json"
            with (
                patch("audit_runtime.execute", side_effect=corrupt),
                patch.object(
                    sys,
                    "argv",
                    [
                        "audit_runtime.py",
                        "--helper",
                        "mul16",
                        "--lhs",
                        "1",
                        "--rhs",
                        "1",
                        "--output",
                        str(output),
                    ],
                ),
            ):
                self.assertEqual(main(), 1)
            report = json.loads(output.read_text())["reports"][0]
            self.assertTrue(
                all(not result["correct"] for result in report["measurements"].values())
            )


if __name__ == "__main__":
    unittest.main()
