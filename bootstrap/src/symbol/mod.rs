//! Rage Bootstrap

pub type SymbolIndex = usize;

#[derive(Debug)]
pub enum SymbolKind {
    Identifier,
    FnDecl
}

#[derive(Debug)]
pub struct Symbol<'a> {
    label: &'a str,
    kind: &'a SymbolKind,
    size: u16,
    width: u16,
}

pub struct SymbolStore<'a> {
    next_index: SymbolIndex,
    symbols: Vec<Symbol<'a>>,
}

impl Default for SymbolStore<'_> {
    fn default() -> Self {
        Self { next_index: 0, symbols: Vec::default() }
    }
}

impl<'a> SymbolStore<'a> {
    pub fn add_symbol(&mut self, symbol: Symbol<'a>) -> SymbolIndex {
        let index = self.next_index;
        self.next_index += 1;
        self.symbols.push(symbol);
        return index;
    }

    pub fn get_symbol(&self, index: SymbolIndex) -> Option<&Symbol> {
        self.symbols.get(index)
    }
}

trait SymbolTy {
    fn hello () -> usize;
}

