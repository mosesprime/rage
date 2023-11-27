//! Rage Bootstrap Lexer

use crate::token::Token;

use self::tokenizer::Tokenizer;

mod tokenizer;

/// Compilation unit that tokenizes the source code.
pub struct Lexer {}

impl Lexer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn tokenize(&self, input: &str) -> Vec<Token> {
        let tokenizer = Tokenizer::new(input.chars());
        return tokenizer.collect();
    }
}
