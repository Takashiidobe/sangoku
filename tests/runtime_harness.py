import hashlib
import re
import subprocess
import tempfile
from dataclasses import dataclass
from pathlib import Path

from emu import Emulator

ROOT = Path(__file__).resolve().parents[1]


@dataclass(frozen=True)
class Section:
    name: str
    area: str
    address: int
    size: int


@dataclass
class Program:
    rom: bytes
    symbols: dict
    sections: dict
    provenance: dict

    def emulator(self, entry):
        return Emulator(self.rom, pc=self.symbols[entry])

    def size(self, area, names=None):
        return sum(
            section.size
            for section in self.sections.values()
            if section.area == area and (names is None or section.name in names)
        )


def assemble(sources, *, artifacts=None):
    if artifacts is None:
        with tempfile.TemporaryDirectory() as directory:
            program = assemble(sources, artifacts=directory)
            program.provenance["artifacts"] = None
            return program
    directory = Path(artifacts).resolve()
    directory.mkdir(parents=True, exist_ok=True)
    commands = []
    objects = []
    for index, (name, source) in enumerate(sources.items()):
        if Path(name).name != name:
            raise ValueError("assembly source names must be plain filenames")
        path = directory / name
        path.write_text(source)
        obj = directory / f"{index}.o"
        command = ["rgbasm", "-I", str(ROOT) + "/", "-o", str(obj), str(path)]
        subprocess.run(command, check=True, text=True)
        commands.append(command)
        objects.append(str(obj))
    command = [
        "rgblink",
        "-o",
        str(directory / "runtime.gb"),
        "-n",
        str(directory / "runtime.sym"),
        "-m",
        str(directory / "runtime.map"),
        *objects,
    ]
    subprocess.run(command, check=True, text=True)
    commands.append(command)
    symbols = {}
    for line in (directory / "runtime.sym").read_text().splitlines():
        match = re.fullmatch(r"([0-9a-fA-F]+):([0-9a-fA-F]+) (.+)", line)
        if match:
            bank, address, name = match.groups()
            if int(bank, 16) != 0:
                raise ValueError(
                    "banked symbols are unsupported by the flat-memory harness"
                )
            symbols[name] = int(address, 16)
    sections = {}
    area = None
    for line in (directory / "runtime.map").read_text().splitlines():
        heading = re.fullmatch(r"(\w+) bank #(\d+):", line)
        if heading:
            area = heading[1]
            if area not in ("ROM0", "WRAM0"):
                raise ValueError(f"unsupported memory region {area}")
        match = re.search(
            r"SECTION: \$([0-9a-fA-F]+)(?:-\$[0-9a-fA-F]+)? "
            r'\(\$([0-9a-fA-F]+) bytes?\) \["(.+)"\]',
            line,
        )
        if match:
            address, size, name = match.groups()
            if name in sections:
                raise ValueError(f"duplicate section name {name}")
            sections[name] = Section(name, area, int(address, 16), int(size, 16))
    if not sections:
        raise ValueError("linker map contains no recognized sections")
    provenance = {
        "source_sha256": {
            name: hashlib.sha256(source.encode()).hexdigest()
            for name, source in sources.items()
        },
        "tools": {
            tool: subprocess.check_output([tool, "--version"], text=True).strip()
            for tool in ("rgbasm", "rgblink")
        },
        "commands": commands,
        "artifacts": str(directory) if Path(artifacts).is_dir() else None,
    }
    return Program(
        (directory / "runtime.gb").read_bytes(), symbols, sections, provenance
    )
