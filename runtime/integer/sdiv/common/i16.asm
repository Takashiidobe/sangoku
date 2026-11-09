INCLUDE "runtime/integer/div/common/signed.inc"

SECTION "Signed 16-bit sdiv", ROM0
__gbc_sdiv16::
    SIGNED_DIV16_PREPARE 1
    call div16_core
    pop de
    bit 7, d
    jr nz, .negative
    bit 7, b
    jr z, .done
    rst $10
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
