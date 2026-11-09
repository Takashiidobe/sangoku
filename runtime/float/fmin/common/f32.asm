INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 __gbc_fmin32", ROM0



__gbc_fmin32::
    LOAD_AB
    call sf_compare
    cp 3
    jp z, sf_ret_nan
    cp 1
    jp z, sf_ret_a
    cp 2
    jp z, sf_ret_b
    ld a, [wA + 3]
    bit 7, a
    jp nz, sf_ret_a
    jp sf_ret_b
