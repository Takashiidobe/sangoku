INCLUDE "runtime/integer/mul/common/i8.inc"

SECTION "8-bit multiplication", ROM0

__gbc_mul8::
    ld d, 8
    xor a
.loop
    MUL8_STEP
    dec d
    jr nz, .loop
    ret
