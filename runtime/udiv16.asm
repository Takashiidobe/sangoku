SECTION "Unsigned 16-bit division", ROM0

MACRO DIV_PREPARE
    ld e, a
    ld hl, sp + 2
    ld d, [hl]
    ld a, b
    ld b, c
    ld c, a
ENDM

EXPORT __gbc_udiv16
__gbc_udiv16::
    DIV_PREPARE
    call div16_core
    ld a, c
    ret

EXPORT __gbc_urem16
__gbc_urem16::
    DIV_PREPARE
    call div16_core
    ld a, l
    ld b, h
    ret

div16_core:
    ld a, d
    or e
    jr nz, .nonzero
    rst $10
.nonzero
    ld l, c
    ld h, b
    ld a, e
    and $80
    or d
    jr nz, .wide
    ld b, 16
    rl l
    rl h
.narrow_loop
    rla
    sub e
    jr nc, .narrow_keep
    add e
.narrow_keep
    ccf
    rl l
    rl h
    dec b
    jr nz, .narrow_loop
    ld d, h
    ld c, l
    ld h, b
    ld l, a
    ld b, d
    ret
.wide
    ld b, 9
    ld a, l
    ld l, h
    ld h, 0
    rr l
.wide_loop
    rl l
    rl h
    ld c, a
    ld a, l
    sbc e
    ld l, a
    ld a, h
    sbc d
    ld h, a
    ld a, c
    jr nc, .wide_keep
    add hl, de
.wide_keep
    ccf
    rla
    dec b
    jr nz, .wide_loop
    rl b
    ld c, a
    ret
