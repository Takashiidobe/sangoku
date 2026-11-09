SECTION "64-bit divide and multiply", ROM0

; BC = context: a (8 bytes), b (8 bytes), op, status. Result replaces a.
__gbc_wide64::
    ld d, c
    ld e, b
    ld a, 8
    jp __gbc_wide_core
