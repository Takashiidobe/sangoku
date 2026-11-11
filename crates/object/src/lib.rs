use std::{collections::HashSet, ops::Range};

use serde::{Deserialize, Serialize};

pub const MAGIC: &[u8; 8] = b"SGOBJ\0\r\n";
pub const VERSION: u16 = 1;
const HEADER_SIZE: usize = MAGIC.len() + 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SectionId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SymbolId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryRegion {
    Rom0,
    RomX,
    Wram0,
    Hram,
}

impl MemoryRegion {
    pub const fn address_range(self) -> Range<u32> {
        match self {
            Self::Rom0 => 0x0000..0x4000,
            Self::RomX => 0x4000..0x8000,
            Self::Wram0 => 0xc000..0xd000,
            Self::Hram => 0xff80..0xffff,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Section {
    pub name: String,
    pub region: MemoryRegion,
    pub alignment: u32,
    pub fixed_address: Option<u16>,
    pub data: Vec<u8>,
    pub size: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Binding {
    Local,
    Global,
    Weak,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum Definition {
    Undefined,
    Section { section: SectionId, offset: u32 },
    Absolute { address: u16 },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Symbol {
    pub name: String,
    pub binding: Binding,
    pub definition: Definition,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelocationKind {
    Absolute16,
    Absolute8,
    Relative8,
    Low8,
    High8,
    HighPage8,
}

impl RelocationKind {
    pub const fn width(self) -> u32 {
        match self {
            Self::Absolute16 => 2,
            Self::Absolute8 | Self::Relative8 | Self::Low8 | Self::High8 | Self::HighPage8 => 1,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Relocation {
    pub section: SectionId,
    pub offset: u32,
    pub kind: RelocationKind,
    pub symbol: SymbolId,
    pub addend: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Object {
    pub sections: Vec<Section>,
    pub symbols: Vec<Symbol>,
    pub relocations: Vec<Relocation>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid or truncated sangoku object header")]
    InvalidHeader,
    #[error("unsupported sangoku object version {0}")]
    UnsupportedVersion(u16),
    #[error("invalid sangoku object payload: {0}")]
    Payload(#[from] serde_json::Error),
    #[error("invalid section {section}: {reason}")]
    Section {
        section: usize,
        reason: &'static str,
    },
    #[error("invalid symbol {symbol}: {reason}")]
    Symbol { symbol: usize, reason: &'static str },
    #[error("invalid relocation {relocation}: {reason}")]
    Relocation {
        relocation: usize,
        reason: &'static str,
    },
}

impl Object {
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        self.validate()?;
        let mut bytes = Vec::new();
        bytes.extend_from_slice(MAGIC);
        bytes.extend_from_slice(&VERSION.to_le_bytes());
        serde_json::to_writer(&mut bytes, self)?;
        Ok(bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, Error> {
        let header = bytes.get(..HEADER_SIZE).ok_or(Error::InvalidHeader)?;
        if &header[..MAGIC.len()] != MAGIC {
            return Err(Error::InvalidHeader);
        }
        let version = u16::from_le_bytes([header[MAGIC.len()], header[MAGIC.len() + 1]]);
        if version != VERSION {
            return Err(Error::UnsupportedVersion(version));
        }
        let object: Self = serde_json::from_slice(&bytes[HEADER_SIZE..])?;
        object.validate()?;
        Ok(object)
    }

    pub fn validate(&self) -> Result<(), Error> {
        for (index, section) in self.sections.iter().enumerate() {
            let invalid = |reason| Error::Section {
                section: index,
                reason,
            };
            let range = section.region.address_range();
            let capacity = range.end - range.start;
            if section.name.is_empty() || section.name.contains('\0') {
                return Err(invalid("name must be nonempty and contain no NUL"));
            }
            if !section.alignment.is_power_of_two() || section.alignment > capacity {
                return Err(invalid(
                    "alignment must be a power of two within the memory region",
                ));
            }
            if section.size > capacity || section.data.len() as u64 > u64::from(section.size) {
                return Err(invalid(
                    "initialized bytes and section size must fit the memory region",
                ));
            }
            if let Some(address) = section.fixed_address {
                let address = u32::from(address);
                if !range.contains(&address)
                    || address + section.size > range.end
                    || address % section.alignment != 0
                {
                    return Err(invalid(
                        "fixed address must satisfy the region, size and alignment",
                    ));
                }
            }
        }

        let mut external_names = HashSet::new();
        for (index, symbol) in self.symbols.iter().enumerate() {
            let invalid = |reason| Error::Symbol {
                symbol: index,
                reason,
            };
            if symbol.name.is_empty() || symbol.name.contains('\0') {
                return Err(invalid("name must be nonempty and contain no NUL"));
            }
            if symbol.binding != Binding::Local && !external_names.insert(&symbol.name) {
                return Err(invalid(
                    "external symbol names must be unique within an object",
                ));
            }
            match symbol.definition {
                Definition::Undefined if symbol.binding == Binding::Local => {
                    return Err(invalid("undefined symbols require global or weak binding"));
                }
                Definition::Section { section, offset } => {
                    let section = self
                        .sections
                        .get(section.0 as usize)
                        .ok_or_else(|| invalid("section does not exist"))?;
                    if offset > section.size {
                        return Err(invalid("offset exceeds section size"));
                    }
                }
                Definition::Undefined | Definition::Absolute { .. } => {}
            }
        }

        let mut patches = vec![Vec::new(); self.sections.len()];
        for (index, relocation) in self.relocations.iter().enumerate() {
            let invalid = |reason| Error::Relocation {
                relocation: index,
                reason,
            };
            let section = self
                .sections
                .get(relocation.section.0 as usize)
                .ok_or_else(|| invalid("section does not exist"))?;
            if self.symbols.get(relocation.symbol.0 as usize).is_none() {
                return Err(invalid("symbol does not exist"));
            }
            let start = u64::from(relocation.offset);
            let end = start + u64::from(relocation.kind.width());
            if end > section.data.len() as u64 {
                return Err(invalid("patch must fit within initialized bytes"));
            }
            patches[relocation.section.0 as usize].push((start, end, index));
        }
        for patches in &mut patches {
            patches.sort_unstable();
            for pair in patches.windows(2) {
                if pair[1].0 < pair[0].1 {
                    return Err(Error::Relocation {
                        relocation: pair[1].2,
                        reason: "patch overlaps another relocation",
                    });
                }
            }
        }
        Ok(())
    }
}
