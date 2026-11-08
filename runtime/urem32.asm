INCLUDE "runtime/wide32_div.inc"

SECTION "Unsigned 32-bit remainder", ROM0

EXPORT __gbc_urem32
__gbc_urem32::
    LOAD_DIV_ARGS
    call __gbc_div32_core
    RETURN_FRAME_VALUE F_R
    add sp, FRAME
    ret
