SECTION "16-bit multiplication", ROM0

EXPORT __gbc_mul16
__gbc_mul16::
    ld d, c
    ld e, b
    ld c, a
    ld hl, sp + 2
    ld a, [hl]
    ld hl, 0
    ld b, 16
    or a
    jr nz, .bit
    ld b, 8
    ld a, c
.loop
    add hl, hl
.bit
    rl c
    rla
    jr nc, .skip_add
    add hl, de
.skip_add
    dec b
    jr nz, .loop
    ld a, l
    ld b, h
    ret
