; Input:  A
; Output: A
; Clobbers: B, C, H, L
;
; Table must be aligned to 256 bytes.
; 31 bytes/ 100 cycles
popcount8_fast:
    ld c, a

    and $0F
    ld l, a
    ld h, HIGH(.table)
    ld b, [hl]

    ld a, c
    swap a
    and $0F
    ld l, a
    ld a, [hl]
    add a, b
    ret

ALIGN 8
.table:
    db 0,1,1,2,1,2,2,3
    db 1,2,2,3,2,3,3,4
