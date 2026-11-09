use super::Object;

pub(super) const OBJECTS: &[Object] = &[
    object!("udiv16", "runtime/integer/udiv/common/i16.asm"),
    object!("urem16", "runtime/integer/urem/common/i16.asm"),
    object!("div16_core", "runtime/integer/div/common/i16_core.asm"),
    object!("mul16", "runtime/integer/mul/common/i16.asm"),
    object!("mul32", "runtime/integer/mul/common/i32.asm"),
    object!("div32_core", "runtime/integer/div/common/i32_core.asm"),
    object!("udiv32", "runtime/integer/udiv/common/i32.asm"),
    object!("urem32", "runtime/integer/urem/common/i32.asm"),
    object!("sdiv32", "runtime/integer/sdiv/common/i32.asm"),
    object!("srem32", "runtime/integer/srem/common/i32.asm"),
    object!("mulo128", "runtime/integer/mulo/common/i128.asm"),
];
