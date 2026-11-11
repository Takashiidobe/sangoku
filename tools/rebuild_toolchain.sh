#!/usr/bin/env bash
# rebuild the rust fork, relink gbc-lr35902 and rebuild the cranelift backend
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd)
host=x86_64-unknown-linux-gnu
build=$root/third_party/rust/build/$host
lib=lib/rustlib/$host/lib

"$root/tools/rust_fork.sh" check

(cd "$root/third_party/rust" && ./x.py build --stage 2 compiler library --set build.description=lr35902)

# bootstrap links the rustc_private rlibs into the stage1 sysroot only
for file in "$build/stage1/$lib"/*; do
    [ -e "$build/stage2/$lib/$(basename "$file")" ] || cp "$file" "$build/stage2/$lib/"
done

rustup toolchain link gbc-lr35902 "$build/stage2"
# rustc -vV is constant across rebuilds (it feeds mangled-name hashes), so cargo cannot detect stale artifacts itself
cargo +gbc-lr35902 clean --manifest-path "$root/third_party/wasmtime/cranelift/codegen/Cargo.toml"
cargo +gbc-lr35902 clean --manifest-path "$root/third_party/rustc_codegen_cranelift/Cargo.toml"
cargo +gbc-lr35902 build --release --manifest-path "$root/third_party/rustc_codegen_cranelift/Cargo.toml"
install -m755 "$root/third_party/rustc_codegen_cranelift/target/release/sangoku-rustc" "$build/stage2/bin/sangoku-rustc"

"$root/tools/rust_fork.sh" core
