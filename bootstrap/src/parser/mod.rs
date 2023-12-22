//! Rage Bootstrap Parser

use crate::{log_debug, token::Token};

/// Parsing compilation unit.
/// Takes tokens from the lexer, analyzes and parses them into an abstract syntax tree.
pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(self, lexed: impl Iterator<Item = Token>) {
        //-> Result<(AST, SymbolTable), Vec<ParseError>> {
        lexed.for_each(|t| {
            log_debug!("{t:?}");
        });
    }
}
