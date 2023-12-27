//! Rage Bootstrap

const DEFAULT_TABLE_CAPACITY: usize = 1_000;

pub struct Symbol<'a> {
    name: &'a str,
    size: u16,
    width: u16,
}

pub struct SymbolTable<'a> {
    //next_index: usize,
    names: Vec<&'a str>,
    sizes: Vec<u16>,
    widths: Vec<u16>,
}

impl<'a> Default for SymbolTable<'a> {
    fn default() -> Self {
        Self {
            //next_index: 0,
            names: Vec::with_capacity(DEFAULT_TABLE_CAPACITY),
            sizes: Vec::with_capacity(DEFAULT_TABLE_CAPACITY),
            widths: Vec::with_capacity(DEFAULT_TABLE_CAPACITY),
        }
    }
}

impl<'a> SymbolTable<'a> {
    pub fn add_symbol(&mut self, symbol: Symbol<'a>) {
        if let Some(i) = self.find_symbol(&symbol.name) {
            todo!("add another reference to table");
            return;
        }
        //let index = self.next_index;
        //self.next_index += 1;
        if (self.names.capacity() - self.names.len()) < 5 {
            self.names.reserve(DEFAULT_TABLE_CAPACITY);
            self.sizes.reserve(DEFAULT_TABLE_CAPACITY);
            self.widths.reserve(DEFAULT_TABLE_CAPACITY);
        }
        self.names.push(symbol.name);
        self.sizes.push(symbol.size);
        self.widths.push(symbol.width);
    }

    pub fn find_symbol(&self, name: &str) -> Option<usize> {
        if let Ok(i) = self.names.binary_search_by_key(&name, |&s| s) {
            return Some(i);
        }
        return None;
    }

    pub fn get_symbol(&self, index: usize) -> Option<Symbol> {
        let name = *self.names.get(index)?;
        let size = *self.sizes.get(index)?;
        let width = *self.widths.get(index)?;
        Some(Symbol { name, size, width })
    }
}
