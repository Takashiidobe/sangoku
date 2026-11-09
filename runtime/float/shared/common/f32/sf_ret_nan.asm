INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_ret_nan", ROM0



sf_ret_nan::
    xor a
    ld b, a
    ld c, $c0
    ld d, $7f
    ret
