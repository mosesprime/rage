//! Rage Bootstrap Lexer

use self::{
    errors::{LexicalError, LexicalErrorKind},
    lexeme::{Lexeme, LexemeKind},
    scanner::Scanner,
};

mod analyzer;
pub mod errors;
mod lexeme;
mod scanner;

/// Compilation unit that tokenizes the source code.
pub struct Lexer<'a> {
    source: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &str) -> Self {
        Self { source }
    }

    pub fn scan(&mut self) -> Result<impl Iterator<Item = Lexeme> + '_, Vec<LexicalError>> {
        let mut scanner = Scanner::new(self.source.chars());
        let mut errors: Vec<LexicalError> = vec![];
        let lexemes = std::iter::from_fn(move || {
            let lexeme = scanner.next()?;
            if lexeme.kind == LexemeKind::UNKNOWN {
                errors.push(LexicalError::new(
                    LexicalErrorKind::UnknownToken,
                    lexeme.index as usize,
                    lexeme.length as usize,
                    "",
                ));
            }
            Some(lexeme)
        });
        if errors.len() > 0 {
            return Err(errors);
        }
        Ok(lexemes)
    }

    /// Gets a slice of the input if able.
    pub fn get_value(&self, index: usize, length: usize) -> Option<&str> {
        self.source.get(index..(index + length))
    }

    /// Gets a single line of the input if able.
    pub fn get_line(&self, line_num: usize) -> Option<&str> {
        self.source.lines().nth(line_num)
    }
}
