SECTION "32-bit multiplication", ROM0

__gbc_mul32::
    DEF MUL32_STEPS EQU 4
    INCLUDE "runtime/integer/mul/common/i32.inc"
