SECTION "bit reversal byte core", ROM0

__gbc_bitrev_byte::
    swap a
    ld l, a
    and $33
    add a, a
    add a, a
    ld h, a
    ld a, l
    and $cc
    rrca
    rrca
    or h
    ld l, a
    and $55
    add a, a
    ld h, a
    ld a, l
    and $aa
    rrca
    or h
    ret
