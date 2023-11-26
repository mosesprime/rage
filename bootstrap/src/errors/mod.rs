//! Rage Bootstrap Error Handler

use std::{sync::mpsc::Receiver, path::PathBuf, fmt::Display};

/// List of all errors produced during compilation.
pub struct ErrorManifest {
    errors: Vec<CompError>,
}

impl Default for ErrorManifest {
    fn default() -> Self {
        Self { errors: Default::default() }
    }
}

impl ErrorManifest {
    pub fn handle(&mut self, rx: Receiver<CompError>) {
        for err in rx {
            self.errors.push(err)
        }
    } 

    pub fn report(self) -> Vec<CompError> {
        self.errors
    }
}

///
pub enum CompErrorLevel {
    Warn,
    Error,
}

impl Display for CompErrorLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CompErrorLevel::Warn => "WARN",
            CompErrorLevel::Error => "ERROR",
        };
        write!(f, "{}", s)
    }
}

/// Compilation error.
pub struct CompError {
    level: CompErrorLevel,
    file_path: PathBuf,
    position: usize,
    length: usize,
    reason: String,
}

impl Display for CompError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}:{}-{} {}", self.level, self.file_path.to_string_lossy(), self.position, self.position + self.length, self.reason)
    }
}

impl CompError {
    pub fn new(level: CompErrorLevel, file_path: PathBuf, position: usize, length: usize, reason: String) -> Self {
        Self {
            level,
            file_path,
            position,
            length,
            reason,
        }
    }
}
