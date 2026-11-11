use sangoku::object::{
    Binding, Definition, Error, MAGIC, MemoryRegion, Object, Relocation, RelocationKind, Section,
    SectionId, Symbol, SymbolId, VERSION,
};

fn fixture() -> Object {
    Object {
        sections: vec![
            Section {
                name: "text".into(),
                region: MemoryRegion::Rom0,
                alignment: 1,
                fixed_address: Some(0x150),
                data: vec![0xcd, 0, 0, 0xc9],
                size: 4,
            },
            Section {
                name: "pointers".into(),
                region: MemoryRegion::RomX,
                alignment: 2,
                fixed_address: None,
                data: vec![0; 6],
                size: 6,
            },
            Section {
                name: "state".into(),
                region: MemoryRegion::Wram0,
                alignment: 2,
                fixed_address: Some(0xc000),
                data: vec![],
                size: 16,
            },
        ],
        symbols: vec![
            Symbol {
                name: "entry".into(),
                binding: Binding::Global,
                definition: Definition::Section {
                    section: SectionId(0),
                    offset: 0,
                },
            },
            Symbol {
                name: "helper".into(),
                binding: Binding::Global,
                definition: Definition::Undefined,
            },
            Symbol {
                name: ".end".into(),
                binding: Binding::Local,
                definition: Definition::Section {
                    section: SectionId(0),
                    offset: 4,
                },
            },
            Symbol {
                name: ".end".into(),
                binding: Binding::Local,
                definition: Definition::Section {
                    section: SectionId(1),
                    offset: 6,
                },
            },
            Symbol {
                name: "serial".into(),
                binding: Binding::Global,
                definition: Definition::Absolute { address: 0xff01 },
            },
            Symbol {
                name: "optional_helper".into(),
                binding: Binding::Weak,
                definition: Definition::Undefined,
            },
        ],
        relocations: vec![
            Relocation {
                section: SectionId(0),
                offset: 1,
                kind: RelocationKind::Absolute16,
                symbol: SymbolId(1),
                addend: 0,
            },
            Relocation {
                section: SectionId(1),
                offset: 0,
                kind: RelocationKind::Absolute16,
                symbol: SymbolId(0),
                addend: -1,
            },
            Relocation {
                section: SectionId(1),
                offset: 2,
                kind: RelocationKind::Absolute16,
                symbol: SymbolId(2),
                addend: 0,
            },
            Relocation {
                section: SectionId(1),
                offset: 4,
                kind: RelocationKind::Absolute16,
                symbol: SymbolId(3),
                addend: 0,
            },
        ],
    }
}

fn unchecked_bytes(object: &Object) -> Vec<u8> {
    let mut bytes = MAGIC.to_vec();
    bytes.extend_from_slice(&VERSION.to_le_bytes());
    serde_json::to_writer(&mut bytes, object).unwrap();
    bytes
}

fn rejects(object: &Object, expected: fn(&Error) -> bool) {
    let encoding = object.encode().unwrap_err();
    assert!(expected(&encoding), "unexpected encode error: {encoding}");
    let decoding = Object::decode(&unchecked_bytes(object)).unwrap_err();
    assert!(expected(&decoding), "unexpected decode error: {decoding}");
}

#[test]
fn object_roundtrip_preserves_imports_local_symbol_identity_and_zero_fill() {
    let mut object = fixture();
    object.relocations[0].addend = i64::MAX;
    object.relocations[1].addend = i64::MIN;
    let bytes = object.encode().unwrap();
    assert_eq!(&bytes[..10], b"SGOBJ\0\r\n\x01\x00");
    let decoded = Object::decode(&bytes).unwrap();
    assert_eq!(decoded, object);
    assert_ne!(decoded.relocations[2].symbol, decoded.relocations[3].symbol);
    assert_eq!(decoded.symbols[2].name, decoded.symbols[3].name);
}

#[test]
fn object_roundtrip_preserves_relative_and_byte_address_relocations() {
    let mut object = fixture();
    object.relocations.clear();
    for (offset, kind) in [
        RelocationKind::Absolute8,
        RelocationKind::Relative8,
        RelocationKind::Low8,
        RelocationKind::High8,
        RelocationKind::HighPage8,
    ]
    .into_iter()
    .enumerate()
    {
        object.relocations.push(Relocation {
            section: SectionId(1),
            offset: offset as u32,
            kind,
            symbol: SymbolId(4),
            addend: -1,
        });
    }
    assert_eq!(Object::decode(&object.encode().unwrap()).unwrap(), object);
}

#[test]
fn object_rejects_truncated_headers_bad_magic_and_unknown_versions() {
    let mut bytes = fixture().encode().unwrap();
    for length in [0, 1, 7, 8, 9] {
        assert!(matches!(
            Object::decode(&bytes[..length]),
            Err(Error::InvalidHeader)
        ));
    }
    bytes[0] = b'X';
    assert!(matches!(Object::decode(&bytes), Err(Error::InvalidHeader)));
    bytes[0] = MAGIC[0];
    bytes[8..10].copy_from_slice(&2u16.to_le_bytes());
    assert!(matches!(
        Object::decode(&bytes),
        Err(Error::UnsupportedVersion(2))
    ));
}

#[test]
fn object_rejects_malformed_payloads_and_unknown_fields() {
    let header = &fixture().encode().unwrap()[..10];
    for payload in [
        b"{".as_slice(),
        br#"{"sections":[],"symbols":[],"relocations":[],"extra":true}"#,
        br#"{"sections":[],"symbols":[],"relocations":[]} trailing"#,
    ] {
        let bytes = [header, payload].concat();
        assert!(matches!(Object::decode(&bytes), Err(Error::Payload(_))));
    }
    let mut payload = serde_json::to_value(fixture()).unwrap();
    payload["symbols"][1]["definition"]["extra"] = serde_json::json!(true);
    let bytes = [header, serde_json::to_vec(&payload).unwrap().as_slice()].concat();
    assert!(matches!(Object::decode(&bytes), Err(Error::Payload(_))));
    let mut payload = serde_json::to_value(fixture()).unwrap();
    payload["symbols"][0]["definition"]["value"]["extra"] = serde_json::json!(true);
    let bytes = [header, serde_json::to_vec(&payload).unwrap().as_slice()].concat();
    assert!(matches!(Object::decode(&bytes), Err(Error::Payload(_))));
}

#[test]
fn object_rejects_illegal_section_alignment_size_and_fixed_placement() {
    let mutations: [fn(&mut Section); 8] = [
        |section| section.alignment = 0,
        |section| section.alignment = 3,
        |section| section.alignment = 0x8000,
        |section| section.size = 3,
        |section| section.size = 0x4001,
        |section| section.fixed_address = Some(0x4000),
        |section| section.fixed_address = Some(0x3fff),
        |section| {
            section.alignment = 4;
            section.fixed_address = Some(0x151);
        },
    ];
    for mutate in mutations {
        let mut object = fixture();
        mutate(&mut object.sections[0]);
        rejects(&object, |error| {
            matches!(error, Error::Section { section: 0, .. })
        });
    }
}

#[test]
fn object_accepts_exact_region_boundaries_and_rejects_ie_as_hram_storage() {
    let mut object = Object::default();
    for region in [
        MemoryRegion::Rom0,
        MemoryRegion::RomX,
        MemoryRegion::Wram0,
        MemoryRegion::Hram,
    ] {
        let range = region.address_range();
        object.sections.push(Section {
            name: format!("{region:?}"),
            region,
            alignment: 1,
            fixed_address: Some(range.start as u16),
            data: vec![],
            size: range.end - range.start,
        });
    }
    object.validate().unwrap();
    object.sections[3].fixed_address = Some(0xffff);
    object.sections[3].size = 1;
    rejects(&object, |error| {
        matches!(error, Error::Section { section: 3, .. })
    });
}

#[test]
fn object_rejects_duplicate_external_names_and_invalid_symbol_definitions() {
    let mutations: [fn(&mut Object); 4] = [
        |object| object.symbols[1].name = "entry".into(),
        |object| object.symbols[1].binding = Binding::Local,
        |object| {
            object.symbols[1].definition = Definition::Section {
                section: SectionId(u32::MAX),
                offset: 0,
            }
        },
        |object| {
            object.symbols[1].definition = Definition::Section {
                section: SectionId(0),
                offset: 5,
            }
        },
    ];
    for mutate in mutations {
        let mut object = fixture();
        mutate(&mut object);
        rejects(&object, |error| {
            matches!(error, Error::Symbol { symbol: 1, .. })
        });
    }
}

#[test]
fn object_rejects_invalid_relocation_references_and_patches_into_zero_fill() {
    let mutations: [fn(&mut Relocation); 5] = [
        |relocation| relocation.section = SectionId(u32::MAX),
        |relocation| relocation.symbol = SymbolId(u32::MAX),
        |relocation| relocation.offset = u32::MAX,
        |relocation| relocation.offset = 3,
        |relocation| {
            relocation.section = SectionId(2);
            relocation.offset = 0;
        },
    ];
    for mutate in mutations {
        let mut object = fixture();
        mutate(&mut object.relocations[0]);
        rejects(&object, |error| {
            matches!(error, Error::Relocation { relocation: 0, .. })
        });
    }
}

#[test]
fn object_rejects_overlapping_patches_independently_of_input_order() {
    let mut object = fixture();
    let mut relocation = object.relocations[0].clone();
    relocation.offset = 2;
    object.relocations.insert(0, relocation);
    rejects(&object, |error| {
        matches!(error, Error::Relocation { reason, .. }
        if *reason == "patch overlaps another relocation")
    });
}
