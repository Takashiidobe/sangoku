use super::Object;

pub(super) const OBJECTS: &[Object] = &[
    object!("popcnt8_size", "runtime/bitshift/popcnt/size/i8.asm"),
    object!("popcnt8_opt", "runtime/bitshift/popcnt/speed/i8.asm"),
    object!("popcnt16_size", "runtime/bitshift/popcnt/size/i16.asm"),
    object!("popcnt16_opt", "runtime/bitshift/popcnt/speed/i16.asm"),
    object!("popcnt_byte", "runtime/bitshift/popcnt/common/byte.asm"),
    object!("popcnt32_size", "runtime/bitshift/popcnt/size/i32.asm"),
    object!("popcnt32_opt", "runtime/bitshift/popcnt/speed/i32.asm"),
    object!("clz8_size", "runtime/bitshift/clz/size/i8.asm"),
    object!("clz8_opt", "runtime/bitshift/clz/speed/i8.asm"),
    object!("clz16_size", "runtime/bitshift/clz/size/i16.asm"),
    object!("clz16_opt", "runtime/bitshift/clz/speed/i16.asm"),
    object!("clz32_size", "runtime/bitshift/clz/size/i32.asm"),
    object!("clz32_opt", "runtime/bitshift/clz/speed/i32.asm"),
    object!("ctz8_size", "runtime/bitshift/ctz/size/i8.asm"),
    object!("ctz8_opt", "runtime/bitshift/ctz/speed/i8.asm"),
    object!("ctz16_size", "runtime/bitshift/ctz/size/i16.asm"),
    object!("ctz16_opt", "runtime/bitshift/ctz/speed/i16.asm"),
    object!("ctz32_size", "runtime/bitshift/ctz/size/i32.asm"),
    object!("ctz32_opt", "runtime/bitshift/ctz/speed/i32.asm"),
    object!("bitrev8_size", "runtime/bitshift/bitrev/size/i8.asm"),
    object!("bitrev8_opt", "runtime/bitshift/bitrev/speed/i8.asm"),
    object!("bitrev16_size", "runtime/bitshift/bitrev/size/i16.asm"),
    object!("bitrev16_opt", "runtime/bitshift/bitrev/speed/i16.asm"),
    object!("bitrev32_size", "runtime/bitshift/bitrev/size/i32.asm"),
    object!("bitrev32_opt", "runtime/bitshift/bitrev/speed/i32.asm"),
    object!("clz_byte_opt", "runtime/bitshift/clz/speed/byte.asm"),
    object!("ctz_byte_opt", "runtime/bitshift/ctz/speed/byte.asm"),
    object!("bitrev_byte_size", "runtime/bitshift/bitrev/size/byte.asm"),
    object!("bitrev_byte_opt", "runtime/bitshift/bitrev/speed/byte.asm"),
    object!("shift8", "runtime/bitshift/shift/common/i8.asm"),
    object!("shift16", "runtime/bitshift/shift/common/i16.asm"),
    object!("shift64_size", "runtime/bitshift/wshift/size/i64.asm"),
    object!("shift64_opt", "runtime/bitshift/wshift/speed/i64.asm"),
    object!(
        "shift_bit64_size",
        "runtime/bitshift/wshift_bit/size/bit64.asm"
    ),
    object!(
        "shift_bit64_opt",
        "runtime/bitshift/wshift_bit/speed/bit64.asm"
    ),
    object!(
        "shift_byte64_opt",
        "runtime/bitshift/wshift_byte/speed/byte64.asm"
    ),
    object!("shift128_size", "runtime/bitshift/wshift/size/i128.asm"),
    object!("shift128_opt", "runtime/bitshift/wshift/speed/i128.asm"),
    object!(
        "shift_bit128_size",
        "runtime/bitshift/wshift_bit/size/bit128.asm"
    ),
    object!(
        "shift_bit128_opt",
        "runtime/bitshift/wshift_bit/speed/bit128.asm"
    ),
    object!(
        "shift_byte128_opt",
        "runtime/bitshift/wshift_byte/speed/byte128.asm"
    ),
];
