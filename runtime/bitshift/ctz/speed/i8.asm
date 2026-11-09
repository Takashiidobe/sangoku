SECTION "8-bit ctz", ROM0

__gbc_ctz8::
    ld a, b
    or a
    jp nz, __gbc_ctz_byte
    ld a, 8
    ret
