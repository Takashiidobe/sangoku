SECTION "population count byte core", ROM0

__gbc_popcnt_byte::
    ld l, a
    rrca
    and $55
    ld h, a
    ld a, l
    sub h
    ld l, a
    and $33
    ld h, a
    ld a, l
    rrca
    rrca
    and $33
    add h
    ld l, a
    swap a
    add l
    and $0f
    ret
