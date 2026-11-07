SECTION "memory", ROM0

__gbc_memset::
    push bc
    ld d, a
    ld a, b
    ld b, c
    ld c, a
    ld hl, sp + 7
    ld a, [hli]
    ld h, [hl]
    ld l, a
.loop:
    ld a, h
    or l
    jr z, .done
    ld a, d
    ld [bc], a
    inc bc
    dec hl
    jr .loop
.done:
    pop bc
    ld a, b
    ld b, c
    ret

