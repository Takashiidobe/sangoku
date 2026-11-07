#!/usr/bin/env python3
"""Smoke-test a bundle without access to the checkout, Cargo, or rustup."""

import argparse
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parent.parent


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("binary", type=Path)
    parser.add_argument("--levels", nargs="+", default=["0", "z"])
    args = parser.parse_args()
    bwrap = shutil.which("bwrap")
    if bwrap is None:
        parser.error("this isolation test requires bubblewrap (bwrap)")
    with tempfile.TemporaryDirectory(prefix="sangoku-relocation-") as temporary:
        work = Path(temporary)
        (work / "bin").mkdir()
        (work / "home").mkdir()
        for name in ["rgbasm", "rgblink", "rgbfix"]:
            executable = shutil.which(name)
            if executable is None:
                parser.error(f"{name} must be installed")
            (work / "bin" / name).symlink_to(executable)
        shutil.copy2(args.binary, work / "sangoku")
        for name in ["hello", "primes", "sort"]:
            shutil.copy2(ROOT / "examples" / f"{name}.rs", work / f"{name}.rs")
        shutil.copy2(ROOT / "tests/fixtures/lower/add16.clif", work / "add.clif")
        environment = {
            "PATH": "/tmp/work/bin",
            "HOME": "/tmp/work/home",
            "SANGOKU_CACHE_DIR": "cache with spaces",
            "CARGO_HOME": "/tmp/work/no-cargo",
            "RUSTUP_HOME": "/tmp/work/no-rustup",
            "LC_ALL": "C",
        }
        prefix = [bwrap, "--die-with-parent", "--ro-bind", "/", "/", "--tmpfs", str(ROOT),
                  "--tmpfs", "/tmp", "--bind", str(work), "/tmp/work", "--chdir", "/tmp/work",
                  "--proc", "/proc", "--dev", "/dev", "/tmp/work/sangoku"]
        invocations = 0

        def invoke(arguments):
            nonlocal invocations
            output = subprocess.run(prefix + arguments, env=environment, text=True, capture_output=True)
            if output.returncode:
                raise RuntimeError(f"{arguments}:\n{output.stdout}\n{output.stderr}")
            extracted = "Unpacking compiler toolchain" in output.stderr
            if extracted != (invocations == 0):
                raise RuntimeError("expected first-use extraction followed by cache reuse")
            invocations += 1
            print("PASS", " ".join(arguments), flush=True)
            return output

        expected = {
            "hello": "Hello, Game Boy!\n6 * 7 = 42\n",
            "primes": "2 3 5 7 11 13 17 19 23 29 31 37 41 43 47 \n15 primes below 50\n",
            "sort": "before: [42, 7, 1999, 3, 256, 88, 1024, 5]\nafter:  [3, 5, 7, 42, 88, 256, 1024, 1999]\nsum = 3424, max = Some(1999)\n",
        }
        for name, text in expected.items():
            output = invoke(["run", f"{name}.rs"])
            if not output.stdout.endswith(text):
                raise RuntimeError(f"incorrect {name} output: {output.stdout!r}")
        for level in args.levels:
            invoke(["build", "hello.rs", f"build/hello-{level}.gb", "--opt-level", level])
        invoke(["example", "build/example.gb"])
        invoke(["frontend", "hello.rs", "build/hello.mir"])
        invoke(["clif", "hello.rs", "build/hello.clif"])
        invoke(["lower", "add.clif", "build/add.asm"])
        if not (work / "build/artifacts").is_dir():
            raise RuntimeError("artifacts were not saved in the caller's working directory")
        print(f"Passed {invocations} isolated invocations; checkout hidden, only RGBDS on PATH.")


if __name__ == "__main__":
    main()
