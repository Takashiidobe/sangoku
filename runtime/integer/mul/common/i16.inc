MACRO MUL16_STEP
    add hl, hl
    rl c
    rla
    jr nc, .skip\@
    add hl, de
.skip\@
ENDM

MACRO MUL16_NARROW_STEP
    add hl, hl
    rla
    jr nc, .skip\@
    add hl, de
.skip\@
ENDM
