//! Rage Bootstrap Parser

/// Parsing compilation unit.
/// Takes tokens from the lexer, analyzes and parses them into an abstract syntax tree.
pub struct Parser {
    /// Unparsed tokens.
    raw_tokens: Vec<Token>,
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Self {
        Self { 
            raw_tokens: input,
        }
    }   
}
