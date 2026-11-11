# Relocatable LR35902 objects

`sangoku-object` defines the shared artifact for the native ROM pipeline. The
host-side crate has no rustc or Cranelift dependencies. Sangoku re-exports it as
`sangoku::object`; compiler and runtime generators can depend directly on its
path under `crates/object`.

Run the format and validation regressions:

```bash
cargo nextest run --test object --no-fail-fast
```

The compiler emits this format for target object and archive output. Existing
standalone ROM builds still use the separate RGBDS assembly path. Native ROM
linking remains in the P0 `sangoku-61a` toolchain epic.

## Compiler artifacts

After building the [linked compiler](compiler-driver.md), emit an object, Rust
metadata, and a crate archive from the minimal fixture:

```bash
third_party/rustc_codegen_cranelift/target/release/sangoku-rustc \
  -Zunstable-options --target targets/lr35902.json \
  --crate-type=rlib --emit=obj,metadata,link -Cpanic=immediate-abort \
  tests/fixtures/driver/answer.rs --out-dir build
```

`build/answer.o` is a normalized object readable with `Object::decode`.
`build/libanswer.rlib` is a standard archive containing native codegen-unit
objects, `lib.rmeta`, and rustc's archive-member manifest. Its symbol index
lists externally visible definitions. Static libraries combine dependency
objects and omit Rust metadata. These archives contain sangoku objects, not ELF
objects; they require the native ROM linker rather than a system linker.

Cargo's default `-Cembed-metadata=no` puts a metadata stub in the archive and
full metadata in a separate `.rmeta`. Preserve both artifacts and pass both
through `--extern` when moving that dependency outside Cargo. With
`-Cembed-metadata=yes`, the archive embeds full metadata and can supply it
alone. The backend honors these rustc options rather than changing Cargo's
contract.

Test Cargo archive output, reuse, copied dependency artifacts, generics,
codegen-unit preservation, and code/data relocations:

```bash
cargo nextest run --test archives --no-fail-fast
```

Native emission currently rejects global/inline/naked assembly, TLS, custom
sections, and `#[used]` statics. Staticlib symbol hiding/renaming is also
unsupported. Normal `no_std` Cargo projects still need a matching native target
sysroot; the archive regression supplies minimal language items with `no_core`.

## Object contents

An `Object` contains ordered vectors of sections, symbols, and relocations.
`SectionId` and `SymbolId` are zero-based indices scoped to one object. Names
are nonempty UTF-8 strings without NUL characters.

A section carries initialized bytes, total allocation size, alignment, a memory
region, and an optional fixed CPU address. The size includes any zero-fill tail;
relocations can patch only initialized bytes. Initialized RAM data remains in
the object so a linker can construct startup copy/zero tables. Loading an object
does not initialize cartridge RAM.

| Region  | CPU address range, end exclusive |
| ------- | -------------------------------- |
| `Rom0`  | `$0000..$4000`                   |
| `RomX`  | `$4000..$8000`                   |
| `Wram0` | `$C000..$D000`                   |
| `Hram`  | `$FF80..$FFFF`                   |

Version 1 describes the existing fixed 32 KiB cartridge: `RomX` is its second 16
KiB region. It does not describe MBC bank constraints. CPU addresses and
eventual ROM file offsets are distinct concepts; the object stores no final file
offset. The interrupt-enable register at `$FFFF` is not HRAM storage.

Alignment is a nonzero power of two no larger than the region. Fixed sections
must start inside the region, fit entirely, and satisfy alignment. Placement
overlaps and reserved header/vector/stack space are linker responsibilities.

Symbols have local, global, or weak binding. A definition is an imported symbol,
a section plus offset, or an absolute 16-bit CPU address. Section-end symbols
are allowed. Local symbols may share a display name because references use IDs.
Global and weak names must be unique within an object; cross-object resolution
and weak-definition selection belong to the linker. Undefined local symbols are
rejected.

## Relocations

Each relocation identifies a source section and byte offset, a target symbol ID,
a signed 64-bit addend, and a kind. Addends retain Cranelift's width; final
address overflow is a linker error. Nonoverlapping adjacent patches are allowed.

The linker contract uses `S` for the resolved CPU address, `A` for the addend,
and `P` for the CPU address of the first patched byte:

| Kind         | Patch                                                  |
| ------------ | ------------------------------------------------------ |
| `Absolute16` | `S + A`, range `$0000..$FFFF`, two little-endian bytes |
| `Absolute8`  | `S + A`, range `$00..$FF`                              |
| `Relative8`  | `S + A - (P + 1)`, signed range `-128..127`            |
| `Low8`       | low byte of a valid 16-bit `S + A`                     |
| `High8`      | high byte of a valid 16-bit `S + A`                    |
| `HighPage8`  | `S + A - $FF00`, range `$00..$FF`                      |

The format validates patch widths and references. It does not resolve symbols,
apply these formulas, or decide whether a cross-bank reference is callable.

## Serialization

`Object::encode` validates an object before writing it. `Object::decode` checks
the header and payload, then validates the reconstructed object.

The header is the eight bytes `SGOBJ\0\r\n` followed by a little-endian `u16`
version, currently 1. The remaining bytes are UTF-8 JSON containing the three
ordered vectors. Encoding preserves their order and IDs. Unsupported versions,
unknown fields or enum values, trailing non-whitespace content, invalid
placement, invalid references, and overlapping relocation patches are errors.
The serializer does not package Rust crate metadata. The compiler's archive
builder packages code and metadata as separate members; `sangoku-61a.2` tracks
the remaining Cargo integration and executable ROM output.
