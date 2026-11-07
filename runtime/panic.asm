SECTION "panic", ROM0

; build replaces __gbc_panic with the mangled name core references
__gbc_panic::
    di
.halt:
    halt
    nop
    jr .halt
