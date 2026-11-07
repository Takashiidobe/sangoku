; generated from softfloat64.c, sha256 79b3fa58351c7494b81d4aa1fc0e6bd9ba5d800fa8daa331f664b26ee80b8848
SECTION "softfloat64", ROM0
_sf64_clear:
ld c, 0x00
.L00102:
ld l, c
ld h, 0x00
add hl, de
ld [hl], 0x00
inc c
ld a, c
sub a, 0x10
jr c, .L00102
ret
_sf64_copy:
add sp, -5
ld hl, sp + 2
ld a, e
ld [hli], a
ld a, d
ld [hli], a
pop de
push bc
ld [hl], 0x00
.L00102:
ld hl, sp + 2
ld a, [hli]
inc hl
add a, [hl]
dec hl
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld hl, sp + 0
ld a, [hl]
ld hl, sp + 4
add a, [hl]
dec hl
dec hl
dec hl
ld e, a
ld a, [hl]
adc a, 0x00
ld d, a
ld a, [de]
ld [bc], a
ld hl, sp + 4
inc [hl]
ld a, [hl]
sub a, 0x10
jr c, .L00102
add sp, 5
ret
_sf64_nonzero:
ld bc, 0x0
.L00102:
ld l, b
ld h, 0x00
add hl, de
ld a, [hl]
or a, c
ld c, a
inc b
ld a, b
sub a, 0x10
jr c, .L00102
ld a, c
sub a, 0x01
ld a, 0x00
rla
xor a, 0x01
ret
_sf64_left:
add sp, -8
ld hl, sp + 5
ld a, e
ld [hli], a
ld [hl], d
ld hl, sp + 0
ld [hl], 0x00
ld hl, sp + 7
ld [hl], 0x00
.L00102:
ld hl, sp + 5
ld a, [hli]
inc hl
add a, [hl]
push af
ld hl, sp + 3
pop af
ld [hl], a
push af
ld hl, sp + 8
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 2
ld [hld], a
ld a, [hli]
ld e, a
ld a, [hli]
ld d, a
ld a, [de]
push af
rlca
and a, 0x01
ld [hli], a
pop af
add a, a
ld [hl], a
ld hl, sp + 0
or a, [hl]
ld hl, sp + 4
ld [hl], a
ld hl, sp + 1
ld a, [hli]
ld e, a
ld a, [hli]
inc hl
ld d, a
ld a, [hld]
ld [de], a
ld a, [hl]
ld hl, sp + 0
ld [hl], a
ld hl, sp + 7
inc [hl]
ld a, [hl]
sub a, 0x10
jr c, .L00102
ld hl, sp + 0
ld a, [hl]
add sp, 8
ret
_sf64_right:
add sp, -3
ld hl, sp + 0
xor a, a
ld [hli], a
inc hl
ld [hl], 0x10
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
_sf64_shl:
ld c, a
.L00101:
ld a, c
dec c
or a, a
ret z
push bc
push de
call _sf64_left
pop de
pop bc
jr .L00101
_sf64_shr:
add sp, -3
pop hl
push de
ld hl, sp + 2
ld [hl], 0x00
ld l, c
ld h, b
ld e, h
ld d, 0x00
ld a, 0x80
cp a, l
ld a, 0x00
sbc a, h
bit 7, e
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
jr nc, .L00116
ld bc, 0x0080
.L00116:
.L00103:
ld l, c
ld h, b
dec bc
ld e, h
xor a, a
ld d, a
cp a, l
sbc a, h
bit 7, e
jr z, .L00145
bit 7, d
jr nz, .L00146
cp a, a
jr .L00146
.L00145:
bit 7, d
jr z, .L00146
scf
.L00146:
jr nc, .L00105
ld l, c
ld h, b
pop de
push de
push hl
call _sf64_right
pop bc
ld hl, sp + 2
or a, [hl]
ld [hl], a
jr .L00103
.L00105:
ld hl, sp + 5
ld a, [hl]
or a, a
jr z, .L00108
pop de
push de
ld a, [de]
ld hl, sp + 2
or a, [hl]
pop hl
push hl
ld [hl], a
.L00108:
add sp, 3
pop hl
inc sp
jp hl
_sf64_add:
add sp, -14
ld hl, sp + 11
ld a, e
ld [hli], a
ld [hl], d
ld hl, sp + 9
ld a, c
ld [hli], a
ld [hl], b
ld hl, sp + 0
ld [hl], 0x00
ld hl, sp + 13
ld [hl], 0x00
.L00102:
ld hl, sp + 11
ld a, [hli]
inc hl
add a, [hl]
push af
ld hl, sp + 3
pop af
ld [hl], a
push af
ld hl, sp + 14
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 2
ld [hld], a
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 8
ld [hli], a
ld a, [hl]
ld hl, sp + 13
add a, [hl]
push af
ld hl, sp + 8
pop af
ld [hli], a
inc hl
inc hl
inc hl
ld a, [hl]
adc a, 0x00
ld hl, sp + 7
ld [hld], a
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld [hl], a
ld hl, sp + 3
ld [hli], a
ld [hl], 0x00
ld hl, sp + 8
ld a, [hl]
ld hl, sp + 5
ld [hli], a
xor a, a
ld [hld], a
ld a, [hld]
dec hl
add a, [hl]
inc hl
inc hl
inc hl
inc hl
ld [hld], a
dec hl
dec hl
ld a, 0x00
adc a, [hl]
ld hl, sp + 8
ld [hl], a
ld hl, sp + 0
ld a, [hl]
ld hl, sp + 6
ld [hl], a
ld a, [hld]
ld [hli], a
xor a, a
ld [hld], a
ld a, [hli]
inc hl
add a, [hl]
dec hl
dec hl
dec hl
dec hl
ld [hl], a
ld a, 0x00
push af
ld hl, sp + 10
pop af
adc a, [hl]
ld hl, sp + 4
ld [hld], a
ld a, [hl]
ld hl, sp + 7
ld [hl], a
ld hl, sp + 4
ld a, [hl]
ld hl, sp + 8
ld [hld], a
ld a, [hld]
ld [hl], a
ld hl, sp + 1
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 6
ld a, [hli]
inc hl
ld [de], a
ld a, [hl]
ld hl, sp + 0
ld [hl], a
ld hl, sp + 13
inc [hl]
ld a, [hl]
sub a, 0x10
jp c, .L00102
ld hl, sp + 0
ld a, [hl]
add sp, 14
ret
_sf64_increment:
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
sub a, 0x10
jr c, .L00104
ret
_sf64_sub:
add sp, -8
ld hl, sp + 5
ld a, e
ld [hli], a
ld [hl], d
ld hl, sp + 3
ld a, c
ld [hli], a
ld a, b
ld [hld], a
dec hl
ld [hl], 0x00
ld hl, sp + 7
ld [hl], 0x00
.L00102:
ld hl, sp + 3
ld a, [hl]
ld hl, sp + 7
add a, [hl]
dec hl
dec hl
dec hl
ld c, a
ld a, [hld]
dec hl
adc a, 0x00
ld b, a
ld a, [bc]
ld c, [hl]
ld b, 0x00
ld l, a
ld h, 0x00
add hl, bc
ld e, l
ld d, h
ld hl, sp + 5
ld a, [hli]
inc hl
add a, [hl]
dec hl
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld a, [bc]
ld hl, sp + 0
ld [hl], a
ld a, [hli]
ld [hli], a
xor a, a
ld [hld], a
ld a, [hli]
sub a, e
ld a, [hl]
sbc a, d
ld a, 0x00
rla
ld [hld], a
dec hl
ld a, [hl]
sub a, e
ld [bc], a
ld hl, sp + 7
inc [hl]
ld a, [hl]
sub a, 0x10
jr c, .L00102
add sp, 8
ret
_sf64_cmp:
add sp, -4
ld hl, sp + 2
ld a, e
ld [hli], a
ld [hl], d
pop de
push bc
ld e, 0x10
.L00103:
ld a, e
or a, a
jr z, .L00105
dec e
ld hl, sp + 2
ld a, [hli]
add a, e
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld a, [bc]
ld d, a
ld hl, sp + 0
ld a, [hli]
add a, e
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld a, [bc]
ld c, a
ld a, d
cp a, c
jr z, .L00103
sub a, c
jr nc, .L00108
ld a, 0xff
jr .L00106
.L00108:
ld a, 0x01
jr .L00106
.L00105:
xor a, a
.L00106:
add sp, 4
ret
_sf64_copy_float:
add sp, -6
ld hl, sp + 4
ld a, e
ld [hli], a
ld a, d
ld [hld], a
ld a, [hli]
ld e, a
ld a, [hld]
ld d, a
ld a, [bc]
ld [de], a
ld a, [hli]
ld e, a
ld d, [hl]
inc de
ld l, c
ld h, b
inc hl
ld a, [hl]
ld [de], a
ld hl, sp + 4
ld a, [hld]
dec hl
dec hl
dec hl
add a, 0x02
ld [hl], a
push af
ld hl, sp + 7
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 1
ld [hl], a
ld l, c
ld h, b
inc hl
inc hl
ld e, l
ld d, h
ld a, [de]
ld hl, sp + 2
ld [hli], a
inc de
ld a, [de]
ld [hl], a
pop de
push de
ld hl, sp + 2
ld a, [hli]
ld [de], a
inc de
ld a, [hli]
ld [de], a
inc bc
inc bc
inc bc
inc bc
ld a, [hli]
add a, 0x04
ld e, a
ld a, [hl]
adc a, 0x00
ld d, a
call _sf64_copy
add sp, 6
ret
_sf64_unpack:
add sp, -15
ld hl, sp + 12
ld a, e
ld [hli], a
ld [hl], d
ld hl, 0x0007
add hl, bc
ld a, [hl]
ld hl, sp + 14
ld [hl], a
ld e, [hl]
res 7, e
ld d, 0x00
sla e
rl d
sla e
rl d
sla e
rl d
sla e
rl d
ld hl, sp + 10
ld a, e
ld [hli], a
ld [hl], d
ld hl, 0x0006
add hl, bc
pop de
ld e, l
ld d, h
push de
ld a, [de]
swap a
and a, 0x0f
ld hl, sp + 10
or a, [hl]
ld hl, sp + 2
ld [hl], a
ld hl, sp + 11
ld a, [hl]
ld hl, sp + 3
ld [hl], a
ld hl, sp + 14
ld a, [hld]
dec hl
rlca
and a, 0x01
ld e, [hl]
inc hl
ld h, [hl]
ld l, e
ld [hl], a
ld hl, sp + 12
ld a, [hl]
add a, 0x02
push af
ld hl, sp + 6
pop af
ld [hl], a
push af
ld hl, sp + 15
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 5
ld [hld], a
ld a, [hli]
ld h, [hl]
ld l, a
xor a, a
ld [hli], a
ld [hl], a
ld hl, sp + 12
ld a, [hl]
add a, 0x04
push af
ld hl, sp + 8
pop af
ld [hl], a
push af
ld hl, sp + 15
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 7
ld [hld], a
push bc
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_clear
pop bc
ld hl, sp + 12
ld a, [hld]
dec hl
dec hl
dec hl
add a, 0x04
ld [hl], a
push af
ld hl, sp + 15
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 9
ld [hl], a
ld hl, sp + 14
ld [hl], 0x00
.L00112:
ld hl, sp + 8
ld a, [hl]
ld hl, sp + 14
add a, [hl]
dec hl
dec hl
dec hl
dec hl
ld [hld], a
ld a, [hli]
inc hl
adc a, 0x00
ld [hl], a
ld a, c
ld hl, sp + 14
add a, [hl]
ld e, a
ld a, b
adc a, 0x00
ld d, a
ld a, [de]
ld hl, sp + 10
ld e, [hl]
inc hl
ld h, [hl]
ld l, e
ld [hl], a
ld hl, sp + 14
inc [hl]
ld a, [hl]
sub a, 0x06
jr c, .L00112
dec hl
dec hl
ld a, [hld]
dec hl
add a, 0x0a
ld [hli], a
inc hl
inc hl
ld a, [hld]
dec hl
adc a, 0x00
ld [hld], a
pop de
push de
ld a, [de]
and a, 0x0f
ld e, [hl]
inc hl
ld h, [hl]
ld l, e
ld [hl], a
pop hl
pop bc
push bc
push hl
ld hl, sp + 12
ld a, [hld]
dec hl
dec hl
dec hl
add a, 0x01
ld [hl], a
push af
ld hl, sp + 15
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 9
ld [hl], a
inc c
jr nz, .L00103
ld a, b
sub a, 0x07
jr nz, .L00103
ld hl, sp + 6
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_nonzero
ld hl, sp + 14
ld [hl], a
or a, a
jr z, .L00116
ld [hl], 0x03
jr .L00117
.L00116:
ld hl, sp + 14
ld [hl], 0x02
.L00117:
ld hl, sp + 14
ld a, [hl]
ld hl, sp + 8
ld e, [hl]
inc hl
ld h, [hl]
ld l, e
ld [hl], a
jp .L00114
.L00103:
ld hl, sp + 3
ld a, [hld]
or a, [hl]
jr nz, .L00105
ld hl, sp + 6
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_nonzero
or a, a
jr nz, .L00105
ld hl, sp + 8
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x00
jr .L00114
.L00105:
ld hl, sp + 8
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x01
ld hl, sp + 4
ld a, [hli]
ld c, a
ld a, [hld]
dec hl
ld b, a
ld a, [hld]
or a, [hl]
jr z, .L00118
ld a, [hli]
ld e, [hl]
add a, 0x01
ld l, a
ld a, e
adc a, 0xfc
ld h, a
jr .L00119
.L00118:
ld hl, 0xfc02
.L00119:
ld a, l
ld [bc], a
inc bc
ld a, h
ld [bc], a
ld hl, sp + 3
ld a, [hld]
or a, [hl]
jr z, .L00109
ld hl, sp + 10
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 14
ld [hl], a
or a, 0x10
ld [hl], a
ld hl, sp + 10
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 14
ld a, [hl]
ld [de], a
.L00109:
ld hl, sp + 10
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
bit 4, a
jr nz, .L00111
ld hl, sp + 6
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_left
ld hl, sp + 4
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld c, a
inc de
ld a, [de]
ld b, a
dec bc
ld a, [hld]
ld l, [hl]
ld h, a
ld a, c
ld [hli], a
ld [hl], b
jr .L00109
.L00111:
ld a, 0x44
ld hl, sp + 6
ld e, [hl]
inc hl
ld d, [hl]
call _sf64_shl
.L00114:
add sp, 15
ret
_sf64_normalize:
add sp, -10
ld hl, sp + 8
ld a, e
ld [hli], a
ld a, d
ld [hld], a
ld a, [hl]
add a, 0x04
push af
ld hl, sp + 2
pop af
ld [hl], a
push af
ld hl, sp + 11
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 1
ld [hl], a
pop de
push de
call _sf64_nonzero
or a, a
jr nz, .L00118
ld hl, sp + 8
ld a, [hli]
ld c, a
ld b, [hl]
inc bc
xor a, a
ld [bc], a
jp .L00109
.L00118:
ld hl, sp + 8
ld a, [hl]
add a, 0x13
push af
ld hl, sp + 4
pop af
ld [hl], a
push af
ld hl, sp + 11
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 3
ld [hl], a
.L00103:
ld hl, sp + 2
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld c, a
ld hl, sp + 8
ld a, [hld]
dec hl
dec hl
dec hl
add a, 0x02
ld [hl], a
push af
ld hl, sp + 11
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 5
ld [hl], a
ld a, 0x01
cp a, c
jr nc, .L00106
push af
inc sp
ld bc, 0x0001
ld hl, sp + 1
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_shr
ld hl, sp + 4
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld c, a
inc de
ld a, [de]
ld b, a
inc bc
ld a, [hld]
ld l, [hl]
ld h, a
ld a, c
ld [hli], a
ld [hl], b
jr .L00103
.L00106:
ld hl, sp + 2
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld c, a
or a, a
jr nz, .L00109
pop de
push de
call _sf64_left
ld hl, sp + 4
ld a, [hli]
ld e, a
ld a, [hli]
ld d, a
ld a, [de]
ld c, a
inc de
ld a, [de]
ld b, a
ld a, c
add a, 0xff
ld [hli], a
ld a, b
adc a, 0xff
ld [hl], a
ld hl, sp + 4
ld a, [hli]
ld e, a
ld a, [hli]
ld d, a
ld a, [hli]
ld [de], a
inc de
ld a, [hl]
ld [de], a
jr .L00106
.L00109:
add sp, 10
ret
_sf64_special:
dec sp
ld hl, sp + 0
ld [hl], a
ld c, 0x00
.L00104:
ld l, c
ld h, 0x00
add hl, de
ld [hl], 0x00
inc c
ld a, c
sub a, 0x08
jr c, .L00104
ld hl, 0x0007
add hl, de
ld c, l
ld b, h
ld hl, sp + 0
ld a, [hl]
rrca
and a, 0x80
ld [bc], a
ld hl, sp + 3
ld a, [hl]
sub a, 0x02
jr c, .L00106
ld a, [bc]
or a, 0x7f
ld [bc], a
ld hl, 0x0006
add hl, de
ld c, l
ld b, h
ld hl, sp + 3
ld a, [hl]
sub a, 0x03
ld a, 0xf8
jr z, .L00109
ld a, 0xf0
.L00109:
ld [bc], a
.L00106:
inc sp
pop hl
inc sp
jp hl
_sf64_pack:
add sp, -17
ld hl, sp + 14
ld a, e
ld [hli], a
ld [hl], d
ld hl, sp + 12
ld a, c
ld [hli], a
ld a, b
ld [hld], a
ld a, [hli]
ld e, a
ld a, [hld]
ld d, a
inc de
ld a, [de]
ld b, a
ld a, [hl]
ld hl, sp + 0
ld [hl], a
ld hl, sp + 13
ld a, [hl]
ld hl, sp + 1
ld [hl], a
ld a, b
dec a
jr z, .L00102
ld c, b
ld a, b
sub a, 0x03
jr z, .L00123
pop de
push de
ld a, [de]
.L00123:
ld h, c
push hl
inc sp
ld hl, sp + 15
ld e, [hl]
inc hl
ld d, [hl]
call _sf64_special
jp .L00120
.L00102:
push de
ld hl, sp + 14
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_normalize
pop de
ld a, [de]
or a, a
jr nz, .L00104
pop de
push de
ld a, [de]
ld h, 0x00
push hl
inc sp
ld hl, sp + 15
ld e, [hl]
inc hl
ld d, [hl]
call _sf64_special
jp .L00120
.L00104:
ld hl, sp + 12
ld a, [hld]
dec hl
dec hl
dec hl
add a, 0x02
ld [hl], a
push af
ld hl, sp + 15
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 9
ld [hld], a
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 4
ld [hli], a
inc de
ld a, [de]
ld [hld], a
ld a, [hli]
inc hl
ld [hld], a
ld a, [hli]
inc hl
ld [hld], a
ld a, 0xff
sub a, [hl]
inc hl
ld a, 0x03
sbc a, [hl]
ld d, 0x03
bit 7, [hl]
jr z, .L00236
bit 7, d
jr nz, .L00237
cp a, a
jr .L00237
.L00236:
bit 7, d
jr z, .L00237
scf
.L00237:
jr nc, .L00106
pop de
push de
ld a, [de]
ld h, 0x02
push hl
inc sp
ld hl, sp + 15
ld e, [hl]
inc hl
ld d, [hl]
call _sf64_special
jp .L00120
.L00106:
ld hl, sp + 12
ld a, [hld]
dec hl
add a, 0x04
ld [hli], a
inc hl
inc hl
ld a, [hld]
dec hl
adc a, 0x00
ld [hl], a
ld hl, sp + 6
ld a, [hli]
sub a, 0x02
ld a, [hl]
sbc a, 0xfc
ld d, [hl]
ld e, 0xfc
bit 7, e
jr z, .L00238
bit 7, d
jr nz, .L00239
cp a, a
jr .L00239
.L00238:
bit 7, d
jr z, .L00239
scf
.L00239:
jr nc, .L00108
ld a, 0x02
ld hl, sp + 4
sub a, [hl]
inc hl
ld c, a
ld a, 0xfc
sbc a, [hl]
ld h, 0x01
push hl
inc sp
ld b, a
ld hl, sp + 11
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_shr
ld hl, sp + 8
ld a, [hli]
ld h, [hl]
ld l, a
ld a, 0x02
ld [hli], a
ld [hl], 0xfc
.L00108:
ld a, 0x01
push af
inc sp
ld bc, 0x0041
ld hl, sp + 11
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_shr
ld hl, sp + 10
ld a, [hli]
ld e, a
ld a, [hld]
ld d, a
ld a, [de]
and a, 0x07
ld e, a
push de
xor a, a
push af
inc sp
ld bc, 0x0003
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_shr
pop de
ld a, 0x04
sub a, e
jr c, .L00109
ld a, e
sub a, 0x04
jr nz, .L00110
ld hl, sp + 10
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
rrca
jr nc, .L00110
.L00109:
ld hl, sp + 10
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_increment
.L00110:
ld hl, sp + 12
ld a, [hl]
add a, 0x0a
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
pop bc
pop de
push de
push bc
ld a, [de]
bit 5, a
jr z, .L00114
ld hl, sp + 10
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_right
ld hl, sp + 8
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld c, a
inc de
ld a, [de]
ld b, a
inc bc
ld a, [hld]
ld l, [hl]
ld h, a
ld a, c
ld [hli], a
ld [hl], b
.L00114:
ld hl, sp + 8
ld a, [hli]
ld e, a
ld a, [hli]
ld d, a
ld a, [de]
ld c, a
inc de
ld a, [de]
ld [hl], c
inc hl
ld [hld], a
ld a, 0xff
sub a, [hl]
inc hl
ld a, 0x03
sbc a, [hl]
ld d, 0x03
bit 7, [hl]
jr z, .L00244
bit 7, d
jr nz, .L00245
cp a, a
jr .L00245
.L00244:
bit 7, d
jr z, .L00245
scf
.L00245:
jr nc, .L00116
pop de
push de
ld a, [de]
ld h, 0x02
push hl
inc sp
ld hl, sp + 15
ld e, [hl]
inc hl
ld d, [hl]
call _sf64_special
jp .L00120
.L00116:
ld hl, sp + 10
ld a, [hl]
sub a, 0x02
jr nz, .L00124
inc hl
ld a, [hl]
sub a, 0xfc
jr nz, .L00124
pop bc
pop de
push de
push bc
ld a, [de]
swap a
and a, 0x01
ld hl, sp + 16
ld [hl], a
or a,a
jr nz, .L00124
ld hl, sp + 10
ld [hli], a
ld [hl], a
jr .L00125
.L00124:
ld hl, sp + 10
ld a, [hli]
add a, 0xff
ld c, a
ld a, [hld]
adc a, 0x03
ld [hl], c
inc hl
ld [hl], a
.L00125:
ld hl, sp + 10
ld a, [hl]
ld hl, sp + 4
ld [hl], a
ld hl, sp + 11
ld a, [hl]
ld hl, sp + 5
ld [hl], a
ld hl, sp + 12
ld a, [hl]
add a, 0x04
push af
ld hl, sp + 8
pop af
ld [hl], a
push af
ld hl, sp + 15
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 7
ld [hl], a
ld hl, sp + 16
ld [hl], 0x00
.L00118:
ld hl, sp + 14
ld a, [hli]
inc hl
add a, [hl]
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
ld hl, sp + 6
ld a, [hl]
ld hl, sp + 16
add a, [hl]
push af
ld hl, sp + 12
pop af
ld [hld], a
dec hl
dec hl
ld a, [hl]
adc a, 0x00
ld hl, sp + 11
ld [hld], a
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld [hl], a
ld hl, sp + 8
ld a, [hli]
ld e, a
ld a, [hli]
inc hl
ld d, a
ld a, [hl]
ld [de], a
ld hl, sp + 16
inc [hl]
ld a, [hl]
sub a, 0x06
jr c, .L00118
dec hl
dec hl
ld a, [hli]
add a, 0x06
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld hl, sp + 2
ld a, [hli]
ld e, a
ld a, [hli]
ld d, a
ld a, [de]
and a, 0x0f
ld e, a
ld a, [hl]
swap a
and a, 0xf0
or a, e
ld [bc], a
ld hl, sp + 14
ld a, [hli]
add a, 0x07
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld hl, sp + 4
ld a, [hl]
ld hl, sp + 10
ld [hl], a
ld hl, sp + 5
ld a, [hl]
ld hl, sp + 11
ld [hl], a
ld a, 0x04
.L00248:
ld hl, sp + 11
srl [hl]
dec hl
rr [hl]
dec a
jr nz, .L00248
pop de
push de
ld a, [de]
rrca
and a, 0x80
ld e, [hl]
or a, e
ld [bc], a
.L00120:
add sp, 17
ret
_sf64_sum:
add sp, -32
ld hl, sp + 30
ld a, e
ld [hli], a
ld [hl], d
ld hl, sp + 28
ld a, c
ld [hli], a
ld a, b
ld [hld], a
ld a, [hl]
ld hl, sp + 18
ld [hl], a
ld hl, sp + 29
ld a, [hl]
ld hl, sp + 19
ld [hl], a
ld hl, sp + 28
ld a, [hli]
ld c, a
ld b, [hl]
inc bc
ld a, [bc]
ld hl, sp + 20
ld [hl], a
ld hl, sp + 30
ld a, [hl]
add a, 0x01
push af
ld hl, sp + 23
pop af
ld [hl], a
push af
ld hl, sp + 33
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 22
ld [hld], a
dec hl
ld a, [hl]
sub a, 0x03
jr z, .L00101
ld hl, sp + 34
ld a, [hli]
ld c, a
ld b, [hl]
inc bc
ld a, [bc]
ld hl, sp + 23
ld [hl], a
sub a, 0x03
jr z, .L00101
ld hl, sp + 20
ld a, [hl]
sub a, 0x02
ld a, 0x01
jr z, .L00244
xor a, a
.L00244:
ld hl, sp + 24
ld [hld], a
ld a, [hl]
sub a, 0x02
ld a, 0x01
jr z, .L00246
xor a, a
.L00246:
ld hl, sp + 25
ld [hl], a
ld hl, sp + 28
ld a, [hli]
ld c, a
ld b, [hl]
ld hl, sp + 34
ld a, [hl]
ld hl, sp + 26
ld [hl], a
ld hl, sp + 35
ld a, [hl]
ld hl, sp + 27
ld [hl], a
ld hl, sp + 24
ld a, [hl]
or a, a
jr z, .L00102
inc hl
ld a, [hl]
or a, a
jr z, .L00102
inc hl
ld a, [bc]
ld e, [hl]
inc hl
ld d, [hl]
push af
ld a, [de]
ld l, a
pop af
sub a, l
jr z, .L00102
.L00101:
ld hl, sp + 21
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x03
jp .L00127
.L00102:
ld hl, sp + 24
ld a, [hl]
or a, a
jr z, .L00108
ld hl, sp + 28
ld a, [hli]
ld c, a
ld a, [hli]
ld b, a
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_copy_float
jp .L00127
.L00108:
ld hl, sp + 25
ld a, [hl]
or a, a
jr z, .L00110
ld hl, sp + 34
ld a, [hli]
ld c, a
ld b, [hl]
ld hl, sp + 30
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_copy_float
jp .L00127
.L00110:
ld hl, sp + 30
ld a, [hl]
ld hl, sp + 24
ld [hl], a
ld hl, sp + 31
ld a, [hl]
ld hl, sp + 25
ld [hl], a
ld hl, sp + 20
ld a, [hl]
or a, a
jr nz, .L00112
ld hl, sp + 23
ld a, [hl]
or a, a
jr nz, .L00112
dec hl
ld a, [hld]
ld l, [hl]
ld h, a
ld [hl], 0x00
ld a, [bc]
ld c, a
ld hl, sp + 26
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
and a, c
ld hl, sp + 24
ld e, [hl]
inc hl
ld h, [hl]
ld l, e
ld [hl], a
jp .L00127
.L00112:
ld hl, sp + 20
ld a, [hl]
or a, a
jr nz, .L00115
ld hl, sp + 34
ld a, [hli]
ld c, a
ld b, [hl]
ld hl, sp + 30
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_copy_float
jp .L00127
.L00115:
ld hl, sp + 23
ld a, [hl]
or a, a
jr nz, .L00117
ld hl, sp + 28
ld a, [hli]
ld c, a
ld a, [hli]
ld b, a
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_copy_float
jp .L00127
.L00117:
ld hl, sp + 28
ld a, [hli]
ld c, a
ld b, [hl]
inc bc
inc bc
ld e, c
ld d, b
ld a, [de]
ld hl, sp + 0
ld [hli], a
inc de
ld a, [de]
ld [hl], a
ld hl, sp + 34
ld a, [hli]
ld c, a
ld b, [hl]
inc bc
inc bc
ld e, c
ld d, b
ld a, [de]
ld hl, sp + 26
ld [hli], a
inc de
ld a, [de]
ld [hl], a
ld hl, sp + 0
ld e, l
ld d, h
ld hl, sp + 26
ld a, [de]
inc de
sub a, [hl]
inc hl
ld a, [de]
sbc a, [hl]
ld a, [de]
ld d, a
bit 7, [hl]
jr z, .L00248
bit 7, d
jr nz, .L00249
cp a, a
jr .L00249
.L00248:
bit 7, d
jr z, .L00249
scf
.L00249:
jr c, .L00118
ld hl, sp + 0
ld a, [hl]
ld hl, sp + 26
sub a, [hl]
jr nz, .L00119
ld hl, sp + 1
ld a, [hl]
ld hl, sp + 27
sub a, [hl]
jr nz, .L00119
ld hl, sp + 34
ld a, [hli]
add a, 0x04
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld hl, sp + 28
ld a, [hli]
add a, 0x04
ld e, a
ld a, [hl]
adc a, 0x00
ld d, a
call _sf64_cmp
bit 7,a
jr z, .L00119
.L00118:
ld hl, sp + 34
ld a, [hl]
ld hl, sp + 18
ld [hl], a
ld hl, sp + 35
ld a, [hl]
ld hl, sp + 19
ld [hl], a
ld hl, sp + 28
ld a, [hl]
ld hl, sp + 34
ld [hl], a
ld hl, sp + 29
ld a, [hl]
ld hl, sp + 35
ld [hl], a
.L00119:
ld hl, sp + 18
ld a, [hli]
ld c, a
ld b, [hl]
ld hl, sp + 30
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_copy_float
ld hl, sp + 34
ld a, [hli]
add a, 0x04
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld hl, 2
add hl, sp
ld e, l
ld d, h
call _sf64_copy
ld hl, sp + 18
ld a, [hli]
ld h, [hl]
ld l, a
inc hl
inc hl
ld a, [hli]
ld c, a
ld b, [hl]
ld hl, sp + 34
ld a, [hli]
ld h, [hl]
ld l, a
inc hl
inc hl
ld a, [hli]
ld h, [hl]
ld l, a
ld a, c
sub a, l
ld c, a
ld a, b
sbc a, h
ld h, 0x01
push hl
inc sp
ld b, a
ld hl, 3
add hl, sp
ld e, l
ld d, h
call _sf64_shr
ld hl, sp + 18
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld c, a
ld hl, sp + 34
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld b, a
ld hl, sp + 30
ld a, [hl]
add a, 0x04
push af
ld hl, sp + 21
pop af
ld [hl], a
push af
ld hl, sp + 33
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 20
ld [hl], a
ld a, c
sub a, b
jr nz, .L00125
ld hl, 2
add hl, sp
ld c, l
ld b, h
ld hl, sp + 19
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_add
jr .L00126
.L00125:
ld hl, 2
add hl, sp
ld c, l
ld b, h
ld hl, sp + 19
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_cmp
ld hl, sp + 27
ld [hl], a
or a, a
jr nz, .L00123
ld hl, sp + 21
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x00
ld hl, sp + 24
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x00
jr .L00127
.L00123:
ld hl, 2
add hl, sp
ld c, l
ld b, h
ld hl, sp + 19
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_sub
.L00126:
ld hl, sp + 30
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_normalize
.L00127:
add sp, 32
pop hl
pop af
jp hl
_sf64_product:
add sp, -46
ld hl, sp + 44
ld a, e
ld [hli], a
ld [hl], d
ld hl, sp + 42
ld a, c
ld [hli], a
ld a, b
ld [hli], a
ld a, [hli]
ld c, a
ld a, [hld]
dec hl
dec hl
ld b, a
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 48
ld e, [hl]
inc hl
ld d, [hl]
push af
ld a, [de]
ld l, a
pop af
xor a, l
ld [bc], a
ld hl, sp + 44
ld a, [hl]
add a, 0x02
push af
ld hl, sp + 34
pop af
ld [hl], a
push af
ld hl, sp + 47
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 33
ld [hld], a
ld a, [hli]
ld h, [hl]
ld l, a
xor a, a
ld [hli], a
ld [hl], a
ld hl, sp + 44
ld a, [hl]
add a, 0x04
push af
ld hl, sp + 36
pop af
ld [hl], a
push af
ld hl, sp + 47
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 35
ld [hld], a
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_clear
ld hl, sp + 42
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
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 36
ld [hl], a
ld hl, sp + 44
ld a, [hl]
add a, 0x01
push af
ld hl, sp + 39
pop af
ld [hl], a
push af
ld hl, sp + 47
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 38
ld [hld], a
dec hl
ld a, [hl]
sub a, 0x03
jr z, .L00101
ld hl, sp + 48
ld a, [hli]
ld c, a
ld b, [hl]
inc bc
ld a, [bc]
ld hl, sp + 39
ld [hl], a
sub a, 0x03
jr z, .L00101
ld hl, sp + 36
ld a, [hl]
sub a, 0x02
ld a, 0x01
jr z, .L00197
xor a, a
.L00197:
ld hl, sp + 40
ld [hl], a
or a, a
jr z, .L00107
dec hl
ld a, [hl]
or a, a
jr z, .L00101
.L00107:
ld hl, sp + 39
ld a, [hl]
sub a, 0x02
ld a, 0x01
jr z, .L00199
xor a, a
.L00199:
ld hl, sp + 41
ld [hl], a
or a, a
jr z, .L00102
ld hl, sp + 36
ld a, [hl]
or a, a
jr nz, .L00102
.L00101:
ld hl, sp + 37
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x03
jp .L00119
.L00102:
ld hl, sp + 40
ld a, [hl]
or a, a
jr nz, .L00108
inc hl
ld a, [hl]
or a, a
jr z, .L00109
.L00108:
ld hl, sp + 37
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x02
jp .L00119
.L00109:
ld hl, sp + 36
ld a, [hl]
or a, a
jr z, .L00111
ld hl, sp + 39
ld a, [hl]
or a, a
jr nz, .L00112
.L00111:
ld hl, sp + 37
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x00
jp .L00119
.L00112:
ld hl, sp + 42
ld a, [hli]
add a, 0x04
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld hl, 0
add hl, sp
ld e, l
ld d, h
call _sf64_copy
xor a, a
push af
inc sp
ld bc, 0x0044
ld hl, 1
add hl, sp
ld e, l
ld d, h
call _sf64_shr
ld hl, sp + 48
ld a, [hli]
add a, 0x04
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld hl, 16
add hl, sp
ld e, l
ld d, h
call _sf64_copy
xor a, a
push af
inc sp
ld bc, 0x0044
ld hl, 17
add hl, sp
ld e, l
ld d, h
call _sf64_shr
ld hl, sp + 34
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_clear
ld hl, sp + 41
ld [hl], 0x00
.L00117:
ld hl, sp + 16
ld a, [hl]
rrca
jr nc, .L00115
ld hl, 0
add hl, sp
ld c, l
ld b, h
ld hl, sp + 34
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_add
.L00115:
ld hl, 0
add hl, sp
ld e, l
ld d, h
call _sf64_left
ld hl, 16
add hl, sp
ld e, l
ld d, h
call _sf64_right
ld hl, sp + 41
inc [hl]
ld a, [hl]
sub a, 0x35
jr c, .L00117
ld hl, sp + 37
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x01
ld hl, sp + 42
ld a, [hli]
ld h, [hl]
ld l, a
inc hl
inc hl
ld a, [hli]
ld c, a
ld b, [hl]
ld hl, sp + 48
ld a, [hli]
ld h, [hl]
ld l, a
inc hl
inc hl
ld a, [hli]
ld h, [hl]
ld l, a
add hl, bc
ld c, l
ld b, h
ld hl, sp + 32
ld a, [hli]
ld h, [hl]
ld l, a
ld a, c
ld [hli], a
ld [hl], b
ld a, 0x10
ld hl, sp + 34
ld e, [hl]
inc hl
ld d, [hl]
call _sf64_shl
ld hl, sp + 44
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_normalize
.L00119:
add sp, 46
pop hl
pop af
jp hl
_sf64_quotient:
add sp, -56
ld hl, sp + 54
ld a, e
ld [hli], a
ld [hl], d
ld hl, sp + 52
ld a, c
ld [hli], a
ld a, b
ld [hli], a
ld a, [hli]
ld c, a
ld a, [hld]
dec hl
dec hl
ld b, a
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 58
ld e, [hl]
inc hl
ld d, [hl]
push af
ld a, [de]
ld l, a
pop af
xor a, l
ld [bc], a
ld hl, sp + 52
ld a, [hli]
ld h, [hl]
ld l, a
inc hl
ld c, [hl]
ld hl, sp + 54
ld a, [hl]
add a, 0x01
push af
ld hl, sp + 50
pop af
ld [hl], a
push af
ld hl, sp + 57
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 49
ld [hl], a
ld a, c
sub a, 0x03
jr z, .L00101
ld hl, sp + 58
ld a, [hli]
ld h, [hl]
ld l, a
inc hl
ld b, [hl]
ld a, b
sub a, 0x03
jr z, .L00101
ld a, c
sub a, 0x02
ld a, 0x01
jr z, .L00218
xor a, a
.L00218:
ld hl, sp + 50
ld [hl], a
ld a, b
sub a, 0x02
ld a, 0x01
jr z, .L00220
xor a, a
.L00220:
ld hl, sp + 51
ld [hld], a
ld a, [hl]
or a, a
jr z, .L00107
inc hl
ld a, [hl]
or a, a
jr nz, .L00101
.L00107:
ld a, c
or a, a
jr nz, .L00102
or a, b
jr nz, .L00102
.L00101:
ld hl, sp + 48
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x03
jp .L00124
.L00102:
ld hl, sp + 50
ld a, [hl]
or a, a
jr nz, .L00108
or a, b
jr nz, .L00109
.L00108:
ld hl, sp + 48
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x02
jp .L00124
.L00109:
ld a, c
or a, a
jr z, .L00111
ld hl, sp + 51
ld a, [hl]
or a, a
jr z, .L00112
.L00111:
ld hl, sp + 48
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x00
jp .L00124
.L00112:
ld hl, sp + 48
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x01
ld hl, sp + 54
ld a, [hl]
add a, 0x02
push af
ld hl, sp + 50
pop af
ld [hl], a
push af
ld hl, sp + 57
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 49
ld [hl], a
ld hl, sp + 52
ld a, [hli]
ld h, [hl]
ld l, a
inc hl
inc hl
ld a, [hli]
ld c, a
ld b, [hl]
ld hl, sp + 58
ld a, [hli]
ld h, [hl]
ld l, a
inc hl
inc hl
ld a, [hli]
ld h, [hl]
ld l, a
ld a, c
sub a, l
ld c, a
ld a, b
sbc a, h
ld b, a
ld hl, sp + 48
ld a, [hli]
ld h, [hl]
ld l, a
ld a, c
ld [hli], a
ld [hl], b
ld hl, sp + 52
ld a, [hli]
add a, 0x04
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld hl, 0
add hl, sp
ld e, l
ld d, h
call _sf64_copy
xor a, a
push af
inc sp
ld bc, 0x0044
ld hl, 1
add hl, sp
ld e, l
ld d, h
call _sf64_shr
ld hl, sp + 58
ld a, [hli]
add a, 0x04
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld hl, 16
add hl, sp
ld e, l
ld d, h
call _sf64_copy
xor a, a
push af
inc sp
ld bc, 0x0044
ld hl, 17
add hl, sp
ld e, l
ld d, h
call _sf64_shr
ld hl, 16
add hl, sp
ld c, l
ld b, h
ld hl, 0
add hl, sp
ld e, l
ld d, h
call _sf64_cmp
bit 7,a
jr z, .L00115
ld a, 0x38
ld hl, 0
add hl, sp
ld e, l
ld d, h
call _sf64_shl
ld hl, sp + 48
ld a, [hli]
ld e, a
ld a, [hli]
ld d, a
ld a, [de]
ld c, a
inc de
ld a, [de]
ld b, a
ld a, c
add a, 0xff
ld [hli], a
ld a, b
adc a, 0xff
ld [hl], a
ld hl, sp + 48
ld a, [hli]
ld e, a
ld a, [hli]
ld d, a
ld a, [hli]
ld [de], a
inc de
ld a, [hl]
ld [de], a
jr .L00116
.L00115:
ld a, 0x37
ld hl, 0
add hl, sp
ld e, l
ld d, h
call _sf64_shl
.L00116:
ld hl, 32
add hl, sp
ld e, l
ld d, h
call _sf64_clear
ld hl, sp + 51
ld [hl], 0x00
.L00122:
ld hl, 0
add hl, sp
ld e, l
ld d, h
call _sf64_left
ld c, a
push bc
ld hl, 34
add hl, sp
ld e, l
ld d, h
call _sf64_left
pop bc
ld hl, sp + 32
ld a, [hl]
or a, c
ld [hl], a
ld hl, 16
add hl, sp
ld c, l
ld b, h
ld hl, 32
add hl, sp
ld e, l
ld d, h
call _sf64_cmp
bit 7,a
jr nz, .L00123
ld hl, 16
add hl, sp
ld c, l
ld b, h
ld hl, 32
add hl, sp
ld e, l
ld d, h
call _sf64_sub
ld hl, sp + 0
set 0, [hl]
.L00123:
ld hl, sp + 51
inc [hl]
ld a, [hl]
sub a, 0x80
jr c, .L00122
ld hl, 32
add hl, sp
ld e, l
ld d, h
call _sf64_nonzero
or a, a
jr z, .L00121
ld hl, sp + 0
set 0, [hl]
.L00121:
ld hl, sp + 54
ld a, [hli]
add a, 0x04
ld e, a
ld a, [hl]
adc a, 0x00
ld d, a
push de
ld hl, 2
add hl, sp
ld c, l
ld b, h
call _sf64_copy
pop de
ld a, 0x41
call _sf64_shl
.L00124:
add sp, 56
pop hl
pop af
jp hl
_sf64_root:
add sp, -53
ld hl, sp + 50
ld a, e
ld [hli], a
ld a, d
ld [hld], a
push bc
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_copy_float
pop bc
ld l, c
ld h, b
inc hl
ld e, [hl]
ld a, e
sub a, 0x03
jr z, .L00101
ld a, [bc]
or a, a
jr z, .L00102
ld a, e
or a, a
jr z, .L00102
.L00101:
ld hl, sp + 50
ld a, [hli]
ld h, [hl]
ld l, a
inc hl
ld [hl], 0x03
jp .L00116
.L00102:
dec e
jp nz, .L00116
inc bc
inc bc
inc bc
inc bc
ld hl, 0
add hl, sp
ld e, l
ld d, h
call _sf64_copy
xor a, a
push af
inc sp
ld bc, 0x0044
ld hl, 1
add hl, sp
ld e, l
ld d, h
call _sf64_shr
ld hl, sp + 50
ld a, [hli]
ld c, a
ld b, [hl]
inc bc
inc bc
ld l, c
ld h, b
ld a, [hli]
rrca
jr nc, .L00108
push bc
ld hl, 2
add hl, sp
ld e, l
ld d, h
call _sf64_left
pop bc
ld l, c
ld h, b
ld a, [hli]
ld l, [hl]
ld e, a
ld d, l
dec de
ld l, c
ld h, b
ld a, e
ld [hli], a
ld [hl], d
.L00108:
ld hl, sp + 48
ld a, c
ld [hli], a
ld [hl], b
ld l, c
ld h, b
ld a, [hli]
ld c, a
ld b, [hl]
ld l, c
ld h, b
bit 7, b
jr z, .L00118
ld l, c
ld h, b
inc hl
.L00118:
ld c, l
ld b, h
sra b
rr c
ld hl, sp + 48
ld a, [hli]
ld h, [hl]
ld l, a
ld a, c
ld [hli], a
ld [hl], b
ld a, 0x3a
ld hl, 0
add hl, sp
ld e, l
ld d, h
call _sf64_shl
ld hl, 16
add hl, sp
ld e, l
ld d, h
call _sf64_clear
ld hl, sp + 50
ld a, [hld]
dec hl
add a, 0x04
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
call _sf64_clear
ld hl, sp + 52
ld [hl], 0x00
.L00114:
ld hl, sp + 15
ld a, [hl]
rlca
rlca
and a, 0x03
ld c, a
push bc
ld a, 0x02
ld hl, 2
add hl, sp
ld e, l
ld d, h
call _sf64_shl
ld a, 0x02
ld hl, 18
add hl, sp
ld e, l
ld d, h
call _sf64_shl
pop bc
ld hl, sp + 16
ld a, [hl]
or a, c
ld [hl], a
ld hl, sp + 48
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_left
ld hl, sp + 32
ld e,l
ld d,h
push hl
push de
push hl
ld hl, sp + 54
ld c, [hl]
ld hl, sp + 55
ld b, [hl]
pop de
call _sf64_copy
pop de
pop hl
push hl
push de
ld e, l
ld d, h
call _sf64_left
pop de
pop hl
ld a, [de]
set 0, a
ld [de], a
push hl
ld c, l
ld b, h
push hl
ld hl, 20
add hl, sp
ld e, l
ld d, h
pop hl
call _sf64_cmp
ld c, a
pop hl
bit 7, c
jr nz, .L00115
ld c, l
ld b, h
ld hl, 16
add hl, sp
ld e, l
ld d, h
call _sf64_sub
ld hl, sp + 48
ld a, [hli]
ld e, a
ld a, [hld]
ld d, a
ld a, [de]
set 0, a
ld e, [hl]
inc hl
ld h, [hl]
ld l, e
ld [hl], a
.L00115:
ld hl, sp + 52
inc [hl]
ld a, [hl]
sub a, 0x40
jp c, .L00114
ld hl, 16
add hl, sp
ld e, l
ld d, h
call _sf64_nonzero
or a, a
jr z, .L00113
ld hl, sp + 48
ld a, [hli]
ld e, a
ld a, [hld]
ld d, a
ld a, [de]
set 0, a
ld e, [hl]
inc hl
ld h, [hl]
ld l, e
ld [hl], a
.L00113:
ld a, 0x41
ld hl, sp + 48
ld e, [hl]
inc hl
ld d, [hl]
call _sf64_shl
.L00116:
add sp, 53
ret
_sf64_compare_raw:
add sp, -8
ld hl, sp + 3
ld a, e
ld [hli], a
ld [hl], d
ld hl, sp + 1
ld a, c
ld [hli], a
ld [hl], b
ld hl, sp + 5
ld [hl], 0x01
ld hl, sp + 10
ld a, [hli]
ld c, a
ld b, [hl]
ld e, c
ld d, b
inc de
ld a, [de]
ld hl, sp + 6
ld [hl], a
sub a, 0x03
jr z, .L00101
ld hl, sp + 12
ld a, [hli]
ld e, a
ld d, [hl]
ld l, e
ld h, d
inc hl
ld a, [hl]
ld hl, sp + 7
ld [hl], a
sub a, 0x03
jr nz, .L00102
.L00101:
ld a, 0x08
jp .L00119
.L00102:
ld hl, sp + 6
ld a, [hl]
or a, a
jr nz, .L00105
inc hl
ld a, [hl]
or a, a
jr nz, .L00105
ld a, 0x01
jr .L00119
.L00105:
ld a, [bc]
ld hl, sp + 0
ld [hl], a
ld a, [de]
sub a, [hl]
jr z, .L00144
ld hl, sp + 0
ld a, [hl]
or a, a
jr z, .L00121
ld a, 0x02
jr .L00119
.L00121:
ld a, 0x04
jr .L00119
.L00144:
ld c, 0x08
.L00113:
ld a, c
or a, a
jr z, .L00115
dec c
ld hl, sp + 3
ld a, [hli]
add a, c
ld e, a
ld a, [hli]
inc hl
adc a, 0x00
ld d, a
ld a, [de]
ld [hl], a
ld hl, sp + 1
ld a, [hli]
add a, c
ld e, a
ld a, [hl]
adc a, 0x00
ld d, a
ld a, [de]
ld hl, sp + 7
ld [hl], a
ld a, c
sub a, 0x07
jr nz, .L00110
dec hl
ld a, [hl]
and a, 0x7f
ld [hli], a
ld a, [hl]
and a, 0x7f
ld [hl], a
.L00110:
ld hl, sp + 6
ld a, [hli]
sub a, [hl]
jr z, .L00113
ld hl, sp + 6
ld a, [hli]
sub a, [hl]
jr nc, .L00123
ld a, 0x02
jr .L00124
.L00123:
ld a, 0x04
.L00124:
ld hl, sp + 5
ld [hl], a
.L00115:
ld hl, sp + 0
ld a, [hl]
or a, a
jr z, .L00117
ld hl, sp + 5
ld a, [hl]
dec a
jr z, .L00117
ld hl, sp + 5
ld a, [hl]
xor a, 0x06
ld [hl], a
.L00117:
ld hl, sp + 5
ld a, [hl]
.L00119:
add sp, 8
pop hl
add sp, 4
jp hl
_sf64_round:
add sp, -34
ld hl, sp + 31
ld a, e
ld [hli], a
ld [hl], d
ld hl, sp + 29
ld a, c
ld [hli], a
ld [hl], b
ld hl, sp + 16
ld [hl], 0x00
ld hl, sp + 33
ld [hl], 0x00
ld hl, sp + 17
ld [hl], 0x00
ld hl, sp + 29
ld a, [hli]
ld h, [hl]
ld l, a
inc hl
ld c, [hl]
dec c
jr nz, .L00101
ld hl, sp + 29
ld a, [hl]
add a, 0x02
push af
ld hl, sp + 20
pop af
ld [hl], a
push af
ld hl, sp + 32
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 19
ld [hld], a
ld a, [hli]
ld e, a
ld a, [hli]
ld d, a
ld a, [de]
ld [hli], a
inc de
ld a, [de]
ld [hld], a
ld a, [hli]
ld c, a
ld b, [hl]
ld a, c
sub a, 0x34
ld a, b
rla
ccf
rra
sbc a, 0x80
jr c, .L00102
.L00101:
ld hl, sp + 29
ld a, [hli]
ld c, a
ld a, [hli]
ld b, a
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_pack
jp .L00130
.L00102:
ld hl, sp + 36
ld a, [hl]
dec a
ld a, 0x01
jr z, .L00317
xor a, a
.L00317:
ld hl, sp + 22
ld [hl], a
ld hl, sp + 29
ld a, [hl]
ld hl, sp + 23
ld [hl], a
ld hl, sp + 30
ld a, [hl]
ld hl, sp + 24
ld [hl], a
ld hl, sp + 36
ld a, [hl]
sub a, 0x02
ld a, 0x01
jr z, .L00319
xor a, a
.L00319:
ld hl, sp + 25
ld [hl], a
ld hl, sp + 36
ld a, [hl]
sub a, 0x03
ld a, 0x01
jr z, .L00321
xor a, a
.L00321:
ld hl, sp + 26
ld [hl], a
ld hl, sp + 29
ld a, [hld]
dec hl
add a, 0x04
ld [hli], a
inc hl
inc hl
ld a, [hld]
dec hl
adc a, 0x00
ld [hl], a
bit 7, b
jp z, .L00114
ld hl, sp + 22
ld a, [hl]
or a, a
jr z, .L00105
inc hl
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 17
ld [hl], a
.L00105:
ld hl, sp + 25
ld a, [hl]
or a, a
jr z, .L00107
dec hl
dec hl
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
sub a, 0x01
ld a, 0x00
rla
ld hl, sp + 17
ld [hl], a
.L00107:
ld hl, sp + 26
ld a, [hl]
or a, a
jr z, .L00109
ld hl, sp + 20
ld a, [hl]
inc a
jr nz, .L00109
inc hl
ld a, [hl]
inc a
jr nz, .L00109
ld hl, sp + 27
ld a, [hli]
ld c, a
ld b, [hl]
ld hl, 0
add hl, sp
ld e, l
ld d, h
call _sf64_copy
ld hl, sp + 15
res 0, [hl]
ld hl, 0
add hl, sp
ld e, l
ld d, h
call _sf64_nonzero
ld hl, sp + 17
ld [hl], a
.L00109:
ld hl, sp + 23
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld h, 0x00
push hl
inc sp
ld hl, sp + 32
ld e, [hl]
inc hl
ld d, [hl]
call _sf64_special
ld hl, sp + 17
ld a, [hl]
or a, a
jp z, .L00130
ld hl, sp + 31
ld a, [hli]
add a, 0x06
ld c, a
ld a, [hl]
adc a, 0x00
ld l, c
ld h, a
ld [hl], 0xf0
ld hl, sp + 31
ld a, [hli]
add a, 0x07
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld a, [bc]
or a, 0x3f
ld [bc], a
jp .L00130
.L00114:
ld hl, sp + 27
ld a, [hli]
ld c, a
ld b, [hl]
ld hl, 0
add hl, sp
ld e, l
ld d, h
call _sf64_copy
ld hl, sp + 18
ld a, [hli]
ld e, a
ld a, [hli]
ld d, a
ld a, [de]
ld c, a
inc de
ld a, [de]
ld b, a
ld a, 0x78
sub a, c
ld [hli], a
sbc a, a
sub a, b
ld [hl], a
ld bc, 0x0000
.L00128:
ld hl, sp + 20
ld a, c
sub a, [hl]
inc hl
ld a, b
sbc a, [hl]
ld d, b
bit 7, [hl]
jr z, .L00324
bit 7, d
jr nz, .L00325
cp a, a
jr .L00325
.L00324:
bit 7, d
jr z, .L00325
scf
.L00325:
jr nc, .L00118
push bc
ld hl, 2
add hl, sp
ld e, l
ld d, h
call _sf64_right
ld e, a
pop bc
ld l, c
ld h, b
inc hl
ld d, h
ld a, l
ld hl, sp + 20
sub a, [hl]
jr nz, .L00116
inc hl
ld a, d
sub a, [hl]
jr nz, .L00116
ld hl, sp + 16
ld [hl], e
jr .L00129
.L00116:
ld hl, sp + 33
ld a, [hl]
or a, e
ld [hl], a
.L00129:
inc bc
jr .L00128
.L00118:
ld hl, sp + 22
ld a, [hl]
or a, a
jr z, .L00120
inc hl
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
or a, a
jr z, .L00132
ld hl, sp + 16
ld a, [hl]
or a, a
jr nz, .L00133
ld hl, sp + 33
ld a, [hl]
or a, a
jr nz, .L00133
.L00132:
ld hl, sp + 22
ld [hl], 0x00
jr .L00134
.L00133:
ld hl, sp + 22
ld [hl], 0x01
.L00134:
ld hl, sp + 22
ld a, [hl]
ld hl, sp + 17
ld [hl], a
.L00120:
ld hl, sp + 25
ld a, [hl]
or a, a
jr z, .L00122
dec hl
dec hl
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
or a, a
jr nz, .L00138
ld hl, sp + 16
ld a, [hl]
or a, a
jr nz, .L00139
ld hl, sp + 33
ld a, [hl]
or a, a
jr nz, .L00139
.L00138:
ld hl, sp + 25
ld [hl], 0x00
jr .L00140
.L00139:
ld hl, sp + 25
ld [hl], 0x01
.L00140:
ld hl, sp + 25
ld a, [hl]
ld hl, sp + 17
ld [hl], a
.L00122:
ld hl, sp + 26
ld a, [hl]
or a, a
jr z, .L00124
ld hl, sp + 16
ld a, [hl]
or a, a
jr z, .L00144
ld hl, sp + 33
ld a, [hl]
or a, a
jr nz, .L00145
ld hl, sp + 0
ld a, [hl]
and a, 0x01
ld hl, sp + 33
ld [hl], a
or a, a
jr nz, .L00145
.L00144:
ld hl, sp + 33
ld [hl], 0x00
jr .L00146
.L00145:
ld hl, sp + 33
ld [hl], 0x01
.L00146:
ld hl, sp + 33
ld a, [hl]
ld hl, sp + 17
ld [hl], a
.L00124:
ld hl, sp + 17
ld a, [hl]
or a, a
jr z, .L00126
ld hl, 0
add hl, sp
ld e, l
ld d, h
call _sf64_increment
.L00126:
ld hl, sp + 20
ld a, [hl]
ld hl, 0
add hl, sp
ld e, l
ld d, h
call _sf64_shl
ld hl, 0
add hl, sp
ld c, l
ld b, h
ld hl, sp + 27
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_copy
ld hl, sp + 29
ld a, [hli]
ld c, a
ld a, [hli]
ld b, a
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_pack
.L00130:
add sp, 34
pop hl
inc sp
jp hl
_sangoku_sf64::
add sp, -112
ld hl, sp + 110
ld a, e
ld [hli], a
ld a, d
ld [hld], a
ld a, [hli]
add a, 0x20
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld a, [bc]
ld hl, sp + 102
ld [hl], a
ld hl, sp + 110
ld a, [hli]
ld c, a
ld b, [hl]
ld hl, 2
add hl, sp
ld e, l
ld d, h
call _sf64_unpack
ld hl, sp + 102
ld a, [hl]
sub a, 0x0c
ld a, 0x01
jr z, .L00239
xor a, a
.L00239:
ld hl, sp + 109
ld [hli], a
ld a, [hl]
add a, 0x08
push af
ld hl, sp + 105
pop af
ld [hl], a
push af
ld hl, sp + 113
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 104
ld [hld], a
dec hl
ld a, [hl]
sub a, 0x04
jr z, .L00102
ld hl, sp + 102
ld a, [hl]
sub a, 0x08
jr c, .L00101
ld hl, sp + 109
ld a, [hl]
or a, a
jr z, .L00102
.L00101:
ld hl, sp + 22
ld e, l
ld d, h
ld hl, sp + 103
ld a, [hli]
ld c, a
ld b, [hl]
call _sf64_unpack
.L00102:
ld hl, sp + 110
ld a, [hl]
add a, 0x18
push af
ld hl, sp + 107
pop af
ld [hl], a
push af
ld hl, sp + 113
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 106
ld [hl], a
ld a, 0x0c
ld hl, sp + 102
sub a, [hl]
jp c, .L00122
ld c, [hl]
ld b, 0x00
ld hl, .L00241
add hl, bc
add hl, bc
ld c, [hl]
inc hl
ld h, [hl]
ld l, c
jp hl
.L00241:
dw .L00105
dw .L00106
dw .L00107
dw .L00108
dw .L00109
dw .L00110
dw .L00113
dw .L00113
dw .L00122
dw .L00122
dw .L00122
dw .L00122
dw .L00113
.L00105:
ld hl, sp + 22
ld c, l
ld b, h
ld hl, sp + 82
ld e, l
ld d, h
push bc
ld hl, 4
add hl, sp
ld c, l
ld b, h
call _sf64_sum
jp .L00123
.L00106:
ld hl, sp + 22
ld c,l
ld b,h
ld a, [hl]
xor a, 0x01
ld [hl], a
ld hl, sp + 82
ld e, l
ld d, h
push bc
ld hl, 4
add hl, sp
ld c, l
ld b, h
call _sf64_sum
jp .L00123
.L00107:
ld hl, sp + 22
ld c, l
ld b, h
ld hl, sp + 82
ld e, l
ld d, h
push bc
ld hl, 4
add hl, sp
ld c, l
ld b, h
call _sf64_product
jp .L00123
.L00108:
ld hl, 22
add hl, sp
push hl
ld hl, 4
add hl, sp
ld c, l
ld b, h
ld hl, 84
add hl, sp
ld e, l
ld d, h
call _sf64_quotient
jp .L00123
.L00109:
ld hl, 2
add hl, sp
ld c, l
ld b, h
ld hl, 82
add hl, sp
ld e, l
ld d, h
call _sf64_root
jp .L00123
.L00110:
ld hl, sp + 110
ld a, [hli]
add a, 0x10
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld hl, 42
add hl, sp
ld e, l
ld d, h
call _sf64_unpack
ld hl, 22
add hl, sp
push hl
ld hl, 4
add hl, sp
ld c, l
ld b, h
ld hl, 64
add hl, sp
ld e, l
ld d, h
call _sf64_product
ld hl, sp + 82
ld e, l
ld d, h
ld hl, 42
add hl, sp
push hl
ld hl, 64
add hl, sp
ld c, l
ld b, h
call _sf64_sum
jp .L00123
.L00113:
ld hl, 22
add hl, sp
push hl
ld hl, 4
add hl, sp
push hl
ld hl, sp + 107
ld a, [hli]
ld c, a
ld b, [hl]
ld hl, sp + 114
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_compare_raw
ld hl, sp + 107
ld [hli], a
inc hl
ld a, [hl]
or a, a
jr z, .L00115
inc hl
ld a, [hld]
dec hl
add a, 0x21
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
ld a, [hld]
dec hl
and a, [hl]
inc hl
inc hl
ld [hl], a
sub a, 0x01
ld a, 0x00
rla
xor a, 0x01
ld hl, sp + 105
ld e, [hl]
inc hl
ld h, [hl]
ld l, e
ld [hl], a
jp .L00126
.L00115:
ld hl, sp + 107
ld a, [hl]
sub a, 0x08
jr nz, .L00117
ld a, 0x03
push af
inc sp
xor a, a
ld hl, sp + 106
ld e, [hl]
inc hl
ld d, [hl]
call _sf64_special
jp .L00126
.L00117:
ld hl, sp + 3
ld a, [hl]
ld hl, sp + 109
ld [hl], a
ld hl, sp + 102
ld a, [hl]
sub a, 0x06
ld a, 0x01
jr z, .L00245
xor a, a
.L00245:
ld hl, sp + 108
ld [hli], a
ld a, [hl]
or a, a
jr nz, .L00119
ld hl, sp + 23
ld a, [hl]
ld hl, sp + 109
ld [hl], a
or a, a
jr nz, .L00119
ld hl, sp + 2
ld a, [hl]
ld hl, sp + 109
ld [hl], a
ld hl, sp + 22
ld c, [hl]
ld hl, sp + 108
ld a, [hl]
or a, a
jr z, .L00128
inc hl
ld a, [hl]
or a, c
jr .L00129
.L00128:
ld hl, sp + 109
ld a, [hl]
and a, c
.L00129:
ld h, 0x00
push hl
inc sp
ld hl, sp + 106
ld e, [hl]
inc hl
ld d, [hl]
call _sf64_special
jp .L00126
.L00119:
ld hl, sp + 108
ld a, [hli]
ld [hl], a
bit 0, [hl]
jr z, .L00137
dec hl
dec hl
ld a, [hl]
sub a, 0x04
jr z, .L00133
.L00137:
ld hl, sp + 102
ld a, [hl]
sub a, 0x07
jr nz, .L00130
ld hl, sp + 107
ld a, [hl]
sub a, 0x02
jr nz, .L00130
.L00133:
ld hl, sp + 103
ld a, [hl]
ld hl, sp + 0
ld [hl], a
ld hl, sp + 104
ld a, [hl]
ld hl, sp + 1
ld [hl], a
jr .L00131
.L00130:
ld hl, sp + 110
ld a, [hl]
ld hl, sp + 0
ld [hl], a
ld hl, sp + 111
ld a, [hl]
ld hl, sp + 1
ld [hl], a
.L00131:
ld hl, sp + 0
ld a, [hl]
ld hl, sp + 107
ld [hl], a
ld hl, sp + 1
ld a, [hl]
ld hl, sp + 108
ld [hli], a
ld [hl], 0x00
.L00124:
ld hl, sp + 109
ld a, [hli]
add a, 0x18
add a, [hl]
inc hl
ld c, a
ld a, 0x00
adc a, [hl]
ld b, a
ld hl, sp + 107
ld a, [hli]
inc hl
add a, [hl]
dec hl
ld e, a
ld a, [hli]
adc a, 0x00
ld d, a
ld a, [de]
ld [bc], a
inc [hl]
ld a, [hl]
sub a, 0x08
jr c, .L00124
jr .L00126
.L00122:
ld hl, sp + 102
ld a, [hl]
add a, 0xf8
push af
inc sp
ld hl, 3
add hl, sp
ld c, l
ld b, h
ld hl, sp + 106
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_round
jr .L00126
.L00123:
ld hl, sp + 82
ld c, l
ld b, h
ld hl, sp + 105
ld a, [hli]
ld e, a
ld d, [hl]
call _sf64_pack
.L00126:
add sp, 112
ret

__gbc_f64::
ld e, b
ld d, c
jp _sangoku_sf64
