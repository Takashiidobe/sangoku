; Soft-float f32 (IEEE 754 binary32, round to nearest even).
;
; Calling convention matches wide32.asm: first i32 argument in B, C, A and the
; first stack byte (just above the return address); later arguments follow on
; the stack. Float results come back in A, B, C, D (low byte first); fcmp
; returns a bool in A. Every register is clobbered. Uses static WRAM, so it is
; not reentrant.
;
; Working mantissas hold the 24-bit significand shifted left by 7, so the
; leading bit is bit 30 and the low 7 bits carry guard, round and sticky bits.

SECTION "softfloat32 state", WRAM0

wA: ds 4
wB: ds 4
wR: ds 4
wXa: ds 7
wXb: ds 7
wE: ds 2
wS: ds 1
wM: ds 4
wP: ds 6
wMc: ds 3
wMask: ds 1
wCnt: ds 1
wRm: ds 4
wMk: ds 4
wFr: ds 4
wHf: ds 4
wRad: ds 8

DEF X_S EQU 0
DEF X_E EQU 1
DEF X_M EQU 3

SECTION "softfloat32", ROM0

MACRO LOAD_A
    ld [wA + 2], a
    ld a, b
    ld [wA], a
    ld a, c
    ld [wA + 1], a
    ld hl, sp + 2
    ld a, [hl]
    ld [wA + 3], a
ENDM

MACRO LOAD_AB
    LOAD_A
    ld hl, sp + 3
    ld a, [hl+]
    ld [wB], a
    ld a, [hl+]
    ld [wB + 1], a
    ld a, [hl+]
    ld [wB + 2], a
    ld a, [hl]
    ld [wB + 3], a
ENDM

; A = 0 finite, 1 infinity, 2 NaN for the float at \1. Clobbers E.
MACRO CLASSIFY
    ld a, [\1 + 3]
    and $7f
    cp $7f
    jr nz, .finite\@
    ld a, [\1 + 2]
    bit 7, a
    jr z, .finite\@
    and $7f
    ld e, a
    ld a, [\1 + 1]
    or e
    ld e, a
    ld a, [\1]
    or e
    jr z, .inf\@
    ld a, 2
    jr .end\@
.inf\@
    ld a, 1
    jr .end\@
.finite\@
    xor a
.end\@
ENDM

; unpack the float at \1 into the sign/exponent/mantissa record at \2
MACRO UNPACK
    ld a, [\1 + 3]
    and $80
    ld [\2 + X_S], a
    ld a, [\1 + 3]
    add a
    ld b, a
    ld a, [\1 + 2]
    rlca
    ld a, b
    adc 0
    ld [\2 + X_E], a
    ld c, a
    xor a
    ld [\2 + X_E + 1], a
    ld [\2 + X_M], a
    ld a, [\1]
    ld [\2 + X_M + 1], a
    ld a, [\1 + 1]
    ld [\2 + X_M + 2], a
    ld a, [\1 + 2]
    and $7f
    ld b, a
    ld a, c
    or a
    jr z, .subnormal\@
    set 7, b
    jr .store\@
.subnormal\@
    ld a, 1
    ld [\2 + X_E], a
.store\@
    ld a, b
    ld [\2 + X_M + 3], a
    ld hl, \2 + X_M + 3
    srl [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
ENDM

; shift the record's mantissa left until bit 30 is set, lowering its exponent
MACRO NORMALIZE
.again\@
    ld a, [\1 + X_M + 3]
    bit 6, a
    jr nz, .done\@
    ld hl, \1 + X_M
    call sf_shl1
    ld hl, \1 + X_E
    call sf_dec16
    jr .again\@
.done\@
ENDM

; hl -> 32-bit value; shift right one bit, jamming the lost bit into bit 0
sf_shr1s:
    inc hl
    inc hl
    inc hl
    srl [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
    ret nc
    set 0, [hl]
    ret

; hl -> 32-bit value; shift left one bit
sf_shl1:
    sla [hl]
    inc hl
    rl [hl]
    inc hl
    rl [hl]
    inc hl
    rl [hl]
    ret

; hl -> 16-bit value
sf_dec16:
    ld a, [hl]
    sub 1
    ld [hl+], a
    ld a, [hl]
    sbc 0
    ld [hl], a
    ret

sf_inc_e:
    ld hl, wE
    inc [hl]
    ret nz
    inc hl
    inc [hl]
    ret

sf_dec_e:
    ld hl, wE
    jr sf_dec16

; Z set when the 32-bit value at hl is zero
sf_is_zero32:
    ld a, [hl+]
    or [hl]
    inc hl
    or [hl]
    inc hl
    or [hl]
    ret

; Z set when the float at hl is +-0
sf_is_zero_mag:
    ld a, [hl+]
    or [hl]
    inc hl
    or [hl]
    inc hl
    ld b, a
    ld a, [hl]
    and $7f
    or b
    ret

; unsigned compare of the 32-bit values at hl (x) and de (y): C set if x < y, Z if equal
sf_cmp32:
    inc hl
    inc hl
    inc hl
    inc de
    inc de
    inc de
    ld c, 4
.next
    ld a, [de]
    ld b, a
    ld a, [hl]
    cp b
    ret nz
    dec hl
    dec de
    dec c
    jr nz, .next
    ret

; compare |wA| with |wB|: C set if smaller, Z if equal
sf_cmp_mag:
    ld hl, wA + 3
    ld de, wB + 3
    ld a, [de]
    and $7f
    ld b, a
    ld a, [hl]
    and $7f
    cp b
    ret nz
    ld c, 3
.next
    dec hl
    dec de
    ld a, [de]
    ld b, a
    ld a, [hl]
    cp b
    ret nz
    dec c
    jr nz, .next
    ret

; hl -> 32-bit x, de -> 32-bit y: x -= y
sf_sub32:
    ld a, [de]
    ld b, a
    ld a, [hl]
    sub b
    ld [hl+], a
    inc de
    ld a, [de]
    ld b, a
    ld a, [hl]
    sbc b
    ld [hl+], a
    inc de
    ld a, [de]
    ld b, a
    ld a, [hl]
    sbc b
    ld [hl+], a
    inc de
    ld a, [de]
    ld b, a
    ld a, [hl]
    sbc b
    ld [hl], a
    ret

; hl -> 64-bit value; shift left one bit, carry out holds the old top bit
sf_shl64:
    sla [hl]
    FOR i, 7
    inc hl
    rl [hl]
    ENDR
    ret

sf_swap_x:
    ld hl, wXa
    ld de, wXb
    ld c, 7
.next
    ld a, [de]
    ld b, [hl]
    ld [hl+], a
    ld a, b
    ld [de], a
    inc de
    dec c
    jr nz, .next
    ret

sf_ret_nan:
    xor a
    ld b, a
    ld c, $c0
    ld d, $7f
    ret

sf_ret_a:
    ld hl, wA
    jr sf_ret_hl

sf_ret_b:
    ld hl, wB
    jr sf_ret_hl

sf_return:
    ld hl, wR
sf_ret_hl:
    ld a, [hl+]
    ld b, [hl]
    inc hl
    ld c, [hl]
    inc hl
    ld d, [hl]
    ret

; signed zero with the sign in wS
sf_ret_zero:
    xor a
    ld [wR], a
    ld [wR + 1], a
    ld [wR + 2], a
    ld a, [wS]
    ld [wR + 3], a
    jr sf_return

sf_ret_inf:
    xor a
    ld [wR], a
    ld [wR + 1], a
    ld a, $80
    ld [wR + 2], a
    ld a, [wS]
    or $7f
    ld [wR + 3], a
    jr sf_return

; Round wM (leading bit 30, or lower when subnormal) with sign wS and
; exponent wE (signed 16-bit, may be <= 0) and pack the result into wR.
sf_roundpack:
    ld a, [wE + 1]
    bit 7, a
    jr nz, .denormal
    ld b, a
    ld a, [wE]
    or b
    jr nz, .round
.denormal
    ld a, [wE]
    cpl
    add 2
    ld l, a
    ld a, [wE + 1]
    cpl
    adc 0
    ld h, a
    ld a, 1
    ld [wE], a
    xor a
    ld [wE + 1], a
    ld a, h
    or a
    jr nz, .jam
    ld a, l
    cp 32
    jr nc, .jam
    ld b, l
    ld hl, wM
.shift
    call sf_shr1s
    dec b
    jr nz, .shift
    jr .round
.jam
    ld hl, wM
    call sf_is_zero32
    ld a, 0
    jr z, .store_jam
    ld a, 1
.store_jam
    ld hl, wM
    ld [hl+], a
    xor a
    ld [hl+], a
    ld [hl+], a
    ld [hl], a
.round
    ld a, [wM]
    ld b, a
    and $7f
    cp $40
    ld c, 0
    jr c, .no_increment
    jr nz, .increment
    bit 7, b
    jr z, .no_increment
.increment
    ld c, 1
.no_increment
    ld hl, wM
    call sf_shl1
    ld a, c
    or a
    jr z, .pack
    ld hl, wM + 1
    inc [hl]
    jr nz, .pack
    inc hl
    inc [hl]
    jr nz, .pack
    inc hl
    inc [hl]
    jr nz, .pack
    ld a, $80
    ld [wM + 3], a
    call sf_inc_e
.pack
    ld a, [wE + 1]
    or a
    jp nz, sf_ret_inf
    ld a, [wE]
    cp 255
    jp nc, sf_ret_inf
    ld a, [wM + 3]
    bit 7, a
    jr nz, .normal
    ld d, 0
    jr .have_field
.normal
    ld a, [wE]
    ld d, a
.have_field
    ld a, [wM + 3]
    and $7f
    ld c, a
    ld a, d
    srl a
    ld b, a
    ld a, c
    jr nc, .store_high
    or $80
.store_high
    ld [wR + 2], a
    ld a, [wS]
    or b
    ld [wR + 3], a
    ld a, [wM + 1]
    ld [wR], a
    ld a, [wM + 2]
    ld [wR + 1], a
    jp sf_return

EXPORT __gbc_fadd32
__gbc_fadd32::
    LOAD_AB
    CLASSIFY wA
    cp 2
    jp z, sf_ret_nan
    ld d, a
    CLASSIFY wB
    cp 2
    jp z, sf_ret_nan
    ld e, a
    ld a, d
    or e
    jr z, .finite
    ld a, d
    or a
    jp z, sf_ret_b
    ld a, e
    or a
    jp z, sf_ret_a
    ld a, [wA + 3]
    ld b, a
    ld a, [wB + 3]
    xor b
    and $80
    jp nz, sf_ret_nan
    jp sf_ret_a
.finite
    UNPACK wA, wXa
    UNPACK wB, wXb
    ld a, [wXa + X_E]
    ld b, a
    ld a, [wXb + X_E]
    cp b
    jr c, .ordered
    jr nz, .swap
    ld hl, wXa + X_M
    ld de, wXb + X_M
    call sf_cmp32
    jr nc, .ordered
.swap
    call sf_swap_x
.ordered
    ld a, [wXb + X_E]
    ld b, a
    ld a, [wXa + X_E]
    sub b
    jr z, .aligned
    ld b, a
    cp 32
    jr nc, .jam
    ld hl, wXb + X_M
.align
    call sf_shr1s
    dec b
    jr nz, .align
    jr .aligned
.jam
    ld hl, wXb + X_M
    call sf_is_zero32
    ld a, 0
    jr z, .store_jam
    ld a, 1
.store_jam
    ld hl, wXb + X_M
    ld [hl+], a
    xor a
    ld [hl+], a
    ld [hl+], a
    ld [hl], a
.aligned
    ld a, [wXa + X_E]
    ld [wE], a
    xor a
    ld [wE + 1], a
    ld a, [wXa + X_S]
    ld [wS], a
    ld b, a
    ld a, [wXb + X_S]
    xor b
    jr nz, .subtract
    ld hl, wXa + X_M
    ld de, wXb + X_M
    ld a, [de]
    add [hl]
    ld [wM], a
    inc de
    inc hl
    ld a, [de]
    adc [hl]
    ld [wM + 1], a
    inc de
    inc hl
    ld a, [de]
    adc [hl]
    ld [wM + 2], a
    inc de
    inc hl
    ld a, [de]
    adc [hl]
    ld [wM + 3], a
    bit 7, a
    jr z, .normalize
    ld hl, wM
    call sf_shr1s
    call sf_inc_e
    jr .normalize
.subtract
    ld hl, wXa + X_M
    ld de, wXb + X_M
    ld a, [hl]
    ld b, a
    ld a, [de]
    ld c, a
    ld a, b
    sub c
    ld [wM], a
    inc de
    inc hl
    ld a, [de]
    ld c, a
    ld a, [hl]
    sbc c
    ld [wM + 1], a
    inc de
    inc hl
    ld a, [de]
    ld c, a
    ld a, [hl]
    sbc c
    ld [wM + 2], a
    inc de
    inc hl
    ld a, [de]
    ld c, a
    ld a, [hl]
    sbc c
    ld [wM + 3], a
    xor a
    ld [wS], a
    ld hl, wM
    call sf_is_zero32
    jr z, .zero
    ld a, [wXa + X_S]
    ld [wS], a
.normalize
    ld hl, wM
    call sf_is_zero32
    jr z, .zero
.step
    ld a, [wM + 3]
    bit 6, a
    jr nz, .done
    ld a, [wE]
    dec a
    ld b, a
    ld a, [wE + 1]
    or b
    jr z, .done
    ld hl, wM
    call sf_shl1
    call sf_dec_e
    jr .step
.done
    jp sf_roundpack
.zero
    jp sf_ret_zero

EXPORT __gbc_fmul32
__gbc_fmul32::
    LOAD_AB
    CLASSIFY wA
    cp 2
    jp z, sf_ret_nan
    ld d, a
    CLASSIFY wB
    cp 2
    jp z, sf_ret_nan
    ld e, a
    ld a, [wA + 3]
    ld b, a
    ld a, [wB + 3]
    xor b
    and $80
    ld [wS], a
    ld a, d
    or e
    jr z, .finite
    ld a, d
    or a
    jr nz, .a_infinite
    ld hl, wA
    call sf_is_zero_mag
    jp z, sf_ret_nan
    jp sf_ret_inf
.a_infinite
    ld a, e
    or a
    jp nz, sf_ret_inf
    ld hl, wB
    call sf_is_zero_mag
    jp z, sf_ret_nan
    jp sf_ret_inf
.finite
    ld hl, wA
    call sf_is_zero_mag
    jp z, sf_ret_zero
    ld hl, wB
    call sf_is_zero_mag
    jp z, sf_ret_zero
    UNPACK wA, wXa
    UNPACK wB, wXb
    NORMALIZE wXa
    NORMALIZE wXb
    ld a, [wXa + X_E]
    ld hl, wXb + X_E
    add [hl]
    ld [wE], a
    ld a, [wXa + X_E + 1]
    inc hl
    adc [hl]
    ld [wE + 1], a
    ld a, [wE]
    sub 127
    ld [wE], a
    ld a, [wE + 1]
    sbc 0
    ld [wE + 1], a
    ld hl, wXa + X_M
    call sf_shl1
    ld a, [wXa + X_M + 1]
    ld [wMc], a
    ld a, [wXa + X_M + 2]
    ld [wMc + 1], a
    ld a, [wXa + X_M + 3]
    ld [wMc + 2], a
    ld hl, wXb + X_M
    call sf_shl1
    ld a, [wXb + X_M + 1]
    ld [wP], a
    ld a, [wXb + X_M + 2]
    ld [wP + 1], a
    ld a, [wXb + X_M + 3]
    ld [wP + 2], a
    xor a
    ld [wP + 3], a
    ld [wP + 4], a
    ld [wP + 5], a
    ld b, 24
.multiply
    ld hl, wP
    bit 0, [hl]
    jr z, .no_add
    ld hl, wP + 3
    ld de, wMc
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
    ld [hl], a
    jr .shift
.no_add
    and a
.shift
    ld hl, wP + 5
    rr [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
    dec b
    jr nz, .multiply
    ld a, [wP + 2]
    ld [wM], a
    ld a, [wP + 3]
    ld [wM + 1], a
    ld a, [wP + 4]
    ld [wM + 2], a
    ld a, [wP + 5]
    ld [wM + 3], a
    bit 7, a
    jr z, .sticky
    ld hl, wM
    call sf_shr1s
    call sf_inc_e
.sticky
    ld a, [wP]
    ld b, a
    ld a, [wP + 1]
    or b
    jr z, .round
    ld hl, wM
    set 0, [hl]
.round
    jp sf_roundpack

; compare: A = 0 equal, 1 less, 2 greater, 3 unordered
sf_compare:
    CLASSIFY wA
    cp 2
    jr z, .unordered
    CLASSIFY wB
    cp 2
    jr z, .unordered
    ld hl, wA
    call sf_is_zero_mag
    jr nz, .nonzero
    ld hl, wB
    call sf_is_zero_mag
    jr nz, .nonzero
    xor a
    ret
.nonzero
    ld a, [wA + 3]
    ld b, a
    ld a, [wB + 3]
    xor b
    and $80
    jr z, .same_sign
    bit 7, b
    ld a, 1
    ret nz
    ld a, 2
    ret
.same_sign
    call sf_cmp_mag
    jr z, .equal
    ld a, 0
    rla
    ld b, a
    ld a, [wA + 3]
    rlca
    and 1
    xor b
    jr z, .greater
    ld a, 1
    ret
.greater
    ld a, 2
    ret
.equal
    xor a
    ret
.unordered
    ld a, 3
    ret

EXPORT __gbc_fcmp32
__gbc_fcmp32::
    LOAD_AB
    ld hl, sp + 7
    ld a, [hl]
    ld [wMask], a
    call sf_compare
    ld b, a
    ld a, [wMask]
    inc b
.shift
    dec b
    jr z, .done
    srl a
    jr .shift
.done
    and 1
    ret

EXPORT __gbc_fmin32
__gbc_fmin32::
    LOAD_AB
    call sf_compare
    cp 3
    jp z, sf_ret_nan
    cp 1
    jp z, sf_ret_a
    cp 2
    jp z, sf_ret_b
    ld a, [wA + 3]
    bit 7, a
    jp nz, sf_ret_a
    jp sf_ret_b

EXPORT __gbc_fmax32
__gbc_fmax32::
    LOAD_AB
    call sf_compare
    cp 3
    jp z, sf_ret_nan
    cp 1
    jp z, sf_ret_b
    cp 2
    jp z, sf_ret_a
    ld a, [wA + 3]
    bit 7, a
    jp nz, sf_ret_b
    jp sf_ret_a

EXPORT __gbc_fdiv32
__gbc_fdiv32::
    LOAD_AB
    CLASSIFY wA
    cp 2
    jp z, sf_ret_nan
    ld d, a
    CLASSIFY wB
    cp 2
    jp z, sf_ret_nan
    ld e, a
    ld a, [wA + 3]
    ld b, a
    ld a, [wB + 3]
    xor b
    and $80
    ld [wS], a
    ld a, d
    or e
    jr z, .finite
    ld a, d
    or a
    jp z, sf_ret_zero
    ld a, e
    or a
    jp nz, sf_ret_nan
    jp sf_ret_inf
.finite
    ld hl, wB
    call sf_is_zero_mag
    jr nz, .divisor_nonzero
    ld hl, wA
    call sf_is_zero_mag
    jp z, sf_ret_nan
    jp sf_ret_inf
.divisor_nonzero
    ld hl, wA
    call sf_is_zero_mag
    jp z, sf_ret_zero
    UNPACK wA, wXa
    UNPACK wB, wXb
    NORMALIZE wXa
    NORMALIZE wXb
    ld a, [wXa + X_E]
    ld hl, wXb + X_E
    sub [hl]
    ld [wE], a
    ld a, [wXa + X_E + 1]
    inc hl
    sbc [hl]
    ld [wE + 1], a
    ld a, [wE]
    add 127
    ld [wE], a
    ld a, [wE + 1]
    adc 0
    ld [wE + 1], a
    FOR i, 4
    ld a, [wXa + X_M + i]
    ld [wRm + i], a
    ENDR
    ld hl, wXa + X_M
    ld de, wXb + X_M
    call sf_cmp32
    jr nc, .quotient
    ld hl, wRm
    call sf_shl1
    call sf_dec_e
.quotient
    xor a
    ld [wM], a
    ld [wM + 1], a
    ld [wM + 2], a
    ld [wM + 3], a
    ld a, 26
    ld [wCnt], a
.divide
    ld hl, wM
    call sf_shl1
    ld hl, wRm
    ld de, wXb + X_M
    call sf_cmp32
    jr c, .no_subtract
    ld hl, wRm
    ld de, wXb + X_M
    call sf_sub32
    ld hl, wM
    set 0, [hl]
.no_subtract
    ld hl, wRm
    call sf_shl1
    ld hl, wCnt
    dec [hl]
    jr nz, .divide
    ld hl, wM
    REPT 5
    call sf_shl1
    ld hl, wM
    ENDR
    ld hl, wRm
    call sf_is_zero32
    jp z, sf_roundpack
    ld hl, wM
    set 0, [hl]
    jp sf_roundpack

EXPORT __gbc_fsqrt32
__gbc_fsqrt32::
    LOAD_A
    CLASSIFY wA
    cp 2
    jp z, sf_ret_nan
    ld d, a
    ld hl, wA
    call sf_is_zero_mag
    jp z, sf_ret_a
    ld a, [wA + 3]
    bit 7, a
    jp nz, sf_ret_nan
    ld a, d
    or a
    jp nz, sf_ret_a
    xor a
    ld [wS], a
    UNPACK wA, wXa
    NORMALIZE wXa
    ld a, [wXa + X_E]
    bit 0, a
    jr nz, .exponent_odd
    ld hl, wXa + X_M
    call sf_shl1
    ld a, 126
    jr .have_bias
.exponent_odd
    ld a, 127
.have_bias
    ld hl, wXa + X_E
    add [hl]
    ld [wE], a
    inc hl
    ld a, [hl]
    adc 0
    sra a
    ld [wE + 1], a
    ld a, [wE]
    rra
    ld [wE], a
    xor a
    FOR i, 4
    ld [wRad + i], a
    ld [wRm + i], a
    ld [wM + i], a
    ENDR
    FOR i, 4
    ld a, [wXa + X_M + i]
    ld [wRad + 4 + i], a
    ENDR
    ld a, 26
    ld [wCnt], a
.root
    REPT 2
    ld hl, wRad
    call sf_shl64
    ld hl, wRm
    rl [hl]
    inc hl
    rl [hl]
    inc hl
    rl [hl]
    inc hl
    rl [hl]
    ENDR
    FOR i, 4
    ld a, [wM + i]
    ld [wFr + i], a
    ENDR
    ld hl, wFr
    call sf_shl1
    ld hl, wFr
    call sf_shl1
    ld hl, wFr
    set 0, [hl]
    ld hl, wM
    call sf_shl1
    ld hl, wRm
    ld de, wFr
    call sf_cmp32
    jr c, .next_digit
    ld hl, wRm
    ld de, wFr
    call sf_sub32
    ld hl, wM
    set 0, [hl]
.next_digit
    ld hl, wCnt
    dec [hl]
    jr nz, .root
    ld hl, wM
    REPT 5
    call sf_shl1
    ld hl, wM
    ENDR
    ld hl, wRm
    call sf_is_zero32
    jp z, sf_roundpack
    ld hl, wM
    set 0, [hl]
    jp sf_roundpack

; clear the fraction bits (wMk mask) of wM
sf_clear_frac:
    FOR i, 4
    ld a, [wMk + i]
    cpl
    ld b, a
    ld a, [wM + i]
    and b
    ld [wM + i], a
    ENDR
    ret

MACRO TIE_BYTE
    ld a, [wM + \1]
    ld b, a
    ld a, [wHf + \1]
    and b
ENDM

; mode byte after the float argument: 0 trunc, 1 floor, 2 ceil, 3 nearest even
EXPORT __gbc_fround32
__gbc_fround32::
    LOAD_A
    ld hl, sp + 3
    ld a, [hl]
    ld [wMask], a
    CLASSIFY wA
    cp 2
    jp z, sf_ret_nan
    or a
    jp nz, sf_ret_a
    ld a, [wA + 3]
    and $80
    ld [wS], a
    ld a, [wA + 3]
    add a
    ld b, a
    ld a, [wA + 2]
    rlca
    ld a, b
    adc 0
    ld d, a
    cp 150
    jp nc, sf_ret_a
    cp 127
    jp c, .small
    ld a, 150
    sub d
    ld c, a
    xor a
    FOR i, 4
    ld [wMk + i], a
    ENDR
.build_mask
    ld hl, wMk
    call sf_shl1
    ld hl, wMk
    set 0, [hl]
    dec c
    jr nz, .build_mask
    ld a, [wA]
    ld [wM], a
    ld a, [wA + 1]
    ld [wM + 1], a
    ld a, [wA + 2]
    ld [wM + 2], a
    ld a, [wA + 3]
    and $7f
    ld [wM + 3], a
    FOR i, 4
    ld a, [wM + i]
    ld b, a
    ld a, [wMk + i]
    and b
    ld [wFr + i], a
    ENDR
    ld hl, wFr
    call sf_is_zero32
    jp z, sf_ret_a
    ld a, [wMk]
    add 1
    ld [wHf], a
    FOR i, 1, 4
    ld a, [wMk + i]
    adc 0
    ld [wHf + i], a
    ENDR
    ld a, [wMask]
    or a
    jp z, .down
    cp 1
    jp z, .floor
    cp 2
    jp z, .ceil
    ld hl, wFr
    call sf_shl1
    ld hl, wFr
    ld de, wHf
    call sf_cmp32
    jp c, .down
    jp nz, .up
    TIE_BYTE 0
    ld c, a
    TIE_BYTE 1
    or c
    ld c, a
    TIE_BYTE 2
    or c
    ld c, a
    TIE_BYTE 3
    or c
    jp nz, .up
    jr .down
.floor
    ld a, [wS]
    or a
    jp nz, .up
    jr .down
.ceil
    ld a, [wS]
    or a
    jr z, .up
.down
    call sf_clear_frac
    jr .emit
.up
    call sf_clear_frac
    FOR i, 4
    ld a, [wMk + i]
    ld b, a
    ld a, [wM + i]
    or b
    ld [wM + i], a
    ENDR
    ld hl, wM
    inc [hl]
    jp nz, .emit
    inc hl
    inc [hl]
    jp nz, .emit
    inc hl
    inc [hl]
    jp nz, .emit
    inc hl
    inc [hl]
.emit
    ld a, [wM]
    ld [wR], a
    ld a, [wM + 1]
    ld [wR + 1], a
    ld a, [wM + 2]
    ld [wR + 2], a
    ld a, [wS]
    ld b, a
    ld a, [wM + 3]
    or b
    ld [wR + 3], a
    jp sf_return
.small
    ld hl, wA
    call sf_is_zero_mag
    jp z, sf_ret_a
    ld a, [wMask]
    or a
    jp z, sf_ret_zero
    cp 1
    jr z, .small_floor
    cp 2
    jr z, .small_ceil
    ld a, d
    cp 126
    jp nz, sf_ret_zero
    ld hl, wA
    ld a, [hl+]
    or [hl]
    inc hl
    ld b, a
    ld a, [hl]
    and $7f
    or b
    jp z, sf_ret_zero
.one
    xor a
    ld [wR], a
    ld [wR + 1], a
    ld a, $80
    ld [wR + 2], a
    ld a, [wS]
    or $3f
    ld [wR + 3], a
    jp sf_return
.small_floor
    ld a, [wS]
    or a
    jr nz, .one
    jp sf_ret_zero
.small_ceil
    ld a, [wS]
    or a
    jr z, .one
    jp sf_ret_zero
