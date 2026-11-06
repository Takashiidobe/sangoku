SECTION "16-bit multiplication", ROM0

EXPORT __gbc_mul16
__gbc_mul16::
    ld d, a
    ld hl, sp + 2
    ld a, [hl]
    ld e, a
    ld hl, 0
    ld a, 16
.loop
    srl d
    rr e
    jr nc, .skip_add
    add hl, bc
.skip_add
    sla c
    rl b
    dec a
    jr nz, .loop
    ld a, l
    ld b, h
    ret
