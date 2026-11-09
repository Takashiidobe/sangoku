SECTION "32-bit ctz", ROM0

__gbc_ctz32::
    ld d, a
    ld hl, sp + 2
    ld e, [hl]
    xor a
.loop
    srl e
    rr d
    rr c
    rr b
    jr c, .done
    inc a
    cp 32
    jr nz, .loop
.done
    ld b, 0
    ld c, b
    ld d, b
    ret
