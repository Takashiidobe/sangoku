; One-bit step over a little-endian byte array. In: DE = array, B = op (0 shl,
; 1 ushr, 2 sshr, 3 rotl, 4 rotr). Keeps B, C and DE; clobbers A, HL and flags.
; \1 = width in bytes, \2 = 1 to unroll the byte loop.
MACRO WSHIFT_BIT
    ld a, b
    or a
    jr z, .shl
    cp 3
    jr z, .rotl
    ld hl, \1 - 1
    add hl, de
    cp 4
    jr z, .rotr
    cp 2
    jr z, .sshr
    srl [hl]
    jr .right
.sshr
    sra [hl]
    jr .right
.rotr
    ld a, [de]
    rra
    rr [hl]
.right
IF \2
    REPT \1 - 1
        dec hl
        rr [hl]
    ENDR
ELSE
    ld a, \1 - 1
.right_loop
    dec hl
    rr [hl]
    dec a
    jr nz, .right_loop
ENDC
    ret
.rotl
    ld hl, \1 - 1
    add hl, de
    ld a, [hl]
    rla
    jr .left
.shl
    or a
.left
    ld h, d
    ld l, e
IF \2
    REPT \1 - 1
        rl [hl]
        inc hl
    ENDR
    rl [hl]
ELSE
    ld a, \1
.left_loop
    rl [hl]
    inc hl
    dec a
    jr nz, .left_loop
ENDC
    ret
ENDM
