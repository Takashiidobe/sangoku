;--------------------------------------------------------
; File Created by SDCC : free open source ISO C Compiler
; Version 4.6.4 #17147 (Linux)
;--------------------------------------------------------
	.module tests_fixtures_shifts_sdcc_size

	.optsdcc -msm83 sdcccall(1)
;--------------------------------------------------------
; Public variables in this module
;--------------------------------------------------------
	.globl _ror32_var
	.globl _rol32_var
	.globl _sar32_var
	.globl _shr32_var
	.globl _shl32_var
	.globl _ror16_var
	.globl _rol16_var
	.globl _sar16_var
	.globl _shr16_var
	.globl _shl16_var
	.globl _ror32_31
	.globl _rol32_31
	.globl _sar32_31
	.globl _shr32_31
	.globl _shl32_31
	.globl _ror32_30
	.globl _rol32_30
	.globl _sar32_30
	.globl _shr32_30
	.globl _shl32_30
	.globl _ror32_29
	.globl _rol32_29
	.globl _sar32_29
	.globl _shr32_29
	.globl _shl32_29
	.globl _ror32_28
	.globl _rol32_28
	.globl _sar32_28
	.globl _shr32_28
	.globl _shl32_28
	.globl _ror32_27
	.globl _rol32_27
	.globl _sar32_27
	.globl _shr32_27
	.globl _shl32_27
	.globl _ror32_26
	.globl _rol32_26
	.globl _sar32_26
	.globl _shr32_26
	.globl _shl32_26
	.globl _ror32_25
	.globl _rol32_25
	.globl _sar32_25
	.globl _shr32_25
	.globl _shl32_25
	.globl _ror32_24
	.globl _rol32_24
	.globl _sar32_24
	.globl _shr32_24
	.globl _shl32_24
	.globl _ror32_23
	.globl _rol32_23
	.globl _sar32_23
	.globl _shr32_23
	.globl _shl32_23
	.globl _ror32_22
	.globl _rol32_22
	.globl _sar32_22
	.globl _shr32_22
	.globl _shl32_22
	.globl _ror32_21
	.globl _rol32_21
	.globl _sar32_21
	.globl _shr32_21
	.globl _shl32_21
	.globl _ror32_20
	.globl _rol32_20
	.globl _sar32_20
	.globl _shr32_20
	.globl _shl32_20
	.globl _ror32_19
	.globl _rol32_19
	.globl _sar32_19
	.globl _shr32_19
	.globl _shl32_19
	.globl _ror32_18
	.globl _rol32_18
	.globl _sar32_18
	.globl _shr32_18
	.globl _shl32_18
	.globl _ror32_17
	.globl _rol32_17
	.globl _sar32_17
	.globl _shr32_17
	.globl _shl32_17
	.globl _ror32_16
	.globl _rol32_16
	.globl _sar32_16
	.globl _shr32_16
	.globl _shl32_16
	.globl _ror32_15
	.globl _rol32_15
	.globl _sar32_15
	.globl _shr32_15
	.globl _shl32_15
	.globl _ror16_15
	.globl _rol16_15
	.globl _sar16_15
	.globl _shr16_15
	.globl _shl16_15
	.globl _ror32_14
	.globl _rol32_14
	.globl _sar32_14
	.globl _shr32_14
	.globl _shl32_14
	.globl _ror16_14
	.globl _rol16_14
	.globl _sar16_14
	.globl _shr16_14
	.globl _shl16_14
	.globl _ror32_13
	.globl _rol32_13
	.globl _sar32_13
	.globl _shr32_13
	.globl _shl32_13
	.globl _ror16_13
	.globl _rol16_13
	.globl _sar16_13
	.globl _shr16_13
	.globl _shl16_13
	.globl _ror32_12
	.globl _rol32_12
	.globl _sar32_12
	.globl _shr32_12
	.globl _shl32_12
	.globl _ror16_12
	.globl _rol16_12
	.globl _sar16_12
	.globl _shr16_12
	.globl _shl16_12
	.globl _ror32_11
	.globl _rol32_11
	.globl _sar32_11
	.globl _shr32_11
	.globl _shl32_11
	.globl _ror16_11
	.globl _rol16_11
	.globl _sar16_11
	.globl _shr16_11
	.globl _shl16_11
	.globl _ror32_10
	.globl _rol32_10
	.globl _sar32_10
	.globl _shr32_10
	.globl _shl32_10
	.globl _ror16_10
	.globl _rol16_10
	.globl _sar16_10
	.globl _shr16_10
	.globl _shl16_10
	.globl _ror32_9
	.globl _rol32_9
	.globl _sar32_9
	.globl _shr32_9
	.globl _shl32_9
	.globl _ror16_9
	.globl _rol16_9
	.globl _sar16_9
	.globl _shr16_9
	.globl _shl16_9
	.globl _ror32_8
	.globl _rol32_8
	.globl _sar32_8
	.globl _shr32_8
	.globl _shl32_8
	.globl _ror16_8
	.globl _rol16_8
	.globl _sar16_8
	.globl _shr16_8
	.globl _shl16_8
	.globl _ror32_7
	.globl _rol32_7
	.globl _sar32_7
	.globl _shr32_7
	.globl _shl32_7
	.globl _ror16_7
	.globl _rol16_7
	.globl _sar16_7
	.globl _shr16_7
	.globl _shl16_7
	.globl _ror32_6
	.globl _rol32_6
	.globl _sar32_6
	.globl _shr32_6
	.globl _shl32_6
	.globl _ror16_6
	.globl _rol16_6
	.globl _sar16_6
	.globl _shr16_6
	.globl _shl16_6
	.globl _ror32_5
	.globl _rol32_5
	.globl _sar32_5
	.globl _shr32_5
	.globl _shl32_5
	.globl _ror16_5
	.globl _rol16_5
	.globl _sar16_5
	.globl _shr16_5
	.globl _shl16_5
	.globl _ror32_4
	.globl _rol32_4
	.globl _sar32_4
	.globl _shr32_4
	.globl _shl32_4
	.globl _ror16_4
	.globl _rol16_4
	.globl _sar16_4
	.globl _shr16_4
	.globl _shl16_4
	.globl _ror32_3
	.globl _rol32_3
	.globl _sar32_3
	.globl _shr32_3
	.globl _shl32_3
	.globl _ror16_3
	.globl _rol16_3
	.globl _sar16_3
	.globl _shr16_3
	.globl _shl16_3
	.globl _ror32_2
	.globl _rol32_2
	.globl _sar32_2
	.globl _shr32_2
	.globl _shl32_2
	.globl _ror16_2
	.globl _rol16_2
	.globl _sar16_2
	.globl _shr16_2
	.globl _shl16_2
	.globl _ror32_1
	.globl _rol32_1
	.globl _sar32_1
	.globl _shr32_1
	.globl _shl32_1
	.globl _ror16_1
	.globl _rol16_1
	.globl _sar16_1
	.globl _shr16_1
	.globl _shl16_1
	.globl _ror32_0
	.globl _rol32_0
	.globl _sar32_0
	.globl _shr32_0
	.globl _shl32_0
	.globl _ror16_0
	.globl _rol16_0
	.globl _sar16_0
	.globl _shr16_0
	.globl _shl16_0
;--------------------------------------------------------
; special function registers
;--------------------------------------------------------
;--------------------------------------------------------
; ram data
;--------------------------------------------------------
	.area _DATA
;--------------------------------------------------------
; ram data
;--------------------------------------------------------
	.area _INITIALIZED
;--------------------------------------------------------
; absolute ram data
;--------------------------------------------------------
	.area _DABS (ABS)
	.area _DABS (ABS)
;--------------------------------------------------------
; global & static initialisations
;--------------------------------------------------------
	.area _HOME
	.area _GSINIT
	.area _GSFINAL
	.area _GSINIT
;--------------------------------------------------------
; Home
;--------------------------------------------------------
	.area _HOME
	.area _HOME
;--------------------------------------------------------
; code
;--------------------------------------------------------
	.area _CODE
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shl16_0
; ---------------------------------
_shl16_0::
	ld	c, e
	ld	b, d
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shr16_0
; ---------------------------------
_shr16_0::
	ld	c, e
	ld	b, d
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function sar16_0
; ---------------------------------
_sar16_0::
	ld	c, e
	ld	b, d
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function rol16_0
; ---------------------------------
_rol16_0::
	ld	a, e
	ld	b, d
	or	a, e
	ld	c, a
	ld	a, b
	or	a, d
	ld	b, a
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function ror16_0
; ---------------------------------
_ror16_0::
	ld	a, e
	ld	b, d
	or	a, e
	ld	c, a
	ld	a, b
	or	a, d
	ld	b, a
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shl32_0
; ---------------------------------
_shl32_0::
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shr32_0
; ---------------------------------
_shr32_0::
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function sar32_0
; ---------------------------------
_sar32_0::
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function rol32_0
; ---------------------------------
_rol32_0::
	add	sp, #-4
	pop	hl
	push	bc
	ldhl	sp,	#2
	ld	a, e
	ld	(hl+), a
	ld	(hl), d
	ld	a, c
	ldhl	sp,	#0
	or	a, (hl)
	inc	hl
	ld	c, a
	ld	a, b
	or	a, (hl)
	inc	hl
	ld	b, a
	ld	a, e
	or	a, (hl)
	inc	hl
	ld	e, a
	ld	a, d
	or	a, (hl)
	ld	d, a
	add	sp, #4
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function ror32_0
; ---------------------------------
_ror32_0::
	add	sp, #-4
	pop	hl
	push	bc
	ldhl	sp,	#2
	ld	a, e
	ld	(hl+), a
	ld	(hl), d
	ld	a, c
	ldhl	sp,	#0
	or	a, (hl)
	inc	hl
	ld	c, a
	ld	a, b
	or	a, (hl)
	inc	hl
	ld	b, a
	ld	a, e
	or	a, (hl)
	inc	hl
	ld	e, a
	ld	a, d
	or	a, (hl)
	ld	d, a
	add	sp, #4
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shl16_1
; ---------------------------------
_shl16_1::
	ld	l, e
	ld	h, d
	add	hl, hl
	ld	c, l
	ld	b, h
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shr16_1
; ---------------------------------
_shr16_1::
	ld	c, e
	ld	b, d
	srl	b
	rr	c
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function sar16_1
; ---------------------------------
_sar16_1::
	ld	c, e
	ld	b, d
	sra	b
	rr	c
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function rol16_1
; ---------------------------------
_rol16_1::
	ld	c, e
	ld	b, d
	ld	a, b
	rla
	rl	c
	rl	b
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function ror16_1
; ---------------------------------
_ror16_1::
	ld	c, e
	ld	b, d
	ld	a, b
	rra
	rr	c
	rr	b
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shl32_1
; ---------------------------------
_shl32_1::
	sla	c
	rl	b
	rl	e
	rl	d
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shr32_1
; ---------------------------------
_shr32_1::
	srl	d
	rr	e
	rr	b
	rr	c
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function sar32_1
; ---------------------------------
_sar32_1::
	sra	d
	rr	e
	rr	b
	rr	c
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function rol32_1
; ---------------------------------
_rol32_1::
	ld	a, d
	rla
	rl	c
	rl	b
	rl	e
	rl	d
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function ror32_1
; ---------------------------------
_ror32_1::
	ld	a, d
	rra
	rr	e
	rr	b
	rr	c
	rr	d
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shl16_2
; ---------------------------------
_shl16_2::
	ld	l, e
	ld	h, d
	add	hl, hl
	add	hl, hl
	ld	c, l
	ld	b, h
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shr16_2
; ---------------------------------
_shr16_2::
	ld	c, e
	ld	b, d
	srl	b
	rr	c
	srl	b
	rr	c
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function sar16_2
; ---------------------------------
_sar16_2::
	ld	c, e
	ld	b, d
	sra	b
	rr	c
	sra	b
	rr	c
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function rol16_2
; ---------------------------------
_rol16_2::
	ld	l, e
	ld	h, d
	add	hl, hl
	add	hl, hl
	ld	a, d
	rlca
	rlca
	and	a, #0x03
	or	a, l
	ld	c, a
	ld	b, h
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function ror16_2
; ---------------------------------
_ror16_2::
	ld	c, e
	ld	b, d
	srl	b
	rr	c
	srl	b
	rr	c
	ld	a, e
	rrca
	rrca
	and	a, #0xc0
	or	a, b
	ld	b, a
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shl32_2
; ---------------------------------
_shl32_2::
	ld	a, #0x02
00103$:
	sla	c
	rl	b
	rl	e
	rl	d
	dec	a
	jr	nz, 00103$
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shr32_2
; ---------------------------------
_shr32_2::
	ld	a, #0x02
00103$:
	srl	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function sar32_2
; ---------------------------------
_sar32_2::
	ld	a, #0x02
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function rol32_2
; ---------------------------------
_rol32_2::
	add	sp, #-4
	pop	hl
	push	bc
	ldhl	sp,	#2
	ld	a, e
	ld	(hl+), a
	ld	(hl), d
	ld	a, #0x02
00103$:
	ldhl	sp,	#0
	sla	(hl)
	inc	hl
	rl	(hl)
	inc	hl
	rl	(hl)
	inc	hl
	rl	(hl)
	dec	a
	jr	nz, 00103$
	ld	c, d
	ld	a, #0x06
00105$:
	srl	c
	dec	a
	jr	nz, 00105$
	ld	a, c
	ldhl	sp,	#0
	or	a, (hl)
	inc	hl
	ld	c, a
	ld	b, (hl)
	pop	hl
	pop	de
	push	de
	push	hl
	add	sp, #4
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function ror32_2
; ---------------------------------
_ror32_2::
	add	sp, #-8
	ldhl	sp,	#4
	ld	a, c
	ld	(hl+), a
	ld	a, b
	ld	(hl+), a
	ld	a, e
	ld	(hl+), a
	ld	(hl), d
	ldhl	sp,	#4
	ld	a, (hl+)
	ld	c, a
	ld	a, (hl+)
	ld	b, a
	ld	a, (hl+)
	ld	e, a
	ld	d, (hl)
	ld	a, #0x02
00103$:
	srl	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	push	af
	ldhl	sp,	#6
	ld	a, (hl-)
	ld	(hl), a
	pop	af
	ldhl	sp,	#0
	xor	a, a
	ld	(hl+), a
	ld	(hl+), a
	ld	(hl), #0x00
	ld	a, #0x06
00105$:
	ldhl	sp,	#3
	sla	(hl)
	dec	a
	jr	nz, 00105$
	ld	a, d
	or	a, (hl)
	ld	d, a
	add	sp, #8
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shl16_3
; ---------------------------------
_shl16_3::
	ld	l, e
	ld	h, d
	add	hl, hl
	add	hl, hl
	add	hl, hl
	ld	c, l
	ld	b, h
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shr16_3
; ---------------------------------
_shr16_3::
	ld	c, e
	ld	b, d
	ld	a, #0x03
00103$:
	srl	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function sar16_3
; ---------------------------------
_sar16_3::
	ld	c, e
	ld	b, d
	ld	a, #0x03
00103$:
	sra	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function rol16_3
; ---------------------------------
_rol16_3::
	ld	l, e
	ld	h, d
	add	hl, hl
	add	hl, hl
	add	hl, hl
	ld	a, d
	swap	a
	rrca
	and	a, #0x07
	or	a, l
	ld	c, a
	ld	b, h
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function ror16_3
; ---------------------------------
_ror16_3::
	ld	c, e
	ld	b, d
	ld	a, #0x03
00103$:
	srl	b
	rr	c
	dec	a
	jr	nz, 00103$
	ld	a, e
	swap	a
	rlca
	and	a, #0xe0
	or	a, b
	ld	b, a
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shl32_3
; ---------------------------------
_shl32_3::
	ld	a, #0x03
00103$:
	sla	c
	rl	b
	rl	e
	rl	d
	dec	a
	jr	nz, 00103$
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shr32_3
; ---------------------------------
_shr32_3::
	ld	a, #0x03
00103$:
	srl	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function sar32_3
; ---------------------------------
_sar32_3::
	ld	a, #0x03
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function rol32_3
; ---------------------------------
_rol32_3::
	add	sp, #-4
	pop	hl
	push	bc
	ldhl	sp,	#2
	ld	a, e
	ld	(hl+), a
	ld	(hl), d
	ld	a, #0x03
00103$:
	ldhl	sp,	#0
	sla	(hl)
	inc	hl
	rl	(hl)
	inc	hl
	rl	(hl)
	inc	hl
	rl	(hl)
	dec	a
	jr	nz, 00103$
	ld	c, d
	ld	a, #0x05
00105$:
	srl	c
	dec	a
	jr	nz, 00105$
	ld	a, c
	ldhl	sp,	#0
	or	a, (hl)
	inc	hl
	ld	c, a
	ld	b, (hl)
	pop	hl
	pop	de
	push	de
	push	hl
	add	sp, #4
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function ror32_3
; ---------------------------------
_ror32_3::
	add	sp, #-8
	ldhl	sp,	#4
	ld	a, c
	ld	(hl+), a
	ld	a, b
	ld	(hl+), a
	ld	a, e
	ld	(hl+), a
	ld	(hl), d
	ldhl	sp,	#4
	ld	a, (hl+)
	ld	c, a
	ld	a, (hl+)
	ld	b, a
	ld	a, (hl+)
	ld	e, a
	ld	d, (hl)
	ld	a, #0x03
00103$:
	srl	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	push	af
	ldhl	sp,	#6
	ld	a, (hl-)
	ld	(hl), a
	pop	af
	ldhl	sp,	#0
	xor	a, a
	ld	(hl+), a
	ld	(hl+), a
	ld	(hl), #0x00
	ld	a, #0x05
00105$:
	ldhl	sp,	#3
	sla	(hl)
	dec	a
	jr	nz, 00105$
	ld	a, d
	or	a, (hl)
	ld	d, a
	add	sp, #8
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shl16_4
; ---------------------------------
_shl16_4::
	ld	l, e
	ld	h, d
	add	hl, hl
	add	hl, hl
	add	hl, hl
	add	hl, hl
	ld	c, l
	ld	b, h
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shr16_4
; ---------------------------------
_shr16_4::
	ld	c, e
	ld	b, d
	ld	a, #0x04
00103$:
	srl	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function sar16_4
; ---------------------------------
_sar16_4::
	ld	c, e
	ld	b, d
	ld	a, #0x04
00103$:
	sra	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function rol16_4
; ---------------------------------
_rol16_4::
	ld	l, e
	ld	h, d
	add	hl, hl
	add	hl, hl
	add	hl, hl
	add	hl, hl
	ld	a, d
	swap	a
	and	a, #0x0f
	or	a, l
	ld	c, a
	ld	b, h
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function ror16_4
; ---------------------------------
_ror16_4::
	ld	c, e
	ld	b, d
	ld	a, #0x04
00103$:
	srl	b
	rr	c
	dec	a
	jr	nz, 00103$
	ld	a, e
	swap	a
	and	a, #0xf0
	or	a, b
	ld	b, a
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shl32_4
; ---------------------------------
_shl32_4::
	ld	a, #0x04
00103$:
	sla	c
	rl	b
	rl	e
	rl	d
	dec	a
	jr	nz, 00103$
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shr32_4
; ---------------------------------
_shr32_4::
	ld	a, #0x04
00103$:
	srl	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function sar32_4
; ---------------------------------
_sar32_4::
	ld	a, #0x04
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function rol32_4
; ---------------------------------
_rol32_4::
	add	sp, #-4
	pop	hl
	push	bc
	ldhl	sp,	#2
	ld	a, e
	ld	(hl+), a
	ld	(hl), d
	ld	a, #0x04
00103$:
	ldhl	sp,	#0
	sla	(hl)
	inc	hl
	rl	(hl)
	inc	hl
	rl	(hl)
	inc	hl
	rl	(hl)
	dec	a
	jr	nz, 00103$
	ld	c, d
	ld	a, #0x04
00105$:
	srl	c
	dec	a
	jr	nz, 00105$
	ld	a, c
	ldhl	sp,	#0
	or	a, (hl)
	inc	hl
	ld	c, a
	ld	b, (hl)
	pop	hl
	pop	de
	push	de
	push	hl
	add	sp, #4
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function ror32_4
; ---------------------------------
_ror32_4::
	add	sp, #-8
	ldhl	sp,	#4
	ld	a, c
	ld	(hl+), a
	ld	a, b
	ld	(hl+), a
	ld	a, e
	ld	(hl+), a
	ld	(hl), d
	ldhl	sp,	#4
	ld	a, (hl+)
	ld	c, a
	ld	a, (hl+)
	ld	b, a
	ld	a, (hl+)
	ld	e, a
	ld	d, (hl)
	ld	a, #0x04
00103$:
	srl	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	push	af
	ldhl	sp,	#6
	ld	a, (hl-)
	ld	(hl), a
	pop	af
	ldhl	sp,	#0
	xor	a, a
	ld	(hl+), a
	ld	(hl+), a
	ld	(hl), #0x00
	ld	a, #0x04
00105$:
	ldhl	sp,	#3
	sla	(hl)
	dec	a
	jr	nz, 00105$
	ld	a, d
	or	a, (hl)
	ld	d, a
	add	sp, #8
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shl16_5
; ---------------------------------
_shl16_5::
	ld	l, e
	ld	h, d
	add	hl, hl
	add	hl, hl
	add	hl, hl
	add	hl, hl
	add	hl, hl
	ld	c, l
	ld	b, h
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shr16_5
; ---------------------------------
_shr16_5::
	ld	l, e
	ld	h, d
	xor	a, a
	add	hl, hl
	rla
	add	hl, hl
	rla
	add	hl, hl
	rla
	ld	b, a
	ld	c, h
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function sar16_5
; ---------------------------------
_sar16_5::
	ld	c, e
	ld	b, d
	ld	a, #0x05
00103$:
	sra	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function rol16_5
; ---------------------------------
_rol16_5::
	ld	l, e
	ld	h, d
	add	hl, hl
	add	hl, hl
	add	hl, hl
	add	hl, hl
	add	hl, hl
	ld	a, d
	swap	a
	rlca
	and	a, #0x1f
	or	a, l
	ld	c, a
	ld	b, h
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function ror16_5
; ---------------------------------
_ror16_5::
	ld	c, e
	ld	b, d
	ld	a, #0x05
00103$:
	srl	b
	rr	c
	dec	a
	jr	nz, 00103$
	ld	a, e
	add	a, a
	add	a, a
	add	a, a
	or	a, b
	ld	b, a
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shl32_5
; ---------------------------------
_shl32_5::
	ld	a, #0x05
00103$:
	sla	c
	rl	b
	rl	e
	rl	d
	dec	a
	jr	nz, 00103$
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shr32_5
; ---------------------------------
_shr32_5::
	ld	a, #0x05
00103$:
	srl	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function sar32_5
; ---------------------------------
_sar32_5::
	ld	a, #0x05
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function rol32_5
; ---------------------------------
_rol32_5::
	add	sp, #-4
	pop	hl
	push	bc
	ldhl	sp,	#2
	ld	a, e
	ld	(hl+), a
	ld	(hl), d
	ld	a, #0x05
00103$:
	ldhl	sp,	#0
	sla	(hl)
	inc	hl
	rl	(hl)
	inc	hl
	rl	(hl)
	inc	hl
	rl	(hl)
	dec	a
	jr	nz, 00103$
	ld	c, d
	ld	a, #0x03
00105$:
	srl	c
	dec	a
	jr	nz, 00105$
	ld	a, c
	ldhl	sp,	#0
	or	a, (hl)
	inc	hl
	ld	c, a
	ld	b, (hl)
	pop	hl
	pop	de
	push	de
	push	hl
	add	sp, #4
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function ror32_5
; ---------------------------------
_ror32_5::
	add	sp, #-8
	ldhl	sp,	#4
	ld	a, c
	ld	(hl+), a
	ld	a, b
	ld	(hl+), a
	ld	a, e
	ld	(hl+), a
	ld	(hl), d
	ldhl	sp,	#4
	ld	a, (hl+)
	ld	c, a
	ld	a, (hl+)
	ld	b, a
	ld	a, (hl+)
	ld	e, a
	ld	d, (hl)
	ld	a, #0x05
00103$:
	srl	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	push	af
	ldhl	sp,	#6
	ld	a, (hl-)
	ld	(hl), a
	pop	af
	ldhl	sp,	#0
	xor	a, a
	ld	(hl+), a
	ld	(hl+), a
	ld	(hl), #0x00
	ld	a, #0x03
00105$:
	ldhl	sp,	#3
	sla	(hl)
	dec	a
	jr	nz, 00105$
	ld	a, d
	or	a, (hl)
	ld	d, a
	add	sp, #8
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shl16_6
; ---------------------------------
_shl16_6::
	ld	l, e
	ld	h, d
	add	hl, hl
	add	hl, hl
	add	hl, hl
	add	hl, hl
	add	hl, hl
	add	hl, hl
	ld	c, l
	ld	b, h
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shr16_6
; ---------------------------------
_shr16_6::
	ld	l, e
	ld	h, d
	xor	a, a
	add	hl, hl
	rla
	add	hl, hl
	rla
	ld	b, a
	ld	c, h
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function sar16_6
; ---------------------------------
_sar16_6::
	ld	l, e
	ld	h, d
	xor	a, a
	add	hl, hl
	jr	nc, 00103$
	dec	a
00103$:
	add	hl, hl
	rla
	ld	b, a
	ld	c, h
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function rol16_6
; ---------------------------------
_rol16_6::
	ld	a, d
	ld	l, e
	ld	h, a
	add	hl, hl
	add	hl, hl
	add	hl, hl
	add	hl, hl
	add	hl, hl
	add	hl, hl
	rrca
	rrca
	and	a, #0x3f
	or	a, l
	ld	c, a
	ld	b, h
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function ror16_6
; ---------------------------------
_ror16_6::
	ld	c, e
	ld	b, d
	ld	a, #0x06
00103$:
	srl	b
	rr	c
	dec	a
	jr	nz, 00103$
	ld	a, e
	add	a, a
	add	a, a
	or	a, b
	ld	b, a
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shl32_6
; ---------------------------------
_shl32_6::
	ld	a, #0x06
00103$:
	sla	c
	rl	b
	rl	e
	rl	d
	dec	a
	jr	nz, 00103$
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shr32_6
; ---------------------------------
_shr32_6::
	ld	a, #0x06
00103$:
	srl	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function sar32_6
; ---------------------------------
_sar32_6::
	ld	a, #0x06
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function rol32_6
; ---------------------------------
_rol32_6::
	add	sp, #-4
	pop	hl
	push	bc
	ldhl	sp,	#2
	ld	a, e
	ld	(hl+), a
	ld	(hl), d
	ld	a, #0x06
00103$:
	ldhl	sp,	#0
	sla	(hl)
	inc	hl
	rl	(hl)
	inc	hl
	rl	(hl)
	inc	hl
	rl	(hl)
	dec	a
	jr	nz, 00103$
	ld	c, d
	ld	a, #0x02
00105$:
	srl	c
	dec	a
	jr	nz, 00105$
	ld	a, c
	ldhl	sp,	#0
	or	a, (hl)
	inc	hl
	ld	c, a
	ld	b, (hl)
	pop	hl
	pop	de
	push	de
	push	hl
	add	sp, #4
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function ror32_6
; ---------------------------------
_ror32_6::
	add	sp, #-8
	ldhl	sp,	#4
	ld	a, c
	ld	(hl+), a
	ld	a, b
	ld	(hl+), a
	ld	a, e
	ld	(hl+), a
	ld	(hl), d
	ldhl	sp,	#4
	ld	a, (hl+)
	ld	c, a
	ld	a, (hl+)
	ld	b, a
	ld	a, (hl+)
	ld	e, a
	ld	d, (hl)
	ld	a, #0x06
00103$:
	srl	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	push	af
	ldhl	sp,	#6
	ld	a, (hl-)
	ld	(hl), a
	pop	af
	ldhl	sp,	#0
	xor	a, a
	ld	(hl+), a
	ld	(hl+), a
	ld	(hl), #0x00
	ld	a, #0x02
00105$:
	ldhl	sp,	#3
	sla	(hl)
	dec	a
	jr	nz, 00105$
	ld	a, d
	or	a, (hl)
	ld	d, a
	add	sp, #8
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shl16_7
; ---------------------------------
_shl16_7::
	ld	b, e
	xor	a, a
	rr	d
	rr	b
	rra
	ld	c, a
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shr16_7
; ---------------------------------
_shr16_7::
	ld	l, e
	ld	h, d
	xor	a, a
	add	hl, hl
	rla
	ld	b, a
	ld	c, h
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function sar16_7
; ---------------------------------
_sar16_7::
	ld	l, e
	ld	h, d
	add	hl, hl
	ld	l, h
	ld	h, #0x00
	jr	nc, 00103$
	dec	h
00103$:
	ld	c, l
	ld	b, h
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function rol16_7
; ---------------------------------
_rol16_7::
	ld	a, d
	ld	l, e
	ld	h, a
	add	hl, hl
	add	hl, hl
	add	hl, hl
	add	hl, hl
	add	hl, hl
	add	hl, hl
	add	hl, hl
	srl	a
	or	a, l
	ld	c, a
	ld	b, h
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function ror16_7
; ---------------------------------
_ror16_7::
	ld	c, e
	ld	a, d
	ld	b, #0x07
00103$:
	srl	a
	rr	c
	dec	b
	jr	nz, 00103$
	ld	d, e
	sla	d
	or	a, d
	ld	b, a
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shl32_7
; ---------------------------------
_shl32_7::
	ld	a, #0x07
00103$:
	sla	c
	rl	b
	rl	e
	rl	d
	dec	a
	jr	nz, 00103$
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function shr32_7
; ---------------------------------
_shr32_7::
	ld	a, #0x07
00103$:
	srl	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function sar32_7
; ---------------------------------
_sar32_7::
	ld	a, #0x07
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function rol32_7
; ---------------------------------
_rol32_7::
	add	sp, #-4
	pop	hl
	push	bc
	ldhl	sp,	#2
	ld	a, e
	ld	(hl+), a
	ld	(hl), d
	ld	a, #0x07
00103$:
	ldhl	sp,	#0
	sla	(hl)
	inc	hl
	rl	(hl)
	inc	hl
	rl	(hl)
	inc	hl
	rl	(hl)
	dec	a
	jr	nz, 00103$
	ld	c, d
	srl	c
	ld	a, c
	ldhl	sp,	#0
	or	a, (hl)
	inc	hl
	ld	c, a
	ld	b, (hl)
	pop	hl
	pop	de
	push	de
	push	hl
	add	sp, #4
	ret
;(null):3: ERROR: Bad address
;	---------------------------------
; Function ror32_7
; ---------------------------------
_ror32_7::
	add	sp, #-8
	ldhl	sp,	#4
	ld	a, c
	ld	(hl+), a
	ld	a, b
	ld	(hl+), a
	ld	a, e
	ld	(hl+), a
	ld	(hl), d
	ldhl	sp,	#4
	ld	a, (hl+)
	ld	c, a
	ld	a, (hl+)
	ld	b, a
	ld	a, (hl+)
	ld	e, a
	ld	d, (hl)
	ld	a, #0x07
00103$:
	srl	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	push	af
	ldhl	sp,	#6
	ld	a, (hl-)
	ld	(hl), a
	pop	af
	ldhl	sp,	#0
	xor	a, a
	ld	(hl+), a
	ld	(hl+), a
	xor	a, a
	ld	(hl+), a
	sla	(hl)
	ld	a, d
	or	a, (hl)
	ld	d, a
	add	sp, #8
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shl16_8
; ---------------------------------
_shl16_8::
	ld	b, e
	ld	c, #0x00
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shr16_8
; ---------------------------------
_shr16_8::
	ld	c, d
	ld	b, #0x00
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function sar16_8
; ---------------------------------
_sar16_8::
	ld	c, d
	ld	a, c
	rlca
	sbc	a, a
	ld	b, a
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function rol16_8
; ---------------------------------
_rol16_8::
	ld	b, e
	ld	c, d
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function ror16_8
; ---------------------------------
_ror16_8::
	ld	b, e
	ld	c, d
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shl32_8
; ---------------------------------
_shl32_8::
	ld	d, e
	ld	e, b
	ld	b, c
	ld	c, #0x00
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shr32_8
; ---------------------------------
_shr32_8::
	ld	c, b
	ld	b, e
	ld	e, d
	ld	d, #0x00
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function sar32_8
; ---------------------------------
_sar32_8::
	ld	a, #0x08
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function rol32_8
; ---------------------------------
_rol32_8::
	add	sp, #-4
	ldhl	sp,	#1
	ld	a, c
	ld	(hl+), a
	ld	a, b
	ld	(hl+), a
	ld	(hl), e
	ldhl	sp,	#0
	xor	a, a
	ld	(hl+), a
	ld	c, d
	ld	b, (hl)
	pop	hl
	pop	de
	push	de
	push	hl
	add	sp, #4
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function ror32_8
; ---------------------------------
_ror32_8::
	add	sp, #-4
	ldhl	sp,	#0
	ld	a, b
	ld	(hl+), a
	ld	a, e
	ld	(hl+), a
	ld	a, d
	ld	(hl+), a
	xor	a, a
	ld	(hl-), a
	ld	d, c
	pop	bc
	push	bc
	ld	e, (hl)
	add	sp, #4
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shl16_9
; ---------------------------------
_shl16_9::
	ld	a, e
	add	a, a
	ld	b, a
	ld	c, #0x00
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shr16_9
; ---------------------------------
_shr16_9::
	ld	a, d
	srl	a
	ld	c, a
	ld	b, #0x00
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function sar16_9
; ---------------------------------
_sar16_9::
	ld	a, d
	sra	a
	ld	c, a
	rlca
	sbc	a, a
	ld	b, a
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function rol16_9
; ---------------------------------
_rol16_9::
	ld	a, e
	add	a, a
	ld	b, #0x07
00103$:
	srl	d
	rr	e
	dec	b
	jr	nz, 00103$
	ld	c, e
	or	a, d
	ld	b, a
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function ror16_9
; ---------------------------------
_ror16_9::
	ld	a, d
	srl	a
	ld	c, a
	xor	a, a
	rr	d
	rr	e
	rra
	or	a, c
	ld	c, a
	ld	b, e
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shl32_9
; ---------------------------------
_shl32_9::
	ld	d, e
	ld	e, b
	ld	b, c
	ld	c, #0x00
	sla	b
	rl	e
	rl	d
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shr32_9
; ---------------------------------
_shr32_9::
	ld	c, b
	ld	b, e
	ld	e, d
	ld	d, #0x00
	srl	e
	rr	b
	rr	c
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function sar32_9
; ---------------------------------
_sar32_9::
	ld	a, #0x09
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function rol32_9
; ---------------------------------
_rol32_9::
	add	sp, #-4
	ldhl	sp,	#1
	ld	a, c
	ld	(hl+), a
	ld	a, b
	ld	(hl+), a
	ld	(hl), e
	ldhl	sp,	#0
	xor	a, a
	ld	(hl+), a
	sla	(hl)
	inc	hl
	rl	(hl)
	inc	hl
	rl	(hl)
	ld	c, e
	ld	b, d
	ld	a, #0x07
00105$:
	srl	b
	rr	c
	dec	a
	jr	nz, 00105$
	ld	a, b
	ldhl	sp,	#1
	or	a, (hl)
	ld	b, a
	pop	hl
	pop	de
	push	de
	push	hl
	add	sp, #4
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function ror32_9
; ---------------------------------
_ror32_9::
	add	sp, #-4
	ldhl	sp,	#0
	ld	a, b
	ld	(hl+), a
	ld	a, e
	ld	(hl+), a
	ld	a, d
	ld	(hl+), a
	xor	a, a
	ld	(hl-), a
	srl	(hl)
	dec	hl
	rr	(hl)
	dec	hl
	rr	(hl)
	ld	e, c
	ld	d, b
	ld	a, #0x07
00105$:
	sla	e
	rl	d
	dec	a
	jr	nz, 00105$
	inc	hl
	inc	hl
	pop	bc
	push	bc
	ld	a, e
	or	a, (hl)
	ld	e, a
	add	sp, #4
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shl16_10
; ---------------------------------
_shl16_10::
	ld	a, e
	add	a, a
	add	a, a
	ld	b, a
	ld	c, #0x00
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shr16_10
; ---------------------------------
_shr16_10::
	ld	a, d
	rrca
	rrca
	and	a, #0x3f
	ld	c, a
	ld	b, #0x00
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function sar16_10
; ---------------------------------
_sar16_10::
	ld	a, d
	sra	a
	sra	a
	ld	c, a
	rlca
	sbc	a, a
	ld	b, a
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function rol16_10
; ---------------------------------
_rol16_10::
	ld	a, e
	add	a, a
	add	a, a
	ld	b, #0x06
00103$:
	srl	d
	rr	e
	dec	b
	jr	nz, 00103$
	ld	c, e
	or	a, d
	ld	b, a
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function ror16_10
; ---------------------------------
_ror16_10::
	ld	a, d
	rrca
	rrca
	and	a, #0x3f
	ld	l, e
	ld	h, d
	add	hl, hl
	add	hl, hl
	add	hl, hl
	add	hl, hl
	add	hl, hl
	add	hl, hl
	or	a, l
	ld	c, a
	ld	b, h
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shl32_10
; ---------------------------------
_shl32_10::
	ld	d, e
	ld	e, b
	ld	b, c
	ld	c, #0x00
	ld	a, #0x02
00103$:
	sla	b
	rl	e
	rl	d
	dec	a
	jr	nz, 00103$
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shr32_10
; ---------------------------------
_shr32_10::
	ld	c, b
	ld	b, e
	ld	e, d
	ld	d, #0x00
	ld	a, #0x02
00103$:
	srl	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function sar32_10
; ---------------------------------
_sar32_10::
	ld	a, #0x0a
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function rol32_10
; ---------------------------------
_rol32_10::
	add	sp, #-4
	ldhl	sp,	#1
	ld	a, c
	ld	(hl+), a
	ld	a, b
	ld	(hl+), a
	ld	(hl), e
	ldhl	sp,	#0
	ld	(hl), #0x00
	ld	a, #0x02
00103$:
	ldhl	sp,	#1
	sla	(hl)
	inc	hl
	rl	(hl)
	inc	hl
	rl	(hl)
	dec	a
	jr	nz, 00103$
	ld	c, e
	ld	b, d
	ld	a, #0x06
00105$:
	srl	b
	rr	c
	dec	a
	jr	nz, 00105$
	ld	a, b
	ldhl	sp,	#1
	or	a, (hl)
	ld	b, a
	pop	hl
	pop	de
	push	de
	push	hl
	add	sp, #4
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function ror32_10
; ---------------------------------
_ror32_10::
	add	sp, #-4
	ldhl	sp,	#0
	ld	a, b
	ld	(hl+), a
	ld	a, e
	ld	(hl+), a
	ld	a, d
	ld	(hl+), a
	ld	(hl), #0x00
	ld	a, #0x02
00103$:
	ldhl	sp,	#2
	srl	(hl)
	dec	hl
	rr	(hl)
	dec	hl
	rr	(hl)
	dec	a
	jr	nz, 00103$
	ld	e, c
	ld	d, b
	ld	a, #0x06
00105$:
	sla	e
	rl	d
	dec	a
	jr	nz, 00105$
	inc	hl
	inc	hl
	pop	bc
	push	bc
	ld	a, e
	or	a, (hl)
	ld	e, a
	add	sp, #4
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shl16_11
; ---------------------------------
_shl16_11::
	ld	a, e
	add	a, a
	add	a, a
	add	a, a
	ld	b, a
	ld	c, #0x00
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shr16_11
; ---------------------------------
_shr16_11::
	ld	a, d
	swap	a
	rlca
	and	a, #0x1f
	ld	c, a
	ld	b, #0x00
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function sar16_11
; ---------------------------------
_sar16_11::
	ld	a, d
	sra	a
	sra	a
	sra	a
	ld	c, a
	rlca
	sbc	a, a
	ld	b, a
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function rol16_11
; ---------------------------------
_rol16_11::
	ld	a, e
	add	a, a
	add	a, a
	add	a, a
	ld	b, #0x05
00103$:
	srl	d
	rr	e
	dec	b
	jr	nz, 00103$
	ld	c, e
	or	a, d
	ld	b, a
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function ror16_11
; ---------------------------------
_ror16_11::
	ld	a, d
	swap	a
	rlca
	and	a, #0x1f
	ld	l, e
	ld	h, d
	add	hl, hl
	add	hl, hl
	add	hl, hl
	add	hl, hl
	add	hl, hl
	or	a, l
	ld	c, a
	ld	b, h
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shl32_11
; ---------------------------------
_shl32_11::
	ld	d, e
	ld	e, b
	ld	b, c
	ld	c, #0x00
	ld	a, #0x03
00103$:
	sla	b
	rl	e
	rl	d
	dec	a
	jr	nz, 00103$
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shr32_11
; ---------------------------------
_shr32_11::
	ld	c, b
	ld	b, e
	ld	e, d
	ld	d, #0x00
	ld	a, #0x03
00103$:
	srl	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function sar32_11
; ---------------------------------
_sar32_11::
	ld	a, #0x0b
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function rol32_11
; ---------------------------------
_rol32_11::
	add	sp, #-4
	ldhl	sp,	#1
	ld	a, c
	ld	(hl+), a
	ld	a, b
	ld	(hl+), a
	ld	(hl), e
	ldhl	sp,	#0
	ld	(hl), #0x00
	ld	a, #0x03
00103$:
	ldhl	sp,	#1
	sla	(hl)
	inc	hl
	rl	(hl)
	inc	hl
	rl	(hl)
	dec	a
	jr	nz, 00103$
	ld	c, e
	ld	b, d
	ld	a, #0x05
00105$:
	srl	b
	rr	c
	dec	a
	jr	nz, 00105$
	ld	a, b
	ldhl	sp,	#1
	or	a, (hl)
	ld	b, a
	pop	hl
	pop	de
	push	de
	push	hl
	add	sp, #4
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function ror32_11
; ---------------------------------
_ror32_11::
	add	sp, #-4
	ldhl	sp,	#0
	ld	a, b
	ld	(hl+), a
	ld	a, e
	ld	(hl+), a
	ld	a, d
	ld	(hl+), a
	ld	(hl), #0x00
	ld	a, #0x03
00103$:
	ldhl	sp,	#2
	srl	(hl)
	dec	hl
	rr	(hl)
	dec	hl
	rr	(hl)
	dec	a
	jr	nz, 00103$
	ld	e, c
	ld	d, b
	ld	a, #0x05
00105$:
	sla	e
	rl	d
	dec	a
	jr	nz, 00105$
	inc	hl
	inc	hl
	pop	bc
	push	bc
	ld	a, e
	or	a, (hl)
	ld	e, a
	add	sp, #4
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shl16_12
; ---------------------------------
_shl16_12::
	ld	a, e
	swap	a
	and	a, #0xf0
	ld	b, a
	ld	c, #0x00
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shr16_12
; ---------------------------------
_shr16_12::
	ld	a, d
	swap	a
	and	a, #0x0f
	ld	c, a
	ld	b, #0x00
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function sar16_12
; ---------------------------------
_sar16_12::
	ld	a, d
	sra	a
	sra	a
	sra	a
	sra	a
	ld	c, a
	rlca
	sbc	a, a
	ld	b, a
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function rol16_12
; ---------------------------------
_rol16_12::
	ld	a, e
	swap	a
	and	a, #0xf0
	ld	b, #0x04
00103$:
	srl	d
	rr	e
	dec	b
	jr	nz, 00103$
	ld	c, e
	or	a, d
	ld	b, a
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function ror16_12
; ---------------------------------
_ror16_12::
	ld	a, d
	swap	a
	and	a, #0x0f
	ld	l, e
	ld	h, d
	add	hl, hl
	add	hl, hl
	add	hl, hl
	add	hl, hl
	or	a, l
	ld	c, a
	ld	b, h
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shl32_12
; ---------------------------------
_shl32_12::
	ld	d, e
	ld	e, b
	ld	b, c
	ld	c, #0x00
	ld	a, #0x04
00103$:
	sla	b
	rl	e
	rl	d
	dec	a
	jr	nz, 00103$
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shr32_12
; ---------------------------------
_shr32_12::
	ld	c, b
	ld	b, e
	ld	e, d
	ld	d, #0x00
	ld	a, #0x04
00103$:
	srl	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function sar32_12
; ---------------------------------
_sar32_12::
	ld	a, #0x0c
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function rol32_12
; ---------------------------------
_rol32_12::
	add	sp, #-4
	ldhl	sp,	#1
	ld	a, c
	ld	(hl+), a
	ld	a, b
	ld	(hl+), a
	ld	(hl), e
	ldhl	sp,	#0
	ld	(hl), #0x00
	ld	a, #0x04
00103$:
	ldhl	sp,	#1
	sla	(hl)
	inc	hl
	rl	(hl)
	inc	hl
	rl	(hl)
	dec	a
	jr	nz, 00103$
	ld	c, e
	ld	b, d
	ld	a, #0x04
00105$:
	srl	b
	rr	c
	dec	a
	jr	nz, 00105$
	ld	a, b
	ldhl	sp,	#1
	or	a, (hl)
	ld	b, a
	pop	hl
	pop	de
	push	de
	push	hl
	add	sp, #4
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function ror32_12
; ---------------------------------
_ror32_12::
	add	sp, #-4
	ldhl	sp,	#0
	ld	a, b
	ld	(hl+), a
	ld	a, e
	ld	(hl+), a
	ld	a, d
	ld	(hl+), a
	ld	(hl), #0x00
	ld	a, #0x04
00103$:
	ldhl	sp,	#2
	srl	(hl)
	dec	hl
	rr	(hl)
	dec	hl
	rr	(hl)
	dec	a
	jr	nz, 00103$
	ld	e, c
	ld	d, b
	ld	a, #0x04
00105$:
	sla	e
	rl	d
	dec	a
	jr	nz, 00105$
	inc	hl
	inc	hl
	pop	bc
	push	bc
	ld	a, e
	or	a, (hl)
	ld	e, a
	add	sp, #4
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shl16_13
; ---------------------------------
_shl16_13::
	ld	a, e
	swap	a
	rlca
	and	a, #0xe0
	ld	b, a
	ld	c, #0x00
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shr16_13
; ---------------------------------
_shr16_13::
	ld	a, d
	swap	a
	rrca
	and	a, #0x07
	ld	c, a
	ld	b, #0x00
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function sar16_13
; ---------------------------------
_sar16_13::
	ld	a, d
	sra	a
	sra	a
	sra	a
	sra	a
	sra	a
	ld	c, a
	rlca
	sbc	a, a
	ld	b, a
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function rol16_13
; ---------------------------------
_rol16_13::
	ld	a, e
	swap	a
	rlca
	and	a, #0xe0
	ld	b, #0x03
00103$:
	srl	d
	rr	e
	dec	b
	jr	nz, 00103$
	ld	c, e
	or	a, d
	ld	b, a
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function ror16_13
; ---------------------------------
_ror16_13::
	ld	a, d
	swap	a
	rrca
	and	a, #0x07
	ld	l, e
	ld	h, d
	add	hl, hl
	add	hl, hl
	add	hl, hl
	or	a, l
	ld	c, a
	ld	b, h
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shl32_13
; ---------------------------------
_shl32_13::
	ld	d, e
	ld	e, b
	ld	b, c
	ld	c, #0x00
	ld	a, #0x05
00103$:
	sla	b
	rl	e
	rl	d
	dec	a
	jr	nz, 00103$
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shr32_13
; ---------------------------------
_shr32_13::
	ld	c, b
	ld	b, e
	ld	e, d
	ld	d, #0x00
	ld	a, #0x05
00103$:
	srl	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function sar32_13
; ---------------------------------
_sar32_13::
	ld	a, #0x0d
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function rol32_13
; ---------------------------------
_rol32_13::
	add	sp, #-4
	ldhl	sp,	#1
	ld	a, c
	ld	(hl+), a
	ld	a, b
	ld	(hl+), a
	ld	(hl), e
	ldhl	sp,	#0
	ld	(hl), #0x00
	ld	a, #0x05
00103$:
	ldhl	sp,	#1
	sla	(hl)
	inc	hl
	rl	(hl)
	inc	hl
	rl	(hl)
	dec	a
	jr	nz, 00103$
	ld	c, e
	ld	b, d
	ld	a, #0x03
00105$:
	srl	b
	rr	c
	dec	a
	jr	nz, 00105$
	ld	a, b
	ldhl	sp,	#1
	or	a, (hl)
	ld	b, a
	pop	hl
	pop	de
	push	de
	push	hl
	add	sp, #4
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function ror32_13
; ---------------------------------
_ror32_13::
	add	sp, #-4
	ldhl	sp,	#0
	ld	a, b
	ld	(hl+), a
	ld	a, e
	ld	(hl+), a
	ld	a, d
	ld	(hl+), a
	ld	(hl), #0x00
	ld	a, #0x05
00103$:
	ldhl	sp,	#2
	srl	(hl)
	dec	hl
	rr	(hl)
	dec	hl
	rr	(hl)
	dec	a
	jr	nz, 00103$
	ld	e, c
	ld	d, b
	ld	a, #0x03
00105$:
	sla	e
	rl	d
	dec	a
	jr	nz, 00105$
	inc	hl
	inc	hl
	pop	bc
	push	bc
	ld	a, e
	or	a, (hl)
	ld	e, a
	add	sp, #4
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shl16_14
; ---------------------------------
_shl16_14::
	ld	a, e
	rrca
	rrca
	and	a, #0xc0
	ld	b, a
	ld	c, #0x00
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shr16_14
; ---------------------------------
_shr16_14::
	ld	a, d
	rlca
	rlca
	and	a, #0x03
	ld	c, a
	ld	b, #0x00
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function sar16_14
; ---------------------------------
_sar16_14::
	ld	a, d
	sra	a
	sra	a
	sra	a
	sra	a
	sra	a
	sra	a
	ld	c, a
	rlca
	sbc	a, a
	ld	b, a
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function rol16_14
; ---------------------------------
_rol16_14::
	ld	a, e
	rrca
	rrca
	and	a, #0xc0
	srl	d
	rr	e
	srl	d
	rr	e
	ld	c, e
	or	a, d
	ld	b, a
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function ror16_14
; ---------------------------------
_ror16_14::
	ld	a, d
	rlca
	rlca
	and	a, #0x03
	ld	l, e
	ld	h, d
	add	hl, hl
	add	hl, hl
	or	a, l
	ld	c, a
	ld	b, h
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shl32_14
; ---------------------------------
_shl32_14::
	ld	d, e
	ld	e, b
	ld	b, c
	ld	c, #0x00
	ld	a, #0x06
00103$:
	sla	b
	rl	e
	rl	d
	dec	a
	jr	nz, 00103$
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shr32_14
; ---------------------------------
_shr32_14::
	ld	c, b
	ld	b, e
	ld	e, d
	ld	d, #0x00
	ld	a, #0x06
00103$:
	srl	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function sar32_14
; ---------------------------------
_sar32_14::
	ld	a, #0x0e
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function rol32_14
; ---------------------------------
_rol32_14::
	add	sp, #-4
	ldhl	sp,	#1
	ld	a, c
	ld	(hl+), a
	ld	a, b
	ld	(hl+), a
	ld	(hl), e
	ldhl	sp,	#0
	ld	(hl), #0x00
	ld	a, #0x06
00103$:
	ldhl	sp,	#1
	sla	(hl)
	inc	hl
	rl	(hl)
	inc	hl
	rl	(hl)
	dec	a
	jr	nz, 00103$
	ld	c, e
	ld	b, d
	ld	a, #0x02
00105$:
	srl	b
	rr	c
	dec	a
	jr	nz, 00105$
	ld	a, b
	ldhl	sp,	#1
	or	a, (hl)
	ld	b, a
	pop	hl
	pop	de
	push	de
	push	hl
	add	sp, #4
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function ror32_14
; ---------------------------------
_ror32_14::
	add	sp, #-4
	ldhl	sp,	#0
	ld	a, b
	ld	(hl+), a
	ld	a, e
	ld	(hl+), a
	ld	a, d
	ld	(hl+), a
	ld	(hl), #0x00
	ld	a, #0x06
00103$:
	ldhl	sp,	#2
	srl	(hl)
	dec	hl
	rr	(hl)
	dec	hl
	rr	(hl)
	dec	a
	jr	nz, 00103$
	ld	e, c
	ld	d, b
	ld	a, #0x02
00105$:
	sla	e
	rl	d
	dec	a
	jr	nz, 00105$
	inc	hl
	inc	hl
	pop	bc
	push	bc
	ld	a, e
	or	a, (hl)
	ld	e, a
	add	sp, #4
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shl16_15
; ---------------------------------
_shl16_15::
	ld	a, e
	rrca
	and	a, #0x80
	ld	b, a
	ld	c, #0x00
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shr16_15
; ---------------------------------
_shr16_15::
	ld	a, d
	rlca
	and	a, #0x01
	ld	c, a
	ld	b, #0x00
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function sar16_15
; ---------------------------------
_sar16_15::
	ld	a, d
	sra	a
	sra	a
	sra	a
	sra	a
	sra	a
	sra	a
	sra	a
	ld	c, a
	rlca
	sbc	a, a
	ld	b, a
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function rol16_15
; ---------------------------------
_rol16_15::
	ld	c, e
	ld	b, d
	ld	a, b
	rra
	rr	c
	rr	b
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function ror16_15
; ---------------------------------
_ror16_15::
	ld	c, e
	ld	b, d
	ld	a, b
	rla
	rl	c
	rl	b
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shl32_15
; ---------------------------------
_shl32_15::
	ld	d, e
	ld	e, b
	ld	b, c
	ld	c, #0x00
	ld	a, #0x07
00103$:
	sla	b
	rl	e
	rl	d
	dec	a
	jr	nz, 00103$
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function shr32_15
; ---------------------------------
_shr32_15::
	ld	c, b
	ld	b, e
	ld	e, d
	ld	d, #0x00
	ld	a, #0x07
00103$:
	srl	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function sar32_15
; ---------------------------------
_sar32_15::
	ld	a, #0x0f
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function rol32_15
; ---------------------------------
_rol32_15::
	add	sp, #-4
	ldhl	sp,	#1
	ld	a, c
	ld	(hl+), a
	ld	a, b
	ld	(hl+), a
	ld	(hl), e
	ldhl	sp,	#0
	ld	(hl), #0x00
	ld	a, #0x07
00103$:
	ldhl	sp,	#1
	sla	(hl)
	inc	hl
	rl	(hl)
	inc	hl
	rl	(hl)
	dec	a
	jr	nz, 00103$
	dec	hl
	dec	hl
	ld	c, e
	ld	b, d
	srl	b
	rr	c
	ld	a, b
	or	a, (hl)
	ld	b, a
	pop	hl
	pop	de
	push	de
	push	hl
	add	sp, #4
	ret
;(null):4: ERROR: Bad address
;	---------------------------------
; Function ror32_15
; ---------------------------------
_ror32_15::
	add	sp, #-4
	ldhl	sp,	#0
	ld	a, b
	ld	(hl+), a
	ld	a, e
	ld	(hl+), a
	ld	a, d
	ld	(hl+), a
	ld	(hl), #0x00
	ld	a, #0x07
00103$:
	ldhl	sp,	#2
	srl	(hl)
	dec	hl
	rr	(hl)
	dec	hl
	rr	(hl)
	dec	a
	jr	nz, 00103$
	inc	hl
	inc	hl
	ld	e, c
	ld	d, b
	sla	e
	rl	d
	pop	bc
	push	bc
	ld	a, e
	or	a, (hl)
	ld	e, a
	add	sp, #4
	ret
;(null):5: ERROR: Bad address
;	---------------------------------
; Function shl32_16
; ---------------------------------
_shl32_16::
	ld	e, c
	ld	d, b
	ld	bc, #0x0000
	ret
;(null):5: ERROR: Bad address
;	---------------------------------
; Function shr32_16
; ---------------------------------
_shr32_16::
	ld	c, e
	ld	b, d
	ld	de, #0x0000
	ret
;(null):5: ERROR: Bad address
;	---------------------------------
; Function sar32_16
; ---------------------------------
_sar32_16::
	ld	a, #0x10
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):5: ERROR: Bad address
;	---------------------------------
; Function rol32_16
; ---------------------------------
_rol32_16::
	ld	a, c
	ld	l, b
	ld	c, e
	ld	e, a
	ld	b, d
	ld	d, l
	ret
;(null):5: ERROR: Bad address
;	---------------------------------
; Function ror32_16
; ---------------------------------
_ror32_16::
	ld	a, c
	ld	l, b
	ld	c, e
	ld	e, a
	ld	b, d
	ld	d, l
	ret
;(null):5: ERROR: Bad address
;	---------------------------------
; Function shl32_17
; ---------------------------------
_shl32_17::
	ld	l, c
	ld	h, b
	ld	bc, #0x0000
	add	hl, hl
	ld	e, l
	ld	d, h
	ret
;(null):5: ERROR: Bad address
;	---------------------------------
; Function shr32_17
; ---------------------------------
_shr32_17::
	ld	c, e
	ld	b, d
	ld	de, #0x0000
	srl	b
	rr	c
	ret
;(null):5: ERROR: Bad address
;	---------------------------------
; Function sar32_17
; ---------------------------------
_sar32_17::
	ld	a, #0x11
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):5: ERROR: Bad address
;	---------------------------------
; Function rol32_17
; ---------------------------------
_rol32_17::
	add	sp, #-4
	ldhl	sp,	#2
	ld	a, c
	ld	(hl+), a
	ld	(hl), b
	ldhl	sp,	#0
	xor	a, a
	ld	(hl+), a
	ld	(hl+), a
	sla	(hl)
	inc	hl
	rl	(hl)
	ld	c, b
	ld	b, e
	ld	e, d
	ld	a, #0x07
00105$:
	srl	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00105$
	ld	a, e
	ldhl	sp,	#2
	or	a, (hl)
	inc	hl
	ld	e, a
	ld	d, (hl)
	add	sp, #4
	ret
;(null):5: ERROR: Bad address
;	---------------------------------
; Function ror32_17
; ---------------------------------
_ror32_17::
	add	sp, #-4
	pop	hl
	push	de
	xor	a, a
	ldhl	sp,	#2
	ld	(hl+), a
	ld	(hl-), a
	dec	hl
	srl	(hl)
	dec	hl
	rr	(hl)
	ld	d, e
	ld	e, b
	ld	b, c
	ld	a, #0x07
00105$:
	sla	b
	rl	e
	rl	d
	dec	a
	jr	nz, 00105$
	ld	a, (hl+)
	ld	c, a
	ld	a, b
	or	a, (hl)
	ld	b, a
	add	sp, #4
	ret
;(null):6: ERROR: Bad address
;	---------------------------------
; Function shl32_18
; ---------------------------------
_shl32_18::
	ld	l, c
	ld	h, b
	ld	bc, #0x0000
	ld	a, #0x02
00103$:
	add	hl, hl
	dec	a
	jr	nz, 00103$
	ld	e, l
	ld	d, h
	ret
;(null):6: ERROR: Bad address
;	---------------------------------
; Function shr32_18
; ---------------------------------
_shr32_18::
	ld	c, e
	ld	b, d
	ld	de, #0x0000
	ld	a, #0x02
00103$:
	srl	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):6: ERROR: Bad address
;	---------------------------------
; Function sar32_18
; ---------------------------------
_sar32_18::
	ld	a, #0x12
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):6: ERROR: Bad address
;	---------------------------------
; Function rol32_18
; ---------------------------------
_rol32_18::
	add	sp, #-4
	ldhl	sp,	#2
	ld	a, c
	ld	(hl+), a
	ld	(hl), b
	ldhl	sp,	#0
	xor	a, a
	ld	(hl+), a
	ld	(hl), a
	ld	a, #0x02
00103$:
	ldhl	sp,	#2
	sla	(hl)
	inc	hl
	rl	(hl)
	dec	a
	jr	nz, 00103$
	ld	c, b
	ld	b, e
	ld	e, d
	ld	a, #0x06
00105$:
	srl	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00105$
	ld	a, e
	ldhl	sp,	#2
	or	a, (hl)
	inc	hl
	ld	e, a
	ld	d, (hl)
	add	sp, #4
	ret
;(null):6: ERROR: Bad address
;	---------------------------------
; Function ror32_18
; ---------------------------------
_ror32_18::
	add	sp, #-4
	pop	hl
	push	de
	xor	a, a
	ldhl	sp,	#2
	ld	(hl+), a
	ld	(hl), a
	ld	a, #0x02
00103$:
	ldhl	sp,	#1
	srl	(hl)
	dec	hl
	rr	(hl)
	dec	a
	jr	nz, 00103$
	ld	d, e
	ld	e, b
	ld	b, c
	ld	a, #0x06
00105$:
	sla	b
	rl	e
	rl	d
	dec	a
	jr	nz, 00105$
	ld	a, (hl+)
	ld	c, a
	ld	a, b
	or	a, (hl)
	ld	b, a
	add	sp, #4
	ret
;(null):6: ERROR: Bad address
;	---------------------------------
; Function shl32_19
; ---------------------------------
_shl32_19::
	ld	l, c
	ld	h, b
	ld	bc, #0x0000
	ld	a, #0x03
00103$:
	add	hl, hl
	dec	a
	jr	nz, 00103$
	ld	e, l
	ld	d, h
	ret
;(null):6: ERROR: Bad address
;	---------------------------------
; Function shr32_19
; ---------------------------------
_shr32_19::
	ld	c, e
	ld	b, d
	ld	de, #0x0000
	ld	a, #0x03
00103$:
	srl	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):6: ERROR: Bad address
;	---------------------------------
; Function sar32_19
; ---------------------------------
_sar32_19::
	ld	a, #0x13
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):6: ERROR: Bad address
;	---------------------------------
; Function rol32_19
; ---------------------------------
_rol32_19::
	add	sp, #-4
	ldhl	sp,	#2
	ld	a, c
	ld	(hl+), a
	ld	(hl), b
	ldhl	sp,	#0
	xor	a, a
	ld	(hl+), a
	ld	(hl), a
	ld	a, #0x03
00103$:
	ldhl	sp,	#2
	sla	(hl)
	inc	hl
	rl	(hl)
	dec	a
	jr	nz, 00103$
	ld	c, b
	ld	b, e
	ld	e, d
	ld	a, #0x05
00105$:
	srl	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00105$
	ld	a, e
	ldhl	sp,	#2
	or	a, (hl)
	inc	hl
	ld	e, a
	ld	d, (hl)
	add	sp, #4
	ret
;(null):6: ERROR: Bad address
;	---------------------------------
; Function ror32_19
; ---------------------------------
_ror32_19::
	add	sp, #-4
	pop	hl
	push	de
	xor	a, a
	ldhl	sp,	#2
	ld	(hl+), a
	ld	(hl), a
	ld	a, #0x03
00103$:
	ldhl	sp,	#1
	srl	(hl)
	dec	hl
	rr	(hl)
	dec	a
	jr	nz, 00103$
	ld	d, e
	ld	e, b
	ld	b, c
	ld	a, #0x05
00105$:
	sla	b
	rl	e
	rl	d
	dec	a
	jr	nz, 00105$
	ld	a, (hl+)
	ld	c, a
	ld	a, b
	or	a, (hl)
	ld	b, a
	add	sp, #4
	ret
;(null):7: ERROR: Bad address
;	---------------------------------
; Function shl32_20
; ---------------------------------
_shl32_20::
	ld	l, c
	ld	h, b
	ld	bc, #0x0000
	ld	a, #0x04
00103$:
	add	hl, hl
	dec	a
	jr	nz, 00103$
	ld	e, l
	ld	d, h
	ret
;(null):7: ERROR: Bad address
;	---------------------------------
; Function shr32_20
; ---------------------------------
_shr32_20::
	ld	c, e
	ld	b, d
	ld	de, #0x0000
	ld	a, #0x04
00103$:
	srl	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):7: ERROR: Bad address
;	---------------------------------
; Function sar32_20
; ---------------------------------
_sar32_20::
	ld	a, #0x14
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):7: ERROR: Bad address
;	---------------------------------
; Function rol32_20
; ---------------------------------
_rol32_20::
	add	sp, #-4
	ldhl	sp,	#2
	ld	a, c
	ld	(hl+), a
	ld	(hl), b
	ldhl	sp,	#0
	xor	a, a
	ld	(hl+), a
	ld	(hl), a
	ld	a, #0x04
00103$:
	ldhl	sp,	#2
	sla	(hl)
	inc	hl
	rl	(hl)
	dec	a
	jr	nz, 00103$
	ld	c, b
	ld	b, e
	ld	e, d
	ld	a, #0x04
00105$:
	srl	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00105$
	ld	a, e
	ldhl	sp,	#2
	or	a, (hl)
	inc	hl
	ld	e, a
	ld	d, (hl)
	add	sp, #4
	ret
;(null):7: ERROR: Bad address
;	---------------------------------
; Function ror32_20
; ---------------------------------
_ror32_20::
	add	sp, #-4
	pop	hl
	push	de
	xor	a, a
	ldhl	sp,	#2
	ld	(hl+), a
	ld	(hl), a
	ld	a, #0x04
00103$:
	ldhl	sp,	#1
	srl	(hl)
	dec	hl
	rr	(hl)
	dec	a
	jr	nz, 00103$
	ld	d, e
	ld	e, b
	ld	b, c
	ld	a, #0x04
00105$:
	sla	b
	rl	e
	rl	d
	dec	a
	jr	nz, 00105$
	ld	a, (hl+)
	ld	c, a
	ld	a, b
	or	a, (hl)
	ld	b, a
	add	sp, #4
	ret
;(null):7: ERROR: Bad address
;	---------------------------------
; Function shl32_21
; ---------------------------------
_shl32_21::
	ld	l, c
	ld	h, b
	ld	bc, #0x0000
	ld	a, #0x05
00103$:
	add	hl, hl
	dec	a
	jr	nz, 00103$
	ld	e, l
	ld	d, h
	ret
;(null):7: ERROR: Bad address
;	---------------------------------
; Function shr32_21
; ---------------------------------
_shr32_21::
	ld	c, e
	ld	b, d
	ld	de, #0x0000
	ld	a, #0x05
00103$:
	srl	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):7: ERROR: Bad address
;	---------------------------------
; Function sar32_21
; ---------------------------------
_sar32_21::
	ld	a, #0x15
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):7: ERROR: Bad address
;	---------------------------------
; Function rol32_21
; ---------------------------------
_rol32_21::
	add	sp, #-4
	ldhl	sp,	#2
	ld	a, c
	ld	(hl+), a
	ld	(hl), b
	ldhl	sp,	#0
	xor	a, a
	ld	(hl+), a
	ld	(hl), a
	ld	a, #0x05
00103$:
	ldhl	sp,	#2
	sla	(hl)
	inc	hl
	rl	(hl)
	dec	a
	jr	nz, 00103$
	ld	c, b
	ld	b, e
	ld	e, d
	ld	a, #0x03
00105$:
	srl	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00105$
	ld	a, e
	ldhl	sp,	#2
	or	a, (hl)
	inc	hl
	ld	e, a
	ld	d, (hl)
	add	sp, #4
	ret
;(null):7: ERROR: Bad address
;	---------------------------------
; Function ror32_21
; ---------------------------------
_ror32_21::
	add	sp, #-4
	pop	hl
	push	de
	xor	a, a
	ldhl	sp,	#2
	ld	(hl+), a
	ld	(hl), a
	ld	a, #0x05
00103$:
	ldhl	sp,	#1
	srl	(hl)
	dec	hl
	rr	(hl)
	dec	a
	jr	nz, 00103$
	ld	d, e
	ld	e, b
	ld	b, c
	ld	a, #0x03
00105$:
	sla	b
	rl	e
	rl	d
	dec	a
	jr	nz, 00105$
	ld	a, (hl+)
	ld	c, a
	ld	a, b
	or	a, (hl)
	ld	b, a
	add	sp, #4
	ret
;(null):8: ERROR: Bad address
;	---------------------------------
; Function shl32_22
; ---------------------------------
_shl32_22::
	ld	l, c
	ld	h, b
	ld	bc, #0x0000
	ld	a, #0x06
00103$:
	add	hl, hl
	dec	a
	jr	nz, 00103$
	ld	e, l
	ld	d, h
	ret
;(null):8: ERROR: Bad address
;	---------------------------------
; Function shr32_22
; ---------------------------------
_shr32_22::
	ld	c, e
	ld	b, d
	ld	de, #0x0000
	ld	a, #0x06
00103$:
	srl	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):8: ERROR: Bad address
;	---------------------------------
; Function sar32_22
; ---------------------------------
_sar32_22::
	ld	a, #0x16
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):8: ERROR: Bad address
;	---------------------------------
; Function rol32_22
; ---------------------------------
_rol32_22::
	add	sp, #-4
	ldhl	sp,	#2
	ld	a, c
	ld	(hl+), a
	ld	(hl), b
	ldhl	sp,	#0
	xor	a, a
	ld	(hl+), a
	ld	(hl), a
	ld	a, #0x06
00103$:
	ldhl	sp,	#2
	sla	(hl)
	inc	hl
	rl	(hl)
	dec	a
	jr	nz, 00103$
	ld	c, b
	ld	b, e
	ld	e, d
	ld	a, #0x02
00105$:
	srl	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00105$
	ld	a, e
	ldhl	sp,	#2
	or	a, (hl)
	inc	hl
	ld	e, a
	ld	d, (hl)
	add	sp, #4
	ret
;(null):8: ERROR: Bad address
;	---------------------------------
; Function ror32_22
; ---------------------------------
_ror32_22::
	add	sp, #-4
	pop	hl
	push	de
	xor	a, a
	ldhl	sp,	#2
	ld	(hl+), a
	ld	(hl), a
	ld	a, #0x06
00103$:
	ldhl	sp,	#1
	srl	(hl)
	dec	hl
	rr	(hl)
	dec	a
	jr	nz, 00103$
	ld	d, e
	ld	e, b
	ld	b, c
	ld	a, #0x02
00105$:
	sla	b
	rl	e
	rl	d
	dec	a
	jr	nz, 00105$
	ld	a, (hl+)
	ld	c, a
	ld	a, b
	or	a, (hl)
	ld	b, a
	add	sp, #4
	ret
;(null):8: ERROR: Bad address
;	---------------------------------
; Function shl32_23
; ---------------------------------
_shl32_23::
	ld	l, c
	ld	h, b
	ld	bc, #0x0000
	ld	a, #0x07
00103$:
	add	hl, hl
	dec	a
	jr	nz, 00103$
	ld	e, l
	ld	d, h
	ret
;(null):8: ERROR: Bad address
;	---------------------------------
; Function shr32_23
; ---------------------------------
_shr32_23::
	ld	c, e
	ld	b, d
	ld	de, #0x0000
	ld	a, #0x07
00103$:
	srl	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):8: ERROR: Bad address
;	---------------------------------
; Function sar32_23
; ---------------------------------
_sar32_23::
	ld	a, #0x17
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):8: ERROR: Bad address
;	---------------------------------
; Function rol32_23
; ---------------------------------
_rol32_23::
	add	sp, #-4
	ldhl	sp,	#2
	ld	a, c
	ld	(hl+), a
	ld	(hl), b
	ldhl	sp,	#0
	xor	a, a
	ld	(hl+), a
	ld	(hl), a
	ld	a, #0x07
00103$:
	ldhl	sp,	#2
	sla	(hl)
	inc	hl
	rl	(hl)
	dec	a
	jr	nz, 00103$
	dec	hl
	ld	c, b
	ld	b, e
	ld	e, d
	srl	e
	rr	b
	rr	c
	ld	a, e
	or	a, (hl)
	inc	hl
	ld	e, a
	ld	d, (hl)
	add	sp, #4
	ret
;(null):8: ERROR: Bad address
;	---------------------------------
; Function ror32_23
; ---------------------------------
_ror32_23::
	add	sp, #-4
	pop	hl
	push	de
	xor	a, a
	ldhl	sp,	#2
	ld	(hl+), a
	ld	(hl), a
	ld	a, #0x07
00103$:
	ldhl	sp,	#1
	srl	(hl)
	dec	hl
	rr	(hl)
	dec	a
	jr	nz, 00103$
	ld	d, e
	ld	e, b
	ld	b, c
	sla	b
	rl	e
	rl	d
	ld	a, (hl+)
	ld	c, a
	ld	a, b
	or	a, (hl)
	ld	b, a
	add	sp, #4
	ret
;(null):9: ERROR: Bad address
;	---------------------------------
; Function shl32_24
; ---------------------------------
_shl32_24::
	ld	d, c
	ld	bc, #0x0000
	ld	e, #0x00
	ret
;(null):9: ERROR: Bad address
;	---------------------------------
; Function shr32_24
; ---------------------------------
_shr32_24::
	ld	c, d
	ld	b, #0x00
	ld	de, #0x0000
	ret
;(null):9: ERROR: Bad address
;	---------------------------------
; Function sar32_24
; ---------------------------------
_sar32_24::
	ld	a, #0x18
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):9: ERROR: Bad address
;	---------------------------------
; Function rol32_24
; ---------------------------------
_rol32_24::
	add	sp, #-4
	ldhl	sp,	#3
	ld	(hl), c
	ldhl	sp,	#0
	xor	a, a
	ld	(hl+), a
	ld	(hl+), a
	xor	a, a
	ld	(hl+), a
	ld	c, b
	ld	b, e
	ld	e, d
	ld	d, (hl)
	add	sp, #4
	ret
;(null):9: ERROR: Bad address
;	---------------------------------
; Function ror32_24
; ---------------------------------
_ror32_24::
	add	sp, #-4
	ldhl	sp,	#0
	ld	a, d
	ld	(hl+), a
	xor	a, a
	ld	(hl+), a
	ld	(hl+), a
	ld	(hl), a
	ld	d, e
	ld	e, b
	ld	b, c
	ldhl	sp,	#0
	ld	c, (hl)
	add	sp, #4
	ret
;(null):9: ERROR: Bad address
;	---------------------------------
; Function shl32_25
; ---------------------------------
_shl32_25::
	ld	d, c
	ld	bc, #0x0000
	ld	e, #0x00
	sla	d
	ret
;(null):9: ERROR: Bad address
;	---------------------------------
; Function shr32_25
; ---------------------------------
_shr32_25::
	ld	c, d
	ld	b, #0x00
	ld	de, #0x0000
	srl	c
	ret
;(null):9: ERROR: Bad address
;	---------------------------------
; Function sar32_25
; ---------------------------------
_sar32_25::
	ld	a, #0x19
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):9: ERROR: Bad address
;	---------------------------------
; Function rol32_25
; ---------------------------------
_rol32_25::
	add	sp, #-4
	ldhl	sp,	#3
	ld	(hl), c
	ldhl	sp,	#0
	xor	a, a
	ld	(hl+), a
	ld	(hl+), a
	xor	a, a
	ld	(hl+), a
	sla	(hl)
	ld	a, #0x07
00105$:
	srl	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00105$
	ld	a, d
	ldhl	sp,	#3
	or	a, (hl)
	ld	d, a
	add	sp, #4
	ret
;(null):9: ERROR: Bad address
;	---------------------------------
; Function ror32_25
; ---------------------------------
_ror32_25::
	add	sp, #-4
	ldhl	sp,	#0
	ld	a, d
	ld	(hl+), a
	xor	a, a
	ld	(hl+), a
	ld	(hl+), a
	ld	(hl), a
	ldhl	sp,	#0
	srl	(hl)
	ld	a, #0x07
00105$:
	sla	c
	rl	b
	rl	e
	rl	d
	dec	a
	jr	nz, 00105$
	ld	a, c
	or	a, (hl)
	ld	c, a
	add	sp, #4
	ret
;(null):10: ERROR: Bad address
;	---------------------------------
; Function shl32_26
; ---------------------------------
_shl32_26::
	ld	d, c
	ld	bc, #0x0000
	ld	e, #0x00
	ld	a, #0x02
00103$:
	sla	d
	dec	a
	jr	nz, 00103$
	ret
;(null):10: ERROR: Bad address
;	---------------------------------
; Function shr32_26
; ---------------------------------
_shr32_26::
	ld	c, d
	ld	b, #0x00
	ld	de, #0x0000
	ld	a, #0x02
00103$:
	srl	c
	dec	a
	jr	nz, 00103$
	ret
;(null):10: ERROR: Bad address
;	---------------------------------
; Function sar32_26
; ---------------------------------
_sar32_26::
	ld	a, #0x1a
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):10: ERROR: Bad address
;	---------------------------------
; Function rol32_26
; ---------------------------------
_rol32_26::
	add	sp, #-4
	ldhl	sp,	#3
	ld	(hl), c
	ldhl	sp,	#0
	xor	a, a
	ld	(hl+), a
	ld	(hl+), a
	ld	(hl), #0x00
	ld	a, #0x02
00103$:
	ldhl	sp,	#3
	sla	(hl)
	dec	a
	jr	nz, 00103$
	ld	a, #0x06
00105$:
	srl	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00105$
	ld	a, d
	ldhl	sp,	#3
	or	a, (hl)
	ld	d, a
	add	sp, #4
	ret
;(null):10: ERROR: Bad address
;	---------------------------------
; Function ror32_26
; ---------------------------------
_ror32_26::
	add	sp, #-4
	ldhl	sp,	#0
	ld	a, d
	ld	(hl+), a
	xor	a, a
	ld	(hl+), a
	ld	(hl+), a
	ld	(hl), a
	ld	a, #0x02
00103$:
	ldhl	sp,	#0
	srl	(hl)
	dec	a
	jr	nz, 00103$
	ld	a, #0x06
00105$:
	sla	c
	rl	b
	rl	e
	rl	d
	dec	a
	jr	nz, 00105$
	ld	a, c
	or	a, (hl)
	ld	c, a
	add	sp, #4
	ret
;(null):10: ERROR: Bad address
;	---------------------------------
; Function shl32_27
; ---------------------------------
_shl32_27::
	ld	d, c
	ld	bc, #0x0000
	ld	e, #0x00
	ld	a, #0x03
00103$:
	sla	d
	dec	a
	jr	nz, 00103$
	ret
;(null):10: ERROR: Bad address
;	---------------------------------
; Function shr32_27
; ---------------------------------
_shr32_27::
	ld	c, d
	ld	b, #0x00
	ld	de, #0x0000
	ld	a, #0x03
00103$:
	srl	c
	dec	a
	jr	nz, 00103$
	ret
;(null):10: ERROR: Bad address
;	---------------------------------
; Function sar32_27
; ---------------------------------
_sar32_27::
	ld	a, #0x1b
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):10: ERROR: Bad address
;	---------------------------------
; Function rol32_27
; ---------------------------------
_rol32_27::
	add	sp, #-4
	ldhl	sp,	#3
	ld	(hl), c
	ldhl	sp,	#0
	xor	a, a
	ld	(hl+), a
	ld	(hl+), a
	ld	(hl), #0x00
	ld	a, #0x03
00103$:
	ldhl	sp,	#3
	sla	(hl)
	dec	a
	jr	nz, 00103$
	ld	a, #0x05
00105$:
	srl	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00105$
	ld	a, d
	ldhl	sp,	#3
	or	a, (hl)
	ld	d, a
	add	sp, #4
	ret
;(null):10: ERROR: Bad address
;	---------------------------------
; Function ror32_27
; ---------------------------------
_ror32_27::
	add	sp, #-4
	ldhl	sp,	#0
	ld	a, d
	ld	(hl+), a
	xor	a, a
	ld	(hl+), a
	ld	(hl+), a
	ld	(hl), a
	ld	a, #0x03
00103$:
	ldhl	sp,	#0
	srl	(hl)
	dec	a
	jr	nz, 00103$
	ld	a, #0x05
00105$:
	sla	c
	rl	b
	rl	e
	rl	d
	dec	a
	jr	nz, 00105$
	ld	a, c
	or	a, (hl)
	ld	c, a
	add	sp, #4
	ret
;(null):11: ERROR: Bad address
;	---------------------------------
; Function shl32_28
; ---------------------------------
_shl32_28::
	ld	d, c
	ld	bc, #0x0000
	ld	e, #0x00
	ld	a, #0x04
00103$:
	sla	d
	dec	a
	jr	nz, 00103$
	ret
;(null):11: ERROR: Bad address
;	---------------------------------
; Function shr32_28
; ---------------------------------
_shr32_28::
	ld	c, d
	ld	b, #0x00
	ld	de, #0x0000
	ld	a, #0x04
00103$:
	srl	c
	dec	a
	jr	nz, 00103$
	ret
;(null):11: ERROR: Bad address
;	---------------------------------
; Function sar32_28
; ---------------------------------
_sar32_28::
	ld	a, #0x1c
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):11: ERROR: Bad address
;	---------------------------------
; Function rol32_28
; ---------------------------------
_rol32_28::
	add	sp, #-4
	ldhl	sp,	#3
	ld	(hl), c
	ldhl	sp,	#0
	xor	a, a
	ld	(hl+), a
	ld	(hl+), a
	ld	(hl), #0x00
	ld	a, #0x04
00103$:
	ldhl	sp,	#3
	sla	(hl)
	dec	a
	jr	nz, 00103$
	ld	a, #0x04
00105$:
	srl	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00105$
	ld	a, d
	ldhl	sp,	#3
	or	a, (hl)
	ld	d, a
	add	sp, #4
	ret
;(null):11: ERROR: Bad address
;	---------------------------------
; Function ror32_28
; ---------------------------------
_ror32_28::
	add	sp, #-4
	ldhl	sp,	#0
	ld	a, d
	ld	(hl+), a
	xor	a, a
	ld	(hl+), a
	ld	(hl+), a
	ld	(hl), a
	ld	a, #0x04
00103$:
	ldhl	sp,	#0
	srl	(hl)
	dec	a
	jr	nz, 00103$
	ld	a, #0x04
00105$:
	sla	c
	rl	b
	rl	e
	rl	d
	dec	a
	jr	nz, 00105$
	ld	a, c
	or	a, (hl)
	ld	c, a
	add	sp, #4
	ret
;(null):11: ERROR: Bad address
;	---------------------------------
; Function shl32_29
; ---------------------------------
_shl32_29::
	ld	d, c
	ld	bc, #0x0000
	ld	e, #0x00
	ld	a, #0x05
00103$:
	sla	d
	dec	a
	jr	nz, 00103$
	ret
;(null):11: ERROR: Bad address
;	---------------------------------
; Function shr32_29
; ---------------------------------
_shr32_29::
	ld	c, d
	ld	b, #0x00
	ld	de, #0x0000
	ld	a, #0x05
00103$:
	srl	c
	dec	a
	jr	nz, 00103$
	ret
;(null):11: ERROR: Bad address
;	---------------------------------
; Function sar32_29
; ---------------------------------
_sar32_29::
	ld	a, #0x1d
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):11: ERROR: Bad address
;	---------------------------------
; Function rol32_29
; ---------------------------------
_rol32_29::
	add	sp, #-4
	ldhl	sp,	#3
	ld	(hl), c
	ldhl	sp,	#0
	xor	a, a
	ld	(hl+), a
	ld	(hl+), a
	ld	(hl), #0x00
	ld	a, #0x05
00103$:
	ldhl	sp,	#3
	sla	(hl)
	dec	a
	jr	nz, 00103$
	ld	a, #0x03
00105$:
	srl	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00105$
	ld	a, d
	ldhl	sp,	#3
	or	a, (hl)
	ld	d, a
	add	sp, #4
	ret
;(null):11: ERROR: Bad address
;	---------------------------------
; Function ror32_29
; ---------------------------------
_ror32_29::
	add	sp, #-4
	ldhl	sp,	#0
	ld	a, d
	ld	(hl+), a
	xor	a, a
	ld	(hl+), a
	ld	(hl+), a
	ld	(hl), a
	ld	a, #0x05
00103$:
	ldhl	sp,	#0
	srl	(hl)
	dec	a
	jr	nz, 00103$
	ld	a, #0x03
00105$:
	sla	c
	rl	b
	rl	e
	rl	d
	dec	a
	jr	nz, 00105$
	ld	a, c
	or	a, (hl)
	ld	c, a
	add	sp, #4
	ret
;(null):12: ERROR: Bad address
;	---------------------------------
; Function shl32_30
; ---------------------------------
_shl32_30::
	ld	d, c
	ld	bc, #0x0000
	ld	e, #0x00
	ld	a, #0x06
00103$:
	sla	d
	dec	a
	jr	nz, 00103$
	ret
;(null):12: ERROR: Bad address
;	---------------------------------
; Function shr32_30
; ---------------------------------
_shr32_30::
	ld	c, d
	ld	b, #0x00
	ld	de, #0x0000
	ld	a, #0x06
00103$:
	srl	c
	dec	a
	jr	nz, 00103$
	ret
;(null):12: ERROR: Bad address
;	---------------------------------
; Function sar32_30
; ---------------------------------
_sar32_30::
	ld	a, #0x1e
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):12: ERROR: Bad address
;	---------------------------------
; Function rol32_30
; ---------------------------------
_rol32_30::
	add	sp, #-4
	ldhl	sp,	#3
	ld	(hl), c
	ldhl	sp,	#0
	xor	a, a
	ld	(hl+), a
	ld	(hl+), a
	ld	(hl), #0x00
	ld	a, #0x06
00103$:
	ldhl	sp,	#3
	sla	(hl)
	dec	a
	jr	nz, 00103$
	ld	a, #0x02
00105$:
	srl	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00105$
	ld	a, d
	ldhl	sp,	#3
	or	a, (hl)
	ld	d, a
	add	sp, #4
	ret
;(null):12: ERROR: Bad address
;	---------------------------------
; Function ror32_30
; ---------------------------------
_ror32_30::
	add	sp, #-4
	ldhl	sp,	#0
	ld	a, d
	ld	(hl+), a
	xor	a, a
	ld	(hl+), a
	ld	(hl+), a
	ld	(hl), a
	ld	a, #0x06
00103$:
	ldhl	sp,	#0
	srl	(hl)
	dec	a
	jr	nz, 00103$
	ld	a, #0x02
00105$:
	sla	c
	rl	b
	rl	e
	rl	d
	dec	a
	jr	nz, 00105$
	ld	a, c
	or	a, (hl)
	ld	c, a
	add	sp, #4
	ret
;(null):12: ERROR: Bad address
;	---------------------------------
; Function shl32_31
; ---------------------------------
_shl32_31::
	ld	d, c
	ld	bc, #0x0000
	ld	e, #0x00
	ld	a, #0x07
00103$:
	sla	d
	dec	a
	jr	nz, 00103$
	ret
;(null):12: ERROR: Bad address
;	---------------------------------
; Function shr32_31
; ---------------------------------
_shr32_31::
	ld	c, d
	ld	b, #0x00
	ld	de, #0x0000
	ld	a, #0x07
00103$:
	srl	c
	dec	a
	jr	nz, 00103$
	ret
;(null):12: ERROR: Bad address
;	---------------------------------
; Function sar32_31
; ---------------------------------
_sar32_31::
	ld	a, #0x1f
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
	dec	a
	jr	nz, 00103$
	ret
;(null):12: ERROR: Bad address
;	---------------------------------
; Function rol32_31
; ---------------------------------
_rol32_31::
	ld	a, d
	rra
	rr	e
	rr	b
	rr	c
	rr	d
	ret
;(null):12: ERROR: Bad address
;	---------------------------------
; Function ror32_31
; ---------------------------------
_ror32_31::
	ld	a, d
	rla
	rl	c
	rl	b
	rl	e
	rl	d
	ret
;(null):13: ERROR: Bad address
;	---------------------------------
; Function shl16_var
; ---------------------------------
_shl16_var::
	and	a, #0x0f
	ld	b, a
	ld	l, e
	ld	h, d
	inc	b
	jr	00104$
00103$:
	add	hl, hl
00104$:
	dec	b
	jr	nz, 00103$
	ld	c, l
	ld	b, h
	ret
;(null):13: ERROR: Bad address
;	---------------------------------
; Function shr16_var
; ---------------------------------
_shr16_var::
	ld	c, e
	ld	b, d
	and	a, #0x0f
	inc	a
	jr	00104$
00103$:
	srl	b
	rr	c
00104$:
	dec	a
	jr	nz, 00103$
	ret
;(null):13: ERROR: Bad address
;	---------------------------------
; Function sar16_var
; ---------------------------------
_sar16_var::
	ld	c, e
	ld	b, d
	and	a, #0x0f
	inc	a
	jr	00104$
00103$:
	sra	b
	rr	c
00104$:
	dec	a
	jr	nz, 00103$
	ret
;(null):13: ERROR: Bad address
;	---------------------------------
; Function rol16_var
; ---------------------------------
_rol16_var::
	and	a, #0x0f
	ld	c, a
	ld	b, c
	ld	l, e
	ld	h, d
	inc	b
	jr	00104$
00103$:
	add	hl, hl
00104$:
	dec	b
	jr	nz, 00103$
	ld	a, b
	sub	a, c
	and	a, #0x0f
	inc	a
	jr	00106$
00105$:
	srl	d
	rr	e
00106$:
	dec	a
	jr	nz, 00105$
	ld	a, l
	or	a, e
	ld	c, a
	ld	a, h
	or	a, d
	ld	b, a
	ret
;(null):13: ERROR: Bad address
;	---------------------------------
; Function ror16_var
; ---------------------------------
_ror16_var::
	ld	l, e
	ld	h, d
	and	a, #0x0f
	ld	e, a
	ld	c, l
	ld	b, h
	inc	a
	jr	00104$
00103$:
	srl	b
	rr	c
00104$:
	dec	a
	jr	nz, 00103$
	sub	a, e
	and	a, #0x0f
	ld	e, a
	inc	e
	jr	00106$
00105$:
	add	hl, hl
00106$:
	dec	e
	jr	nz, 00105$
	ld	a, c
	or	a, l
	ld	c, a
	ld	a, b
	or	a, h
	ld	b, a
	ret
;(null):13: ERROR: Bad address
;	---------------------------------
; Function shl32_var
; ---------------------------------
_shl32_var::
	ldhl	sp,	#2
	ld	a, (hl)
	and	a, #0x1f
	ld	l, a
	inc	l
	jr	00104$
00103$:
	sla	c
	rl	b
	rl	e
	rl	d
00104$:
	dec	l
	jr	nz, 00103$
	pop	hl
	inc	sp
	jp	(hl)
;(null):13: ERROR: Bad address
;	---------------------------------
; Function shr32_var
; ---------------------------------
_shr32_var::
	ldhl	sp,	#2
	ld	a, (hl)
	and	a, #0x1f
	inc	a
	jr	00104$
00103$:
	srl	d
	rr	e
	rr	b
	rr	c
00104$:
	dec	a
	jr	nz, 00103$
	pop	hl
	inc	sp
	jp	(hl)
;(null):13: ERROR: Bad address
;	---------------------------------
; Function sar32_var
; ---------------------------------
_sar32_var::
	ldhl	sp,	#2
	ld	a, (hl)
	and	a, #0x1f
	inc	a
	jr	00104$
00103$:
	sra	d
	rr	e
	rr	b
	rr	c
00104$:
	dec	a
	jr	nz, 00103$
	pop	hl
	inc	sp
	jp	(hl)
;(null):13: ERROR: Bad address
;	---------------------------------
; Function rol32_var
; ---------------------------------
_rol32_var::
	add	sp, #-5
	ldhl	sp,	#7
	ld	a, (hl)
	and	a, #0x1f
	ld	(hl), a
	pop	hl
	push	bc
	ldhl	sp,	#2
	ld	(hl), e
	inc	hl
	ld	(hl), d
	inc	a
	jr	00104$
00103$:
	ldhl	sp,	#0
	sla	(hl)
	inc	hl
	rl	(hl)
	inc	hl
	rl	(hl)
	inc	hl
	rl	(hl)
00104$:
	dec	a
	jr	nz, 00103$
	ldhl	sp,	#7
	ld	a, (hl)
	ldhl	sp,	#4
	ld	(hl), a
	xor	a, a
	sub	a, (hl)
	and	a, #0x1f
	inc	a
	jr	00106$
00105$:
	srl	d
	rr	e
	rr	b
	rr	c
00106$:
	dec	a
	jr	nz, 00105$
	ld	a, c
	ldhl	sp,	#0
	or	a, (hl)
	inc	hl
	ld	c, a
	ld	a, b
	or	a, (hl)
	inc	hl
	ld	b, a
	ld	a, e
	or	a, (hl)
	inc	hl
	ld	e, a
	ld	a, d
	or	a, (hl)
	ld	d, a
	add	sp, #5
	pop	hl
	inc	sp
	jp	(hl)
;(null):13: ERROR: Bad address
;	---------------------------------
; Function ror32_var
; ---------------------------------
_ror32_var::
	add	sp, #-5
	ldhl	sp,	#7
	ld	a, (hl)
	and	a, #0x1f
	ld	(hl), a
	pop	hl
	push	bc
	ldhl	sp,	#2
	ld	(hl), e
	inc	hl
	ld	(hl), d
	inc	a
	jr	00104$
00103$:
	ldhl	sp,	#3
	srl	(hl)
	dec	hl
	rr	(hl)
	dec	hl
	rr	(hl)
	dec	hl
	rr	(hl)
00104$:
	dec	a
	jr	nz, 00103$
	ldhl	sp,	#7
	ld	a, (hl)
	ldhl	sp,	#4
	ld	(hl), a
	xor	a, a
	sub	a, (hl)
	and	a, #0x1f
	ld	l, a
	inc	l
	jr	00106$
00105$:
	sla	c
	rl	b
	rl	e
	rl	d
00106$:
	dec	l
	jr	nz, 00105$
	ld	a, c
	ldhl	sp,	#0
	or	a, (hl)
	inc	hl
	ld	c, a
	ld	a, b
	or	a, (hl)
	inc	hl
	ld	b, a
	ld	a, e
	or	a, (hl)
	inc	hl
	ld	e, a
	ld	a, d
	or	a, (hl)
	ld	d, a
	add	sp, #5
	pop	hl
	inc	sp
	jp	(hl)
	.area _CODE
	.area _INITIALIZER
	.area _CABS (ABS)
