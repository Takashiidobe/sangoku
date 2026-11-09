use super::Object;

pub(super) const OBJECTS: &[Object] = &[
    object!("memcmp_size", "runtime/memory/memcmp/size/pointer.asm"),
    object!("memcmp_opt", "runtime/memory/memcmp/speed/pointer.asm"),
    object!("memset_size", "runtime/memory/memset/size/pointer.asm"),
    object!("memset_opt", "runtime/memory/memset/speed/pointer.asm"),
    object!("memmove_size", "runtime/memory/memmove/size/pointer.asm"),
    object!("memmove_opt", "runtime/memory/memmove/speed/pointer.asm"),
    object!("memcpy_size", "runtime/memory/memcpy/size/pointer.asm"),
    object!("memcpy_opt", "runtime/memory/memcpy/speed/pointer.asm"),
];
