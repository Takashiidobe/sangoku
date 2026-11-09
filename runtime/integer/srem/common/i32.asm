INCLUDE "runtime/integer/div/common/frame.inc"

SECTION "Signed 32-bit remainder", ROM0

EXPORT __gbc_srem32
__gbc_srem32::
    SIGNED_PREPARE
    call __gbc_div32_core
    ld hl, sp + F_SIGNS
    bit 0, [hl]
    jr z, .done
    NEG32 F_R
.done
    RETURN_FRAME_VALUE F_R
    add sp, FRAME
    ret
