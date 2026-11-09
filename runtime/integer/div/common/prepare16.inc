IF !DEF(SANGOKU_DIV16_MACRO)
DEF SANGOKU_DIV16_MACRO EQU 1
MACRO DIV_PREPARE
    ld e, a
    ld hl, sp + 2
    ld d, [hl]
    ld a, b
    ld b, c
    ld c, a
ENDM

ENDC
