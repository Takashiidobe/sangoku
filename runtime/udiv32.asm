INCLUDE "runtime/wide32_div.inc"

SECTION "Unsigned 32-bit division", ROM0

EXPORT __gbc_udiv32
__gbc_udiv32::
    LOAD_DIV_ARGS
    call __gbc_div32_core
    RETURN_FRAME_VALUE F_N
    add sp, FRAME
    ret
