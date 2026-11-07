; generated from softfloat128.c, sha256 2b959111c315ac61cf5a684b5f83b433ce14c601a67b0bff5aaee7ecd06e9071
SECTION "softfloat128", ROM0
_sf128_clear:
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
_sf128_copy:
add sp, -5
ld hl, sp + 2
ld a, e
ld [hli], a
ld a, d
ld [hli], a
pop de
push bc
ld [hl], 0x00
.L00103:
ld hl, sp + 4
ld a, [hl]
ld hl, sp + 7
sub a, [hl]
jr nc, .L00105
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
jr .L00103
.L00105:
add sp, 5
pop hl
inc sp
jp hl
_sf128_nonzero:
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
_sf128_left:
dec sp
dec sp
ld hl, sp + 1
ld [hld], a
ld [hl], 0x00
ld c, 0x00
.L00103:
ld a, c
ld hl, sp + 1
sub a, [hl]
jr nc, .L00101
ld l, c
ld h, 0x00
add hl, de
ld b, [hl]
ld a, b
rlca
and a, 0x01
sla b
push af
ld a, b
push hl
ld hl, sp + 4
or a, [hl]
pop hl
ld b, a
pop af
ld [hl], b
ld hl, sp + 0
ld [hl], a
inc c
jr .L00103
.L00101:
ld hl, sp + 0
ld a, [hl]
inc sp
inc sp
ret
_sf128_right:
add sp, -8
ld hl, sp + 5
ld [hl], e
inc hl
ld [hl], d
inc hl
ld [hl], a
ld hl, sp + 0
ld [hl], 0x00
.L00101:
ld hl, sp + 7
ld a, [hl]
or a, a
jr z, .L00103
ld a, [hl]
dec a
ld [hld], a
dec hl
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
and a, 0x01
ld [hli], a
pop af
srl a
ld [hl], a
ld hl, sp + 0
ld a, [hl]
rrca
and a, 0x80
ld hl, sp + 4
or a, [hl]
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
jr .L00101
.L00103:
ld hl, sp + 0
ld a, [hl]
add sp, 8
ret
_sf128_shl:
ld c, a
.L00101:
ld a, c
dec c
or a, a
jr z, .L00104
push bc
push de
ld hl, sp + 6
ld a, [hl]
call _sf128_left
pop de
pop bc
jr .L00101
.L00104:
pop hl
inc sp
jp hl
_sf128_shr:
add sp, -7
ld hl, sp + 4
ld a, e
ld [hli], a
ld a, d
ld [hli], a
ld [hl], 0x00
ld hl, sp + 14
ld c, [hl]
ld l, c
ld h, 0x00
add hl, hl
add hl, hl
add hl, hl
ld e, l
ld d, h
pop hl
push de
xor a, a
ld hl, sp + 2
ld [hli], a
ld [hl], a
ld hl, sp + 0
ld e, l
ld d, h
ld hl, sp + 9
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
ld hl, sp + 0
ld d, h
ld e, l
ld hl, sp + 9
ld a, [de]
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
.L00102:
ld hl, sp + 9
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
ld a, c
ld hl, sp + 8
ld e, [hl]
inc hl
ld d, [hl]
call _sf128_right
pop de
pop bc
ld hl, sp + 6
or a, [hl]
ld [hl], a
jr .L00103
.L00105:
ld hl, sp + 13
ld a, [hl]
or a, a
jr z, .L00108
ld hl, sp + 4
ld a, [hli]
ld e, a
ld a, [hli]
ld d, a
ld a, [de]
or a, [hl]
dec hl
dec hl
ld e, [hl]
inc hl
ld h, [hl]
ld l, e
ld [hl], a
.L00108:
add sp, 7
pop hl
add sp, 6
jp hl
_sf128_add:
add sp, -9
ld hl, sp + 6
ld a, e
ld [hli], a
ld [hl], d
ld hl, sp + 4
ld a, c
ld [hli], a
ld [hl], b
ld hl, sp + 0
ld [hl], 0x00
ld hl, sp + 8
ld [hl], 0x00
.L00103:
ld hl, sp + 8
ld a, [hl]
ld hl, sp + 11
sub a, [hl]
jr nc, .L00101
ld hl, sp + 6
ld a, [hli]
inc hl
add a, [hl]
dec hl
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld a, [bc]
ld hl, sp + 1
ld [hl], a
ld hl, sp + 4
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
ld hl, sp + 2
ld [hli], a
xor a, a
ld [hld], a
dec hl
ld a, [hli]
ld d, 0x00
add a, [hl]
ld e, a
jr nc, .L00124
inc d
.L00124:
dec hl
dec hl
ld l, [hl]
ld h, 0x00
add hl, de
ld a, l
ld [bc], a
ld a, h
ld hl, sp + 0
ld [hl], a
ld hl, sp + 8
inc [hl]
jr .L00103
.L00101:
ld hl, sp + 0
ld a, [hl]
add sp, 9
pop hl
inc sp
jp hl
_sf128_increment:
add sp, -3
ld hl, sp + 1
ld [hl], e
inc hl
ld [hl], d
dec hl
dec hl
ld [hl], a
ld c, 0x00
.L00105:
ld a, c
ld hl, sp + 0
sub a, [hl]
jr nc, .L00107
inc hl
ld a, [hli]
add a, c
ld e, a
ld a, [hl]
adc a, 0x00
ld d, a
ld a, [de]
inc a
ld [de], a
or a, a
jr nz, .L00107
inc c
jr .L00105
.L00107:
add sp, 3
ret
_sf128_sub:
add sp, -8
ld hl, sp + 6
ld a, e
ld [hli], a
ld [hl], d
ld hl, sp + 4
ld a, c
ld [hli], a
ld a, b
ld [hld], a
dec hl
ld [hl], 0x00
ld c, 0x00
.L00103:
ld a, c
ld hl, sp + 10
sub a, [hl]
jr nc, .L00105
ld hl, sp + 4
ld a, [hli]
add a, c
ld e, a
ld a, [hld]
dec hl
adc a, 0x00
ld d, a
ld a, [de]
ld e, [hl]
ld d, 0x00
ld l, a
ld h, 0x00
add hl, de
ld e, l
ld d, h
pop hl
push de
ld hl, sp + 6
ld a, [hli]
add a, c
ld e, a
ld a, [hl]
adc a, 0x00
ld d, a
ld a, [de]
ld b, a
ld hl, sp + 2
ld a, b
ld [hli], a
ld [hl], 0x00
push de
ld hl, sp + 4
ld e, l
ld d, h
ld hl, sp + 2
ld a, [de]
inc de
sub a, [hl]
inc hl
ld a, [de]
sbc a, [hl]
pop de
ld a, 0x00
rla
ld hl, sp + 3
ld [hl], a
ld hl, sp + 0
ld a, b
sub a, [hl]
ld [de], a
inc c
jr .L00103
.L00105:
add sp, 8
pop hl
inc sp
jp hl
_sf128_cmp:
add sp, -4
ld hl, sp + 2
ld a, e
ld [hli], a
ld [hl], d
pop de
push bc
ld hl, sp + 6
ld e, [hl]
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
pop hl
inc sp
jp hl
_sf128_copy_float:
add sp, -8
ld hl, sp + 6
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
ld hl, sp + 6
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
ld [hli], a
inc de
ld a, [de]
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
inc de
ld a, [hli]
ld [de], a
inc de
ld a, [hl]
ld [de], a
ld hl, 0x0006
add hl, bc
ld c, l
ld b, h
ld hl, sp + 6
ld a, [hli]
add a, 0x06
ld e, a
ld a, [hl]
adc a, 0x00
ld h, 0x10
push hl
inc sp
ld d, a
call _sf128_copy
add sp, 8
ret
_sf128_unpack:
add sp, -17
ld hl, sp + 14
ld a, e
ld [hli], a
ld [hl], d
ld hl, 0x000f
add hl, bc
ld d, [hl]
ld e, d
res 7, e
ld hl, sp + 13
ld a, e
ld [hld], a
ld [hl], 0x00
ld hl, 0x000e
add hl, bc
ld e, [hl]
ld hl, sp + 4
ld [hl], e
ld hl, sp + 13
ld a, [hl]
ld hl, sp + 5
ld [hl], a
ld a, d
rlca
and a, 0x01
ld hl, sp + 14
ld e, [hl]
inc hl
ld h, [hl]
ld l, e
ld [hl], a
ld hl, sp + 14
ld a, [hl]
add a, 0x02
push af
ld hl, sp + 8
pop af
ld [hl], a
push af
ld hl, sp + 17
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 7
ld [hld], a
ld a, [hli]
ld h, [hl]
ld l, a
xor a, a
ld [hli], a
ld [hli], a
ld [hli], a
ld [hl], a
ld hl, sp + 14
ld a, [hl]
add a, 0x06
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
ld [hld], a
push bc
ld a, 0x10
ld e, [hl]
inc hl
ld d, [hl]
call _sf128_clear
pop bc
ld hl, sp + 14
ld a, [hld]
dec hl
dec hl
dec hl
add a, 0x06
ld [hl], a
push af
ld hl, sp + 17
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 11
ld [hl], a
ld hl, sp + 16
ld [hl], 0x00
.L00112:
ld hl, sp + 10
ld a, [hl]
ld hl, sp + 16
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
ld hl, sp + 16
add a, [hl]
ld e, a
ld a, b
adc a, 0x00
ld d, a
ld a, [de]
ld hl, sp + 12
ld e, [hl]
inc hl
ld h, [hl]
ld l, e
ld [hl], a
ld hl, sp + 16
inc [hl]
ld a, [hl]
sub a, 0x0e
jr c, .L00112
ld hl, sp + 4
ld a, [hli]
ld e, a
ld a, [hl]
ld hl, sp + 14
ld c, [hl]
inc hl
ld b, [hl]
inc bc
cp a, 0x7f
jr nz, .L00103
inc e
jr nz, .L00103
push bc
ld a, 0x10
ld hl, sp + 10
ld e, [hl]
inc hl
ld d, [hl]
call _sf128_nonzero
pop bc
or a, a
jr z, .L00116
ld hl, sp + 16
ld [hl], 0x03
jr .L00117
.L00116:
ld hl, sp + 16
ld [hl], 0x02
.L00117:
ld hl, sp + 16
ld a, [hl]
ld [bc], a
jp .L00114
.L00103:
ld hl, sp + 5
ld a, [hld]
or a, [hl]
jr nz, .L00105
push bc
ld a, 0x10
ld hl, sp + 10
ld e, [hl]
inc hl
ld d, [hl]
call _sf128_nonzero
pop bc
or a,a
jr nz, .L00105
ld [bc], a
jp .L00114
.L00105:
ld a, 0x01
ld [bc], a
ld hl, sp + 6
ld a, [hli]
ld c, a
ld a, [hld]
dec hl
ld b, a
ld a, [hld]
or a, [hl]
jr z, .L00118
ld a, [hl]
ld hl, sp + 0
ld [hl], a
ld hl, sp + 5
ld a, [hl]
ld hl, sp + 1
ld [hli], a
xor a, a
ld [hli], a
ld [hl], a
pop de
push de
ld a, e
sub a, 0xff
ld e, a
ld a, d
sbc a, 0x3f
push af
ld hl, sp + 13
ld [hld], a
ld [hl], e
ld hl, sp + 4
ld a, [hli]
ld e, a
ld d, [hl]
pop af
ld a, e
sbc a, 0x00
ld e, a
ld a, d
sbc a, 0x00
ld hl, sp + 13
ld [hld], a
ld [hl], e
jr .L00119
.L00118:
ld hl, sp + 10
ld a, 0x02
ld [hli], a
ld a, 0xc0
ld [hli], a
ld a, 0xff
ld [hli], a
ld [hl], 0xff
.L00119:
ld hl, sp + 10
ld a, [hli]
ld [bc], a
inc bc
ld a, [hli]
ld [bc], a
inc bc
ld a, [hli]
ld [bc], a
inc bc
ld a, [hli]
ld [bc], a
ld a, [hli]
add a, 0x14
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld hl, sp + 5
ld a, [hld]
or a, [hl]
jr z, .L00139
ld a, [bc]
set 0, a
ld [bc], a
.L00139:
.L00109:
ld a, [bc]
rrca
jr c, .L00111
push bc
ld a, 0x10
ld hl, sp + 10
ld e, [hl]
inc hl
ld d, [hl]
call _sf128_left
pop bc
ld hl, sp + 6
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 2
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
ld hl, sp + 13
ld [hld], a
ld [hl], e
ld hl, sp + 6
ld a, [hli]
ld e, a
ld d, [hl]
pop af
ld a, e
sbc a, 0x00
ld e, a
ld a, d
sbc a, 0x00
ld hl, sp + 13
ld [hld], a
ld [hl], e
ld hl, sp + 6
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 10
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
jr .L00109
.L00111:
ld a, 0x10
push af
inc sp
ld a, 0x08
ld hl, sp + 9
ld e, [hl]
inc hl
ld d, [hl]
call _sf128_shl
.L00114:
add sp, 17
ret
_sf128_normalize:
add sp, -14
ld c, e
ld b, d
ld hl, 0x0006
add hl, bc
pop de
push hl
push bc
ld a, 0x10
pop hl
pop de
push de
push hl
call _sf128_nonzero
pop bc
or a, a
jr nz, .L00118
inc bc
xor a, a
ld [bc], a
jp .L00109
.L00118:
ld hl, 0x0015
add hl, bc
push hl
ld a, l
ld hl, sp + 4
ld [hl], a
pop hl
ld a, h
ld hl, sp + 3
ld [hl], a
.L00103:
ld hl, sp + 2
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 13
ld [hl], a
ld hl, 0x0002
add hl, bc
push hl
ld a, l
ld hl, sp + 6
ld [hl], a
pop hl
ld a, h
ld hl, sp + 5
ld [hl], a
ld a, 0x01
ld hl, sp + 13
sub a, [hl]
jr nc, .L00106
push bc
ld hl, 0x1001
push hl
xor a, a
rrca
push af
ld de, 0x0001
push de
ld hl, sp + 8
ld a, [hli]
ld e, a
ld d, [hl]
call _sf128_shr
pop bc
ld hl, sp + 4
ld a, [hli]
ld e, a
ld a, [hli]
ld d, a
ld a, [de]
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
ld hl, sp + 13
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
ld hl, sp + 13
ld [hld], a
ld [hl], e
ld hl, sp + 4
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 10
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
jr .L00103
.L00106:
pop bc
pop de
push de
push bc
ld a, [de]
or a, a
jr nz, .L00109
ld a, 0x10
pop de
push de
call _sf128_left
ld hl, sp + 4
ld a, [hli]
ld e, a
ld a, [hli]
ld d, a
ld a, [de]
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
ld hl, sp + 13
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
ld hl, sp + 13
ld [hld], a
ld [hl], e
ld hl, sp + 4
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 10
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
jr .L00106
.L00109:
add sp, 14
ret
_sf128_special:
ld l, a
ld h, 0x00
.L00106:
ld a, e
add a, h
ld c, a
ld a, d
adc a, 0x00
ld b, a
xor a, a
ld [bc], a
inc h
ld a, h
sub a, 0x10
jr c, .L00106
ld a, e
add a, 0x0f
ld c, a
ld a, d
adc a, 0x00
ld b, a
ld a, l
rrca
and a, 0x80
ld [bc], a
ld hl, sp + 2
ld a, [hl]
sub a, 0x02
jr c, .L00108
ld a, [bc]
or a, 0x7f
ld [bc], a
ld hl, 0x000e
add hl, de
ld [hl], 0xff
ld hl, sp + 2
ld a, [hl]
sub a, 0x03
jr nz, .L00108
ld hl, 0x000d
add hl, de
ld [hl], 0x80
.L00108:
pop hl
inc sp
jp hl
_sf128_pack:
add sp, -21
ld hl, sp + 18
ld a, e
ld [hli], a
ld [hl], d
ld e, c
ld d, b
inc de
ld a, [de]
ld hl, sp + 8
ld [hl], c
inc hl
ld [hl], b
cp a, 0x01
jr z, .L00102
ld hl, sp + 20
ld [hl], a
cp a, 0x03
jr nz, .L00122
xor a, a
jr .L00123
.L00122:
ld hl, sp + 8
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
.L00123:
ld hl, sp + 20
ld h, [hl]
push hl
inc sp
ld hl, sp + 19
ld e, [hl]
inc hl
ld d, [hl]
call _sf128_special
jp .L00120
.L00102:
push bc
push de
ld e, c
ld d, b
call _sf128_normalize
pop de
pop bc
ld a, [de]
or a, a
jr nz, .L00104
ld hl, sp + 8
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld h, 0x00
push hl
inc sp
ld hl, sp + 19
ld e, [hl]
inc hl
ld d, [hl]
call _sf128_special
jp .L00120
.L00104:
ld hl, 0x0002
add hl, bc
push hl
ld a, l
ld hl, sp + 6
ld [hl], a
pop hl
ld a, h
ld hl, sp + 5
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
ld hl, sp + 0
ld a, 0xff
sub a, [hl]
inc hl
ld a, 0x3f
sbc a, [hl]
inc hl
ld a, 0x00
sbc a, [hl]
inc hl
ld a, 0x00
sbc a, [hl]
ld d, 0x00
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
ld hl, sp + 8
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld h, 0x02
push hl
inc sp
ld hl, sp + 19
ld e, [hl]
inc hl
ld d, [hl]
call _sf128_special
jp .L00120
.L00106:
ld hl, 0x0006
add hl, bc
push hl
ld a, l
ld hl, sp + 18
ld [hl], a
pop hl
ld a, h
ld hl, sp + 17
ld [hl], a
ld hl, sp + 0
ld a, [hli]
sub a, 0x02
ld a, [hli]
sbc a, 0xc0
ld a, [hli]
sbc a, 0xff
ld a, [hl]
sbc a, 0xff
ld d, [hl]
ld e, 0xff
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
ld de, 0xc002
ld a, e
ld hl, sp + 0
sub a, [hl]
inc hl
ld e, a
ld a, d
sbc a, [hl]
push af
ld hl, sp + 15
ld [hld], a
ld [hl], e
ld de, 0xffff
ld hl, sp + 4
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
push bc
ld de, 0x1001
push de
ld a, [hli]
ld e, a
ld d, [hl]
push de
ld hl, sp + 18
ld a, [hli]
ld e, a
ld d, [hl]
push de
ld hl, sp + 24
ld a, [hli]
ld e, a
ld d, [hl]
call _sf128_shr
pop bc
ld hl, sp + 4
ld a, [hli]
ld h, [hl]
ld l, a
ld a, 0x02
ld [hli], a
ld a, 0xc0
ld [hli], a
ld a, 0xff
ld [hli], a
ld [hl], 0xff
.L00108:
push bc
ld hl, 0x1001
push hl
xor a, a
rrca
push af
ld de, 0x0005
push de
ld hl, sp + 24
ld a, [hli]
ld e, a
ld d, [hl]
call _sf128_shr
pop bc
ld hl, sp + 16
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
and a, 0x07
ld hl, sp + 20
ld [hl], a
push bc
ld hl, 0x1000
push hl
xor a, a
rrca
push af
ld de, 0x0003
push de
ld hl, sp + 24
ld a, [hli]
ld e, a
ld d, [hl]
call _sf128_shr
pop bc
ld a, 0x04
ld hl, sp + 20
sub a, [hl]
jr c, .L00109
ld a, [hl]
sub a, 0x04
jr nz, .L00110
ld hl, sp + 16
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
rrca
jr nc, .L00110
.L00109:
push bc
ld a, 0x10
ld hl, sp + 18
ld e, [hl]
inc hl
ld d, [hl]
call _sf128_increment
pop bc
.L00110:
ld hl, 0x0014
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
bit 1, a
jr z, .L00114
push bc
ld a, 0x10
ld hl, sp + 18
ld e, [hl]
inc hl
ld d, [hl]
call _sf128_right
pop bc
ld hl, sp + 4
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 10
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
ld hl, sp + 17
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
ld hl, sp + 17
ld [hld], a
ld [hl], e
ld hl, sp + 4
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 14
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
.L00114:
ld hl, sp + 4
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 14
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
ld hl, sp + 14
ld a, 0xff
sub a, [hl]
inc hl
ld a, 0x3f
sbc a, [hl]
inc hl
ld a, 0x00
sbc a, [hl]
inc hl
ld a, 0x00
sbc a, [hl]
ld d, 0x00
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
ld hl, sp + 8
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld h, 0x02
push hl
inc sp
ld hl, sp + 19
ld e, [hl]
inc hl
ld d, [hl]
call _sf128_special
jp .L00120
.L00116:
ld hl, sp + 14
ld a, [hl]
sub a, 0x02
jr nz, .L00124
inc hl
ld a, [hl]
sub a, 0xc0
jr nz, .L00124
inc hl
ld a, [hl]
inc a
jr nz, .L00124
inc hl
ld a, [hl]
inc a
jr nz, .L00124
ld hl, sp + 6
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
and a, 0x01
jr nz, .L00124
ld de, 0x0000
jr .L00125
.L00124:
ld hl, sp + 14
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, 0x3fff
add hl, de
ld e, l
ld d, h
.L00125:
ld hl, sp + 10
ld a, e
ld [hli], a
ld [hl], d
ld hl, 0x0006
add hl, bc
push hl
ld a, l
ld hl, sp + 14
ld [hl], a
pop hl
ld a, h
ld hl, sp + 13
ld [hl], a
ld hl, sp + 20
ld [hl], 0x00
.L00118:
ld hl, sp + 18
ld a, [hli]
inc hl
add a, [hl]
push af
ld hl, sp + 16
pop af
ld [hl], a
push af
ld hl, sp + 21
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 15
ld [hl], a
ld hl, sp + 12
ld a, [hl]
ld hl, sp + 20
add a, [hl]
dec hl
dec hl
dec hl
dec hl
ld [hld], a
dec hl
dec hl
ld a, [hl]
adc a, 0x00
ld hl, sp + 17
ld [hld], a
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld [hl], a
ld hl, sp + 14
ld a, [hli]
ld e, a
ld a, [hli]
inc hl
ld d, a
ld a, [hl]
ld [de], a
ld hl, sp + 20
inc [hl]
ld a, [hl]
sub a, 0x0e
jr c, .L00118
dec hl
dec hl
ld a, [hli]
add a, 0x0e
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld hl, sp + 10
ld a, [hl]
ld [bc], a
ld hl, sp + 18
ld a, [hli]
add a, 0x0f
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld hl, sp + 11
ld a, [hld]
dec hl
dec hl
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
.L00120:
add sp, 21
ret
_sf128_sum:
add sp, -39
ld hl, sp + 37
ld a, e
ld [hli], a
ld [hl], d
ld hl, sp + 35
ld a, c
ld [hli], a
ld a, b
ld [hld], a
ld a, [hl]
ld hl, sp + 24
ld [hl], a
ld hl, sp + 36
ld a, [hl]
ld hl, sp + 25
ld [hl], a
ld hl, sp + 35
ld a, [hli]
ld c, a
ld b, [hl]
inc bc
ld a, [bc]
ld hl, sp + 26
ld [hl], a
ld hl, sp + 37
ld a, [hl]
add a, 0x01
push af
ld hl, sp + 29
pop af
ld [hl], a
push af
ld hl, sp + 40
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 28
ld [hld], a
dec hl
ld a, [hl]
sub a, 0x03
jr z, .L00101
ld hl, sp + 41
ld a, [hli]
ld h, [hl]
ld l, a
inc hl
ld c, [hl]
ld a, c
sub a, 0x03
jr z, .L00101
ld hl, sp + 26
ld a, [hl]
sub a, 0x02
ld a, 0x01
jr z, .L00244
xor a, a
.L00244:
ld hl, sp + 29
ld [hl], a
ld a, c
sub a, 0x02
ld a, 0x01
jr z, .L00246
xor a, a
.L00246:
ld hl, sp + 30
ld [hl], a
ld hl, sp + 35
ld a, [hl]
ld hl, sp + 31
ld [hl], a
ld hl, sp + 36
ld a, [hl]
ld hl, sp + 32
ld [hl], a
ld hl, sp + 41
ld a, [hl]
ld hl, sp + 33
ld [hl], a
ld hl, sp + 42
ld a, [hl]
ld hl, sp + 34
ld [hl], a
ld hl, sp + 29
ld a, [hl]
or a, a
jr z, .L00102
inc hl
ld a, [hl]
or a, a
jr z, .L00102
inc hl
ld a, [hli]
ld e, a
ld a, [hli]
ld d, a
ld a, [de]
ld e, [hl]
inc hl
ld d, [hl]
push af
ld a, [de]
ld b, a
pop af
sub a, b
jr z, .L00102
.L00101:
ld hl, sp + 27
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x03
jp .L00127
.L00102:
ld hl, sp + 29
ld a, [hl]
or a, a
jr z, .L00108
ld hl, sp + 35
ld a, [hli]
ld c, a
ld a, [hli]
ld b, a
ld a, [hli]
ld e, a
ld d, [hl]
call _sf128_copy_float
jp .L00127
.L00108:
ld hl, sp + 30
ld a, [hl]
or a, a
jr z, .L00110
ld hl, sp + 41
ld a, [hli]
ld c, a
ld b, [hl]
ld hl, sp + 37
ld a, [hli]
ld e, a
ld d, [hl]
call _sf128_copy_float
jp .L00127
.L00110:
ld hl, sp + 37
ld a, [hl]
ld hl, sp + 29
ld [hl], a
ld hl, sp + 38
ld a, [hl]
ld hl, sp + 30
ld [hl], a
ld hl, sp + 26
ld a, [hl]
or a, a
jr nz, .L00112
or a, c
jr nz, .L00112
inc hl
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x00
ld hl, sp + 31
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld [hli], a
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
ld hl, sp + 29
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 34
ld a, [hl]
ld [de], a
jp .L00127
.L00112:
ld hl, sp + 26
ld a, [hl]
or a, a
jr nz, .L00115
ld hl, sp + 41
ld a, [hli]
ld c, a
ld b, [hl]
ld hl, sp + 37
ld a, [hli]
ld e, a
ld d, [hl]
call _sf128_copy_float
jp .L00127
.L00115:
ld a, c
or a, a
jr nz, .L00117
ld hl, sp + 35
ld a, [hli]
ld c, a
ld a, [hli]
ld b, a
ld a, [hli]
ld e, a
ld d, [hl]
call _sf128_copy_float
jp .L00127
.L00117:
ld hl, sp + 35
ld a, [hli]
ld c, a
ld b, [hl]
inc bc
inc bc
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
ld [hl], a
ld hl, sp + 41
ld a, [hli]
ld c, a
ld b, [hl]
inc bc
inc bc
ld e, c
ld d, b
ld a, [de]
ld hl, sp + 31
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
ld hl, sp + 4
ld e, l
ld d, h
ld hl, sp + 31
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
ld hl, sp + 4
ld a, [hl]
ld hl, sp + 31
sub a, [hl]
jr nz, .L00119
ld hl, sp + 5
ld a, [hl]
ld hl, sp + 32
sub a, [hl]
jr nz, .L00119
ld hl, sp + 6
ld a, [hl]
ld hl, sp + 33
sub a, [hl]
jr nz, .L00119
ld hl, sp + 7
ld a, [hl]
ld hl, sp + 34
sub a, [hl]
jr nz, .L00119
ld hl, sp + 41
ld a, [hli]
add a, 0x06
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld hl, sp + 35
ld a, [hli]
add a, 0x06
ld e, a
ld a, [hl]
adc a, 0x00
ld h, 0x10
push hl
inc sp
ld d, a
call _sf128_cmp
bit 7,a
jr z, .L00119
.L00118:
ld hl, sp + 41
ld a, [hl]
ld hl, sp + 24
ld [hl], a
ld hl, sp + 42
ld a, [hl]
ld hl, sp + 25
ld [hl], a
ld hl, sp + 35
ld a, [hl]
ld hl, sp + 41
ld [hl], a
ld hl, sp + 36
ld a, [hl]
ld hl, sp + 42
ld [hl], a
.L00119:
ld hl, sp + 24
ld a, [hli]
ld c, a
ld b, [hl]
ld hl, sp + 37
ld a, [hli]
ld e, a
ld d, [hl]
call _sf128_copy_float
ld hl, sp + 41
ld a, [hli]
add a, 0x06
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld a, 0x10
push af
inc sp
ld hl, 9
add hl, sp
ld e, l
ld d, h
call _sf128_copy
ld hl, sp + 24
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
ld [hli], a
inc de
ld a, [de]
ld [hli], a
inc de
ld a, [de]
ld [hl], a
ld hl, sp + 41
ld a, [hli]
ld c, a
ld b, [hl]
inc bc
inc bc
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
ld [hl], a
pop de
push de
ld a, e
ld hl, sp + 4
sub a, [hl]
inc hl
ld e, a
ld a, d
sbc a, [hl]
push af
ld hl, sp + 34
ld [hld], a
ld [hl], e
ld hl, sp + 4
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 8
pop af
ld a, e
sbc a, [hl]
inc hl
ld e, a
ld a, d
sbc a, [hl]
ld hl, sp + 34
ld [hld], a
ld [hl], e
ld de, 0x1001
push de
ld a, [hli]
ld e, a
ld d, [hl]
push de
ld hl, sp + 35
ld a, [hli]
ld e, a
ld d, [hl]
push de
ld hl, 14
add hl, sp
ld e, l
ld d, h
call _sf128_shr
ld hl, sp + 24
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld c, a
ld hl, sp + 41
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld b, a
ld hl, sp + 37
ld a, [hl]
add a, 0x06
push af
ld hl, sp + 34
pop af
ld [hl], a
push af
ld hl, sp + 40
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 33
ld [hl], a
ld a, c
sub a, b
jr nz, .L00125
ld a, 0x10
push af
inc sp
ld hl, 9
add hl, sp
ld c, l
ld b, h
ld hl, sp + 33
ld a, [hli]
ld e, a
ld d, [hl]
call _sf128_add
jr .L00126
.L00125:
ld a, 0x10
push af
inc sp
ld hl, 9
add hl, sp
ld c, l
ld b, h
ld hl, sp + 33
ld a, [hli]
ld e, a
ld d, [hl]
call _sf128_cmp
ld hl, sp + 34
ld [hl], a
or a, a
jr nz, .L00123
ld hl, sp + 27
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x00
ld hl, sp + 29
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x00
jr .L00127
.L00123:
ld a, 0x10
push af
inc sp
ld hl, 9
add hl, sp
ld c, l
ld b, h
ld hl, sp + 33
ld a, [hli]
ld e, a
ld d, [hl]
call _sf128_sub
.L00126:
ld hl, sp + 37
ld a, [hli]
ld e, a
ld d, [hl]
call _sf128_normalize
.L00127:
add sp, 39
pop hl
pop af
jp hl
_sf128_product:
add sp, -51
ld hl, sp + 48
ld a, e
ld [hli], a
ld [hl], d
ld hl, sp + 46
ld a, c
ld [hli], a
ld [hl], b
ld hl, sp + 50
xor a, a
ld [hld], a
dec hl
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
ld hl, sp + 53
ld e, [hl]
inc hl
ld d, [hl]
push af
ld a, [de]
ld l, a
pop af
xor a, l
ld [bc], a
ld hl, sp + 48
ld a, [hl]
add a, 0x02
push af
ld hl, sp + 38
pop af
ld [hl], a
push af
ld hl, sp + 51
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 37
ld [hld], a
ld a, [hli]
ld h, [hl]
ld l, a
xor a, a
ld [hli], a
ld [hli], a
ld [hli], a
ld [hl], a
ld hl, sp + 48
ld a, [hl]
add a, 0x06
push af
ld hl, sp + 40
pop af
ld [hl], a
push af
ld hl, sp + 51
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 39
ld [hld], a
ld a, 0x10
ld e, [hl]
inc hl
ld d, [hl]
call _sf128_clear
ld hl, sp + 46
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
ld hl, sp + 40
ld [hl], a
ld hl, sp + 48
ld a, [hl]
add a, 0x01
push af
ld hl, sp + 43
pop af
ld [hl], a
push af
ld hl, sp + 51
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 42
ld [hld], a
dec hl
ld a, [hl]
sub a, 0x03
jr z, .L00101
ld hl, sp + 53
ld a, [hli]
ld c, a
ld b, [hl]
inc bc
ld a, [bc]
ld hl, sp + 43
ld [hl], a
sub a, 0x03
jr z, .L00101
ld hl, sp + 40
ld a, [hl]
sub a, 0x02
ld a, 0x01
jr z, .L00197
xor a, a
.L00197:
ld hl, sp + 44
ld [hl], a
or a, a
jr z, .L00107
dec hl
ld a, [hl]
or a, a
jr z, .L00101
.L00107:
ld hl, sp + 43
ld a, [hl]
sub a, 0x02
ld a, 0x01
jr z, .L00199
xor a, a
.L00199:
ld hl, sp + 45
ld [hl], a
or a, a
jr z, .L00102
ld hl, sp + 40
ld a, [hl]
or a, a
jr nz, .L00102
.L00101:
ld hl, sp + 41
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x03
jp .L00119
.L00102:
ld hl, sp + 44
ld a, [hl]
or a, a
jr nz, .L00108
inc hl
ld a, [hl]
or a, a
jr z, .L00109
.L00108:
ld hl, sp + 41
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x02
jp .L00119
.L00109:
ld hl, sp + 40
ld a, [hl]
or a, a
jr z, .L00111
ld hl, sp + 43
ld a, [hl]
or a, a
jr nz, .L00112
.L00111:
ld hl, sp + 41
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x00
jp .L00119
.L00112:
ld hl, sp + 46
ld a, [hli]
add a, 0x06
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld a, 0x10
push af
inc sp
ld hl, 5
add hl, sp
ld e, l
ld d, h
call _sf128_copy
ld a, 0x10
push af
inc sp
ld a, 0x01
ld hl, 5
add hl, sp
ld e, l
ld d, h
call _sf128_shl
ld hl, sp + 53
ld a, [hli]
add a, 0x06
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld a, 0x10
push af
inc sp
ld hl, 21
add hl, sp
ld e, l
ld d, h
call _sf128_copy
ld hl, 0x1000
push hl
xor a, a
rrca
push af
ld de, 0x0008
push de
ld hl, 26
add hl, sp
ld e, l
ld d, h
call _sf128_shr
ld hl, sp + 45
ld [hl], 0x00
.L00117:
ld hl, sp + 20
ld a, [hl]
rrca
jr nc, .L00115
ld a, 0x10
push af
inc sp
ld hl, 5
add hl, sp
ld c, l
ld b, h
ld hl, sp + 39
ld a, [hli]
ld e, a
ld d, [hl]
call _sf128_add
.L00115:
ld a, 0x10
ld hl, sp + 38
ld e, [hl]
inc hl
ld d, [hl]
call _sf128_right
ld hl, sp + 50
or a, [hl]
ld [hl], a
ld a, 0x10
ld hl, 20
add hl, sp
ld e, l
ld d, h
call _sf128_right
ld hl, sp + 45
inc [hl]
ld a, [hl]
sub a, 0x71
jr c, .L00117
ld hl, sp + 38
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 50
or a, [hl]
ld hl, sp + 38
ld e, [hl]
inc hl
ld h, [hl]
ld l, e
ld [hl], a
ld hl, sp + 41
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x01
ld hl, sp + 46
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
ld [hli], a
inc de
ld a, [de]
ld [hli], a
inc de
ld a, [de]
ld [hl], a
ld hl, sp + 53
ld a, [hli]
ld c, a
ld b, [hl]
inc bc
inc bc
ld e, c
ld d, b
ld a, [de]
ld hl, sp + 38
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
pop de
push de
ld a, e
ld hl, sp + 38
add a, [hl]
inc hl
ld e, a
ld a, d
adc a, [hl]
push af
ld hl, sp + 45
ld [hld], a
ld [hl], e
ld hl, sp + 4
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 42
pop af
ld a, e
adc a, [hl]
inc hl
ld e, a
ld a, d
adc a, [hl]
ld hl, sp + 45
ld [hld], a
ld [hl], e
ld hl, sp + 36
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 42
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
ld hl, sp + 48
ld a, [hli]
ld e, a
ld d, [hl]
call _sf128_normalize
.L00119:
add sp, 51
pop hl
pop af
jp hl
_sf128_quotient:
add sp, -50
ld hl, sp + 48
ld a, e
ld [hli], a
ld [hl], d
ld hl, sp + 46
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
ld hl, sp + 52
ld e, [hl]
inc hl
ld d, [hl]
push af
ld a, [de]
ld l, a
pop af
xor a, l
ld [bc], a
ld hl, sp + 46
ld a, [hli]
ld c, a
ld b, [hl]
inc bc
ld a, [bc]
ld hl, sp + 44
ld [hl], a
ld hl, sp + 48
ld a, [hli]
ld e, a
ld d, [hl]
inc de
ld hl, sp + 44
ld a, [hl]
sub a, 0x03
jr z, .L00101
ld hl, sp + 52
ld a, [hli]
ld h, [hl]
ld l, a
inc hl
ld c, [hl]
ld a, c
sub a, 0x03
jr z, .L00101
ld hl, sp + 44
ld a, [hl]
sub a, 0x02
ld a, 0x01
jr z, .L00217
xor a, a
.L00217:
ld hl, sp + 45
ld [hl], a
ld a, c
sub a, 0x02
ld a, 0x01
jr z, .L00219
xor a, a
.L00219:
ld b, a
ld hl, sp + 45
ld a, [hl]
or a, a
jr z, .L00107
ld a, b
or a, a
jr nz, .L00101
.L00107:
ld hl, sp + 44
ld a, [hl]
or a, a
jr nz, .L00102
or a, c
jr nz, .L00102
.L00101:
ld a, 0x03
ld [de], a
jp .L00123
.L00102:
ld hl, sp + 45
ld a, [hl]
or a, a
jr nz, .L00108
or a, c
jr nz, .L00109
.L00108:
ld a, 0x02
ld [de], a
jp .L00123
.L00109:
ld hl, sp + 44
ld a, [hl]
or a, a
jr z, .L00111
ld a, b
or a, a
jr z, .L00112
.L00111:
xor a, a
ld [de], a
jp .L00123
.L00112:
ld a, 0x01
ld [de], a
ld hl, sp + 48
ld a, [hl]
add a, 0x02
push af
ld hl, sp + 34
pop af
ld [hl], a
push af
ld hl, sp + 51
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 33
ld [hl], a
ld hl, sp + 46
ld a, [hli]
ld c, a
ld b, [hl]
inc bc
inc bc
ld e, c
ld d, b
ld a, [de]
ld hl, sp + 34
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
ld hl, sp + 52
ld a, [hli]
ld c, a
ld b, [hl]
inc bc
inc bc
ld e, c
ld d, b
ld a, [de]
ld hl, sp + 38
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
ld hl, sp + 34
ld a, [hli]
ld e, a
ld d, [hl]
ld a, e
ld hl, sp + 38
sub a, [hl]
inc hl
ld e, a
ld a, d
sbc a, [hl]
push af
ld hl, sp + 45
ld [hld], a
ld [hl], e
ld hl, sp + 38
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 42
pop af
ld a, e
sbc a, [hl]
inc hl
ld e, a
ld a, d
sbc a, [hl]
ld hl, sp + 45
ld [hld], a
ld [hl], e
ld hl, sp + 32
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 42
ld a, [hli]
ld [de], a
inc de
ld a, [hli]
ld [de], a
inc de
ld a, [hli]
ld [de], a
inc de
ld a, [hli]
ld [de], a
ld a, [hli]
add a, 0x06
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld a, 0x10
push af
inc sp
ld hl, 1
add hl, sp
ld e, l
ld d, h
call _sf128_copy
ld hl, 0x1000
push hl
xor a, a
rrca
push af
ld de, 0x0008
push de
ld hl, 6
add hl, sp
ld e, l
ld d, h
call _sf128_shr
ld hl, sp + 52
ld a, [hli]
add a, 0x06
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld a, 0x10
push af
inc sp
ld hl, 17
add hl, sp
ld e, l
ld d, h
call _sf128_copy
ld hl, 0x1000
push hl
xor a, a
rrca
push af
ld de, 0x0008
push de
ld hl, 22
add hl, sp
ld e, l
ld d, h
call _sf128_shr
ld a, 0x10
push af
inc sp
ld hl, 17
add hl, sp
ld c, l
ld b, h
ld hl, 1
add hl, sp
ld e, l
ld d, h
call _sf128_cmp
bit 7,a
jr z, .L00115
ld a, 0x10
ld hl, 0
add hl, sp
ld e, l
ld d, h
call _sf128_left
ld hl, sp + 32
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 38
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
ld hl, sp + 45
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
ld hl, sp + 45
ld [hld], a
ld [hl], e
ld hl, sp + 32
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 42
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
ld hl, sp + 48
ld a, [hl]
add a, 0x06
push af
ld hl, sp + 44
pop af
ld [hl], a
push af
ld hl, sp + 51
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 43
ld [hld], a
ld a, 0x10
ld e, [hl]
inc hl
ld d, [hl]
call _sf128_clear
ld hl, sp + 45
ld [hl], 0x00
.L00121:
ld a, 0x10
ld hl, sp + 42
ld e, [hl]
inc hl
ld d, [hl]
call _sf128_left
ld a, 0x10
push af
inc sp
ld hl, 17
add hl, sp
ld c, l
ld b, h
ld hl, 1
add hl, sp
ld e, l
ld d, h
call _sf128_cmp
bit 7,a
jr nz, .L00117
ld a, 0x10
push af
inc sp
ld hl, 17
add hl, sp
ld c, l
ld b, h
ld hl, 1
add hl, sp
ld e, l
ld d, h
call _sf128_sub
ld hl, sp + 42
ld a, [hli]
ld e, a
ld a, [hli]
ld d, a
ld a, [de]
or a, 0x01
ld [hld], a
dec hl
ld a, [hli]
ld e, a
ld a, [hli]
ld d, a
ld a, [hl]
ld [de], a
.L00117:
ld a, 0x10
ld hl, 0
add hl, sp
ld e, l
ld d, h
call _sf128_left
ld hl, sp + 45
inc [hl]
ld a, [hl]
sub a, 0x74
jr c, .L00121
ld a, 0x10
ld hl, 0
add hl, sp
ld e, l
ld d, h
call _sf128_nonzero
or a, a
jr z, .L00120
ld hl, sp + 42
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
.L00120:
ld a, 0x10
push af
inc sp
ld a, 0x05
ld hl, sp + 43
ld e, [hl]
inc hl
ld d, [hl]
call _sf128_shl
.L00123:
add sp, 50
pop hl
pop af
jp hl
_sf128_root:
add sp, -63
ld hl, sp + 58
ld a, e
ld [hli], a
ld a, d
ld [hld], a
push bc
ld a, [hli]
ld e, a
ld d, [hl]
call _sf128_copy_float
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
ld hl, sp + 58
ld a, [hli]
ld h, [hl]
ld l, a
inc hl
ld [hl], 0x03
jp .L00122
.L00102:
dec e
jp nz, .L00122
ld hl, 0x0006
add hl, bc
ld c, l
ld b, h
ld a, 0x10
push af
inc sp
ld hl, 9
add hl, sp
ld e, l
ld d, h
call _sf128_copy
ld hl, 0x1000
push hl
xor a, a
rrca
push af
ld de, 0x0008
push de
ld hl, 14
add hl, sp
ld e, l
ld d, h
call _sf128_shr
ld hl, sp + 58
ld a, [hli]
inc hl
inc hl
add a, 0x02
ld [hld], a
dec hl
ld a, [hl]
adc a, 0x00
ld hl, sp + 62
ld [hld], a
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
bit 0,a
jr z, .L00108
ld a, 0x10
ld hl, 8
add hl, sp
ld e, l
ld d, h
call _sf128_left
ld hl, sp + 61
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
ld [hli], a
inc hl
pop de
push de
ld a, e
sub a, 0x01
ld e, a
ld a, d
sbc a, 0x00
push af
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
ld hl, sp + 7
ld [hld], a
ld [hl], e
ld hl, sp + 61
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 4
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
ld hl, sp + 61
ld a, [hli]
ld c, a
ld a, [hld]
ld b, a
ld a, [hli]
ld e, a
ld d, [hl]
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
ld hl, sp + 4
ld d, h
ld e, l
ld hl, sp + 0
ld a, [de]
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
ld hl, sp + 7
bit 7, [hl]
jr z, .L00124
dec hl
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
ld hl, sp + 3
ld [hld], a
ld [hl], e
ld hl, sp + 8
ld a, [hli]
ld e, a
ld d, [hl]
pop af
ld a, e
adc a, 0x00
ld e, a
ld a, d
adc a, 0x00
ld hl, sp + 3
ld [hld], a
ld [hl], e
.L00124:
ld hl, sp + 0
ld a, [hl]
ld hl, sp + 4
ld [hl], a
ld hl, sp + 1
ld a, [hl]
ld hl, sp + 5
ld [hl], a
ld hl, sp + 2
ld a, [hl]
ld hl, sp + 6
ld [hl], a
ld hl, sp + 3
ld a, [hl]
ld hl, sp + 7
ld [hl], a
sra [hl]
dec hl
rr [hl]
dec hl
rr [hl]
dec hl
rr [hl]
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
ld a, 0x10
ld hl, 24
add hl, sp
ld e, l
ld d, h
call _sf128_clear
ld hl, sp + 58
ld a, [hld]
dec hl
add a, 0x06
ld [hli], a
inc hl
inc hl
ld a, [hld]
dec hl
adc a, 0x00
ld [hld], a
ld a, 0x10
ld e, [hl]
inc hl
ld d, [hl]
call _sf128_clear
ld hl, sp + 62
ld a, 0xe7
ld [hld], a
dec hl
ld [hl], 0x00
.L00120:
ld hl, sp + 61
ld [hl], 0x00
ld c, 0x02
.L00119:
ld hl, sp + 61
sla [hl]
inc hl
ld a, [hl]
cp a, 0x76
jr c, .L00110
add a, 0x8a
ld b, a
ld e, b
srl e
srl e
srl e
ld d, 0x00
ld hl, 8
add hl, sp
add hl, de
ld e, [hl]
ld a, b
and a, 0x07
inc a
jr .L00229
.L00228:
srl e
.L00229:
dec a
jr nz, .L00228
ld a, e
and a, 0x01
ld hl, sp + 61
or a, [hl]
ld [hl], a
.L00110:
ld hl, sp + 62
ld a, [hl]
add a, 0xff
ld [hl], a
dec c
jr nz, .L00119
ld a, 0x10
push af
inc sp
ld a, 0x02
ld hl, 25
add hl, sp
ld e, l
ld d, h
call _sf128_shl
ld hl, sp + 24
ld a, [hl]
ld hl, sp + 61
or a, [hl]
ld hl, sp + 24
ld [hl], a
ld a, 0x10
ld hl, sp + 56
ld e, [hl]
inc hl
ld d, [hl]
call _sf128_left
ld hl, sp + 40
ld e,l
ld d,h
push hl
push de
ld a, 0x10
push af
inc sp
push hl
ld hl, sp + 63
ld c, [hl]
ld hl, sp + 64
ld b, [hl]
pop de
call _sf128_copy
pop de
pop hl
push hl
push de
ld a, 0x10
ld e, l
ld d, h
call _sf128_left
pop de
pop hl
ld a, [de]
set 0, a
ld [de], a
push hl
ld a, 0x10
push af
inc sp
ld c, l
ld b, h
push hl
ld hl, 29
add hl, sp
ld e, l
ld d, h
pop hl
call _sf128_cmp
ld c, a
pop hl
bit 7, c
jr nz, .L00121
ld a, 0x10
push af
inc sp
ld c, l
ld b, h
ld hl, 25
add hl, sp
ld e, l
ld d, h
call _sf128_sub
ld hl, sp + 56
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
or a, 0x01
ld hl, sp + 61
ld [hl], a
ld hl, sp + 56
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 61
ld a, [hl]
ld [de], a
.L00121:
ld hl, sp + 60
inc [hl]
ld a, [hl]
sub a, 0x74
jp c, .L00120
ld a, 0x10
ld hl, 24
add hl, sp
ld e, l
ld d, h
call _sf128_nonzero
or a, a
jr z, .L00116
ld hl, sp + 56
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
.L00116:
ld a, 0x10
push af
inc sp
ld a, 0x05
ld hl, sp + 57
ld e, [hl]
inc hl
ld d, [hl]
call _sf128_shl
.L00122:
add sp, 63
ret
_sf128_raw_kind:
add sp, -4
ld c, 0x00
ld hl, 0x000f
add hl, de
ld b, [hl]
res 7, b
ld hl, sp + 1
ld a, b
ld [hld], a
ld [hl], 0x00
ld hl, 0x000e
add hl, de
ld b, [hl]
ld hl, sp + 2
ld a, b
ld [hld], a
ld a, [hli]
inc hl
ld [hl], a
ld b, 0x00
.L00104:
ld l, b
ld h, 0x00
add hl, de
ld a, [hl]
or a, c
ld c, a
inc b
ld a, b
sub a, 0x0e
jr c, .L00104
ld hl, sp + 2
ld a, [hli]
ld b, a
ld a, [hl]
cp a, 0x7f
jr nz, .L00103
inc b
jr nz, .L00103
ld a, c
or a, a
jr z, .L00108
ld a, 0x03
jr .L00106
.L00108:
ld a, 0x02
jr .L00106
.L00103:
ld hl, sp + 3
ld a, [hld]
or a, [hl]
jr nz, .L00113
or a, c
jr z, .L00110
.L00113:
ld a, 0x01
jr .L00111
.L00110:
xor a, a
.L00111:
.L00106:
add sp, 4
ret
_sf128_compare_raw:
add sp, -8
ld hl, sp + 5
ld a, e
ld [hli], a
ld [hl], d
ld hl, sp + 3
ld a, c
ld [hli], a
ld a, b
ld [hli], a
ld a, [hli]
ld e, a
ld d, [hl]
call _sf128_raw_kind
ld d, a
push de
ld hl, sp + 5
ld a, [hli]
ld e, a
ld d, [hl]
call _sf128_raw_kind
ld hl, sp + 2
ld [hl], a
pop de
ld hl, sp + 5
ld a, [hli]
add a, 0x0f
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld a, [bc]
rlca
and a, 0x01
ld hl, sp + 1
ld [hli], a
inc hl
ld a, [hli]
add a, 0x0f
ld c, a
ld a, [hld]
dec hl
adc a, 0x00
ld b, a
ld a, [bc]
rlca
and a, 0x01
ld [hl], a
ld hl, sp + 7
ld [hl], 0x01
ld a, d
sub a, 0x03
jr z, .L00101
ld hl, sp + 0
ld a, [hl]
sub a, 0x03
jr nz, .L00102
.L00101:
ld a, 0x08
jr .L00119
.L00102:
ld a, d
or a, a
jr nz, .L00105
ld hl, sp + 0
ld a, [hl]
or a, a
jr nz, .L00105
ld a, 0x01
jr .L00119
.L00105:
ld hl, sp + 1
ld a, [hli]
sub a, [hl]
jr z, .L00144
ld hl, sp + 1
ld a, [hl]
or a, a
jr z, .L00121
ld a, 0x02
jr .L00119
.L00121:
ld a, 0x04
jr .L00119
.L00144:
ld c, 0x10
.L00113:
ld a, c
or a, a
jr z, .L00115
dec c
ld hl, sp + 5
ld a, [hli]
add a, c
ld b, a
ld a, [hl]
adc a, 0x00
ld l, b
ld h, a
ld b, [hl]
ld hl, sp + 3
ld a, [hli]
add a, c
ld e, a
ld a, [hl]
adc a, 0x00
ld d, a
ld a, [de]
ld e, a
ld a, c
sub a, 0x0f
jr nz, .L00110
res 7, b
res 7, e
.L00110:
ld a, b
cp a, e
jr z, .L00113
sub a, e
jr nc, .L00123
ld a, 0x02
jr .L00124
.L00123:
ld a, 0x04
.L00124:
ld hl, sp + 7
ld [hl], a
.L00115:
ld hl, sp + 1
ld a, [hl]
or a, a
jr z, .L00117
ld hl, sp + 7
ld a, [hl]
dec a
jr z, .L00117
ld hl, sp + 7
ld a, [hl]
xor a, 0x06
ld [hl], a
.L00117:
ld hl, sp + 7
ld a, [hl]
.L00119:
add sp, 8
ret
_sf128_round:
add sp, -36
ld hl, sp + 33
ld a, e
ld [hli], a
ld [hl], d
ld hl, sp + 31
ld a, c
ld [hli], a
ld [hl], b
ld hl, sp + 16
ld [hl], 0x00
ld hl, sp + 35
ld [hl], 0x00
ld hl, sp + 17
ld [hl], 0x00
ld hl, sp + 31
ld a, [hli]
ld h, [hl]
ld l, a
inc hl
ld c, [hl]
dec c
jr nz, .L00101
ld hl, sp + 31
ld a, [hl]
add a, 0x02
push af
ld hl, sp + 20
pop af
ld [hl], a
push af
ld hl, sp + 34
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
ld [hli], a
inc de
ld a, [de]
ld [hli], a
inc de
ld a, [de]
ld [hl], a
ld hl, sp + 20
ld a, [hli]
sub a, 0x70
ld a, [hli]
sbc a, 0x00
ld a, [hli]
sbc a, 0x00
ld a, [hl]
sbc a, 0x00
ld d, [hl]
ld e, 0x00
bit 7, e
jr z, .L00316
bit 7, d
jr nz, .L00317
cp a, a
jr .L00317
.L00316:
bit 7, d
jr z, .L00317
scf
.L00317:
jr c, .L00102
.L00101:
ld hl, sp + 31
ld a, [hli]
ld c, a
ld a, [hli]
ld b, a
ld a, [hli]
ld e, a
ld d, [hl]
call _sf128_pack
jp .L00130
.L00102:
ld hl, sp + 38
ld a, [hl]
dec a
ld a, 0x01
jr z, .L00319
xor a, a
.L00319:
ld hl, sp + 24
ld [hl], a
ld hl, sp + 31
ld a, [hl]
ld hl, sp + 25
ld [hl], a
ld hl, sp + 32
ld a, [hl]
ld hl, sp + 26
ld [hl], a
ld hl, sp + 38
ld a, [hl]
sub a, 0x02
ld a, 0x01
jr z, .L00321
xor a, a
.L00321:
ld hl, sp + 27
ld [hl], a
ld hl, sp + 38
ld a, [hl]
sub a, 0x03
ld a, 0x01
jr z, .L00323
xor a, a
.L00323:
ld hl, sp + 28
ld [hl], a
ld hl, sp + 31
ld a, [hld]
dec hl
add a, 0x06
ld [hli], a
inc hl
inc hl
ld a, [hld]
dec hl
adc a, 0x00
ld [hl], a
ld hl, sp + 23
bit 7, [hl]
jp z, .L00114
inc hl
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
ld hl, sp + 27
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
ld hl, sp + 28
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
inc hl
ld a, [hl]
inc a
jr nz, .L00109
inc hl
ld a, [hl]
inc a
jr nz, .L00109
ld a, 0x10
push af
inc sp
ld hl, sp + 30
ld a, [hli]
ld c, a
ld b, [hl]
ld hl, 1
add hl, sp
ld e, l
ld d, h
call _sf128_copy
ld hl, sp + 15
res 0, [hl]
ld a, 0x10
ld hl, 0
add hl, sp
ld e, l
ld d, h
call _sf128_nonzero
ld hl, sp + 17
ld [hl], a
.L00109:
ld hl, sp + 25
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld h, 0x00
push hl
inc sp
ld hl, sp + 34
ld e, [hl]
inc hl
ld d, [hl]
call _sf128_special
ld hl, sp + 17
ld a, [hl]
or a, a
jp z, .L00130
ld hl, sp + 33
ld a, [hli]
add a, 0x0e
ld c, a
ld a, [hl]
adc a, 0x00
ld l, c
ld h, a
ld [hl], 0xff
ld hl, sp + 33
ld a, [hli]
add a, 0x0f
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld a, [bc]
or a, 0x3f
ld [bc], a
jp .L00130
.L00114:
ld a, 0x10
push af
inc sp
ld hl, sp + 30
ld a, [hli]
ld c, a
ld b, [hl]
ld hl, 1
add hl, sp
ld e, l
ld d, h
call _sf128_copy
ld hl, sp + 18
ld a, [hli]
ld e, a
ld a, [hli]
inc hl
inc hl
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
ld hl, sp + 22
ld a, c
sub a, [hl]
inc hl
ld a, b
sbc a, [hl]
ld d, b
bit 7, [hl]
jr z, .L00326
bit 7, d
jr nz, .L00327
cp a, a
jr .L00327
.L00326:
bit 7, d
jr z, .L00327
scf
.L00327:
jr nc, .L00118
push bc
ld a, 0x10
ld hl, 2
add hl, sp
ld e, l
ld d, h
call _sf128_right
ld e, a
pop bc
ld l, c
ld h, b
inc hl
ld d, h
ld a, l
ld hl, sp + 22
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
ld hl, sp + 35
ld a, [hl]
or a, e
ld [hl], a
.L00129:
inc bc
jr .L00128
.L00118:
ld hl, sp + 24
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
ld hl, sp + 35
ld a, [hl]
or a, a
jr nz, .L00133
.L00132:
ld hl, sp + 24
ld [hl], 0x00
jr .L00134
.L00133:
ld hl, sp + 24
ld [hl], 0x01
.L00134:
ld hl, sp + 24
ld a, [hl]
ld hl, sp + 17
ld [hl], a
.L00120:
ld hl, sp + 27
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
ld hl, sp + 35
ld a, [hl]
or a, a
jr nz, .L00139
.L00138:
ld hl, sp + 27
ld [hl], 0x00
jr .L00140
.L00139:
ld hl, sp + 27
ld [hl], 0x01
.L00140:
ld hl, sp + 27
ld a, [hl]
ld hl, sp + 17
ld [hl], a
.L00122:
ld hl, sp + 28
ld a, [hl]
or a, a
jr z, .L00124
ld hl, sp + 16
ld a, [hl]
or a, a
jr z, .L00144
ld hl, sp + 35
ld a, [hl]
or a, a
jr nz, .L00145
ld hl, sp + 0
ld a, [hl]
and a, 0x01
ld hl, sp + 35
ld [hl], a
or a, a
jr nz, .L00145
.L00144:
ld hl, sp + 35
ld [hl], 0x00
jr .L00146
.L00145:
ld hl, sp + 35
ld [hl], 0x01
.L00146:
ld hl, sp + 35
ld a, [hl]
ld hl, sp + 17
ld [hl], a
.L00124:
ld hl, sp + 17
ld a, [hl]
or a, a
jr z, .L00126
ld a, 0x10
ld hl, 0
add hl, sp
ld e, l
ld d, h
call _sf128_increment
.L00126:
ld hl, sp + 22
ld a, [hl]
ld h, 0x10
push hl
inc sp
ld hl, 1
add hl, sp
ld e, l
ld d, h
call _sf128_shl
ld a, 0x10
push af
inc sp
ld hl, 1
add hl, sp
ld c, l
ld b, h
ld hl, sp + 30
ld a, [hli]
ld e, a
ld d, [hl]
call _sf128_copy
ld hl, sp + 31
ld a, [hli]
ld c, a
ld a, [hli]
ld b, a
ld a, [hli]
ld e, a
ld d, [hl]
call _sf128_pack
.L00130:
add sp, 36
pop hl
inc sp
jp hl
_sf128_fused_finite:
add sp, -94
ld hl, sp + 87
ld a, e
ld [hli], a
ld [hl], d
ld l, c
ld h, b
inc hl
inc hl
ld e, l
ld d, h
ld a, [de]
ld hl, sp + 79
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
ld hl, sp + 96
ld a, [hli]
ld h, [hl]
ld l, a
inc hl
inc hl
ld e, l
ld d, h
ld a, [de]
ld hl, sp + 83
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
ld hl, sp + 79
ld a, [hli]
ld e, a
ld d, [hl]
ld a, e
ld hl, sp + 83
add a, [hl]
inc hl
ld e, a
ld a, d
adc a, [hl]
push af
ld hl, sp + 92
ld [hld], a
ld [hl], e
ld hl, sp + 83
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 87
pop af
ld a, e
adc a, [hl]
inc hl
ld e, a
ld a, d
adc a, [hl]
ld hl, sp + 92
ld [hld], a
ld [hl], e
ld hl, sp + 98
ld a, [hl]
ld hl, sp + 74
ld [hl], a
ld hl, sp + 99
ld a, [hl]
ld hl, sp + 75
ld [hld], a
ld a, [hli]
ld h, [hl]
ld l, a
inc hl
inc hl
ld e, l
ld d, h
ld a, [de]
ld hl, sp + 76
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
ld a, [bc]
ld hl, sp + 96
ld e, [hl]
inc hl
ld d, [hl]
push af
ld a, [de]
ld l, a
pop af
xor a, l
ld hl, sp + 80
ld [hl], a
push bc
ld a, 0x1d
ld hl, 2
add hl, sp
ld e, l
ld d, h
call _sf128_clear
ld a, 0x1d
ld hl, 31
add hl, sp
ld e, l
ld d, h
call _sf128_clear
pop bc
ld hl, 0x0006
add hl, bc
push hl
ld a, l
ld hl, sp + 87
ld [hl], a
pop hl
ld a, h
ld hl, sp + 86
ld [hl], a
ld hl, sp + 93
ld [hl], 0x00
.L00126:
ld hl, sp + 93
ld e, [hl]
ld d, 0x00
ld hl, 29
add hl, sp
add hl, de
ld c, l
ld b, h
ld hl, sp + 85
ld a, [hl]
ld hl, sp + 93
add a, [hl]
ld e, a
push af
ld hl, sp + 88
pop af
ld a, [hl]
adc a, 0x00
ld d, a
ld a, [de]
ld [bc], a
ld hl, sp + 93
inc [hl]
ld a, [hl]
sub a, 0x10
jr c, .L00126
ld hl, 0x1d00
push hl
xor a, a
rrca
push af
ld de, 0x0008
push de
ld hl, 35
add hl, sp
ld e, l
ld d, h
call _sf128_shr
ld hl, sp + 96
ld a, [hli]
add a, 0x06
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld a, 0x10
push af
inc sp
ld hl, 59
add hl, sp
ld e, l
ld d, h
call _sf128_copy
ld hl, 0x1000
push hl
xor a, a
rrca
push af
ld de, 0x0008
push de
ld hl, 64
add hl, sp
ld e, l
ld d, h
call _sf128_shr
ld hl, sp + 93
ld [hl], 0x00
.L00128:
ld hl, sp + 58
ld a, [hl]
rrca
jr nc, .L00103
ld a, 0x1d
push af
inc sp
ld hl, 30
add hl, sp
ld c, l
ld b, h
ld hl, 1
add hl, sp
ld e, l
ld d, h
call _sf128_add
.L00103:
ld a, 0x1d
ld hl, 29
add hl, sp
ld e, l
ld d, h
call _sf128_left
ld a, 0x10
ld hl, 58
add hl, sp
ld e, l
ld d, h
call _sf128_right
ld hl, sp + 93
inc [hl]
ld a, [hl]
sub a, 0x71
jr c, .L00128
ld a, 0x1d
push af
inc sp
ld a, 0x04
ld hl, 1
add hl, sp
ld e, l
ld d, h
call _sf128_shl
ld hl, sp + 28
bit 5, [hl]
jr z, .L00106
ld hl, 0x1d01
push hl
xor a, a
rrca
push af
ld de, 0x0001
push de
ld hl, 6
add hl, sp
ld e, l
ld d, h
call _sf128_shr
ld hl, sp + 89
inc [hl]
jr nz, .L00267
inc hl
inc [hl]
jr nz, .L00267
inc hl
inc [hl]
jr nz, .L00267
inc hl
inc [hl]
.L00267:
.L00106:
ld a, 0x1d
ld hl, 29
add hl, sp
ld e, l
ld d, h
call _sf128_clear
ld hl, sp + 74
ld a, [hl]
add a, 0x06
push af
ld hl, sp + 83
pop af
ld [hl], a
push af
ld hl, sp + 77
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 82
ld [hl], a
ld hl, sp + 93
ld [hl], 0x00
.L00130:
ld hl, sp + 93
ld e, [hl]
ld d, 0x00
ld hl, 29
add hl, sp
add hl, de
push hl
ld a, l
ld hl, sp + 85
ld [hl], a
pop hl
ld a, h
ld hl, sp + 84
ld [hl], a
ld hl, sp + 81
ld a, [hl]
ld hl, sp + 93
add a, [hl]
push af
ld hl, sp + 87
pop af
ld [hld], a
dec hl
dec hl
ld a, [hl]
adc a, 0x00
ld hl, sp + 86
ld [hld], a
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld [hl], a
ld hl, sp + 83
ld a, [hli]
ld e, a
ld a, [hli]
inc hl
ld d, a
ld a, [hl]
ld [de], a
ld hl, sp + 93
inc [hl]
ld a, [hl]
sub a, 0x10
jr c, .L00130
ld a, 0x1d
push af
inc sp
ld a, 0x6c
ld hl, 30
add hl, sp
ld e, l
ld d, h
call _sf128_shl
ld hl, sp + 76
ld e, l
ld d, h
ld hl, sp + 89
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
jr z, .L00268
bit 7, d
jr nz, .L00269
cp a, a
jr .L00269
.L00268:
bit 7, d
jr z, .L00269
scf
.L00269:
jr nc, .L00111
ld hl, sp + 89
ld a, [hli]
ld e, a
ld d, [hl]
ld a, e
ld hl, sp + 76
sub a, [hl]
inc hl
ld e, a
ld a, d
sbc a, [hl]
push af
ld hl, sp + 86
ld [hld], a
ld [hl], e
ld hl, sp + 93
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 80
pop af
ld a, e
sbc a, [hl]
inc hl
ld e, a
ld a, d
sbc a, [hl]
ld hl, sp + 86
ld [hld], a
ld [hl], e
ld de, 0x1d01
push de
ld a, [hli]
ld e, a
ld d, [hl]
push de
ld hl, sp + 87
ld a, [hli]
ld e, a
ld d, [hl]
push de
ld hl, 35
add hl, sp
ld e, l
ld d, h
call _sf128_shr
jr .L00112
.L00111:
ld hl, sp + 89
ld e, l
ld d, h
ld hl, sp + 76
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
jr z, .L00270
bit 7, d
jr nz, .L00271
cp a, a
jr .L00271
.L00270:
bit 7, d
jr z, .L00271
scf
.L00271:
jr nc, .L00112
ld hl, sp + 76
ld a, [hli]
ld e, a
ld d, [hl]
ld a, e
ld hl, sp + 89
sub a, [hl]
inc hl
ld e, a
ld a, d
sbc a, [hl]
push af
ld hl, sp + 86
ld [hld], a
ld [hl], e
ld hl, sp + 80
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 93
pop af
ld a, e
sbc a, [hl]
inc hl
ld e, a
ld a, d
sbc a, [hl]
ld hl, sp + 86
ld [hld], a
ld [hl], e
ld de, 0x1d01
push de
ld a, [hli]
ld e, a
ld d, [hl]
push de
ld hl, sp + 87
ld a, [hli]
ld e, a
ld d, [hl]
push de
ld hl, 6
add hl, sp
ld e, l
ld d, h
call _sf128_shr
ld hl, sp + 76
ld d, h
ld e, l
ld hl, sp + 89
ld a, [de]
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
.L00112:
ld hl, sp + 87
ld a, [hld]
dec hl
dec hl
dec hl
add a, 0x01
ld [hl], a
push af
ld hl, sp + 90
pop af
ld a, [hl]
adc a, 0x00
ld hl, sp + 84
ld [hld], a
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x01
ld hl, sp + 87
ld a, [hld]
dec hl
add a, 0x02
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
ld hl, sp + 89
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
ld hl, sp + 74
ld a, [hl]
ld hl, sp + 92
ld [hl], a
ld hl, sp + 75
ld a, [hl]
ld hl, sp + 93
ld [hld], a
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld c, a
ld hl, sp + 87
ld a, [hl]
ld hl, sp + 90
ld [hld], a
dec hl
ld a, [hl]
ld hl, sp + 91
ld [hl], a
ld hl, sp + 80
ld a, [hl]
sub a, c
jr nz, .L00117
ld a, 0x1d
push af
inc sp
ld hl, 30
add hl, sp
ld c, l
ld b, h
ld hl, 1
add hl, sp
ld e, l
ld d, h
call _sf128_add
ld hl, sp + 90
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 80
ld a, [hl]
ld [de], a
jr .L00118
.L00117:
ld a, 0x1d
push af
inc sp
ld hl, 30
add hl, sp
ld c, l
ld b, h
ld hl, 1
add hl, sp
ld e, l
ld d, h
call _sf128_cmp
bit 7,a
jr nz, .L00114
ld a, 0x1d
push af
inc sp
ld hl, 30
add hl, sp
ld c, l
ld b, h
ld hl, 1
add hl, sp
ld e, l
ld d, h
call _sf128_sub
ld hl, sp + 90
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 80
ld a, [hl]
ld [de], a
jr .L00118
.L00114:
ld a, 0x1d
push af
inc sp
ld hl, 1
add hl, sp
ld c, l
ld b, h
ld hl, 30
add hl, sp
ld e, l
ld d, h
call _sf128_sub
ld a, 0x1d
push af
inc sp
ld hl, 30
add hl, sp
ld c, l
ld b, h
ld hl, 1
add hl, sp
ld e, l
ld d, h
call _sf128_copy
ld hl, sp + 92
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 90
ld e, [hl]
inc hl
ld h, [hl]
ld l, e
ld [hl], a
.L00118:
ld a, 0x1d
ld hl, 0
add hl, sp
ld e, l
ld d, h
call _sf128_nonzero
or a, a
jr nz, .L00120
ld hl, sp + 83
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x00
ld hl, sp + 90
ld a, [hli]
ld h, [hl]
ld l, a
ld [hl], 0x00
jp .L00132
.L00120:
ld hl, sp + 28
bit 5, [hl]
jr z, .L00123
ld hl, 0x1d01
push hl
xor a, a
rrca
push af
ld de, 0x0001
push de
ld hl, 6
add hl, sp
ld e, l
ld d, h
call _sf128_shr
ld hl, sp + 85
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 81
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
ld hl, sp + 93
ld [hld], a
ld [hl], e
ld hl, sp + 85
ld a, [hli]
ld e, a
ld d, [hl]
pop af
ld a, e
adc a, 0x00
ld e, a
ld a, d
adc a, 0x00
ld hl, sp + 93
ld [hld], a
ld [hl], e
ld hl, sp + 85
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 90
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
.L00123:
ld hl, sp + 28
bit 4, [hl]
jr nz, .L00125
ld a, 0x1d
ld hl, 0
add hl, sp
ld e, l
ld d, h
call _sf128_left
ld hl, sp + 85
ld a, [hli]
ld e, a
ld d, [hl]
ld a, [de]
ld hl, sp + 81
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
ld hl, sp + 93
ld [hld], a
ld [hl], e
ld hl, sp + 85
ld a, [hli]
ld e, a
ld d, [hl]
pop af
ld a, e
sbc a, 0x00
ld e, a
ld a, d
sbc a, 0x00
ld hl, sp + 93
ld [hld], a
ld [hl], e
ld hl, sp + 85
ld a, [hli]
ld e, a
ld d, [hl]
ld hl, sp + 90
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
jr .L00123
.L00125:
ld hl, 0x1d01
push hl
xor a, a
rrca
push af
ld de, 0x006c
push de
ld hl, 6
add hl, sp
ld e, l
ld d, h
call _sf128_shr
ld hl, sp + 87
ld a, [hli]
add a, 0x06
ld e, a
ld a, [hl]
adc a, 0x00
ld h, 0x10
push hl
inc sp
ld hl, 1
add hl, sp
ld c, l
ld b, h
ld d, a
call _sf128_copy
.L00132:
add sp, 94
pop hl
add sp, 4
jp hl
_sf128_fma_context:
add sp, -91
ld hl, sp + 89
ld a, e
ld [hli], a
ld a, d
ld [hld], a
ld a, [hli]
ld c, a
ld b, [hl]
ld hl, 0
add hl, sp
ld e, l
ld d, h
call _sf128_unpack
ld hl, sp + 89
ld a, [hli]
add a, 0x10
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld hl, 22
add hl, sp
ld e, l
ld d, h
call _sf128_unpack
ld hl, sp + 89
ld a, [hli]
add a, 0x20
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld hl, 44
add hl, sp
ld e, l
ld d, h
call _sf128_unpack
ld hl, sp + 1
ld c, [hl]
dec c
ld a, 0x01
jr z, .L00175
xor a, a
.L00175:
ld c, a
bit 0, c
jr z, .L00111
ld hl, sp + 23
ld b, [hl]
dec b
jr nz, .L00111
ld hl, sp + 45
ld b, [hl]
dec b
jr z, .L00112
.L00111:
ld a, c
or a, a
jr z, .L00107
ld hl, sp + 23
ld c, [hl]
dec c
jr nz, .L00107
ld hl, sp + 45
ld a, [hl]
sub a, 0x02
jr c, .L00107
dec hl
ld c, l
ld b, h
ld hl, 66
add hl, sp
ld e, l
ld d, h
call _sf128_copy_float
jr .L00113
.L00107:
ld hl, 22
add hl, sp
push hl
ld hl, 2
add hl, sp
ld c, l
ld b, h
ld hl, 68
add hl, sp
ld e, l
ld d, h
call _sf128_product
ld hl, sp + 45
ld a, [hl]
ld hl, sp + 88
ld [hl], a
or a, a
jr z, .L00104
ld hl, 44
add hl, sp
push hl
ld hl, 68
add hl, sp
ld c, l
ld b, h
ld e, l
ld d, h
call _sf128_sum
jr .L00113
.L00104:
ld hl, sp + 67
ld a, [hl]
or a, a
jr nz, .L00113
dec hl
ld c, l
ld b, h
ld a, [bc]
ld hl, sp + 44
and a, [hl]
ld [bc], a
jr .L00113
.L00112:
ld hl, sp + 66
ld e, l
ld d, h
ld hl, 44
add hl, sp
push hl
ld hl, 24
add hl, sp
push hl
ld hl, 4
add hl, sp
ld c, l
ld b, h
call _sf128_fused_finite
.L00113:
ld hl, sp + 66
ld c, l
ld b, h
ld hl, sp + 89
ld a, [hli]
add a, 0x30
ld e, a
ld a, [hl]
adc a, 0x00
ld d, a
call _sf128_pack
add sp, 91
ret
_sf128_compare_context:
add sp, -6
ld hl, sp + 4
ld a, e
ld [hli], a
ld a, d
ld [hld], a
ld a, [hli]
add a, 0x40
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld a, [bc]
ld hl, sp + 0
ld [hl], a
ld hl, sp + 4
ld a, [hli]
add a, 0x10
ld e, a
ld a, [hld]
adc a, 0x00
ld d, a
push de
ld c, e
ld b, d
ld a, [hli]
ld e, a
ld d, [hl]
call _sf128_compare_raw
ld b, a
pop de
ld hl, sp + 4
ld a, [hld]
dec hl
dec hl
add a, 0x30
ld [hli], a
inc hl
inc hl
inc hl
ld a, [hl]
adc a, 0x00
ld hl, sp + 2
ld [hld], a
dec hl
ld a, [hl]
sub a, 0x0c
jr nz, .L00102
ld hl, sp + 4
ld a, [hli]
add a, 0x41
ld e, a
ld a, [hl]
adc a, 0x00
ld d, a
ld a, [de]
and a, b
sub a, 0x01
ld a, 0x00
rla
xor a, 0x01
ld hl, sp + 1
ld e, [hl]
inc hl
ld h, [hl]
ld l, e
ld [hl], a
jp .L00111
.L00102:
ld a, b
sub a, 0x08
jr nz, .L00104
ld a, 0x03
push af
inc sp
xor a, a
pop hl
pop de
push de
push hl
call _sf128_special
jp .L00111
.L00104:
push bc
push de
ld hl, sp + 8
ld a, [hli]
ld e, a
ld d, [hl]
call _sf128_raw_kind
ld c, a
pop de
pop af
ld b, a
ld hl, sp + 0
ld a, [hl]
sub a, 0x06
ld a, 0x01
jr z, .L00196
xor a, a
.L00196:
ld hl, sp + 3
ld [hl], a
ld a, c
or a, a
jr nz, .L00106
push bc
push de
call _sf128_raw_kind
ld c, a
pop de
pop af
ld b, a
ld a, c
or a, a
jr nz, .L00106
ld hl, sp + 4
ld a, [hli]
add a, 0x0f
ld c, a
ld a, [hld]
adc a, 0x00
ld b, a
ld a, [hli]
add a, 0x1f
ld e, a
ld a, [hld]
dec hl
adc a, 0x00
ld d, a
ld a, [bc]
ld c, a
ld a, [de]
ld e, a
ld a, [hl]
or a, a
jr z, .L00113
ld a, c
or a, e
jr .L00114
.L00113:
ld a, c
and a, e
.L00114:
rlca
and a, 0x01
ld h, 0x00
push hl
inc sp
pop hl
pop de
push de
push hl
call _sf128_special
jr .L00111
.L00106:
ld hl, sp + 3
bit 0, [hl]
jr z, .L00122
ld a, b
sub a, 0x04
jr z, .L00116
.L00122:
ld hl, sp + 0
ld a, [hl]
sub a, 0x07
jr nz, .L00115
ld a, b
sub a, 0x02
jr z, .L00116
.L00115:
ld hl, sp + 4
ld a, [hli]
ld e, a
ld d, [hl]
.L00116:
ld hl, sp + 1
ld a, e
ld [hli], a
ld a, d
ld [hli], a
ld [hl], 0x00
.L00109:
ld hl, sp + 3
ld a, [hli]
add a, 0x30
add a, [hl]
inc hl
ld c, a
ld a, 0x00
adc a, [hl]
ld b, a
ld hl, sp + 1
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
sub a, 0x10
jr c, .L00109
.L00111:
add sp, 6
ret
_sf128_binary:
add sp, -68
ld hl, sp + 66
ld a, e
ld [hli], a
ld a, d
ld [hld], a
ld a, [hli]
ld c, a
ld b, [hl]
ld hl, 0
add hl, sp
ld e, l
ld d, h
call _sf128_unpack
ld hl, sp + 66
ld a, [hli]
add a, 0x10
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld hl, 22
add hl, sp
ld e, l
ld d, h
call _sf128_unpack
ld hl, sp + 66
ld a, [hli]
add a, 0x40
ld c, a
ld a, [hl]
adc a, 0x00
ld b, a
ld a, [bc]
or a, a
jr z, .L00101
cp a, 0x01
jr z, .L00102
cp a, 0x02
jr z, .L00103
jr .L00104
.L00101:
ld hl, sp + 44
ld e, l
ld d, h
ld hl, 22
add hl, sp
push hl
ld hl, 2
add hl, sp
ld c, l
ld b, h
call _sf128_sum
jr .L00105
.L00102:
ld hl, 22
add hl, sp
ld a, [hl]
xor a, 0x01
ld [hl], a
ld hl, sp + 44
ld e, l
ld d, h
ld hl, 22
add hl, sp
push hl
ld hl, 2
add hl, sp
ld c, l
ld b, h
call _sf128_sum
jr .L00105
.L00103:
ld hl, sp + 44
ld e, l
ld d, h
ld hl, 22
add hl, sp
push hl
ld hl, 2
add hl, sp
ld c, l
ld b, h
call _sf128_product
jr .L00105
.L00104:
ld hl, 22
add hl, sp
push hl
ld hl, 2
add hl, sp
ld c, l
ld b, h
ld hl, 46
add hl, sp
ld e, l
ld d, h
call _sf128_quotient
.L00105:
ld hl, sp + 44
ld c, l
ld b, h
ld hl, sp + 66
ld a, [hli]
add a, 0x30
ld e, a
ld a, [hl]
adc a, 0x00
ld d, a
call _sf128_pack
add sp, 68
ret
_sf128_unary:
add sp, -44
ld hl, sp + 0
push hl
push de
ld c, e
ld b, d
ld e, l
ld d, h
call _sf128_unpack
pop de
pop hl
ld a, e
add a, 0x40
ld c, a
ld a, d
adc a, 0x00
ld b, a
ld a, [bc]
ld c, a
ld a, e
add a, 0x30
ld e, a
jr nc, .L00114
inc d
.L00114:
ld a, c
sub a, 0x04
jr nz, .L00102
push de
ld c, l
ld b, h
ld hl, 24
add hl, sp
ld e, l
ld d, h
call _sf128_root
pop de
ld hl, 22
add hl, sp
ld c, l
ld b, h
call _sf128_pack
jr .L00104
.L00102:
ld a, c
add a, 0xf8
push af
inc sp
ld c, l
ld b, h
call _sf128_round
.L00104:
add sp, 44
ret
_sangoku_sf128::
ld hl, 0x0040
add hl, de
ld a, [hl]
cp a, 0x04
jp c, _sf128_binary
cp a, 0x05
jp z, _sf128_fma_context
cp a, 0x06
jp z, _sf128_compare_context
cp a, 0x07
jp z, _sf128_compare_context
cp a, 0x0c
jp nz, _sf128_unary
jp _sf128_compare_context

__gbc_f128::
ld e, b
ld d, c
jp _sangoku_sf128
