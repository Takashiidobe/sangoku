INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_compare", ROM0


; compare: A = 0 equal, 1 less, 2 greater, 3 unordered

sf_compare::
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
