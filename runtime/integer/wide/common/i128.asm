SECTION "128-bit divide and multiply", ROM0

; BC = context: a (16 bytes), b (16 bytes), op, status. Result replaces a.
__gbc_wide128::
    ld d, c
    ld e, b
    ld a, 16
    jp __gbc_wide_core
