//! Rage Bootstrap Parser


use crate::{token::Token, symbol::{Symbol, SymbolKind, SymbolTable}};

const DEFAULT_LOOKBACK_CAPACITY: usize = 1_000;

/// Parsing compilation unit.
/// Takes tokens from the lexer, analyzes and parses them into an abstract syntax tree.
pub struct Parser<'a> {
    /// Lookback buffer. Contains index of tokens.
    lookback: Vec<usize>,
    ///
    symbol_table: SymbolTable<'a>,
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Self { 
            lookback: Vec::with_capacity(DEFAULT_LOOKBACK_CAPACITY),
            symbol_table: SymbolTable::new(),
        }
    }   

    pub fn run(mut self, mut token_iter: impl Iterator<Item = Token>) -> SymbolTable<'a> {
        let mut next_lookback_index = 0;
        while let Some(token) = token_iter.next() {
            self.lookback.push(next_lookback_index);
            self.symbol_table.add_symbol(Symbol::new("true", SymbolKind::Bool, 1, 0));
            println!("{token:?}");
            next_lookback_index += 1;
        }
        self.symbol_table.symbol_iter().for_each(|s| println!("{s:?}"));
        return self.symbol_table;
    }
}
