//! Rage Bootstrap Lexer

use crate::token::Token;

use self::tokenizer::Tokenizer;

mod tokenizer;

/// Compilation unit that tokenizes the source code.
pub struct Lexer<'a> {
    input: &'a str
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
        }
    }

    pub fn tokenize(&self) -> impl Iterator<Item = Token> + '_ {
        Tokenizer::new(self.input.chars())
    }
}
