INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_dec16", ROM0


; hl -> 16-bit value

sf_dec16::
    ld a, [hl]
    sub 1
    ld [hl+], a
    ld a, [hl]
    sbc 0
    ld [hl], a
    ret
