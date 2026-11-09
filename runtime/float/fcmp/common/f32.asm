INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 __gbc_fcmp32", ROM0



__gbc_fcmp32::
    LOAD_AB
    ld hl, sp + 7
    ld a, [hl]
    ld [wMask], a
    call sf_compare
    ld b, a
    ld a, [wMask]
    inc b
.shift
    dec b
    jr z, .done
    srl a
    jr .shift
.done
    and 1
    ret
