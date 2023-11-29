//! Rage Bootstrap Parser

use crate::token::Token;

/// Parsing compilation unit.
/// Takes tokens from the lexer, analyzes and parses them into an abstract syntax tree.
pub struct Parser {
    /// Lookback buffer of previous tokens.
    lookback: Vec<Token>,
}

impl Parser {
    pub fn new() -> Self {
        Self { 
            lookback: Default::default()
        }
    }   

    pub fn feed(&mut self, token: Token) {
        todo!();
        self.lookback.push(token);
    }
}
