INCLUDE "runtime/integer/div/common/signed.inc"

SECTION "Signed 16-bit srem", ROM0
__gbc_srem16::
    SIGNED_DIV16_PREPARE 0
    call div16_core
    ld b, h
    ld c, l
    pop de
    bit 7, d
    jr nz, .negative
    jr .done
.negative
    xor a
    sub c
    ld c, a
    ld a, 0
    sbc b
    ld b, a
.done
    ld a, c
    ret
