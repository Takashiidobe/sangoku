MACRO MUL32_STEP
    ld hl, sp + 3
    srl [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
    jr nc, .skip\@
    ld hl, sp + 4
    ld a, [hl+]
    add e
    ld e, a
    ld a, [hl+]
    adc d
    ld d, a
    ld a, [hl+]
    adc c
    ld c, a
    ld a, [hl]
    adc b
    ld b, a
.skip\@
    ld hl, sp + 4
    sla [hl]
    inc hl
    rl [hl]
    inc hl
    rl [hl]
    inc hl
    rl [hl]
ENDM

    ld d, a
    add sp, -8
    ld hl, sp + 0
    ld a, b
    ld [hl+], a
    ld a, c
    ld [hl+], a
    ld a, d
    ld [hl+], a
    ld hl, sp + 10
    ld a, [hl]
    ld hl, sp + 3
    ld [hl], a
    FOR I, 4
        ld hl, sp + 11 + I
        ld a, [hl]
        ld hl, sp + 4 + I
        ld [hl], a
    ENDR
    ld bc, 0
    ld de, 0
.loop
    ld hl, sp + 0
    ld a, [hl+]
    or [hl]
    inc hl
    or [hl]
    inc hl
    or [hl]
    IF MUL32_STEPS == 1
        jr z, .done
    ELSE
        jp z, .done
    ENDC
    REPT MUL32_STEPS
        MUL32_STEP
    ENDR
    IF MUL32_STEPS == 1
        jr .loop
    ELSE
        jp .loop
    ENDC
.done
    add sp, 8
    ld a, e
    ld e, b
    ld b, d
    ld d, e
    ret
