SECTION "32-bit runtime", ROM0

; Calling convention: first i32 argument in B, C, A and the first stack byte
; (just above the return address); later i32 arguments follow on the stack.
; Results come back in A, B, C, D (low byte first). Every register is clobbered.

DEF F_N EQU 0
DEF F_R EQU 4
DEF F_D EQU 8
DEF F_COUNT EQU 12
DEF F_SIGNS EQU 13
DEF FRAME EQU 14

; copy i32 arguments into a frame of FRAME bytes: dividend -> N, divisor -> D
MACRO LOAD_DIV_ARGS
    ld d, a
    add sp, -FRAME
    ld hl, sp + F_N
    ld a, b
    ld [hl+], a
    ld a, c
    ld [hl+], a
    ld a, d
    ld [hl+], a
    ld hl, sp + FRAME + 2
    ld a, [hl]
    ld hl, sp + F_N + 3
    ld [hl], a
    FOR I, 4
        ld hl, sp + FRAME + 3 + I
        ld a, [hl]
        ld hl, sp + F_D + I
        ld [hl], a
    ENDR
    ld hl, sp + F_R
    xor a
    ld [hl+], a
    ld [hl+], a
    ld [hl+], a
    ld [hl], a
    ld hl, sp + F_D
    ld a, [hl+]
    or [hl]
    inc hl
    or [hl]
    inc hl
    or [hl]
    jr nz, .divisor_nonzero\@
    rst $10
.divisor_nonzero\@
ENDM

; negate the 32-bit value at sp + \1
MACRO NEG32
    ld hl, sp + \1
    xor a
    sub [hl]
    ld [hl+], a
    ld a, 0
    sbc [hl]
    ld [hl+], a
    ld a, 0
    sbc [hl]
    ld [hl+], a
    ld a, 0
    sbc [hl]
    ld [hl], a
ENDM

; load the 32-bit value at sp + \1 into A, B, C, D
MACRO RETURN_FRAME_VALUE
    ld hl, sp + \1
    ld a, [hl+]
    ld b, [hl]
    inc hl
    ld c, [hl]
    inc hl
    ld d, [hl]
ENDM

EXPORT __gbc_mul32
__gbc_mul32::
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
    jr z, .done
    ld hl, sp + 3
    srl [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
    jr nc, .skip
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
.skip
    ld hl, sp + 4
    sla [hl]
    inc hl
    rl [hl]
    inc hl
    rl [hl]
    inc hl
    rl [hl]
    jr .loop
.done
    add sp, 8
    ld a, e
    ld e, b
    ld b, d
    ld d, e
    ret

; one restoring-division pass over the frame at sp + 2; leaves quotient in N, remainder in R
div32_core:
    ld hl, sp + 2 + F_COUNT
    ld [hl], 32
.loop
    ld hl, sp + 2 + F_N
    sla [hl]
    inc hl
    rl [hl]
    inc hl
    rl [hl]
    inc hl
    rl [hl]
    inc hl
    rl [hl]
    inc hl
    rl [hl]
    inc hl
    rl [hl]
    inc hl
    rl [hl]
    sbc a, a
    ld b, a
    ld hl, sp + 2 + F_D
    ld d, h
    ld e, l
    ld hl, sp + 2 + F_R
    ld a, [de]
    ld c, a
    ld a, [hl]
    sub c
    ld [hl+], a
    inc de
    ld a, [de]
    ld c, a
    ld a, [hl]
    sbc c
    ld [hl+], a
    inc de
    ld a, [de]
    ld c, a
    ld a, [hl]
    sbc c
    ld [hl+], a
    inc de
    ld a, [de]
    ld c, a
    ld a, [hl]
    sbc c
    ld [hl], a
    sbc a, a
    ld c, a
    ld a, b
    cpl
    and c
    jr nz, .restore
    ld hl, sp + 2 + F_N
    set 0, [hl]
    jr .next
.restore
    ld hl, sp + 2 + F_D
    ld d, h
    ld e, l
    ld hl, sp + 2 + F_R
    ld a, [de]
    add [hl]
    ld [hl+], a
    inc de
    ld a, [de]
    adc [hl]
    ld [hl+], a
    inc de
    ld a, [de]
    adc [hl]
    ld [hl+], a
    inc de
    ld a, [de]
    adc [hl]
    ld [hl], a
.next
    ld hl, sp + 2 + F_COUNT
    dec [hl]
    jr nz, .loop
    ret

EXPORT __gbc_udiv32
__gbc_udiv32::
    LOAD_DIV_ARGS
    call div32_core
    RETURN_FRAME_VALUE F_N
    add sp, FRAME
    ret

EXPORT __gbc_urem32
__gbc_urem32::
    LOAD_DIV_ARGS
    call div32_core
    RETURN_FRAME_VALUE F_R
    add sp, FRAME
    ret

; bit 0 of F_SIGNS: dividend negative, bit 1: divisor negative
MACRO SIGNED_PREPARE
    LOAD_DIV_ARGS
    ld hl, sp + F_SIGNS
    ld [hl], 0
    ld hl, sp + F_N + 3
    bit 7, [hl]
    jr z, .dividend_positive\@
    ld hl, sp + F_SIGNS
    set 0, [hl]
    NEG32 F_N
.dividend_positive\@
    ld hl, sp + F_D + 3
    bit 7, [hl]
    jr z, .divisor_positive\@
    ld hl, sp + F_SIGNS
    set 1, [hl]
    NEG32 F_D
.divisor_positive\@
ENDM

EXPORT __gbc_sdiv32
__gbc_sdiv32::
    SIGNED_PREPARE
    call div32_core
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

EXPORT __gbc_srem32
__gbc_srem32::
    SIGNED_PREPARE
    call div32_core
    ld hl, sp + F_SIGNS
    bit 0, [hl]
    jr z, .done
    NEG32 F_R
.done
    RETURN_FRAME_VALUE F_R
    add sp, FRAME
    ret

; load the 32-bit argument into B, C, D, E
MACRO LOAD_UNARY_ARG
    ld d, a
    ld hl, sp + 2
    ld e, [hl]
ENDM

EXPORT __gbc_popcnt32
__gbc_popcnt32::
    LOAD_UNARY_ARG
    ld h, 0
    FOR I, 4
        IF I == 0
            DEF REG EQUS "b"
        ELIF I == 1
            DEF REG EQUS "c"
        ELIF I == 2
            DEF REG EQUS "d"
        ELSE
            DEF REG EQUS "e"
        ENDC
.byte{u:I}
        ld a, {REG}
        or a
        jr z, .byte_done{u:I}
        srl {REG}
        jr nc, .byte{u:I}
        inc h
        jr .byte{u:I}
.byte_done{u:I}
        PURGE REG
    ENDR
    ld a, h
    ld b, 0
    ld c, 0
    ld d, 0
    ret

EXPORT __gbc_ctz32
__gbc_ctz32::
    LOAD_UNARY_ARG
    ld h, 0
    FOR I, 4
        IF I == 0
            DEF REG EQUS "b"
        ELIF I == 1
            DEF REG EQUS "c"
        ELIF I == 2
            DEF REG EQUS "d"
        ELSE
            DEF REG EQUS "e"
        ENDC
        ld a, {REG}
        or a
        jr z, .empty{u:I}
.shift{u:I}
        bit 0, {REG}
        jr nz, .found
        srl {REG}
        inc h
        jr .shift{u:I}
.empty{u:I}
        ld a, h
        add 8
        ld h, a
        PURGE REG
    ENDR
.found
    ld a, h
    ld b, 0
    ld c, 0
    ld d, 0
    ret

EXPORT __gbc_clz32
__gbc_clz32::
    LOAD_UNARY_ARG
    ld h, 0
    FOR I, 4
        IF I == 0
            DEF REG EQUS "e"
        ELIF I == 1
            DEF REG EQUS "d"
        ELIF I == 2
            DEF REG EQUS "c"
        ELSE
            DEF REG EQUS "b"
        ENDC
        ld a, {REG}
        or a
        jr z, .empty{u:I}
.shift{u:I}
        bit 7, {REG}
        jr nz, .found
        sla {REG}
        inc h
        jr .shift{u:I}
.empty{u:I}
        ld a, h
        add 8
        ld h, a
        PURGE REG
    ENDR
.found
    ld a, h
    ld b, 0
    ld c, 0
    ld d, 0
    ret

MACRO REVERSE_BYTE
    ld a, \1
    ld \1, 0
    ld l, 8
.bit\@
    rra
    rl \1
    dec l
    jr nz, .bit\@
ENDM

EXPORT __gbc_bitrev32
__gbc_bitrev32::
    LOAD_UNARY_ARG
    REVERSE_BYTE b
    REVERSE_BYTE c
    REVERSE_BYTE d
    REVERSE_BYTE e
    ld a, e
    ld e, b
    ld b, d
    ld d, e
    ret

; 8-bit shift or rotate by a variable count. Arguments: B value, C count,
; A op (0 shl, 1 ushr, 2 sshr, 3 rotl, 4 rotr). Result in A; the count is
; taken modulo 8 as CLIF specifies. Every register is clobbered.
EXPORT __gbc_shift8
__gbc_shift8::
    ld d, a
    ld a, c
    and 7
    ld c, a
    ld e, b
    ld b, d
.loop
    ld a, c
    or a
    jr z, .done
    dec c
    ld a, b
    or a
    jr z, .shl
    dec a
    jr z, .ushr
    dec a
    jr z, .sshr
    dec a
    jr z, .rotl
    rrc e
    jr .loop
.shl
    sla e
    jr .loop
.ushr
    srl e
    jr .loop
.sshr
    sra e
    jr .loop
.rotl
    rlc e
    jr .loop
.done
    ld a, e
    ret

; 16-bit shift or rotate by a variable count. Arguments: B,C value (low,
; high), A = op << 4 | count (op 0 shl, 1 ushr, 2 sshr, 3 rotl, 4 rotr,
; count 0..15). Result in A (low) and B (high). Every register is clobbered.
EXPORT __gbc_shift16
__gbc_shift16::
    ld d, a
    and $0f
    ld e, a
    ld a, d
    swap a
    and $0f
    ld d, a
.loop
    ld a, e
    or a
    jr z, .done
    dec e
    ld a, d
    or a
    jr z, .shl
    dec a
    jr z, .ushr
    dec a
    jr z, .sshr
    dec a
    jr z, .rotl
    srl c
    rr b
    jr nc, .loop
    set 7, c
    jr .loop
.shl
    sla b
    rl c
    jr .loop
.ushr
    srl c
    rr b
    jr .loop
.sshr
    sra c
    rr b
    jr .loop
.rotl
    sla b
    rl c
    jr nc, .loop
    set 0, b
    jr .loop
.done
    ld a, b
    ld b, c
    ret
