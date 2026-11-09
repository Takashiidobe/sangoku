use super::Object;

pub(super) const OBJECTS: &[Object] = &[
    object!("popcnt32_size", "runtime/bitshift/popcnt/size/i32.asm"),
    object!("popcnt32_opt", "runtime/bitshift/popcnt/speed/i32.asm"),
    object!("ctz32", "runtime/bitshift/ctz/common/i32.asm"),
    object!("clz32", "runtime/bitshift/clz/common/i32.asm"),
    object!("bitrev32", "runtime/bitshift/bitrev/common/i32.asm"),
    object!("shift8", "runtime/bitshift/shift/common/i8.asm"),
    object!("shift16", "runtime/bitshift/shift/common/i16.asm"),
];
