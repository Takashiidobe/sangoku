use miette::Diagnostic;
use thiserror::Error;

use crate::link::FormattingDependency;

#[derive(Debug, Error, Diagnostic)]
#[error("retained code includes formatting ({bytes} bytes in identified formatting sections)")]
#[diagnostic(
    code(sangoku::retained_formatting),
    severity(Warning),
    help("{details}")
)]
pub struct RetainedFormatting {
    bytes: usize,
    details: String,
}

impl RetainedFormatting {
    pub fn new(dependency: FormattingDependency, total: usize, artifacts: &str) -> Self {
        Self {
            bytes: dependency.bytes,
            details: format!(
                "Dependency path:\n{}\nRetained ROM sections: {total} bytes; cartridge capacity: 32768 bytes.\nPanic failure paths can retain formatting even with immediate abort. Inspect {artifacts}/probe.map for section sizes.",
                dependency.path.join("\n  -> ")
            ),
        }
    }
}
