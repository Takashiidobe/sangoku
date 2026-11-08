INCLUDE "runtime/wide32_div.inc"

SECTION "Signed 32-bit division", ROM0

EXPORT __gbc_sdiv32
__gbc_sdiv32::
    SIGNED_PREPARE
    call __gbc_div32_core
    ld hl, sp + F_SIGNS
    ld a, [hl]
    ld b, a
    rrca
    xor b
    and 1
    jr z, .positive
    NEG32 F_N
    jr .done
.positive
    ld hl, sp + F_N + 3
    bit 7, [hl]
    jr z, .done
    rst $10
.done
    RETURN_FRAME_VALUE F_N
    add sp, FRAME
    ret
