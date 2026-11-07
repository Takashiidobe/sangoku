SECTION "Unreachable trap vector", ROM0[$10]
TrapVector:
    di
.loop
    jr .loop

SECTION "Cartridge entry", ROM0[$100]
    jp Start
    ds $150 - @, 0

SECTION "Startup", ROM0
Start:
    ld sp, $E000
    call __rust_gbc_main
.idle
    halt
    jr .idle
