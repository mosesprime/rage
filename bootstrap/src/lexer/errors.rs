use std::{error::Error, fmt::Display};

#[derive(Debug, Copy)]
pub enum LexicalErrorKind {
    UnknownToken,
}

#[derive(Debug, Copy)]
pub struct LexicalError {
    kind: LexicalErrorKind,
    index: usize,
    length: usize,
    context: String,
}

impl LexicalError {
    pub fn new(
        kind: LexicalErrorKind,
        index: usize,
        length: usize,
        context: impl ToString,
    ) -> Self {
        Self {
            kind,
            index,
            length,
            context: context.to_string(),
        }
    }
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self.kind {
            LexicalErrorKind::UnknownToken => "Unknown Token",
        };
        write!(f, "{} {}", kind, self.context)
    }
}

impl Error for LexicalError {}
