SECTION "memcmp", ROM0

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
.loop:
    ld a, b
    or c
    jr z, .equal
    ld a, [de]
    cp [hl]
    jr nz, .different
    inc hl
    inc de
    dec bc
    jr .loop
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
