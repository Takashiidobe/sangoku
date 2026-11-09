INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_shl1", ROM0


; hl -> 32-bit value; shift left one bit

sf_shl1::
    sla [hl]
    inc hl
    rl [hl]
    inc hl
    rl [hl]
    inc hl
    rl [hl]
    ret
