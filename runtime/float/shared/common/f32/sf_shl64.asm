INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_shl64", ROM0


; hl -> 64-bit value; shift left one bit, carry out holds the old top bit

sf_shl64::
    sla [hl]
    FOR i, 7
    inc hl
    rl [hl]
    ENDR
    ret
