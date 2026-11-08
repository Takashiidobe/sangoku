SECTION "compare", ROM0

__rust_memcmp::
    ld d, c
    ld e, b
    ld c, a
    ld hl, sp + 2
    ld b, [hl]
    ld hl, sp + 3
    ld a, [hli]
    ld h, [hl]
    ld l, a
    push bc
    push hl
    pop bc
    pop hl
    srl b
    rr c
    jr nc, .skip_one
    ld a, [de]
    cp [hl]
    jr nz, .different
    inc hl
    inc de
.skip_one:
    srl b
    rr c
    inc b
    inc c
    jr nc, .test
    jr .two
.four:
    ld a, [de]
    cp [hl]
    jr nz, .different
    inc hl
    inc de
    ld a, [de]
    cp [hl]
    jr nz, .different
    inc hl
    inc de
.two:
    ld a, [de]
    cp [hl]
    jr nz, .different
    inc hl
    inc de
    ld a, [de]
    cp [hl]
    jr nz, .different
    inc hl
    inc de
.test:
    dec c
    jr nz, .four
    dec b
    jr nz, .four
    jr .equal
.different:
    jr c, .less
    ld a, 1
    ld b, 0
    ld c, b
    ld d, b
    ret
.less:
    ld a, $ff
    ld b, a
    ld c, a
    ld d, a
    ret
.equal:
    xor a
    ld b, a
    ld c, a
    ld d, a
    ret
