//! Rage Bootstrap Error Handler

use std::{fmt::Display, path::PathBuf};

/// Compilation error.
#[derive(Clone)]
pub struct CompError {
    file_path: PathBuf,
    line_num: usize,
    char_pos: usize,
    reason: String,
}

impl Display for CompError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{} {}",
            self.file_path.display(),
            self.line_num,
            self.char_pos,
            self.reason
        )
    }
}

impl CompError {
    pub fn new(
        file_path: PathBuf,
        line_num: usize,
        char_pos: usize,
        reason: impl ToString,
    ) -> Self {
        Self {
            file_path,
            line_num,
            char_pos,
            reason: reason.to_string(),
        }
    }
}
