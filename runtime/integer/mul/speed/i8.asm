INCLUDE "runtime/integer/mul/common/i8.inc"

SECTION "8-bit multiplication", ROM0

__gbc_mul8::
    xor a
    REPT 8
        MUL8_STEP
    ENDR
    ret
