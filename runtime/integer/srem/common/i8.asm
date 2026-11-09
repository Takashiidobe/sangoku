INCLUDE "runtime/integer/div/common/signed.inc"

SECTION "Signed 8-bit srem", ROM0
__gbc_srem8::
    SIGNED_DIV8_PREPARE 0
    call __gbc_div8_core
    pop de
    bit 7, d
    jr nz, .negative
    jr .done
.negative
    cpl
    inc a
.done
    ret
