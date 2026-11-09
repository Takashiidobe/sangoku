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
    ld a, b
    or c
    jr z, .done
.loop:
    ld a, [hli]
    ld [de], a
    inc de
    dec bc
    ld a, b
    or c
    jr nz, .loop
.done:
    pop bc
    ld a, b
    ld b, c
    ret
