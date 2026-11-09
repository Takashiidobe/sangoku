INCLUDE "runtime/integer/mulwide/common/step.inc"

SECTION "unsigned 16-bit widening multiplication", ROM0

__gbc_umul16wide::
    ld d, c
    ld e, b
    ld c, a
    ld hl, sp + 2
    ld b, [hl]
    ld hl, 0
    ld a, d
    or e
    jp z, .zero
    ld a, b
    or c
    jp z, .zero
    REPT 16
        WIDEN_MUL16_STEP
    ENDR
    ld a, c
    ld c, l
    ld d, h
    ret
.zero
    xor a
    ld b, a
    ld c, a
    ld d, a
    ret
