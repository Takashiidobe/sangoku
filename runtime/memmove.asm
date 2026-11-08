SECTION "memmove", ROM0

memmove::
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
    ld a, e
    sub l
    ld a, d
    sbc h
    jr c, .forward
    add hl, bc
    ld a, e
    add c
    ld e, a
    ld a, d
    adc b
    ld d, a
.backward:
    ld a, b
    or c
    jr z, .done
    dec hl
    dec de
    ld a, [hl]
    ld [de], a
    dec bc
    jr .backward
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
