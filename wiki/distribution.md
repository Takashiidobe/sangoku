# Single-file distribution

Build one Linux x86-64 executable containing the pinned compiler, backend,
shared libraries, runtime assembly, and precompiled target libraries. RGBDS
remains an external requirement.

From an already built checkout:

```bash
rustup target add x86_64-unknown-linux-musl
python3 tools/bundle_toolchain.py
```

The output is `build/dist/sangoku`. Copy that file to another directory or
machine with RGBDS installed, then use it directly:

```bash
./sangoku run game.rs
./sangoku build game.rs game.gb --opt-level z
```

All six optimization levels are included in the same executable. Use
`--levels 0` when packaging a smaller prototype with only the default target
libraries.

The launcher is statically linked against musl. Its appended gzip archive
contains the dynamically linked compiler, backend, loader, and their shared
libraries. On first use, the launcher verifies the archive checksum and extracts
it into a cache; later invocations reuse that directory. The compiler and
libraries are not statically linked into the launcher.

The cache defaults to `$XDG_CACHE_HOME/sangoku` or `~/.cache/sangoku`. Override
it with `SANGOKU_CACHE_DIR`. Build artifacts stay in the caller's
`build/artifacts/` directory. Normal compilation uses no Cargo, rustup, or
repository checkout.

Run the relocation smoke test with bubblewrap installed:

```bash
python3 tools/test_bundle.py build/dist/sangoku
```

The test hides the checkout, exposes only installed RGBDS tools on `PATH`, uses
an isolated home/cache, and checks examples, optimization levels, MIR, CLIF,
assembly lowering, and cache reuse.
