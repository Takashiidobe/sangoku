INCLUDE "runtime/integer/div/common/prepare16.inc"
SECTION "Unsigned 16-bit remainder", ROM0
__gbc_urem16::
    DIV_PREPARE
    call div16_core
    ld a, l
    ld b, h
    ret
