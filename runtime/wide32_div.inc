IF !DEF(_GBC_DIV32_DEFS)
DEF _GBC_DIV32_DEFS EQU 1

DEF F_N EQU 0
DEF F_R EQU 4
DEF F_D EQU 8
DEF F_COUNT EQU 12
DEF F_SIGNS EQU 13
DEF FRAME EQU 14

; copy i32 arguments into a frame of FRAME bytes: dividend -> N, divisor -> D
MACRO LOAD_DIV_ARGS
    ld d, a
    add sp, -FRAME
    ld hl, sp + F_N
    ld a, b
    ld [hl+], a
    ld a, c
    ld [hl+], a
    ld a, d
    ld [hl+], a
    ld hl, sp + FRAME + 2
    ld a, [hl]
    ld hl, sp + F_N + 3
    ld [hl], a
    FOR I, 4
        ld hl, sp + FRAME + 3 + I
        ld a, [hl]
        ld hl, sp + F_D + I
        ld [hl], a
    ENDR
    ld hl, sp + F_R
    xor a
    ld [hl+], a
    ld [hl+], a
    ld [hl+], a
    ld [hl], a
    ld hl, sp + F_D
    ld a, [hl+]
    or [hl]
    inc hl
    or [hl]
    inc hl
    or [hl]
    jr nz, .divisor_nonzero\@
    rst $10
.divisor_nonzero\@
ENDM

; negate the 32-bit value at sp + \1
MACRO NEG32
    ld hl, sp + \1
    xor a
    sub [hl]
    ld [hl+], a
    ld a, 0
    sbc [hl]
    ld [hl+], a
    ld a, 0
    sbc [hl]
    ld [hl+], a
    ld a, 0
    sbc [hl]
    ld [hl], a
ENDM

; load the 32-bit value at sp + \1 into A, B, C, D
MACRO RETURN_FRAME_VALUE
    ld hl, sp + \1
    ld a, [hl+]
    ld b, [hl]
    inc hl
    ld c, [hl]
    inc hl
    ld d, [hl]
ENDM

; bit 0 of F_SIGNS: dividend negative, bit 1: divisor negative
MACRO SIGNED_PREPARE
    LOAD_DIV_ARGS
    ld hl, sp + F_SIGNS
    ld [hl], 0
    ld hl, sp + F_N + 3
    bit 7, [hl]
    jr z, .dividend_positive\@
    ld hl, sp + F_SIGNS
    set 0, [hl]
    NEG32 F_N
.dividend_positive\@
    ld hl, sp + F_D + 3
    bit 7, [hl]
    jr z, .divisor_positive\@
    ld hl, sp + F_SIGNS
    set 1, [hl]
    NEG32 F_D
.divisor_positive\@
ENDM
ENDC
