INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_return", ROM0

sf_return::
    ld hl, wR
sf_ret_hl::
    ld a, [hl+]
    ld b, [hl]
    inc hl
    ld c, [hl]
    inc hl
    ld d, [hl]
    ret
