;--------------------------------------------------------
; File Created by SDCC : free open source ISO C Compiler
; Version 4.6.4 #17147 (Linux)
;--------------------------------------------------------
	.module build_popcount_audit_sdcc_native

	.optsdcc -msm83 sdcccall(1)
;--------------------------------------------------------
; Public variables in this module
;--------------------------------------------------------
	.globl _audit_popcount
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
;tests/fixtures/popcount/sdcc-loop.c:1: unsigned int audit_popcount(unsigned int value)
;	---------------------------------
; Function audit_popcount
; ---------------------------------
_audit_popcount::
;tests/fixtures/popcount/sdcc-loop.c:4: for (unsigned char i = 0; i < 16; i++)
	ld	bc, #0x0
00105$:
;tests/fixtures/popcount/sdcc-loop.c:5: if (value & (1u << i))
	ld	a, b
	cp	a, #0x10
	jr	nc, 00103$
	push	af
	ld	hl, #0x0001
	pop	af
	inc	a
	jr	00135$
00134$:
	add	hl, hl
00135$:
	dec	a
	jr	nz, 00134$
	ld	a, l
	and	a, e
	ld	l, a
	ld	a, h
	and	a, d
	or	a, l
	jr	z, 00106$
;tests/fixtures/popcount/sdcc-loop.c:6: count++;
	inc	c
00106$:
;tests/fixtures/popcount/sdcc-loop.c:4: for (unsigned char i = 0; i < 16; i++)
	inc	b
	jr	00105$
00103$:
;tests/fixtures/popcount/sdcc-loop.c:7: return count;
	ld	b, #0x00
;tests/fixtures/popcount/sdcc-loop.c:8: }
	ret
	.area _CODE
	.area _INITIALIZER
	.area _CABS (ABS)
