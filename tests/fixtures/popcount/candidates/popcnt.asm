; Input: HL = pointer, D = byte count
; Output: A = popcount
; Clobbers: B, C, D, E, F, HL
popcount_bytes:
    ld e, 0

.loop:
    ld a, [hli]
    call popcount8
    add a, e
    ld e, a

    dec d
    jr nz, .loop

    ld a, e
    ret

popcount16:
    ld d, 2
    jp popcount_bytes

popcount32:
    ld d, 4
    jp popcount_bytes

popcount64:
    ld d, 8
    jp popcount_bytes

popcount128:
    ld d, 16
    jp popcount_bytes
