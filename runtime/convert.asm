; generated from convert.c, sha256 e953fcea7e5f753f0b911c5143a8e6f6664763ff8f0716e10045a189a4f39511
SECTION "convert", ROM0
_clear:
ld c, a
ld b, 0x00
.L00103:
ld a, b
sub a, c
ret nc
ld l, b
ld h, 0x00
add hl, de
ld [hl], 0x00
inc b
jr .L00103
_nonzero:
dec sp
ld hl, sp + 0
ld [hl], a
ld bc, 0x0
.L00103:
ld a, b
ld hl, sp + 0
sub a, [hl]
jr nc, .L00101
ld l, b
ld h, 0x00
add hl, de
ld a, [hl]
or a, c
ld c, a
inc b
jr .L00103
.L00101:
ld a, c
sub a, 0x01
ld a, 0x00
rla
xor a, 0x01
inc sp
ret
_bit:
ld c, a
swap a
rlca
and a, 0x1f
ld l, a
ld h, 0x00
add hl, de
ld e, [hl]
ld a, c
and a, 0x07
inc a
jr .L00104
.L00103:
srl e
.L00104:
dec a
jr nz, .L00103
ld a, e
and a, 0x01
ret
_setbit:
ld b, a
swap a
rlca
and a, 0x1f
ld l, a
ld h, 0x00
add hl, de
ld c, [hl]
ld a, b
and a, 0x07
ld b, a
ld a, 0x01
inc b
jr .L00104
.L00103:
add a, a
.L00104:
dec b
jr nz, .L00103
or a, c
ld [hl], a
ret
_left:
dec sp
ld hl, sp + 0
ld [hl], 0x00
ld c, 0x00
.L00102:
ld l, c
ld h, 0x00
add hl, de
ld a, [hl]
push af
rlca
and a, 0x01
ld b, a
pop af
add a, a
push hl
ld hl, sp + 2
or a, [hl]
pop hl
ld [hl], a
ld hl, sp + 0
ld [hl], b
inc c
ld a, c
sub a, 0x11
jr c, .L00102
inc sp
ret
_right:
add sp, -3
ld hl, sp + 0
xor a, a
ld [hli], a
inc hl
ld [hl], 0x11
.L00101:
ld hl, sp + 2
ld a, [hl]
or a, a
jr z, .L00103
dec [hl]
ld a, [hl]
add a,e
ld c, a
ld a, d
adc a, 0x00
ld b, a
ld a, [bc]
ld l, a
and a, 0x01
push af
ld a, l
srl a
ld hl, sp + 3
ld [hld], a
ld a, [hl]
rrca
and a, 0x80
ld l, a
push hl
ld hl, sp + 5
or a, [hl]
pop hl
ld [bc], a
pop af
ld hl, sp + 0
ld [hl], a
jr .L00101
.L00103:
ld hl, sp + 0
ld a, [hl]
add sp, 3
ret
_shift:
dec sp
dec sp
pop hl
push de
ld c, 0x00
ld hl, sp + 4
ld a, 0x88
sub a, [hl]
inc hl
ld a, 0x00
sbc a, [hl]
inc hl
ld a, 0x00
sbc a, [hl]
inc hl
ld a, 0x00
sbc a, [hl]
ld d, 0x00
bit 7, [hl]
jr z, .L00143
bit 7, d
jr nz, .L00144
cp a, a
jr .L00144
.L00143:
bit 7, d
jr z, .L00144
scf
.L00144:
jr nc, .L00102
ld hl, sp + 4
ld a, 0x88
ld [hli], a
xor a, a
ld [hli], a
ld [hli], a
ld [hl], a
.L00102:
ld hl, sp + 4
ld a, [hli]
ld e, a
ld d, [hl]
.L00103:
ld a, e
ld b, d
dec de
or a, b
jr z, .L00105
push bc
push de
ld hl, sp + 4
ld a, [hli]
ld e, a
ld d, [hl]
call _right
pop de
pop bc
or a, c
ld c, a
jr .L00103
.L00105:
ld hl, sp + 8
ld a, [hl]
or a, a
jr z, .L00108
pop de
push de
ld a, [de]
or a, c
pop hl
push hl
ld [hl], a
.L00108:
inc sp
inc sp
pop hl
add sp, 5
jp hl
_increment:
ld c, e
ld b, d
ld e, 0x00
.L00104:
ld l, e
ld h, 0x00
add hl, bc
inc [hl]
ret nz
inc e
ld a, e
sub a, 0x11
jr c, .L00104
ret
_negate:
add sp, -5
ld c, e
ld b, d
ld hl, sp + 3
ld [hl], a
ld hl, sp + 0
ld [hl], 0x01
ld hl, sp + 4
ld [hl], 0x00
.L00103:
ld hl, sp + 4
ld a, [hld]
sub a, [hl]
jr nc, .L00105
inc hl
ld a, c
add a, [hl]
dec hl
dec hl
dec hl
ld [hli], a
ld a, b
adc a, 0x00
ld [hld], a
ld a, [hli]
ld e, a
ld a, [hld]
dec hl
ld d, a
ld a, [de]
cpl
ld e, [hl]
ld d, 0x00
ld l, a
ld h, 0x00
add hl, de
ld a, l
push hl
ld hl, sp + 3
ld e, [hl]
inc hl
ld h, [hl]
ld l, e
ld [hl], a
pop hl
ld a, h
ld hl, sp + 0
ld [hl], a
ld hl, sp + 4
inc [hl]
jr .L00103
.L00105:
add sp, 5
ret
_format:
dec sp
ld hl, sp + 0
ld [hl], a
ld [de], a
ld c, e
ld b, d
inc bc
ld l, e
ld h, d
inc hl
inc hl
inc de
inc de
inc de
push hl
ld hl, sp + 2
ld a, [hl]
sub a, 0x02
pop hl
jr nz, .L00108
ld a, 0x0a
ld [bc], a
ld [hl], 0x05
ld a, 0x0f
ld [de], a
inc de
xor a, a
ld [de], a
inc de
ld [de], a
inc de
ld [de], a
jr .L00110
.L00108:
push hl
ld hl, sp + 2
ld a, [hl]
sub a, 0x04
pop hl
jr nz, .L00105
ld a, 0x17
ld [bc], a
ld [hl], 0x08
ld a, 0x7f
ld [de], a
inc de
xor a, a
ld [de], a
inc de
ld [de], a
inc de
ld [de], a
jr .L00110
.L00105:
push hl
ld hl, sp + 2
ld a, [hl]
sub a, 0x08
pop hl
jr nz, .L00102
ld a, 0x34
ld [bc], a
ld [hl], 0x0b
ld a, 0xff
ld [de], a
inc de
ld a, 0x03
ld [de], a
inc de
xor a, a
ld [de], a
inc de
ld [de], a
jr .L00110
.L00102:
ld a, 0x70
ld [bc], a
ld [hl], 0x0f
ld a, 0xff
ld [de], a
inc de
ld a, 0x3f
ld [de], a
inc de
xor a, a
ld [de], a
inc de
ld [de], a
.L00110:
inc sp
ret
_normalize:
add sp, -14
ld hl, sp + 12
ld a, e
ld [hli], a
ld a, d
ld [hld], a
ld a, [hl]
add a, 0x02
push af
ld hl, sp + 2
pop af
ld [hl], a
push af
ld hl, sp + 15
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 1
ld [hl], a
ld a, 0x11
pop de
push de
call _nonzero
or a, a
jr nz, .L00113
ld hl, sp + 12
ld a, [hld]
dec hl
add a, 0x01
ld [hli], a
inc hl
inc hl
ld a, [hld]
dec hl
adc a, 0x00
ld [hld], a
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x00
jr .L00106
.L00113:
ld hl, sp + 12
ld a, [hl]
add a, 0x12
push af
ld hl, sp + 4
pop af
ld [hl], a
push af
ld hl, sp + 15
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 3
ld [hl], a
.L00103:
pop bc
pop de
push de
push bc
ld a, [de]
or a, a
jr nz, .L00106
pop de
push de
call _left
ld hl, sp + 12
ld a, [hli]
add a, 0x13
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld e, c
ld d, b
ld a, [de]
ld hl, sp + 4
ld [hli], a
inc de
ld a, [de]
ld [hli], a
inc de
ld a, [de]
ld [hli], a
inc de
ld a, [de]
ld [hld], a
dec hl
dec hl
ld a, [hli]
ld e, a
ld d, [hl]
ld a, e
sub a, 0x01
ld e, a
ld a, d
sbc a, 0x00
push af
ld hl, sp + 11
ld [hld], a
ld a, e
ld [hld], a
dec hl
ld a, [hli]
ld e, a
ld d, [hl]
pop af
ld a, e
sbc a, 0x00
ld e, a
ld a, d
sbc a, 0x00
ld hl, sp + 11
ld [hld], a
ld a, e
ld [hld], a
dec hl
ld a, [hli]
ld [bc], a
inc bc
ld a, [hli]
ld [bc], a
inc bc
ld a, [hli]
ld [bc], a
inc bc
ld a, [hl]
ld [bc], a
jr .L00103
.L00106:
add sp, 14
ret
_unpack:
add sp, -24
ld hl, sp + 22
ld a, e
ld [hli], a
ld [hl], d
ld hl, sp + 20
ld a, c
ld [hli], a
ld [hl], b
xor a, a
ld hl, sp + 18
ld [hli], a
ld [hl], a
ld hl, sp + 26
ld a, [hl]
add a, 0x02
push af
ld hl, sp + 12
pop af
ld [hl], a
push af
ld hl, sp + 29
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 11
ld [hld], a
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld c, a
ld hl, 0x0001
inc c
jr .L00196
.L00195:
add hl, hl
.L00196:
dec c
jr nz, .L00195
ld c, l
ld b, h
dec bc
ld hl, sp + 12
ld a, c
ld [hli], a
ld [hl], b
ld hl, sp + 22
ld a, [hl]
add a, 0x02
push af
ld hl, sp + 16
pop af
ld [hl], a
push af
ld hl, sp + 25
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 15
ld [hld], a
ld a, 0x11
ld e, [hl]
inc hl
ld d, [hl]
call _clear
ld hl, sp + 22
ld a, [hli]
ld c, a
ld b, [hl]
ld hl, sp + 26
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld d, 0x00
ld e, a
dec de
ld hl, sp + 20
ld a, [hli]
add a, e
ld e, a
ld a, [hl]
adc a, d
ld d, a
ld a, [de]
rlca
and a, 0x01
ld [bc], a
ld hl, sp + 26
ld a, [hl]
add a, 0x01
push af
ld hl, sp + 2
pop af
ld [hl], a
push af
ld hl, sp + 29
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 1
ld [hl], a
ld hl, sp + 17
ld [hl], 0x00
.L00110:
pop de
push de
ld a, [de]
ld c, a
ld hl, sp + 17
ld a, [hl]
cp a, c
jr nc, .L00103
ld hl, sp + 20
ld e, [hl]
inc hl
ld d, [hl]
call _bit
or a, a
jr z, .L00111
ld hl, sp + 17
ld a, [hl]
ld hl, sp + 14
ld e, [hl]
inc hl
ld d, [hl]
call _setbit
.L00111:
ld hl, sp + 17
inc [hl]
jr .L00110
.L00103:
ld c, 0x00
.L00113:
ld hl, sp + 10
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld b, a
ld a, c
sub a, b
jr nc, .L00104
pop de
push de
ld a, [de]
add a, c
push bc
ld hl, sp + 22
ld e, [hl]
inc hl
ld d, [hl]
call _bit
pop bc
ld e, 0x00
ld b, c
inc b
jr .L00198
.L00197:
add a, a
rl e
.L00198:
dec b
jr nz, .L00197
ld hl, sp + 18
or a, [hl]
ld [hli], a
ld a, e
or a, [hl]
ld [hl], a
inc c
jr .L00113
.L00104:
ld hl, sp + 22
ld a, [hl]
add a, 0x01
push af
ld hl, sp + 18
pop af
ld [hl], a
push af
ld hl, sp + 25
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 17
ld [hld], a
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x01
ld hl, sp + 22
ld a, [hl]
add a, 0x13
push af
ld hl, sp + 4
pop af
ld [hl], a
push af
ld hl, sp + 25
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 3
ld [hl], a
pop de
pop hl
push hl
push de
ld a, 0x80
ld [hli], a
xor a, a
ld [hli], a
ld [hli], a
ld [hl], a
ld hl, sp + 18
ld a, [hl]
ld hl, sp + 12
sub a, [hl]
jr nz, .L00106
ld hl, sp + 19
ld a, [hl]
ld hl, sp + 13
sub a, [hl]
jr nz, .L00106
ld hl, sp + 16
ld a, [hli]
ld c, a
ld b, [hl]
push bc
ld a, 0x11
ld hl, sp + 16
ld e, [hl]
inc hl
ld d, [hl]
call _nonzero
pop bc
or a, a
ld a, 0x03
jr nz, .L00118
ld a, 0x02
.L00118:
ld [bc], a
jp .L00115
.L00106:
ld hl, sp + 19
ld a, [hld]
or a, [hl]
jr z, .L00108
pop de
push de
ld a, [de]
ld hl, sp + 14
ld e, [hl]
inc hl
ld d, [hl]
call _setbit
.L00108:
ld hl, sp + 22
ld a, [hli]
ld e, a
ld d, [hl]
call _normalize
pop bc
pop de
push de
push bc
ld a, [de]
ld hl, sp + 4
ld [hli], a
inc de
ld a, [de]
ld [hli], a
inc de
ld a, [de]
ld [hli], a
inc de
ld a, [de]
ld [hl], a
ld hl, sp + 19
ld a, [hld]
or a, [hl]
jr z, .L00119
ld a, [hl]
ld hl, sp + 8
ld [hl], a
ld hl, sp + 19
ld a, [hl]
ld hl, sp + 9
ld [hli], a
xor a, a
ld [hli], a
ld [hl], a
jr .L00120
.L00119:
ld hl, sp + 8
ld a, 0x01
ld [hli], a
xor a, a
ld [hli], a
ld [hli], a
ld [hl], a
.L00120:
ld hl, sp + 26
ld a, [hl]
add a, 0x03
push af
ld hl, sp + 20
pop af
ld [hl], a
push af
ld hl, sp + 29
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 19
ld [hld], a
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 12
ld [hli], a
inc de
ld a, [de]
ld [hli], a
inc de
ld a, [de]
ld [hli], a
inc de
ld a, [de]
ld [hl], a
ld hl, sp + 8
ld a, [hli]
ld e, a
ld d, [hl]
ld a, e
ld hl, sp + 12
sub a, [hl]
inc hl
ld e, a
ld a, d
sbc a, [hl]
push af
ld hl, sp + 19
ld [hld], a
ld [hl], e
ld hl, sp + 12
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 16
pop af
ld a, e
sbc a, [hl]
inc hl
ld e, a
ld a, d
sbc a, [hl]
ld hl, sp + 19
ld [hld], a
ld [hl], e
pop de
push de
ld a, [de]
ld hl, sp + 15
ld [hl], a
ld hl, sp + 8
ld [hli], a
xor a, a
ld [hli], a
ld [hli], a
ld [hl], a
ld hl, sp + 16
ld a, [hli]
ld e, a
ld d, [hl]
ld a, e
ld hl, sp + 8
sub a, [hl]
inc hl
ld e, a
ld a, d
sbc a, [hl]
push af
ld hl, sp + 15
ld [hld], a
ld [hl], e
ld hl, sp + 20
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 12
pop af
ld a, e
sbc a, [hl]
inc hl
ld e, a
ld a, d
sbc a, [hl]
ld hl, sp + 15
ld [hld], a
ld [hl], e
ld hl, sp + 4
ld a, [hli]
ld e, a
ld d, [hl]
ld a, e
ld hl, sp + 12
add a, [hl]
inc hl
ld e, a
ld a, d
adc a, [hl]
push af
ld hl, sp + 19
ld [hld], a
ld [hl], e
ld hl, sp + 8
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 16
pop af
ld a, e
adc a, [hl]
inc hl
ld e, a
ld a, d
adc a, [hl]
ld hl, sp + 19
ld [hld], a
ld [hl], e
pop hl
pop de
push de
push hl
ld hl, sp + 16
ld a, [hli]
ld [de], a
inc de
ld a, [hli]
ld [de], a
inc de
ld a, [hli]
ld [de], a
inc de
ld a, [hl]
ld [de], a
.L00115:
add sp, 24
pop hl
pop af
jp hl
_pack:
add sp, -36
ld hl, sp + 34
ld a, e
ld [hli], a
ld [hl], d
ld hl, sp + 32
ld a, c
ld [hli], a
ld [hl], b
xor a, a
ld hl, sp + 4
ld [hli], a
ld [hl], a
ld hl, sp + 38
ld a, [hli]
ld c, a
ld b, [hl]
ld hl, 0x0002
add hl, bc
push hl
ld a, l
ld hl, sp + 8
ld [hl], a
pop hl
ld a, h
ld hl, sp + 7
ld [hld], a
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld l, a
ld e, 0x01
xor a, a
inc l
jr .L00257
.L00256:
sla e
adc a, a
.L00257:
dec l
jr nz, .L00256
ld d, a
dec de
ld hl, sp + 8
ld a, e
ld [hli], a
ld a, d
ld [hli], a
ld a, c
ld [hli], a
ld a, b
ld [hld], a
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
push bc
ld hl, sp + 36
ld e, [hl]
inc hl
ld d, [hl]
call _clear
pop bc
ld hl, sp + 32
ld a, [hl]
add a, 0x01
push af
ld hl, sp + 14
pop af
ld [hl], a
push af
ld hl, sp + 35
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 13
ld [hld], a
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 31
ld [hl], a
ld l, c
ld h, b
inc hl
push hl
ld a, l
ld hl, sp + 16
ld [hl], a
pop hl
ld a, h
ld hl, sp + 15
ld [hl], a
ld hl, sp + 31
ld a, [hl]
dec a
jp nz, .L00116
ld hl, sp + 32
ld a, [hl]
add a, 0x13
push af
ld hl, sp + 18
pop af
ld [hl], a
push af
ld hl, sp + 35
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 17
ld [hld], a
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 0
ld [hli], a
inc de
ld a, [de]
ld [hli], a
inc de
ld a, [de]
ld [hli], a
inc de
ld a, [de]
ld [hl], a
inc bc
inc bc
inc bc
ld hl, sp + 18
ld a, c
ld [hli], a
ld a, b
ld [hld], a
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 28
ld [hli], a
inc de
ld a, [de]
ld [hli], a
inc de
ld a, [de]
ld [hli], a
inc de
ld a, [de]
ld [hl], a
ld de, 0x0001
ld a, e
ld hl, sp + 28
sub a, [hl]
inc hl
ld e, a
ld a, d
sbc a, [hl]
push af
ld hl, sp + 27
ld [hld], a
ld [hl], e
ld de, 0x0000
ld hl, sp + 32
pop af
ld a, e
sbc a, [hl]
inc hl
ld e, a
ld a, d
sbc a, [hl]
ld hl, sp + 27
ld [hld], a
ld [hl], e
ld hl, sp + 32
ld a, [hl]
add a, 0x02
push af
ld hl, sp + 22
pop af
ld [hl], a
push af
ld hl, sp + 35
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 21
ld [hl], a
ld hl, sp + 0
ld e, l
ld d, h
ld hl, sp + 24
ld a, [de]
inc de
sub a, [hl]
inc hl
ld a, [de]
inc de
sbc a, [hl]
inc hl
ld a, [de]
inc de
sbc a, [hl]
inc hl
ld a, [de]
sbc a, [hl]
ld a, [de]
ld d, a
bit 7, [hl]
jr z, .L00260
bit 7, d
jr nz, .L00261
cp a, a
jr .L00261
.L00260:
bit 7, d
jr z, .L00261
scf
.L00261:
jp nc, .L00102
ld hl, sp + 24
ld a, [hli]
ld e, a
ld d, [hl]
ld a, e
ld hl, sp + 0
sub a, [hl]
inc hl
ld e, a
ld a, d
sbc a, [hl]
push af
ld hl, sp + 31
ld [hld], a
ld a, e
ld [hld], a
dec hl
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 4
pop af
ld a, e
sbc a, [hl]
inc hl
ld e, a
ld a, d
sbc a, [hl]
ld hl, sp + 31
ld [hld], a
ld [hl], e
ld a, 0x01
push af
inc sp
ld a, [hli]
ld e, a
ld d, [hl]
push de
ld hl, sp + 31
ld a, [hli]
ld e, a
ld d, [hl]
push de
ld hl, sp + 25
ld a, [hli]
ld e, a
ld d, [hl]
call _shift
ld hl, sp + 18
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 24
ld [hli], a
inc de
ld a, [de]
ld [hli], a
inc de
ld a, [de]
ld [hli], a
inc de
ld a, [de]
ld [hl], a
ld de, 0x0001
ld a, e
ld hl, sp + 24
sub a, [hl]
inc hl
ld e, a
ld a, d
sbc a, [hl]
push af
ld hl, sp + 31
ld [hld], a
ld a, e
ld [hld], a
dec hl
ld de, 0x0000
pop af
ld a, e
sbc a, [hl]
inc hl
ld e, a
ld a, d
sbc a, [hl]
ld hl, sp + 31
ld [hld], a
ld [hl], e
ld hl, sp + 16
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 28
ld a, [hli]
ld [de], a
inc de
ld a, [hli]
ld [de], a
inc de
ld a, [hli]
ld [de], a
inc de
ld a, [hl]
ld [de], a
.L00102:
ld hl, sp + 14
ld a, [hl]
ld hl, sp + 22
ld [hl], a
ld hl, sp + 15
ld a, [hl]
ld hl, sp + 23
ld [hl], a
ld hl, sp + 14
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld c, a
ld a, 0x7e
sub a, c
ld c, a
sbc a, a
ld b, a
rlca
sbc a, a
ld d, a
ld h, 0x01
push hl
inc sp
ld e, a
push de
push bc
ld hl, sp + 25
ld a, [hli]
ld e, a
ld d, [hl]
call _shift
ld hl, sp + 20
ld a, [hli]
ld e, a
ld a, [hld]
ld d, a
ld a, [de]
and a, 0x03
ld c, a
push bc
ld a, [hli]
ld e, a
ld d, [hl]
call _right
ld hl, sp + 22
ld a, [hli]
ld e, a
ld d, [hl]
call _right
pop bc
ld a, 0x02
sub a, c
jr c, .L00103
ld a, c
sub a, 0x02
jr nz, .L00104
ld hl, sp + 20
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
rrca
jr nc, .L00104
.L00103:
ld hl, sp + 20
ld a, [hli]
ld e, a
ld d, [hl]
call _increment
.L00104:
ld hl, sp + 14
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
inc a
ld hl, sp + 20
ld e, [hl]
inc hl
ld d, [hl]
call _bit
or a, a
jr z, .L00108
ld hl, sp + 20
ld a, [hli]
ld e, a
ld d, [hl]
call _right
ld hl, sp + 16
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 24
ld [hli], a
inc de
ld a, [de]
ld [hli], a
inc de
ld a, [de]
ld [hli], a
inc de
ld a, [de]
ld [hld], a
dec hl
dec hl
ld a, [hli]
ld e, a
ld d, [hl]
ld a, e
add a, 0x01
ld e, a
ld a, d
adc a, 0x00
push af
ld hl, sp + 31
ld [hld], a
ld a, e
ld [hld], a
dec hl
ld a, [hli]
ld e, a
ld d, [hl]
pop af
ld a, e
adc a, 0x00
ld e, a
ld a, d
adc a, 0x00
ld hl, sp + 31
ld [hld], a
ld [hl], e
ld hl, sp + 16
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 28
ld a, [hli]
ld [de], a
inc de
ld a, [hli]
ld [de], a
inc de
ld a, [hli]
ld [de], a
inc de
ld a, [hl]
ld [de], a
.L00108:
ld hl, sp + 16
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 24
ld [hli], a
inc de
ld a, [de]
ld [hli], a
inc de
ld a, [de]
ld [hli], a
inc de
ld a, [de]
ld [hl], a
ld hl, sp + 18
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 28
ld [hli], a
inc de
ld a, [de]
ld [hli], a
inc de
ld a, [de]
ld [hli], a
inc de
ld a, [de]
ld [hl], a
ld hl, sp + 28
ld e, l
ld d, h
ld hl, sp + 24
ld a, [de]
inc de
sub a, [hl]
inc hl
ld a, [de]
inc de
sbc a, [hl]
inc hl
ld a, [de]
inc de
sbc a, [hl]
inc hl
ld a, [de]
sbc a, [hl]
ld a, [de]
ld d, a
bit 7, [hl]
jr z, .L00265
bit 7, d
jr nz, .L00266
cp a, a
jr .L00266
.L00265:
bit 7, d
jr z, .L00266
scf
.L00266:
jr nc, .L00113
ld hl, sp + 12
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x02
jr .L00116
.L00113:
ld hl, sp + 14
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 20
ld e, [hl]
inc hl
ld d, [hl]
call _bit
or a, a
jr z, .L00132
ld hl, sp + 16
ld a, [hli]
ld e, a
ld a, [hli]
ld d, a
ld a, [de]
ld c, a
inc de
ld a, [de]
ld b, a
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld l, a
inc de
ld a, [de]
ld h, a
add hl, bc
ld c, l
ld a, h
ld hl, sp + 30
ld [hl], c
inc hl
ld [hl], a
jr .L00133
.L00132:
xor a, a
ld hl, sp + 30
ld [hli], a
ld [hl], a
.L00133:
ld hl, sp + 30
ld a, [hl]
ld hl, sp + 4
ld [hl], a
ld hl, sp + 31
ld a, [hl]
ld hl, sp + 5
ld [hl], a
ld c, 0x00
.L00125:
ld hl, sp + 22
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld b, a
ld a, c
sub a, b
jr nc, .L00116
push bc
ld a, c
ld hl, sp + 22
ld e, [hl]
inc hl
ld d, [hl]
call _bit
pop bc
or a, a
jr z, .L00126
push bc
ld a, c
ld hl, sp + 36
ld e, [hl]
inc hl
ld d, [hl]
call _setbit
pop bc
.L00126:
inc c
jr .L00125
.L00116:
ld hl, sp + 12
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
cp a, 0x02
jr c, .L00163
push af
ld hl, sp + 10
ld a, [hl]
ld hl, sp + 6
ld [hl], a
ld hl, sp + 11
ld a, [hl]
ld hl, sp + 7
ld [hl], a
pop af
cp a, 0x03
jr nz, .L00163
ld hl, sp + 14
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 31
ld [hl], a
dec a
ld [hl], a
ld hl, sp + 34
ld e, [hl]
inc hl
ld d, [hl]
call _setbit
.L00163:
ld hl, sp + 14
ld a, [hli]
ld c, a
ld b, [hl]
ld hl, sp + 31
ld [hl], 0x00
.L00128:
ld hl, sp + 6
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 30
ld [hli], a
ld a, [hld]
sub a, [hl]
jr nc, .L00123
inc hl
ld a, [hl]
ld hl, sp + 4
ld e, [hl]
inc hl
ld d, [hl]
inc a
jr .L00270
.L00269:
srl d
rr e
.L00270:
dec a
jr nz, .L00269
bit 0, e
jr z, .L00129
ld a, [bc]
ld hl, sp + 31
add a, [hl]
push bc
ld hl, sp + 36
ld e, [hl]
inc hl
ld d, [hl]
call _setbit
pop bc
.L00129:
ld hl, sp + 31
inc [hl]
jr .L00128
.L00123:
ld hl, sp + 10
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld b, 0x00
ld c, a
dec bc
ld a, c
ld hl, sp + 34
add a, [hl]
inc hl
ld c, a
ld a, b
adc a, [hl]
dec hl
dec hl
dec hl
ld b, a
ld a, [bc]
ld e, [hl]
inc hl
ld d, [hl]
push af
ld a, [de]
rrca
and a, 0x80
ld e, a
pop af
or a, e
ld [bc], a
add sp, 36
pop hl
pop af
jp hl
_from_integer:
add sp, -9
ld hl, sp + 6
ld a, e
ld [hli], a
ld [hl], d
ld hl, sp + 4
ld a, c
ld [hli], a
ld a, b
ld [hli], a
ld a, [hl]
add a, 0x02
push af
ld hl, sp + 2
pop af
ld [hl], a
push af
ld hl, sp + 9
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 1
ld [hl], a
ld a, 0x11
pop de
push de
call _clear
ld hl, sp + 6
ld a, [hli]
ld c, a
ld a, [hli]
ld b, a
inc bc
inc bc
ld [hl], 0x00
.L00105:
ld hl, sp + 8
ld a, [hl]
ld hl, sp + 11
sub a, [hl]
jr nc, .L00101
ld a, c
ld hl, sp + 8
add a, [hl]
push af
ld hl, sp + 4
pop af
ld [hli], a
ld a, b
adc a, 0x00
ld [hli], a
ld a, [hl]
ld hl, sp + 8
add a, [hl]
dec hl
dec hl
dec hl
ld e, a
ld a, [hl]
adc a, 0x00
ld d, a
ld a, [de]
pop de
pop hl
push hl
push de
ld [hl], a
ld hl, sp + 8
inc [hl]
jr .L00105
.L00101:
ld hl, sp + 6
ld a, [hli]
ld c, a
ld b, [hl]
ld hl, sp + 12
ld a, [hl]
or a, a
jr z, .L00109
dec hl
ld e, [hl]
xor a, a
ld d, a
dec de
ld hl, sp + 4
ld a, [hli]
add a, e
ld e, a
ld a, [hl]
adc a, d
ld d, a
ld a, [de]
rlca
jr c, .L00110
.L00109:
xor a, a
jr .L00111
.L00110:
ld a, 0x01
.L00111:
ld [bc], a
or a, a
jr z, .L00103
ld hl, sp + 11
ld a, [hl]
pop de
push de
call _negate
.L00103:
ld hl, sp + 6
ld a, [hli]
ld h, [hl]
ld l, a
inc hl
ld [hl], 0x01
ld hl, sp + 6
ld a, [hli]
add a, 0x13
ld c, a
ld a, [hld]
adc a, 0x00
ld b, a
ld a, 0x80
ld [bc], a
inc bc
xor a, a
ld [bc], a
inc bc
ld [bc], a
inc bc
ld [bc], a
ld a, [hli]
ld e, a
ld d, [hl]
call _normalize
add sp, 9
pop hl
pop af
jp hl
_to_integer:
add sp, -18
ld hl, sp + 16
ld a, e
ld [hli], a
ld [hl], d
ld hl, sp + 14
ld a, c
ld [hli], a
ld [hl], b
ld hl, sp + 0
ld [hl], 0x00
ld hl, sp + 20
ld a, [hl]
add a, a
add a, a
add a, a
ld hl, sp + 13
ld [hl], a
ld hl, sp + 21
sub a, [hl]
ld hl, sp + 1
ld [hl], a
ld hl, sp + 20
ld a, [hl]
ld hl, sp + 16
ld e, [hl]
inc hl
ld d, [hl]
call _clear
ld hl, sp + 14
ld a, [hli]
ld c, a
ld b, [hl]
inc bc
ld a, [bc]
ld hl, sp + 12
ld [hl], a
sub a, 0x03
jr nz, .L00102
ld a, 0x02
jp .L00137
.L00102:
ld hl, sp + 12
ld a, [hl]
or a, a
jr nz, .L00104
xor a, a
jp .L00137
.L00104:
ld hl, sp + 14
ld a, [hl]
add a, 0x02
push af
ld hl, sp + 4
pop af
ld [hl], a
push af
ld hl, sp + 17
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 3
ld [hl], a
ld hl, sp + 14
ld a, [hl]
ld hl, sp + 4
ld [hl], a
ld hl, sp + 15
ld a, [hl]
ld hl, sp + 5
ld [hl], a
ld hl, sp + 12
ld a, [hl]
sub a, 0x02
jr z, .L00114
ld hl, sp + 14
ld a, [hli]
add a, 0x13
ld c, a
ld a, [hl]
adc a, 0x00
ld e, c
ld d, a
ld a, [de]
ld hl, sp + 6
ld [hli], a
inc de
ld a, [de]
ld [hli], a
inc de
ld a, [de]
ld [hli], a
inc de
ld a, [de]
ld [hl], a
ld hl, sp + 13
ld a, [hl]
ld hl, sp + 10
ld [hli], a
xor a, a
ld [hli], a
ld [hli], a
ld [hl], a
ld hl, sp + 6
ld e, l
ld d, h
ld hl, sp + 10
ld a, [de]
inc de
sub a, [hl]
inc hl
ld a, [de]
inc de
sbc a, [hl]
inc hl
ld a, [de]
inc de
sbc a, [hl]
inc hl
ld a, [de]
sbc a, [hl]
ld a, [de]
ld d, a
ld e, [hl]
bit 7, e
jr z, .L00299
bit 7, d
jr nz, .L00300
cp a, a
jr .L00300
.L00299:
bit 7, d
jr z, .L00300
scf
.L00300:
jr c, .L00115
.L00114:
ld hl, sp + 0
ld [hl], 0x01
jp .L00116
.L00115:
ld hl, sp + 9
bit 7, [hl]
jr z, .L00112
xor a, a
jp .L00137
.L00112:
ld de, 0x0080
ld a, e
ld hl, sp + 6
sub a, [hl]
inc hl
ld e, a
ld a, d
sbc a, [hl]
push af
ld hl, sp + 13
ld [hld], a
ld a, e
ld [hld], a
dec hl
ld de, 0x0000
pop af
ld a, e
sbc a, [hl]
inc hl
ld e, a
ld a, d
sbc a, [hl]
ld hl, sp + 13
ld [hld], a
ld a, e
ld [hli], a
inc hl
ld a, [hl]
add a, 0x02
push af
ld hl, sp + 10
pop af
ld [hl], a
push af
ld hl, sp + 17
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 9
ld [hl], a
xor a, a
push af
inc sp
ld hl, sp + 13
ld a, [hli]
ld e, a
ld d, [hl]
push de
ld hl, sp + 13
ld a, [hli]
ld e, a
ld d, [hl]
push de
ld hl, sp + 13
ld a, [hli]
ld e, a
ld d, [hl]
call _shift
ld hl, sp + 21
ld a, [hl]
or a, a
jr z, .L00106
ld hl, sp + 1
ld a, [hl]
ld hl, sp + 8
ld e, [hl]
inc hl
ld d, [hl]
call _bit
or a, a
jr z, .L00106
ld hl, sp + 20
ld a, [hl]
dec a
ld hl, sp + 2
add a, [hl]
push af
ld hl, sp + 13
pop af
ld [hl], a
ld a, 0x00
push af
ld hl, sp + 5
pop af
adc a, [hl]
ld hl, sp + 12
ld [hld], a
ld a, [hli]
ld e, a
ld a, [hld]
ld d, a
ld a, [de]
res 7, a
ld e, [hl]
inc hl
ld h, [hl]
ld l, e
ld [hl], a
ld hl, sp + 4
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 13
ld [hl], a
or a, a
jr z, .L00140
ld a, 0x11
ld hl, sp + 8
ld e, [hl]
inc hl
ld d, [hl]
call _nonzero
ld hl, sp + 13
ld [hl], a
or a, a
jr nz, .L00140
ld [hl], a
jr .L00141
.L00140:
ld hl, sp + 13
ld [hl], 0x01
.L00141:
ld hl, sp + 13
ld a, [hl]
ld hl, sp + 0
ld [hl], a
ld hl, sp + 11
ld a, [hli]
ld e, a
ld a, [hld]
ld d, a
ld a, [de]
set 7, a
ld e, [hl]
inc hl
ld h, [hl]
ld l, e
ld [hl], a
.L00106:
ld hl, sp + 21
ld a, [hl]
or a, a
jr nz, .L00116
ld hl, sp + 4
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
or a, a
jr z, .L00116
ld a, 0x11
ld hl, sp + 8
ld e, [hl]
inc hl
ld d, [hl]
call _nonzero
ld hl, sp + 0
ld [hl], a
.L00116:
ld hl, sp + 0
ld a, [hl]
or a, a
jr z, .L00178
ld hl, sp + 4
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 13
ld [hl], a
ld hl, sp + 20
ld c, [hl]
xor a, a
ld b, a
dec bc
ld a, c
ld hl, sp + 16
add a, [hl]
push af
ld hl, sp + 11
pop af
ld [hl], a
ld a, b
push af
ld hl, sp + 19
pop af
adc a, [hl]
ld hl, sp + 10
ld [hl], a
ld hl, sp + 13
ld a, [hl]
or a, a
jr nz, .L00124
ld [hl], a
.L00132:
ld hl, sp + 13
ld a, [hl]
ld hl, sp + 20
sub a, [hl]
jr nc, .L00118
ld hl, sp + 16
ld a, [hl]
ld hl, sp + 13
add a, [hl]
dec hl
dec hl
ld [hl], a
push af
ld hl, sp + 19
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 12
ld [hld], a
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0xff
ld hl, sp + 13
inc [hl]
jr .L00132
.L00118:
ld hl, sp + 21
ld a, [hl]
or a, a
jr z, .L00125
ld hl, sp + 9
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x7f
jr .L00125
.L00124:
ld hl, sp + 21
ld a, [hl]
or a, a
jr z, .L00125
ld hl, sp + 9
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x80
.L00125:
ld a, 0x01
jr .L00137
.L00178:
ld hl, sp + 2
ld a, [hl]
ld hl, sp + 11
ld [hl], a
ld hl, sp + 3
ld a, [hl]
ld hl, sp + 12
ld [hli], a
ld [hl], 0x00
.L00135:
ld hl, sp + 13
ld a, [hl]
ld hl, sp + 20
sub a, [hl]
jr nc, .L00128
ld hl, sp + 16
ld a, [hl]
ld hl, sp + 13
add a, [hl]
push af
ld hl, sp + 9
pop af
ld [hl], a
push af
ld hl, sp + 19
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 8
ld [hl], a
ld hl, sp + 11
ld a, [hli]
inc hl
add a, [hl]
dec hl
dec hl
dec hl
dec hl
ld [hli], a
inc hl
inc hl
ld a, [hld]
dec hl
adc a, 0x00
ld [hld], a
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld [hl], a
ld hl, sp + 7
ld a, [hli]
ld e, a
ld a, [hli]
inc hl
ld d, a
ld a, [hl]
ld [de], a
ld hl, sp + 13
inc [hl]
jr .L00135
.L00128:
ld hl, sp + 4
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
or a, a
jr z, .L00130
ld hl, sp + 20
ld a, [hl]
ld hl, sp + 16
ld e, [hl]
inc hl
ld d, [hl]
call _negate
.L00130:
xor a, a
.L00137:
add sp, 18
pop hl
pop bc
jp hl
_sangoku_convert::
add sp, -39
ld hl, sp + 37
ld a, e
ld [hli], a
ld a, d
ld [hld], a
ld a, [hli]
add a, 0x20
ld c, a
ld a, [hld]
dec hl
adc a, 0x00
ld b, a
ld a, [bc]
ld [hli], a
ld a, [hli]
add a, 0x21
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld a, [bc]
ld hl, sp + 30
ld [hl], a
ld hl, sp + 37
ld a, [hli]
add a, 0x22
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld a, [bc]
ld hl, sp + 31
ld [hli], a
rrca
rrca
and a, 0x01
ld [hld], a
ld a, [hl]
rrca
jr nc, .L00102
ld hl, sp + 32
ld a, [hl]
push af
inc sp
ld hl, sp + 37
ld a, [hli]
push af
inc sp
ld a, [hli]
ld c, a
ld b, [hl]
ld hl, 2
add hl, sp
ld e, l
ld d, h
call _from_integer
jr .L00103
.L00102:
ld hl, sp + 23
ld c, l
ld b, h
push bc
ld hl, sp + 38
ld a, [hl]
ld e, c
ld d, b
call _format
pop bc
ld hl, sp + 0
ld e, l
ld d, h
push bc
ld hl, sp + 39
ld a, [hli]
ld c, a
ld b, [hl]
call _unpack
.L00103:
ld hl, sp + 37
ld a, [hld]
dec hl
dec hl
dec hl
add a, 0x23
ld [hl], a
push af
ld hl, sp + 40
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 34
ld [hld], a
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x00
ld hl, sp + 37
ld a, [hld]
dec hl
add a, 0x10
ld [hli], a
inc hl
inc hl
ld a, [hld]
dec hl
adc a, 0x00
ld [hl], a
ld hl, sp + 31
ld a, [hl]
and a, 0x02
jr z, .L00105
ld hl, sp + 0
ld c, l
ld b, h
ld hl, sp + 32
ld a, [hld]
dec hl
push af
inc sp
ld a, [hl]
push af
inc sp
ld hl, sp + 37
ld a, [hli]
ld e, a
ld d, [hl]
call _to_integer
ld hl, sp + 33
ld e, [hl]
inc hl
ld h, [hl]
ld l, e
ld [hl], a
jr .L00107
.L00105:
ld hl, sp + 30
ld a, [hl]
ld hl, 23
add hl, sp
ld e, l
ld d, h
call _format
ld hl, 23
add hl, sp
push hl
ld hl, 2
add hl, sp
ld c, l
ld b, h
ld hl, sp + 37
ld a, [hli]
ld e, a
ld d, [hl]
call _pack
.L00107:
add sp, 39
ret

__gbc_convert::
ld e, b
ld d, c
jp _sangoku_convert
