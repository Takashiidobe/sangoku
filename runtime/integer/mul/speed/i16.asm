INCLUDE "runtime/integer/mul/common/i16.inc"

SECTION "16-bit multiplication", ROM0

__gbc_mul16::
    ld d, c
    ld e, b
    ld c, a
    ld hl, sp + 2
    ld a, [hl]
    ld hl, 0
    or a
    jp z, .narrow
    REPT 16
        MUL16_STEP
    ENDR
    jr .done
.narrow
    ld a, c
    REPT 8
        MUL16_NARROW_STEP
    ENDR
.done
    ld a, l
    ld b, h
    ret
