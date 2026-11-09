macro_rules! object {
    ($name:literal, $path:literal $(, $prelude:literal)?) => {
        Object {
            name: $name,
            path: $path,
            source: concat!(
                $(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $prelude)),)?
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path))
            ),
        }
    };
}

mod bitshift;
mod float;
mod integer;
mod memory;
mod system;

pub const BUILTIN_REPLACEMENTS: &[&str] = &["__rust_memcmp"];

pub struct Object {
    pub name: &'static str,
    pub path: &'static str,
    pub source: &'static str,
}

impl Object {
    pub fn supports(&self, opt_level: &str) -> bool {
        let size = matches!(opt_level, "s" | "z");
        (!self.path.contains("/size/") || size) && (!self.path.contains("/speed/") || !size)
    }
}

pub fn objects() -> impl Iterator<Item = &'static Object> {
    [
        integer::OBJECTS,
        bitshift::OBJECTS,
        float::OBJECTS,
        memory::OBJECTS,
        system::OBJECTS,
    ]
    .into_iter()
    .flatten()
}

pub fn object(name: &str) -> &'static Object {
    objects()
        .find(|object| object.name == name)
        .expect("registered runtime object")
}
