# Rust fork accommodations

`library/core` matches upstream base `2d20b42b883` byte for byte. There are no
remaining `target_arch = "z80"` gates in core, and no target-specific string,
character, formatting or numeric-method implementations.

## Removed restrictions

| Files in `library/core`                          | Restored behavior                                                                                  |
| ------------------------------------------------ | -------------------------------------------------------------------------------------------------- |
| `src/lib.rs`, `src/error.rs`                     | `random`, `time` and the floating-duration error implementation are available.                     |
| `src/fmt/num.rs`                                 | Integer display, debug, radix and exponent formatting use upstream implementations at every width. |
| `src/num/imp/fmt.rs`                             | Decimal parts and formatted-length saturation use upstream algorithms.                             |
| `src/num/{mod,int_macros,uint_macros}.rs`        | Powers, carrying multiplication and logarithm optimizations are available at every width.          |
| `src/num/{wrapping,saturating,nonzero}.rs`       | Wrapper methods no longer disappear on the target.                                                 |
| `src/num/niche_types.rs`, `src/mem/type_info.rs` | Debug formatting no longer returns an error for wide values.                                       |
| `src/panicking.rs`                               | Invalid-enum panics retain the source value in the upstream message.                               |
| `src/ptr/mod.rs`                                 | Alignment-offset inversion uses checked multiplication, including on 16-bit pointers.              |
| `src/intrinsics/mod.rs`                          | `is_val_statically_known` uses the upstream attributes and fallback body.                          |
| `Cargo.toml`                                     | The unused custom-architecture cfg allowance is removed.                                           |

The earlier UTF-8 restoration already removed the ASCII-only string, character,
escaping and Unicode-table replacements. The existing `alloctests` regression
for UTF-8 iteration remains in the fork.

## Remaining compiler accommodation

`compiler/rustc_target/src/callconv/mod.rs` selects the LR35902 C ABI classifier
in `callconv/z80.rs`. It passes aggregates indirectly and retains the target's
scalar ABI. This is compiler target support; it does not replace core behavior.
The target specification lives in `targets/lr35902.json` in sangoku.

Keep unsupported operations in the backend work queue rather than hiding APIs or
replacing core implementations. `sangoku-30k.5` tracks formatting execution
checks, `sangoku-30k.7` tracks broader 16-bit pointer assumptions, and
`sangoku-30k.9` tracks full target core/alloc build coverage.

## Verification

Rebuild the entire chain after changing core:

```sh
tools/rebuild_toolchain.sh
cargo nextest run --no-fail-fast
cargo run -- build examples/blank.rs build/blank.gb
```

`target_core_preserves_upstream_api_surface` compiles core metadata for the
actual 16-bit target, then checks formatting trait implementations, wide
arithmetic methods, wrapper methods, time and random APIs against that metadata.
This checks the target frontend and API surface; it does not establish that all
of core can generate and link into a ROM.

To check core drift against the upstream base:

```sh
git -C third_party/rust diff 2d20b42b883 -- library/core
```
