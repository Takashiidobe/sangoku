MACRO WIDEN_MUL16_STEP
    ; a skipped add must shift a zero carry into the product
    or a
    bit 0, c
    jr z, .skip\@
    add hl, de
.skip\@
    rr h
    rr l
    rr b
    rr c
ENDM
