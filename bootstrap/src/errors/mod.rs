//! Rage Bootstrap Error Handler

use std::{fmt::Display, path::PathBuf};

use crate::lexer::errors::LexicalErrorKind;

pub enum CompErrorKind {
    LexicalError(LexicalErrorKind),
}

impl Display for CompErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

/// Compilation error.
pub struct CompError {
    kind: CompErrorKind,
    file_path: PathBuf,
    line_num: usize,
    char_pos: usize,
    reason: String,
}

impl Display for CompError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}:{}:{} {}",
            self.kind,
            self.file_path.display(),
            self.line_num,
            self.char_pos,
            self.reason
        )
    }
}

impl CompError {
    pub fn new(
        kind: CompErrorKind,
        file_path: PathBuf,
        line_num: usize,
        char_pos: usize,
        reason: impl ToString,
    ) -> Self {
        Self {
            kind,
            file_path,
            line_num,
            char_pos,
            reason: reason.to_string(),
        }
    }
}
