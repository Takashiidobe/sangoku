SECTION "heap", ROM0

__gbc_heap_init::
    ld hl, $C7FE
    ld a, $00
    ld [hli], a
    ld a, $C8
    ld [hl], a
    ret
