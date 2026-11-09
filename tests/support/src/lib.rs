pub mod cli;
pub mod compiler;
pub mod division;
pub use sangoku::emu;
pub mod harness;
pub mod popcount;
pub mod random;
pub mod runtime;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        Box::<dyn std::error::Error + Send + Sync>::from(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => {
        return Err($crate::error!($($arg)*))
    };
}

#[macro_export]
macro_rules! ensure {
    ($condition:expr $(,)?) => {
        $crate::ensure!($condition, "assertion failed: {}", stringify!($condition))
    };
    ($condition:expr, $($arg:tt)*) => {
        if !$condition {
            $crate::bail!($($arg)*);
        }
    };
}
pub use serde_json::{Value, json};
mod asm;
pub fn number(v: &Value) -> u64 {
    v.as_u64().expect("unsigned report field")
}
pub use serde_json;
pub use tempfile;
pub mod optimization;
pub mod shifts;
