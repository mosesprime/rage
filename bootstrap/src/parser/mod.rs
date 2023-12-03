//! Rage Bootstrap Parser


use crate::{token::{Token, TokenKind}, symbol::{SymbolTable}};

use self::scope::Block;

const DEFAULT_LOOKBACK_CAPACITY: usize = 10;
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

    pub fn run(mut self, mut token_iter: impl Iterator<Item = Token>) -> SymbolTable<'a> {
        let blocks = Block::scan(&token_iter.collect());
        blocks.iter().for_each(|b| println!("{b:?}"));
        self.symbol_table
    }
}
