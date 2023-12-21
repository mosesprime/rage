//! Rage Bootstrap Lexer

pub mod errors;
mod tokenizer;

use crate::token::Token;
use tokenizer::Tokenizer;

/// Compilation unit that tokenizes the source code.
pub struct Lexer<'a> {
    source: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &str) -> Self {
        Self { source }
    }

    pub fn tokenize(&mut self) -> impl Iterator<Item = Token> + '_ {
        Tokenizer::new(self.source.chars())
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
