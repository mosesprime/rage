//! Rage Bootstrap Error Handler

use std::{sync::{Arc, Mutex}, path::PathBuf, fmt::Display};

use crate::TextColor;

/// List of all errors produced during compilation.
#[derive(Clone)]
pub struct ErrorManifest {
    errors: Vec<CompError>,
}

impl ErrorManifest {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self { errors: Default::default() }))
    }

    /// Reports the number of errors & warnings in a tupple.
    pub fn report(&self) -> (usize, usize) {
        let mut error_counter = 0;
        let mut warning_counter = 0;
        self.errors.iter().for_each(|e| match e.level {
            CompErrorLevel::Warn => warning_counter += 1,
            CompErrorLevel::Error => error_counter += 1,
            _ => {},
        });
        (error_counter, warning_counter)
    }

    pub fn print(&self) {
        self.errors.iter().for_each(|e| { println!("{e}"); });
    }

    pub fn push(&mut self, error: CompError) {
        if error.level == CompErrorLevel::Panic {
            println!("{error}");
            panic!();
        }
        self.errors.push(error)
    }
}

///
#[derive(Clone, PartialEq)]
pub enum CompErrorLevel {
    /// Needs fixed before release.
    Warn,
    /// Will not compile. Throws error at end of phase.
    Error,
    /// Will not compile. Throws error immediately.
    Panic,
}

impl Display for CompErrorLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CompErrorLevel::Warn => "WARN",
            CompErrorLevel::Error => "ERROR",
            CompErrorLevel::Panic => "PANIC",
        };
        write!(f, "{}", s)
    }
}

/// Compilation error.
#[derive(Clone)]
pub struct CompError {
    level: CompErrorLevel,
    file_path: PathBuf,
    position: usize,
    length: usize,
    reason: String,
}

impl Display for CompError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let colored_level = match self.level {
            CompErrorLevel::Warn => TextColor::wrap_text(self.level.to_string(), TextColor::Yellow),
            CompErrorLevel::Error => TextColor::wrap_text(self.level.to_string(), TextColor::Red),
            CompErrorLevel::Panic => TextColor::wrap_text(self.level.to_string(), TextColor::BrightRed),
        };
        write!(f, "[{}] {}:{}-{} {}", colored_level, self.file_path.to_string_lossy(), self.position, self.position + self.length, self.reason)
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


