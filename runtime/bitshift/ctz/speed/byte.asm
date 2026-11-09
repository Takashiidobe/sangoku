SECTION "ctz byte core", ROM0

__gbc_ctz_byte::
    rrca
    jr c, .bit0
    rrca
    jr c, .bit1
    rrca
    jr c, .bit2
    rrca
    jr c, .bit3
    rrca
    jr c, .bit4
    rrca
    jr c, .bit5
    rrca
    jr c, .bit6
    ld a, 7
    ret
.bit0
    xor a
    ret
.bit1
    ld a, 1
    ret
.bit2
    ld a, 2
    ret
.bit3
    ld a, 3
    ret
.bit4
    ld a, 4
    ret
.bit5
    ld a, 5
    ret
.bit6
    ld a, 6
    ret
