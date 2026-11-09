MACRO MUL8_STEP
    add a, a
    sla c
    jr nc, .skip\@
    add b
.skip\@
ENDM
