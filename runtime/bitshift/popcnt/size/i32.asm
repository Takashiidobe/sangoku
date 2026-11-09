SECTION "32-bit population count", ROM0

__gbc_popcnt32::
    ld d, a
    ld hl, sp + 2
    ld e, [hl]
    ld l, 32
    xor a
.loop:
    srl e
    rr d
    rr c
    rr b
    adc 0
    dec l
    jr nz, .loop
    ld b, 0
    ld c, b
    ld d, b
    ret
