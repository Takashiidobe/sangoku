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
    ld a, b
    or c
    jr z, .done
.copy:
    ld a, [hli]
    ld [de], a
    inc de
    dec bc
    ld a, b
    or c
    jr nz, .copy
.done:
    pop bc
    ld a, b
    ld b, c
    ret
