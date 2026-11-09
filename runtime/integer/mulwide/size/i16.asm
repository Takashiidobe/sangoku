INCLUDE "runtime/integer/mulwide/common/step.inc"

SECTION "unsigned 16-bit widening multiplication", ROM0

__gbc_umul16wide::
    ld d, c
    ld e, b
    ld c, a
    ld hl, sp + 2
    ld b, [hl]
    ld hl, 0
    ld a, 16
.loop
    WIDEN_MUL16_STEP
    dec a
    jr nz, .loop
    ld a, c
    ld c, l
    ld d, h
    ret
