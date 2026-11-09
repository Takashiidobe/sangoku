SECTION "wide divide and multiply core", ROM0

; In: DE = context, A = operand length L in bytes (8 or 16).
; Context: a[L], b[L], op, status. Ops: 0 udiv, 1 urem, 2 sdiv, 3 srem,
; 4 umulhi, 5 smulhi, 6 imul. The result replaces a. Status 1 is a zero
; divisor and 2 is signed division overflow. Every register is clobbered.
; Frame: 0..31 scratch, 32 L, 33 op, 34 context, 36 sign a, 37 sign b,
; 38 bit counter, 39 temporary.

MACRO NEG_IF
    ld hl, sp + \1
    ld c, [hl]
    call __gbc_wide_ctx
IF \2 == 0
    ld h, d
    ld l, e
ENDC
    ld a, c
    and a
    call nz, __gbc_wide_neg
ENDM

__gbc_wide_core::
    add sp, -40
    ld hl, sp + 32
    ld [hli], a
    inc hl
    ld [hl], e
    inc hl
    ld [hl], d
    call __gbc_wide_ctx
    ld c, b
    ld b, 0
    add hl, bc
    ld a, [hli]
    ld [hl], 0
    ld hl, sp + 33
    ld [hl], a
    ld hl, sp + 36
    ld [hl], 0
    inc hl
    ld [hl], 0
    cp 4
    jp nc, .mul
    call __gbc_wide_ctx
    xor a
.zero_check
    or [hl]
    inc hl
    dec b
    jr nz, .zero_check
    and a
    ld c, 1
    jp z, .fail
    ld hl, sp + 33
    ld a, [hl]
    bit 1, a
    jr z, .unsigned
    call __gbc_wide_ctx
    call __gbc_wide_sign
    ld hl, sp + 37
    ld [hl], a
    call __gbc_wide_ctx
    ld h, d
    ld l, e
    call __gbc_wide_sign
    ld hl, sp + 36
    ld [hl], a
    NEG_IF 36, 0
    NEG_IF 37, 1
.unsigned
    call __gbc_wide_ctx
    ld a, b
    add a
    add a
    add a
    ld hl, sp + 38
    ld [hl], a
    ld hl, sp + 0
    xor a
.clear
    ld [hli], a
    dec b
    jr nz, .clear
.bit
    call __gbc_wide_ctx
    ld c, b
    ld h, d
    ld l, e
    and a
    call __gbc_wide_shl1
    ld a, 0
    rla
    ld hl, sp + 0
    ld b, c
    rra
    call __gbc_wide_shl1
    ld a, 0
    rla
    ld hl, sp + 39
    ld [hl], a
    call __gbc_wide_ctx
    ld d, h
    ld e, l
    ld hl, sp + 0
    call __gbc_wide_sub
    ld a, 0
    rla
    xor 1
    ld hl, sp + 39
    or [hl]
    ld [hl], a
    jr nz, .take
    call __gbc_wide_ctx
    ld d, h
    ld e, l
    ld hl, sp + 0
    call __gbc_wide_add
.take
    ld hl, sp + 39
    ld c, [hl]
    call __gbc_wide_ctx
    ld a, [de]
    or c
    ld [de], a
    ld hl, sp + 38
    dec [hl]
    jp nz, .bit
    ld hl, sp + 33
    ld a, [hl]
    and 1
    jr z, .quotient
    call __gbc_wide_ctx
    ld hl, sp + 0
    call __gbc_wide_copy
.quotient
    ld hl, sp + 33
    ld c, [hl]
    bit 1, c
    jr z, .done
    ld hl, sp + 36
    ld a, [hli]
    ld b, [hl]
    bit 0, c
    jr nz, .sign_ready
    xor b
.sign_ready
    ld hl, sp + 36
    ld [hl], a
    NEG_IF 36, 0
    ld hl, sp + 33
    ld a, [hl]
    cp 2
    jr nz, .done
    ld hl, sp + 36
    ld a, [hl]
    and a
    jr nz, .done
    call __gbc_wide_ctx
    ld h, d
    ld l, e
    call __gbc_wide_sign
    and a
    jr z, .done
    ld c, 2
.fail
    call __gbc_wide_ctx
    ld e, b
    ld d, 0
    add hl, de
    inc hl
    ld [hl], c
.done
    add sp, 40
    ret

.mul
    call __gbc_wide_ctx
    ld c, b
    ld d, h
    ld e, l
    ld hl, sp + 0
.copy_b
    ld a, [de]
    ld [hli], a
    inc de
    dec b
    jr nz, .copy_b
    ld b, c
    xor a
.clear_high
    ld [hli], a
    dec b
    jr nz, .clear_high
    ld a, c
    add a
    add a
    add a
    ld hl, sp + 38
    ld [hl], a
.mul_bit
    ld hl, sp + 0
    ld a, [hl]
    rra
    ld a, 0
    jr nc, .shift
    call __gbc_wide_ctx
    call __gbc_wide_high
    call __gbc_wide_add
    ld a, 0
    rla
.shift
    ld hl, sp + 32
    ld b, [hl]
    call __gbc_wide_shr_product
    ld hl, sp + 38
    dec [hl]
    jr nz, .mul_bit
    ld hl, sp + 33
    ld a, [hl]
    cp 5
    jr nz, .mul_result
    call __gbc_wide_ctx
    ld h, d
    ld l, e
    call __gbc_wide_sign
    and a
    jr z, .after_a
    call __gbc_wide_ctx
    ld d, h
    ld e, l
    call __gbc_wide_high
    call __gbc_wide_sub
.after_a
    call __gbc_wide_ctx
    call __gbc_wide_sign
    and a
    jr z, .mul_result
    call __gbc_wide_ctx
    call __gbc_wide_high
    call __gbc_wide_sub
.mul_result
    call __gbc_wide_ctx
    ld hl, sp + 33
    ld a, [hl]
    ld hl, sp + 0
    cp 6
    jr z, .mul_copy
    call __gbc_wide_high
.mul_copy
    call __gbc_wide_copy
    add sp, 40
    ret

; Out: DE = a, HL = b, B = L. Keeps C. Only valid one call deep.
__gbc_wide_ctx:
    ld hl, sp + 36
    ld e, [hl]
    inc hl
    ld d, [hl]
    ld hl, sp + 34
    ld b, [hl]
    ld h, 0
    ld l, b
    add hl, de
    ret

; Out: HL = scratch + L, B = L. Only valid one call deep.
__gbc_wide_high:
    ld hl, sp + 2
    ld c, b
    ld b, 0
    add hl, bc
    ld b, c
    ret

; In: HL = array, B = L. Out: A = 1 when negative, else 0.
__gbc_wide_sign:
    ld c, b
    ld b, 0
    add hl, bc
    dec hl
    ld a, [hl]
    rlca
    and 1
    ret

; In: HL = array, B = L, carry in. Out: carry.
__gbc_wide_shl1:
.loop
    rl [hl]
    inc hl
    dec b
    jr nz, .loop
    ret

; In: HL = dst, DE = src, B = L. dst += src; out: carry.
__gbc_wide_add:
    and a
.loop
    ld a, [de]
    adc [hl]
    ld [hli], a
    inc de
    dec b
    jr nz, .loop
    ret

; In: HL = dst, DE = src, B = L. dst -= src; out: carry is borrow.
__gbc_wide_sub:
    and a
.loop
    ld a, [de]
    ld c, a
    ld a, [hl]
    sbc c
    ld [hli], a
    inc de
    dec b
    jr nz, .loop
    ret

; In: HL = array, B = L. Two's complement negate.
__gbc_wide_neg:
    scf
.loop
    ld a, [hl]
    cpl
    adc 0
    ld [hli], a
    dec b
    jr nz, .loop
    ret

; In: HL = src, DE = dst, B = L.
__gbc_wide_copy:
.loop
    ld a, [hli]
    ld [de], a
    inc de
    dec b
    jr nz, .loop
    ret

; In: B = L, A = carry bit. Shifts the 2L byte product right one bit.
__gbc_wide_shr_product:
    ld c, a
    ld a, b
    add a
    ld b, a
    ld hl, sp + 2
    ld d, 0
    ld e, b
    add hl, de
    dec hl
    ld a, c
    rra
.loop
    rr [hl]
    dec hl
    dec b
    jr nz, .loop
    ret
