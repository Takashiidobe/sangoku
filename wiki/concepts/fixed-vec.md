# Fixed-capacity vector

_created 2026-11-08_

`sangoku_vec::FixedVec<T, N>` is a vector with inline storage of `N` elements
and no allocator. It is a runtime crate in `runtime/vec`, built and linked like
`sangoku_console` and `sangoku_audio`; programs just write
`use sangoku_vec::FixedVec;`. See `examples/fixed_vec.rs`:

```rust
let mut values = FixedVec::<u8, 16>::new();
let _ = values.push(10);
let _ = values.push(20);
let sum = values.iter().copied().fold(0, u8::wrapping_add);
```

`push` returns `Err(value)` when full instead of panicking. `pop`, `clear`,
`len`, `capacity`, `is_empty`, `as_slice` and `as_mut_slice` are provided, and
`Deref<Target = [T]>` gives indexing and iteration. Elements are dropped on
`pop`, `clear` and drop. Host tests are in `tests/vec.rs`.

## Size

Linked bytes at opt z for two pushes and a sum (see
[size-baseline](size-baseline.md) for the method):

| implementation                  |  total |
| ------------------------------- | -----: |
| `alloc::Vec`                    | 12,498 |
| `heapless::Vec<u8, 16>` 0.9.3   |  3,035 |
| inline single-file `FixedVec`   |  1,763 |
| `sangoku_vec::FixedVec<u8, 16>` |  1,976 |

`heapless` is generic over a storage type and a length type, which leaves
non-inlined `push`, `deref` and index functions (about 1.1 KB) plus panic
location data. The crate version is about 210 bytes larger than the single-file
one because the slice and iterator methods become separate functions in
`sangoku_vec` (280 bytes) rather than inlining into `gbc_main`. This was
measured for `u8` and `N = 16` only; the `heapless` build was linked by hand
with a rustc wrapper, since the driver cannot take extra crates from users.
