//! Rage Bootstrap Error Handler

use std::{sync::{Arc, Mutex}, path::PathBuf, fmt::Display};

use crate::logging::LogLevel;

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
            CompErrorLevel::Warn => {
                warning_counter += 1;
                LogLevel::Warn.println(format!("{e}"));
            },
            CompErrorLevel::Error => {
                error_counter += 1;
                LogLevel::Error.println(format!("{e}"))
            },
            _ => {},
        });
        (error_counter, warning_counter)
    }

    /// Prints out the collection of errors & warnings to the stdout.
    pub fn print(&self) {
        self.errors.iter().for_each(|e| { 
            let log_level = match e.level {
                CompErrorLevel::Warn => LogLevel::Warn,
                CompErrorLevel::Error => LogLevel::Error,
                _ => unreachable!(),
            };
            log_level.println(e); 
        });
    }

    /// Add a new error to report. Panics if [`CompErrorLevel::Panic`].
    pub fn push(&mut self, error: CompError) {
        if error.level == CompErrorLevel::Panic {
            LogLevel::Panic.println(error);
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
        write!(f, "{}:{}-{} {}", self.file_path.to_string_lossy(), self.position, self.position + self.length, self.reason)
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


