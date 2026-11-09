IF !DEF(SANGOKU_F32_MACROS)
DEF SANGOKU_F32_MACROS EQU 1
DEF X_S EQU 0
DEF X_E EQU 1
DEF X_M EQU 3

MACRO LOAD_A
    ld [wA + 2], a
    ld a, b
    ld [wA], a
    ld a, c
    ld [wA + 1], a
    ld hl, sp + 2
    ld a, [hl]
    ld [wA + 3], a
ENDM

MACRO LOAD_AB
    LOAD_A
    ld hl, sp + 3
    ld a, [hl+]
    ld [wB], a
    ld a, [hl+]
    ld [wB + 1], a
    ld a, [hl+]
    ld [wB + 2], a
    ld a, [hl]
    ld [wB + 3], a
ENDM

MACRO CLASSIFY
    ld a, [\1 + 3]
    and $7f
    cp $7f
    jr nz, .finite\@
    ld a, [\1 + 2]
    bit 7, a
    jr z, .finite\@
    and $7f
    ld e, a
    ld a, [\1 + 1]
    or e
    ld e, a
    ld a, [\1]
    or e
    jr z, .inf\@
    ld a, 2
    jr .end\@
.inf\@
    ld a, 1
    jr .end\@
.finite\@
    xor a
.end\@
ENDM

MACRO UNPACK
    ld a, [\1 + 3]
    and $80
    ld [\2 + X_S], a
    ld a, [\1 + 3]
    add a
    ld b, a
    ld a, [\1 + 2]
    rlca
    ld a, b
    adc 0
    ld [\2 + X_E], a
    ld c, a
    xor a
    ld [\2 + X_E + 1], a
    ld [\2 + X_M], a
    ld a, [\1]
    ld [\2 + X_M + 1], a
    ld a, [\1 + 1]
    ld [\2 + X_M + 2], a
    ld a, [\1 + 2]
    and $7f
    ld b, a
    ld a, c
    or a
    jr z, .subnormal\@
    set 7, b
    jr .store\@
.subnormal\@
    ld a, 1
    ld [\2 + X_E], a
.store\@
    ld a, b
    ld [\2 + X_M + 3], a
    ld hl, \2 + X_M + 3
    srl [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
ENDM

MACRO NORMALIZE
.again\@
    ld a, [\1 + X_M + 3]
    bit 6, a
    jr nz, .done\@
    ld hl, \1 + X_M
    call sf_shl1
    ld hl, \1 + X_E
    call sf_dec16
    jr .again\@
.done\@
ENDM

MACRO TIE_BYTE
    ld a, [wM + \1]
    ld b, a
    ld a, [wHf + \1]
    and b
ENDM

ENDC
