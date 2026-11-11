# Linked compiler driver

`sangoku-rustc` uses the pinned Rust fork with the LR35902 Cranelift backend
linked into the executable. Target compilation needs no backend shared library
or `-Zcodegen-backend` option. Host compilation uses rustc's default backend so
Cargo can build and run build scripts and procedural macros.

Build the driver and the compatibility plugin:

```bash
cargo +gbc-lr35902 build --release --locked \
  --manifest-path third_party/rustc_codegen_cranelift/Cargo.toml
```

Compile a minimal fixture directly to RGBDS assembly:

```bash
third_party/rustc_codegen_cranelift/target/release/sangoku-rustc \
  -Zunstable-options --target targets/lr35902.json \
  --crate-type=lib --emit=asm -Cpanic=immediate-abort \
  tests/fixtures/driver/answer.rs -o build/answer.asm
```

This fixture supplies its own language items. Normal `no_std` sources also need
matching target `core` metadata and their dependencies. Use
`cargo run -- build examples/hello.rs build/hello.gb` for the existing ROM build
path, which prepares those libraries and invokes RGBDS.

`tools/rebuild_toolchain.sh` also installs `sangoku-rustc` into the pinned
toolchain's `bin` directory. It leaves that toolchain's `rustc` executable in
place. The bundled distribution packages the linked driver as its compiler.

## Cargo integration

Set Cargo's `RUSTC` environment variable to the absolute path of
`sangoku-rustc`. JSON targets currently require Cargo's `-Zjson-target-spec`
flag and rustc's `-Zunstable-options` flag. Prefer target-specific flags so host
tools retain their normal compilation settings.

The regression test checks a custom-target library from outside the checkout,
including a host build script and host procedural macro:

```bash
cargo nextest run --test compiler_driver --no-fail-fast
```

This verifies target probing, metadata checking, direct assembly emission, and
host tools. It does not establish that `cargo build --target` can produce a ROM.
The target still disables executables. Cargo needs target crate artifacts that
preserve metadata and generated code across dependencies, target library
support, and a final ROM link stage using runtime selection, section placement,
and RGBDS. `sangoku-61a.2` tracks artifacts and ROM linking; `sangoku-61a`
tracks the complete Cargo workflow.

## Distribution limits

The backend is a Rust library linked into the driver. A small compatibility
plugin still produces `librustc_codegen_cranelift.so` for existing tests and
tools that explicitly load a backend. The driver does not depend on it.

The driver still links dynamically to the pinned `librustc_driver` and its
dependencies. Linux builds embed library search paths for the build toolchain
and an adjacent `../lib` directory. A copied driver can use the original
toolchain while it remains installed; distributing it alone does not supply that
toolchain. Use the [single-file bundle](../distribution.md) to package the
compiler libraries, target metadata, runtime assets, and host procedural macros.
