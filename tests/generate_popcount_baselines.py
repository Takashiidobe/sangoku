import argparse
import hashlib
import json
import subprocess
from pathlib import Path

from runtime_harness import ROOT


def generate(directory, sdcc):
    directory = directory.resolve()
    directory.mkdir(parents=True, exist_ok=True)
    commands = []

    def run(name, command, *, stdin=None):
        result = subprocess.run(
            command, cwd=ROOT, input=stdin, capture_output=True, text=True
        )
        (directory / f"{name}.log").write_text(result.stdout + result.stderr)
        commands.append(command)
        result.check_returncode()
        return result.stdout

    run(
        "backend-build",
        [
            "cargo",
            "+gbc-lr35902",
            "build",
            "--release",
            "--locked",
            "--manifest-path",
            "third_party/rustc_codegen_cranelift/Cargo.toml",
        ],
    )
    rust_source = ROOT / "tests/fixtures/popcount/baseline.rs"
    backend = (
        ROOT
        / "third_party/rustc_codegen_cranelift/target/release/librustc_codegen_cranelift.so"
    )
    run(
        "rustc",
        [
            "rustc",
            "+gbc-lr35902",
            "-Zunstable-options",
            f"-Zcodegen-backend={backend}",
            "--target",
            str(ROOT / "targets/lr35902.json"),
            "--emit=asm",
            "--crate-type=lib",
            "-Cpanic=abort",
            "-Copt-level=2",
            str(rust_source),
            "-o",
            str(directory / "baseline.asm"),
        ],
    )
    c_source = ROOT / "tests/fixtures/popcount/sdcc-loop.c"
    preprocessed = f"#line 1 {json.dumps(str(c_source))}\n" + c_source.read_text()
    (directory / "sdcc-loop.i").write_text(preprocessed)
    run(
        "sdcc",
        [
            str(sdcc.resolve()),
            "--c1mode",
            "-msm83",
            "--std-c23",
            "--opt-code-speed",
            "-o",
            str(directory / "sdcc-native.s"),
        ],
        stdin=preprocessed,
    )
    metadata = {
        "commands": commands,
        "rustc_version": run("rustc-version", ["rustc", "+gbc-lr35902", "-vV"]),
        "sdcc_version": run("sdcc-version", [str(sdcc.resolve()), "--version"]),
        "sha256": {
            str(path.relative_to(ROOT))
            if path.is_relative_to(ROOT)
            else str(path): hashlib.sha256(path.read_bytes()).hexdigest()
            for path in (
                rust_source,
                c_source,
                backend,
                ROOT
                / "third_party/wasmtime/cranelift/codegen/src/isa/lr35902/lower.rs",
                ROOT
                / "third_party/wasmtime/cranelift/codegen/src/isa/lr35902/inst/emit.rs",
                ROOT
                / "third_party/wasmtime/cranelift/codegen/src/isa/lr35902/inst/regs.rs",
                ROOT / "third_party/wasmtime/cranelift/codegen/src/machinst/vcode.rs",
                directory / "baseline.asm",
                directory / "sdcc-native.s",
            )
        },
        "git_revisions": {
            name: run(
                name.replace("/", "-") + "-revision",
                ["git", "-C", str(ROOT / name), "rev-parse", "HEAD"],
            ).strip()
            for name in (
                "third_party/rust",
                "third_party/rustc_codegen_cranelift",
                "third_party/wasmtime",
            )
        },
    }
    (directory / "provenance.json").write_text(json.dumps(metadata, indent=2) + "\n")
    return metadata


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--output", type=Path, default=ROOT / "build/popcount-audit/regenerated"
    )
    parser.add_argument("--sdcc", type=Path, required=True)
    args = parser.parse_args()
    generate(args.output, args.sdcc)


if __name__ == "__main__":
    main()
