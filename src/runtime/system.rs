use super::Object;

pub(super) const OBJECTS: &[Object] = &[
    object!("panic", "runtime/panic.asm"),
    object!("heap", "runtime/heap.asm"),
];
