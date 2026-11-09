#!/usr/bin/env python3
"""Measure bounded upstream formatting probes without changing core."""
import argparse
import json
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--types', nargs='+', choices=['u64', 'u128', 'f32', 'f64'],
                        default=['u64', 'u128', 'f32', 'f64'])
    parser.add_argument('--output', type=Path, default=ROOT / 'build/formatting-audit')
    args = parser.parse_args()
    directory = args.output.resolve()
    directory.mkdir(parents=True, exist_ok=True)

    def run(command, log):
        result = subprocess.run(command, cwd=ROOT, capture_output=True, text=True)
        (directory / log).write_text(result.stdout + result.stderr)
        return result

    backend = ROOT / 'third_party/rustc_codegen_cranelift/target/release/librustc_codegen_cranelift.so'
    build = run(['cargo', '+gbc-lr35902', 'build', '--release', '--manifest-path',
                 'third_party/rustc_codegen_cranelift/Cargo.toml'], 'backend.log')
    if build.returncode:
        raise SystemExit('backend build failed; see backend.log')
    common = ['rustup', 'run', 'gbc-lr35902', 'rustc', '--edition=2024', '--crate-type=lib',
              '--target', str(ROOT / 'targets/lr35902.json'), '-Zunstable-options',
              '-Cpanic=immediate-abort', '-Copt-level=z', '-Ccodegen-units=1',
              '-Zmir-opt-level=3', '-Zinline-mir=yes', '-Zcross-crate-inline-threshold=always']
    metadata = directory / 'libcore.rmeta'
    core = run(common + ['--crate-name', 'core', '-Zalways-encode-mir', '--cfg',
                         'feature="optimize_for_size"', '--emit=metadata',
                         str(ROOT / 'third_party/rust/library/core/src/lib.rs'),
                         '-o', str(metadata)], 'core.log')
    if core.returncode:
        raise SystemExit('core metadata compilation failed; see core.log')
    template = (ROOT / 'tests/fixtures/core/format-u64.rs').read_text()
    stop = directory / 'stop.asm'
    stop.write_text('SECTION "reserved", ROM0[$0000]\nds $100, 0\ndone::\nnop\n')
    report = {}
    for ty in args.types:
        source, assembly, obj = [directory / f'format-{ty}.{ext}' for ext in ('rs', 'asm', 'o')]
        source.write_text(template.replace('u64', ty))
        compiled = run(common + ['--crate-name', f'format_{ty}', '--extern', f'core={metadata}',
                                 f'-Zcodegen-backend={backend}', '--emit=asm', str(source),
                                 '-o', str(assembly)], f'{ty}-compile.log')
        entry = dict(rust_to_assembly=compiled.returncode == 0)
        report[ty] = entry
        if compiled.returncode:
            entry['error'] = compiled.stderr.splitlines()[0]
            continue
        text = assembly.read_text()
        entry['assembly_text_bytes'] = assembly.stat().st_size
        assembled = run(['rgbasm', '-o', str(obj), str(assembly)], f'{ty}-assemble.log')
        entry['assembles'] = assembled.returncode == 0
        if assembled.returncode:
            entry['error'] = assembled.stderr.strip()
            size = re.search(r'reached 0x([0-9A-Fa-f]+)', assembled.stderr)
            if size:
                entry['code_section_bytes'] = int(size[1], 16)
            continue
        names = ['integer/all', 'integer/udiv/common/i16', 'integer/urem/common/i16', 'integer/div/common/i16_core', 'integer/mul/common/i16', 'memory/memcpy/speed/pointer', 'memory/memmove/speed/pointer', 'memory/memset/speed/pointer']
        if '__gbc_f64' in text:
            names.append('softfloat64')
        if '__gbc_f128' in text:
            names.append('softfloat128')
        if '__gbc_f' in text.replace('__gbc_f64', '').replace('__gbc_f128', ''):
            names.append('softfloat32')
        if '__gbc_convert' in text:
            names.append('convert')
        objects = [str(obj)]
        for name, path in [('stop', stop)] + [(name, ROOT / f'runtime/{name}.asm') for name in names]:
            output = directory / f'{name}.o'
            result = run(['rgbasm', '-o', str(output), str(path)], f'{name}-assemble.log')
            if result.returncode:
                raise SystemExit(f'{name} assembly failed')
            objects.append(str(output))
        linked = run(['rgblink', '-t', '-o', str(directory / f'{ty}.gb'), '-m',
                      str(directory / f'{ty}.map')] + objects, f'{ty}-link.log')
        entry['links_in_32k'] = linked.returncode == 0
        if linked.returncode:
            entry['error'] = linked.stderr.strip()
        else:
            entry['map_summary'] = (directory / f'{ty}.map').read_text().splitlines()[1].strip()
    output = json.dumps(report, indent=2) + '\n'
    (directory / 'report.json').write_text(output)
    print(output, end='')


if __name__ == '__main__':
    main()
