pub const STACK_FIXTURE: &str = r#"SECTION "Audit helper", ROM0[$200]
audit_fixture::
    push bc
    ld hl, audit_table
    ld a, [hl]
    ld [audit_scratch], a
    call audit_inner
    pop bc
    ret
audit_inner:
    push af
    ld a, 9
    sub 4
    sbc 0
    cp 5
    pop af
    ret
SECTION "Audit table", ROM0[$400]
audit_table:
    db 42, 99
SECTION "Audit scratch", WRAM0[$c000]
audit_scratch:
    ds 4
"#;

pub const LOAD_INPUT: &str = r#"    ld hl, audit_input
    ld b, [hl]
    inc hl
    ld c, [hl]
"#;

pub const ADD_STACK_INPUT: &str = r#"    pop de
    add d
    ld c, a
    ld a, b
    adc e
    ld b, a
    ld a, c
"#;

pub const ADD_REGISTER_INPUT: &str = r#"    add l
    ld c, a
    ld a, b
    adc h
    ld b, a
    ld a, c
"#;
