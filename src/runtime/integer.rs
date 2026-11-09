use super::Object;

pub(super) const OBJECTS: &[Object] = &[
    object!("udiv16", "runtime/integer/udiv/common/i16.asm"),
    object!("urem16", "runtime/integer/urem/common/i16.asm"),
    object!("div16_core", "runtime/integer/div/common/i16_core.asm"),
    object!("mul8_size", "runtime/integer/mul/size/i8.asm"),
    object!("mul8_opt", "runtime/integer/mul/speed/i8.asm"),
    object!("mul16_size", "runtime/integer/mul/size/i16.asm"),
    object!("mul16_opt", "runtime/integer/mul/speed/i16.asm"),
    object!("mul32_size", "runtime/integer/mul/size/i32.asm"),
    object!("mul32_opt", "runtime/integer/mul/speed/i32.asm"),
    object!("div32_core", "runtime/integer/div/common/i32_core.asm"),
    object!("udiv32", "runtime/integer/udiv/common/i32.asm"),
    object!("urem32", "runtime/integer/urem/common/i32.asm"),
    object!("sdiv32", "runtime/integer/sdiv/common/i32.asm"),
    object!("srem32", "runtime/integer/srem/common/i32.asm"),
    object!("mulo128", "runtime/integer/mulo/common/i128.asm"),
];
