use super::Object;

pub(super) const OBJECTS: &[Object] = &[
    object!("popcnt8_size", "runtime/bitshift/popcnt/size/i8.asm"),
    object!("popcnt8_opt", "runtime/bitshift/popcnt/speed/i8.asm"),
    object!("popcnt16_size", "runtime/bitshift/popcnt/size/i16.asm"),
    object!("popcnt16_opt", "runtime/bitshift/popcnt/speed/i16.asm"),
    object!("popcnt_byte", "runtime/bitshift/popcnt/common/byte.asm"),
    object!("popcnt32_size", "runtime/bitshift/popcnt/size/i32.asm"),
    object!("popcnt32_opt", "runtime/bitshift/popcnt/speed/i32.asm"),
    object!("ctz32", "runtime/bitshift/ctz/common/i32.asm"),
    object!("clz32", "runtime/bitshift/clz/common/i32.asm"),
    object!("bitrev32", "runtime/bitshift/bitrev/common/i32.asm"),
    object!("shift8", "runtime/bitshift/shift/common/i8.asm"),
    object!("shift16", "runtime/bitshift/shift/common/i16.asm"),
];
