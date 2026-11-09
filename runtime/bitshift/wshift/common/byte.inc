; Moves a little-endian byte array by one byte. In: DE = array, B = op (see
; WSHIFT_BIT). Keeps BC and DE; clobbers A, HL and flags. \1 = width in bytes.
MACRO WSHIFT_BYTE
    push bc
    push de
    ld a, b
    cp 3
    jr z, .rotl
    cp 4
    jr z, .rotr
    or a
    jr z, .shl
    cp 2
    jr z, .sshr
    ld c, 0
    jr .right
.sshr
    ld hl, \1 - 1
    add hl, de
    ld a, [hl]
    add a
    sbc a
    ld c, a
    jr .right
.rotr
    ld a, [de]
    ld c, a
.right
    ld h, d
    ld l, e
    inc hl
    ld b, \1 - 1
.right_loop
    ld a, [hli]
    ld [de], a
    inc de
    dec b
    jr nz, .right_loop
    ld a, c
    ld [de], a
    jr .done
.rotl
    ld hl, \1 - 1
    add hl, de
    ld a, [hl]
    ld c, a
    jr .left
.shl
    ld c, 0
.left
    ld hl, \1 - 1
    add hl, de
    ld d, h
    ld e, l
    dec hl
    ld b, \1 - 1
.left_loop
    ld a, [hld]
    ld [de], a
    dec de
    dec b
    jr nz, .left_loop
    ld a, c
    ld [de], a
.done
    pop de
    pop bc
    ret
ENDM
