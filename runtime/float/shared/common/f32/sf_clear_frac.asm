INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_clear_frac", ROM0


; clear the fraction bits (wMk mask) of wM

sf_clear_frac::
    FOR i, 4
    ld a, [wMk + i]
    cpl
    ld b, a
    ld a, [wM + i]
    and b
    ld [wM + i], a
    ENDR
    ret
