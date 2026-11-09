INCLUDE "runtime/integer/div/common/prepare16.inc"
IF !DEF(SANGOKU_SIGNED_DIV)
DEF SANGOKU_SIGNED_DIV EQU 1
MACRO SIGNED_DIV8_PREPARE
    ld a, b
    IF \1
        xor c
    ENDC
    push af
    bit 7, b
    jr z, .lhs_positive
    xor a
    sub b
    ld b, a
.lhs_positive
    bit 7, c
    jr z, .rhs_positive
    xor a
    sub c
    ld c, a
.rhs_positive
ENDM
MACRO SIGNED_DIV16_PREPARE
    DIV_PREPARE
    ld a, b
    IF \1
        xor d
    ENDC
    push af
    bit 7, b
    jr z, .lhs_positive
    xor a
    sub c
    ld c, a
    ld a, 0
    sbc b
    ld b, a
.lhs_positive
    bit 7, d
    jr z, .rhs_positive
    xor a
    sub e
    ld e, a
    ld a, 0
    sbc d
    ld d, a
.rhs_positive
ENDM
ENDC
