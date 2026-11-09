INCLUDE "runtime/integer/div/common/signed.inc"

SECTION "Signed 8-bit sdiv", ROM0
__gbc_sdiv8::
    SIGNED_DIV8_PREPARE 1
    call __gbc_div8_core
    ld a, b
    pop de
    bit 7, d
    jr nz, .negative
    bit 7, a
    jr z, .done
    rst $10
.negative
    cpl
    inc a
.done
    ret
