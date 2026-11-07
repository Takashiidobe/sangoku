use std::{io, process::ExitStatus};

use miette::Diagnostic;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("bundled toolchain: {0}")]
    #[diagnostic(code(sangoku::bundle))]
    Bundle(String),
    #[error("system clock error")]
    #[diagnostic(code(sangoku::clock))]
    Clock(#[source] std::time::SystemTimeError),
    #[error("{0}")]
    #[diagnostic(code(sangoku::build))]
    Message(String),
    #[error("{context}")]
    #[diagnostic(code(sangoku::io))]
    Io {
        context: String,
        #[source]
        source: io::Error,
    },
    #[error("{command} exited with {status}")]
    #[diagnostic(code(sangoku::command))]
    CommandFailed { command: String, status: ExitStatus },
    #[error(
        "ROM needs {used} bytes, exceeding the {capacity}-byte cartridge capacity by {excess} bytes"
    )]
    #[diagnostic(
        code(sangoku::rom_capacity),
        help(
            "Largest retained sections:\n{largest}\nInspect {artifacts}/probe.map for all section sizes. Reduce retained code to fit the cartridge."
        )
    )]
    RomCapacity {
        used: usize,
        capacity: usize,
        excess: usize,
        artifacts: String,
        largest: String,
    },
    #[error(transparent)]
    #[diagnostic(transparent)]
    Emulator(#[from] crate::emu::Error),
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Self::Message(message)
    }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Self::Message(message.into())
    }
}

impl Error {
    pub fn io(context: impl Into<String>, source: io::Error) -> Self {
        Self::Io {
            context: context.into(),
            source,
        }
    }
}
