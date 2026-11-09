INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_is_zero32", ROM0

; Z set when the 32-bit value at hl is zero

sf_is_zero32::
    ld a, [hl+]
    or [hl]
    inc hl
    or [hl]
    inc hl
    or [hl]
    ret
