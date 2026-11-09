INCLUDE "runtime/integer/div/common/prepare16.inc"
SECTION "Unsigned 16-bit division", ROM0
__gbc_udiv16::
    DIV_PREPARE
    call div16_core
    ld a, c
    ret
