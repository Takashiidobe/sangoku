#!/usr/bin/env python3
"""Build a Linux x86-64 single-file sangoku distribution."""

import argparse
import gzip
import hashlib
import os
from pathlib import Path
import re
import shutil
import struct
import subprocess
import tarfile
import tempfile

ROOT = Path(__file__).resolve().parent.parent
MAGIC = b"SANGOKU-BUNDLE1!"
HOST = "x86_64-unknown-linux-gnu"
LAUNCHER = "x86_64-unknown-linux-musl"


def run(args, **kwargs):
    print("+", " ".join(map(str, args)), flush=True)
    subprocess.run(list(map(str, args)), cwd=ROOT, check=True, **kwargs)


def copy(source, destination):
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(source.resolve(), destination)


def shared_libraries(binary, destination, driver_directory):
    environment = dict(os.environ, LD_LIBRARY_PATH=str(driver_directory))
    output = subprocess.check_output(["ldd", str(binary)], env=environment, text=True)
    if "not found" in output:
        raise RuntimeError(f"unresolved dependencies for {binary}:\n{output}")
    for line in output.splitlines():
        match = re.match(r"\s*(\S+) => (/\S+)", line)
        if match:
            name, source = match.groups()
            copy(Path(source), destination / Path(name).name)
        else:
            match = re.match(r"\s*(/\S+) \(", line)
            if match:
                source = Path(match[1])
                copy(source, destination / source.name)


def archive_metadata(info):
    info.uid = info.gid = 0
    info.uname = info.gname = ""
    info.mtime = 0
    return info


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=ROOT / "build/dist/sangoku")
    parser.add_argument("--levels", nargs="+", choices=["0", "1", "2", "3", "s", "z"],
                        default=["0", "1", "2", "3", "s", "z"])
    args = parser.parse_args()
    if os.uname().sysname != "Linux" or os.uname().machine != "x86_64":
        parser.error("this prototype requires a Linux x86-64 build host")
    output = args.output.resolve()
    run(["cargo", "build", "--release", "--bin", "sangoku"])
    preparation = ROOT / "build/bundle-prepare.rs"
    preparation.parent.mkdir(parents=True, exist_ok=True)
    preparation.write_text("#![no_std]\n#[unsafe(no_mangle)]\npub fn gbc_main() -> u8 { 0 }\n")
    for level in args.levels:
        run([ROOT / "target/release/sangoku", "build", preparation,
             ROOT / "build/bundle-prepare.gb", "--opt-level", level])
    run(["cargo", "build", "--release", "-p", "sangoku-lower"])
    run(["cargo", "build", "--release", "--target", LAUNCHER, "--bin", "sangoku"])
    compiler = ROOT / "third_party/rust/build" / HOST / "stage2"
    backend = ROOT / "third_party/rustc_codegen_cranelift/target/release/librustc_codegen_cranelift.so"
    output.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="sangoku-package-") as temporary:
        staging = Path(temporary) / "payload"
        staging.mkdir()
        (staging / "bundle-version").write_text("1\n")
        copy(compiler / "bin/rustc", staging / "toolchain/bin/rustc")
        copy(ROOT / "target/release/sangoku-lower", staging / "toolchain/bin/sangoku-lower")
        copy(backend, staging / "backend" / backend.name)
        for executable in [compiler / "bin/rustc", backend, ROOT / "target/release/sangoku-lower"]:
            shared_libraries(executable, staging / "toolchain/lib", compiler / "lib")
        for level in args.levels:
            source = ROOT / "build/core/immediate-abort" / f"opt-{level}"
            for name in ["core.s", "compiler_builtins.s", "libcore.rmeta", "libcompiler_builtins.rmeta"]:
                copy(source / name, staging / "libraries" / f"opt-{level}" / name)
        for directory in ["runtime", "targets", "rom", "examples"]:
            for source in (ROOT / directory).iterdir():
                if source.is_file() and source.suffix in {".asm", ".rs", ".json"}:
                    copy(source, staging / directory / source.name)
        for source in [ROOT / "runtime/COPYING", ROOT / "runtime/NOTICE",
                       ROOT / "third_party/rust/LICENSE-APACHE", ROOT / "third_party/rust/LICENSE-MIT",
                       ROOT / "third_party/rust/COPYRIGHT"]:
            copy(source, staging / "licenses" / source.parent.name / source.name)
        archive = Path(temporary) / "payload.tar.gz"
        with archive.open("wb") as stream:
            with gzip.GzipFile(fileobj=stream, mode="wb", mtime=0) as compressed:
                with tarfile.open(fileobj=compressed, mode="w|") as package:
                    for source in sorted(staging.iterdir()):
                        package.add(source, arcname=source.name, filter=archive_metadata)
        candidate = output.with_name(output.name + ".tmp")
        try:
            digest = hashlib.sha256()
            with candidate.open("wb") as stream:
                with (ROOT / "target" / LAUNCHER / "release/sangoku").open("rb") as launcher:
                    shutil.copyfileobj(launcher, stream)
                with archive.open("rb") as compressed:
                    while chunk := compressed.read(1024 * 1024):
                        stream.write(chunk)
                        digest.update(chunk)
                stream.write(struct.pack("<Q", archive.stat().st_size))
                stream.write(digest.digest())
                stream.write(MAGIC)
            candidate.chmod(0o755)
            candidate.replace(output)
        finally:
            candidate.unlink(missing_ok=True)
    print(f"Wrote {output} ({output.stat().st_size:,} bytes); RGBDS is external.")


if __name__ == "__main__":
    main()
