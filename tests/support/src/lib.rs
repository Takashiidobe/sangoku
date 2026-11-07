pub mod cli;
pub mod compiler;
pub mod division;
pub use sangoku::emu;
pub mod harness;
pub mod popcount;
pub mod random;
pub mod runtime;
pub use anyhow::{Result, bail, ensure};
pub use serde_json::{Value, json};
mod asm;
pub fn number(v: &Value) -> u64 {
    v.as_u64().expect("unsigned report field")
}
pub use serde_json;
pub use tempfile;
pub mod shifts;
