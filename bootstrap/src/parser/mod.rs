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

    pub fn run(&mut self, token_iter: impl Iterator<Item = Token>) {
        token_iter.for_each(|t| { println!("{t:?}") })
    }
}
