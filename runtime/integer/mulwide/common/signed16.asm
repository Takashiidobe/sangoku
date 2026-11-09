SECTION "signed 16-bit widening multiplication", ROM0

__gbc_smul16wide::
    ld d, a
    add sp, -6
    ld hl, sp + 8
    ld a, [hl]
    ld hl, sp + 0
    ld [hl], a
    ld hl, sp + 2
    ld [hl], b
    inc hl
    ld [hl], c
    inc hl
    ld [hl], d
    inc hl
    ld [hl], a
    ld a, d
    call __gbc_umul16wide
    ld hl, sp + 1
    ld [hl], a
    ld hl, sp + 3
    bit 7, [hl]
    jr z, .rhs_sign
    ld hl, sp + 4
    ld a, c
    sub [hl]
    ld c, a
    inc hl
    ld a, d
    sbc [hl]
    ld d, a
.rhs_sign
    ld hl, sp + 5
    bit 7, [hl]
    jr z, .done
    ld hl, sp + 2
    ld a, c
    sub [hl]
    ld c, a
    inc hl
    ld a, d
    sbc [hl]
    ld d, a
.done
    ld hl, sp + 1
    ld a, [hl]
    add sp, 6
    ret
