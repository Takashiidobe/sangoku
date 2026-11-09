SECTION "clz byte core", ROM0

__gbc_clz_byte::
    add a, a
    jr c, .bit0
    add a, a
    jr c, .bit1
    add a, a
    jr c, .bit2
    add a, a
    jr c, .bit3
    add a, a
    jr c, .bit4
    add a, a
    jr c, .bit5
    add a, a
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
