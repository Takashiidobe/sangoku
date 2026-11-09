SECTION "8-bit clz", ROM0

__gbc_clz8::
    ld a, b
    or a
    jp nz, __gbc_clz_byte
    ld a, 8
    ret
