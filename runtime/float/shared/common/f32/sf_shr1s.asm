INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_shr1s", ROM0

sf_shr1s::
    inc hl
    inc hl
    inc hl
    srl [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
    ret nc
    set 0, [hl]
    ret
