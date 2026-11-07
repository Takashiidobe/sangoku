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
wC: ds 4
wKa: ds 1
wKb: ds 1
wKc: ds 1
wSp: ds 1
wSc: ds 1
wEp: ds 2
wEc: ds 2
wFp: ds 8
wFc: ds 8

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

; hl -> 32-bit value; shift right one bit
sf_shr1:
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

; hl -> 64-bit value; shift right one bit, jamming the lost bit into bit 0
sf_shr1j64:
    FOR i, 7
    inc hl
    ENDR
    srl [hl]
    FOR i, 7
    dec hl
    rr [hl]
    ENDR
    ret nc
    set 0, [hl]
    ret

; hl -> 64-bit value, bc = shift count (capped at 64): shift right with jamming
sf_shift_jam:
    ld a, b
    or a
    jr nz, .cap
    ld a, c
    cp 64
    jr c, .go
.cap
    ld c, 64
.go
    ld a, c
    or a
    ret z
.loop
    push hl
    call sf_shr1j64
    pop hl
    dec c
    jr nz, .loop
    ret

; hl -> 64-bit x, de -> 64-bit y: x += y
sf_add64:
    ld a, [de]
    add [hl]
    ld [hl+], a
    inc de
    FOR i, 7
    ld a, [de]
    adc [hl]
    ld [hl+], a
    inc de
    ENDR
    ret

; hl -> 64-bit x, de -> 64-bit y: x -= y
sf_sub64:
    ld a, [de]
    ld b, a
    ld a, [hl]
    sub b
    ld [hl+], a
    inc de
    FOR i, 7
    ld a, [de]
    ld b, a
    ld a, [hl]
    sbc b
    ld [hl+], a
    inc de
    ENDR
    ret

; unsigned compare of the 64-bit values at hl (x) and de (y): C if x < y, Z if equal
sf_cmp64:
    FOR i, 7
    inc hl
    inc de
    ENDR
    ld c, 8
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

sf_ret_c:
    ld hl, wC
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

; wXa/wXb normalized: wP (48 bits) = significand product
sf_mul48:
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
    ret

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
    call sf_mul48
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

; i32 argument, then a byte: 1 when the integer is signed
EXPORT __gbc_fcvtfromint32
__gbc_fcvtfromint32::
    LOAD_A
    ld hl, sp + 3
    ld a, [hl]
    ld [wMask], a
    xor a
    ld [wS], a
    ld a, [wMask]
    or a
    jr z, .magnitude
    ld a, [wA + 3]
    bit 7, a
    jr z, .magnitude
    ld a, $80
    ld [wS], a
    ld a, [wA]
    cpl
    add 1
    ld [wA], a
    FOR i, 1, 4
    ld a, [wA + i]
    cpl
    adc 0
    ld [wA + i], a
    ENDR
.magnitude
    ld hl, wA
    call sf_is_zero32
    jp z, sf_ret_zero
    FOR i, 4
    ld a, [wA + i]
    ld [wM + i], a
    ENDR
    ld a, 158
    ld [wE], a
    xor a
    ld [wE + 1], a
.normalize
    ld a, [wM + 3]
    bit 7, a
    jr nz, .normalized
    ld hl, wM
    call sf_shl1
    call sf_dec_e
    jr .normalize
.normalized
    ld hl, wM
    call sf_shr1s
    jp sf_roundpack

; f32 argument, then a mode byte: bit 0 unsigned, bit 1 saturating,
; bits 2-3 result width code (0 = 8, 1 = 16, 2 = 32 bits). Out of range or
; NaN traps unless saturating; the result is a sign-extended i32.
EXPORT __gbc_fcvttoint32
__gbc_fcvttoint32::
    LOAD_A
    ld hl, sp + 3
    ld a, [hl]
    ld [wMask], a
    CLASSIFY wA
    cp 2
    jp z, .nan
    ld d, a
    ld a, [wA + 3]
    and $80
    ld [wS], a
    ld a, [wMask]
    rrca
    rrca
    and 3
    ld b, a
    xor a
    ld [wMk + 1], a
    ld [wMk + 2], a
    ld [wMk + 3], a
    ld a, $ff
    ld [wMk], a
    ld a, b
    or a
    jr z, .mask_done
    ld a, $ff
    ld [wMk + 1], a
    ld a, b
    cp 2
    jr c, .mask_done
    ld a, $ff
    ld [wMk + 2], a
    ld [wMk + 3], a
.mask_done
    ld a, [wMask]
    bit 0, a
    jr nz, .unsigned_limit
    ld hl, wMk + 3
    srl [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
    ld a, [wS]
    or a
    jr z, .have_limit
    ld hl, wMk
    inc [hl]
    jr nz, .have_limit
    inc hl
    inc [hl]
    jr nz, .have_limit
    inc hl
    inc [hl]
    jr nz, .have_limit
    inc hl
    inc [hl]
    jr .have_limit
.unsigned_limit
    ld a, [wS]
    or a
    jr z, .have_limit
    xor a
    FOR i, 4
    ld [wMk + i], a
    ENDR
.have_limit
    ld a, d
    or a
    jr nz, .overflow
    ld a, [wA + 3]
    add a
    ld b, a
    ld a, [wA + 2]
    rlca
    ld a, b
    adc 0
    ld d, a
    cp 127
    jr c, .zero
    cp 159
    jr nc, .overflow
    ld a, [wA]
    ld [wM], a
    ld a, [wA + 1]
    ld [wM + 1], a
    ld a, [wA + 2]
    and $7f
    or $80
    ld [wM + 2], a
    xor a
    ld [wM + 3], a
    ld a, d
    sub 127
    cp 23
    jr c, .shift_right
    sub 23
    jr z, .have_magnitude
    ld c, a
.shift_left
    ld hl, wM
    call sf_shl1
    dec c
    jr nz, .shift_left
    jr .have_magnitude
.shift_right
    ld c, a
    ld a, 23
    sub c
    ld c, a
.shift_right_loop
    ld hl, wM
    call sf_shr1
    dec c
    jr nz, .shift_right_loop
.have_magnitude
    ld hl, wM
    ld de, wMk
    call sf_cmp32
    jr c, .emit
    jr z, .emit
.overflow
    ld a, [wMask]
    bit 1, a
    jr nz, .saturate
    rst $10
.saturate
    FOR i, 4
    ld a, [wMk + i]
    ld [wM + i], a
    ENDR
    jr .emit
.zero
    xor a
    FOR i, 4
    ld [wM + i], a
    ENDR
.emit
    ld a, [wS]
    or a
    jr z, .return
    ld a, [wM]
    cpl
    add 1
    ld [wM], a
    FOR i, 1, 4
    ld a, [wM + i]
    cpl
    adc 0
    ld [wM + i], a
    ENDR
.return
    ld hl, wM
    jp sf_ret_hl
.nan
    ld a, [wMask]
    bit 1, a
    jr nz, .nan_zero
    rst $10
.nan_zero
    xor a
    ld b, a
    ld c, a
    ld d, a
    ret

; f16 bits zero-extended to i32 in, f32 bits out
EXPORT __gbc_f16tof32
__gbc_f16tof32::
    LOAD_A
    ld a, [wA + 1]
    and $80
    ld [wS], a
    ld a, [wA + 1]
    and $7c
    rrca
    rrca
    ld d, a
    ld a, [wA]
    ld [wM], a
    ld a, [wA + 1]
    and 3
    ld [wM + 1], a
    ld a, d
    cp 31
    jr z, .special
    or a
    jr nz, .normal
    ld a, [wM]
    ld b, a
    ld a, [wM + 1]
    or b
    jp z, sf_ret_zero
    ld a, 113
    ld [wE], a
.subnormal
    ld a, [wM + 1]
    bit 2, a
    jr nz, .subnormal_done
    ld hl, wM
    sla [hl]
    inc hl
    rl [hl]
    ld hl, wE
    dec [hl]
    jr .subnormal
.subnormal_done
    ld a, [wM + 1]
    and 3
    ld [wM + 1], a
    ld a, [wE]
    ld d, a
    jr .pack
.normal
    ld a, d
    add 112
    ld d, a
.pack
    ld a, [wM]
    and 7
    swap a
    sla a
    ld [wR + 1], a
    ld a, [wM + 1]
    rrca
    rrca
    rrca
    ld b, a
    ld a, [wM]
    rrca
    rrca
    rrca
    and $1f
    or b
    ld b, a
    ld a, d
    srl a
    ld c, a
    ld a, b
    jr nc, .store_high
    or $80
.store_high
    ld [wR + 2], a
    ld a, [wS]
    or c
    ld [wR + 3], a
    xor a
    ld [wR], a
    jp sf_return
.special
    ld a, [wM]
    ld b, a
    ld a, [wM + 1]
    or b
    jp nz, sf_ret_nan
    jp sf_ret_inf

; f32 in, f16 bits out in A, B (C and D are zero), round to nearest even
EXPORT __gbc_f32tof16
__gbc_f32tof16::
    LOAD_A
    CLASSIFY wA
    ld d, a
    ld a, [wA + 3]
    and $80
    ld [wS], a
    ld a, d
    cp 2
    jp z, .nan
    or a
    jp nz, .inf
    ld hl, wA
    call sf_is_zero_mag
    jp z, .zero
    UNPACK wA, wXa
    NORMALIZE wXa
    ld a, [wXa + X_E]
    sub 112
    ld [wE], a
    ld a, [wXa + X_E + 1]
    sbc 0
    ld [wE + 1], a
    FOR i, 4
    ld a, [wXa + X_M + i]
    ld [wM + i], a
    ENDR
    ld a, [wE + 1]
    bit 7, a
    jr nz, .low_exponent
    or a
    jp nz, .inf
    ld a, [wE]
    cp 31
    jp nc, .inf
    or a
    jr nz, .extract
.low_exponent
    ld a, [wE]
    cpl
    add 2
    ld l, a
    ld a, [wE + 1]
    cpl
    adc 0
    or a
    jp nz, .zero
    ld a, l
    cp 12
    jp nc, .zero
    ld b, a
.denormalize
    ld hl, wM
    call sf_shr1s
    dec b
    jr nz, .denormalize
    ld a, 1
    ld [wE], a
.extract
    ld a, [wM + 3]
    swap a
    ld b, a
    and $0f
    ld h, a
    ld a, b
    and $f0
    ld b, a
    ld a, [wM + 2]
    swap a
    and $0f
    or b
    ld l, a
    ld a, [wE]
    dec a
    sla a
    sla a
    add h
    ld h, a
    ld a, [wM + 2]
    bit 3, a
    jr z, .pack
    and 7
    ld b, a
    ld a, [wM + 1]
    or b
    ld b, a
    ld a, [wM]
    or b
    jr nz, .round_up
    ld a, l
    bit 0, a
    jr z, .pack
.round_up
    inc l
    jr nz, .pack
    inc h
.pack
    ld a, h
    cp $7c
    jr c, .out
    ld h, $7c
    ld l, 0
.out
    ld a, [wS]
    or h
    ld b, a
    ld a, l
    ld c, 0
    ld d, 0
    ret
.nan
    xor a
    ld b, $7e
    ld c, a
    ld d, a
    ret
.inf
    ld a, [wS]
    or $7c
    ld b, a
    xor a
    ld c, a
    ld d, a
    ret
.zero
    ld a, [wS]
    ld b, a
    xor a
    ld c, a
    ld d, a
    ret

; a * b + c with a single rounding; fourth argument byte: 1 rounds the result
; to odd at f32 precision (for f16 callers that demote afterwards)
EXPORT __gbc_ffma32
__gbc_ffma32::
    LOAD_AB
    ld hl, sp + 7
    ld a, [hl+]
    ld [wC], a
    ld a, [hl+]
    ld [wC + 1], a
    ld a, [hl+]
    ld [wC + 2], a
    ld a, [hl+]
    ld [wC + 3], a
    ld a, [hl]
    ld [wMask], a
    CLASSIFY wA
    cp 2
    jp z, sf_ret_nan
    ld [wKa], a
    CLASSIFY wB
    cp 2
    jp z, sf_ret_nan
    ld [wKb], a
    CLASSIFY wC
    cp 2
    jp z, sf_ret_nan
    ld [wKc], a
    ld a, [wA + 3]
    ld b, a
    ld a, [wB + 3]
    xor b
    and $80
    ld [wSp], a
    ld a, [wC + 3]
    and $80
    ld [wSc], a
    ld a, [wKa]
    ld b, a
    ld a, [wKb]
    or b
    jr z, .product_finite
    ld a, [wKa]
    or a
    jr nz, .a_infinite
    ld hl, wA
    call sf_is_zero_mag
    jp z, sf_ret_nan
    jr .product_infinite
.a_infinite
    ld a, [wKb]
    or a
    jr nz, .product_infinite
    ld hl, wB
    call sf_is_zero_mag
    jp z, sf_ret_nan
.product_infinite
    ld a, [wKc]
    or a
    jr z, .return_product_infinity
    ld a, [wSp]
    ld b, a
    ld a, [wSc]
    xor b
    jp nz, sf_ret_nan
.return_product_infinity
    ld a, [wSp]
    ld [wS], a
    jp sf_ret_inf
.product_finite
    ld a, [wKc]
    or a
    jp nz, sf_ret_c
    ld hl, wA
    call sf_is_zero_mag
    jr z, .product_zero
    ld hl, wB
    call sf_is_zero_mag
    jr nz, .nonzero_product
.product_zero
    ld hl, wC
    call sf_is_zero_mag
    jp nz, sf_ret_c
    ld a, [wSp]
    ld b, a
    ld a, [wSc]
    xor b
    jr z, .same_zero_sign
    xor a
    ld [wS], a
    jp sf_ret_zero
.same_zero_sign
    ld a, b
    ld [wS], a
    jp sf_ret_zero
.nonzero_product
    UNPACK wA, wXa
    UNPACK wB, wXb
    NORMALIZE wXa
    NORMALIZE wXb
    ld a, [wXa + X_E]
    ld hl, wXb + X_E
    add [hl]
    ld [wEp], a
    ld a, [wXa + X_E + 1]
    inc hl
    adc [hl]
    ld [wEp + 1], a
    ld a, [wEp]
    sub 127
    ld [wEp], a
    ld a, [wEp + 1]
    sbc 0
    ld [wEp + 1], a
    call sf_mul48
    xor a
    FOR i, 8
    ld [wFp + i], a
    ld [wFc + i], a
    ENDR
    FOR i, 6
    ld a, [wP + i]
    ld [wFp + 2 + i], a
    ENDR
    ld hl, wFp
    call sf_shr1j64
    ld hl, wC
    call sf_is_zero_mag
    jr nz, .addend_nonzero
    ld a, [wSp]
    ld [wSc], a
    ld a, [wEp]
    ld [wEc], a
    ld a, [wEp + 1]
    ld [wEc + 1], a
    jp .have_addend
.addend_nonzero
    UNPACK wC, wXb
    NORMALIZE wXb
    ld a, [wXb + X_E]
    ld [wEc], a
    ld a, [wXb + X_E + 1]
    ld [wEc + 1], a
    FOR i, 4
    ld a, [wXb + X_M + i]
    ld [wFc + 4 + i], a
    ENDR
    ld hl, wFc
    call sf_shr1j64
.have_addend
    ld a, [wEp]
    ld hl, wEc
    sub [hl]
    ld c, a
    ld a, [wEp + 1]
    inc hl
    sbc [hl]
    ld b, a
    bit 7, b
    jr nz, .product_smaller
    ld a, [wEp]
    ld [wE], a
    ld a, [wEp + 1]
    ld [wE + 1], a
    ld hl, wFc
    call sf_shift_jam
    jr .aligned
.product_smaller
    ld a, c
    cpl
    add 1
    ld c, a
    ld a, b
    cpl
    adc 0
    ld b, a
    ld a, [wEc]
    ld [wE], a
    ld a, [wEc + 1]
    ld [wE + 1], a
    ld hl, wFp
    call sf_shift_jam
.aligned
    ld a, [wSp]
    ld b, a
    ld a, [wSc]
    xor b
    jr nz, .subtract
    ld a, b
    ld [wS], a
    ld hl, wFp
    ld de, wFc
    call sf_add64
    jr .normalize
.subtract
    ld hl, wFp
    ld de, wFc
    call sf_cmp64
    jr z, .exact_zero
    jr c, .addend_larger
    ld a, [wSp]
    ld [wS], a
    ld hl, wFp
    ld de, wFc
    call sf_sub64
    jr .normalize
.addend_larger
    ld a, [wSc]
    ld [wS], a
    ld hl, wFc
    ld de, wFp
    call sf_sub64
    FOR i, 8
    ld a, [wFc + i]
    ld [wFp + i], a
    ENDR
    jr .normalize
.exact_zero
    xor a
    ld [wS], a
    jp sf_ret_zero
.normalize
    ld a, [wFp + 7]
    bit 7, a
    jr nz, .leading
    ld hl, wFp
    call sf_shl64
    call sf_dec_e
    jr .normalize
.leading
    call sf_inc_e
    call sf_inc_e
    FOR i, 4
    ld a, [wFp + 4 + i]
    ld [wM + i], a
    ENDR
    ld hl, wM
    call sf_shr1s
    ld hl, wFp
    call sf_is_zero32
    jr z, .odd_check
    ld hl, wM
    set 0, [hl]
.odd_check
    ld a, [wMask]
    or a
    jp z, sf_roundpack
    ld a, [wM]
    and $7f
    jp z, sf_roundpack
    ld a, [wM]
    and $80
    or $80
    ld [wM], a
    jp sf_roundpack
