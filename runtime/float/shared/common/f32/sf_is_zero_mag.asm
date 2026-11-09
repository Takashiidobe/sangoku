INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_is_zero_mag", ROM0


; Z set when the float at hl is +-0

sf_is_zero_mag::
    ld a, [hl+]
    or [hl]
    inc hl
    or [hl]
    inc hl
    ld b, a
    ld a, [hl]
    and $7f
    or b
    ret
