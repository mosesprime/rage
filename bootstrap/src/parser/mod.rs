//! Rage Bootstrap Parser

use crate::{symbol::SymbolTable, token::Token};

use self::scope::Block;

mod scope;

/// Parsing compilation unit.
/// Takes tokens from the lexer, analyzes and parses them into an abstract syntax tree.
pub struct Parser<'a> {
    /// List of symbols for this parser.
    symbol_table: SymbolTable<'a>,
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Self { 
            symbol_table: SymbolTable::new(),
        }
    }   

    pub fn run(self, tokens: &Vec<Token>) -> SymbolTable<'a> {
        let blocks = Block::scan(tokens);
        blocks.iter().for_each(|b| println!("{b:?}"));
        self.symbol_table
    }
}
