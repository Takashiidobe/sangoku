SECTION "memcpy", ROM0

memcpy::
    push bc
    ld d, c
    ld e, b
    ld c, a
    ld hl, sp + 4
    ld b, [hl]
    ld hl, sp + 5
    ld a, [hli]
    ld h, [hl]
    ld l, a
    push bc
    push hl
    pop bc
    pop hl
.forward:
    srl b
    rr c
    jr nc, .skip_one
    ld a, [hli]
    ld [de], a
    inc de
.skip_one:
    srl b
    rr c
    inc b
    inc c
    jr nc, .test
    jr .copy_two
.copy_four:
    ld a, [hli]
    ld [de], a
    inc de
    ld a, [hli]
    ld [de], a
    inc de
.copy_two:
    ld a, [hli]
    ld [de], a
    inc de
    ld a, [hli]
    ld [de], a
    inc de
.test:
    dec c
    jr nz, .copy_four
    dec b
    jr nz, .copy_four
.done:
    pop bc
    ld a, b
    ld b, c
    ret
