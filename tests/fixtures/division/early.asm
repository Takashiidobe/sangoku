    ld a, d
    or a
    jr nz, .compare
    ld a, e
    cp 1
    jr nz, .compare
    ld hl, 0
    ret
.compare
    ld a, b
    cp d
    jr c, .smaller
    jr nz, .normal
    ld a, c
    cp e
    jr c, .smaller
    jr nz, .normal
    ld bc, 1
    ld hl, 0
    ret
.smaller
    ld h, b
    ld l, c
    ld bc, 0
    ret
.normal
