SECTION "bit reversal byte core", ROM0

__gbc_bitrev_byte::
    ld l, a
    ld h, 8
    xor a
.loop
    rr l
    rla
    dec h
    jr nz, .loop
    ret
