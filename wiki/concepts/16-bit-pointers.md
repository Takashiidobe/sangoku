# 16-bit pointers and sizes

`usize`, `isize` and pointers are 16 bits on LR35902. The upstream core and
alloc sources type-check for this target without pointer-width workarounds. No
Rust-fork source changes were needed for this audit.

## Size and allocation boundaries

| Quantity                      | Target limit                                            |
| ----------------------------- | ------------------------------------------------------- |
| `usize::MAX`                  | 65,535                                                  |
| `isize::MAX`                  | 32,767                                                  |
| Ordinary object size          | At most 32,767 bytes                                    |
| `[u8; N]` layout              | At most 32,767 elements                                 |
| `[u16; N]` layout             | At most 16,383 elements, or 32,766 bytes                |
| Zero-sized array/slice length | Can reach 65,535 without allocating element storage     |
| `Layout` size                 | Must still fit `isize::MAX` after rounding to alignment |

The larger unsigned limit does not permit a 65,535-byte allocation. For example,
`Layout::from_size_align(32767, 2)` fails because rounding would require 32,768
bytes. `Layout::array::<u16>(16384)` also fails. These are upstream bounds, not
GBC-specific exclusions.

`Vec<T>` uses the layout and checked-capacity paths for ordinary elements. An
empty `Vec<()>` reports capacity `usize::MAX` because its elements consume no
storage; an empty `Vec<u8>` reports zero capacity. Neither result promises that
an allocator or sufficient physical RAM exists.

Slices carry a two-byte pointer and a two-byte length. The compiler's object
size checks, `Layout` and slice safety requirements still apply to actual
backing storage. The regression constructs the maximum-length zero-sized slice,
without fabricating a reference to nonexistent byte storage.

## Arithmetic, ranges and hashing

Pointer-sized integer arithmetic uses the existing i16 lowering. Bounded
Rust-to-emulator tests cover wraparound at 32,768 and 65,536, plus wrapping
pointer offsets, at none/speed optimization. Wrapping a raw address does not
make the resulting address valid to dereference.

Core's `Step` implementation returns `(usize::MAX, None)` when a range's length
cannot fit in `usize`. A range ending at 66,000 therefore has an unknown upper
size bound on this target; an exclusive u16 range of length 65,535 fits.

The upstream source explicitly retains historical `ExactSizeIterator`
implementations for some wider ranges and full-width inclusive ranges. Do not
rely on `.len()` for a range longer than 65,535 elements: the exact-length
contract cannot be represented and the default implementation can panic. This
audit preserves upstream behavior rather than introducing a fork-only API
change. See `library/core/src/iter/range.rs` in the Rust checkout.

`Hasher::write_usize` writes the native-width byte representation: two bytes on
this little-endian target. Hashing a `usize` need not produce the same result as
hashing it on a 64-bit host. SipHash's mixing state remains u64; that is an
operation-cost question, not a requirement that `usize` itself be 32 bits.

## Verification scope

Run the bounded checks with the pinned toolchain installed:

```sh
cargo nextest run --no-fail-fast -E 'test(target_core_and_alloc_respect_16_bit_size_limits) | test(rust_usize_and_pointer_offsets_use_16_bits)'
```

The frontend regression builds target metadata for core, compiler-builtins and
alloc, then checks layout, const evaluation, range bounds and collection API
fixtures. Negative fixtures verify rejection of an oversized object and a
65,536-valued `usize` literal. The emulator regression exercises real Rust
`usize` arithmetic and raw pointer offsets.

Metadata compilation does not establish complete target machine-code emission,
core/alloc linking, heap allocation or collection execution on hardware.
`sangoku-30k.9` retains that build/integration work. See
[practical limits](practical-limits.md) for ROM and runtime costs.
