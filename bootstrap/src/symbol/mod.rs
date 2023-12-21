//! Rage Bootstrap
//! Symbols

use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, Mutex},
};

const DEFAULT_TABLE_CAPACITY: usize = 1_000;

/// The type of the [`Symbol`]
#[derive(Clone, Copy, Debug)]
pub enum SymbolKind {
    /// Boolean primative. true, false
    Bool,
    /// Unicode character primative. 'a', '\n', ...
    Char,
    /// Memory blob primative. b32, b128, ...
    Blob(usize),
    /// Signed integer primative. i16, i32, isize, ...
    Int(usize),
    /// Unsigned integer primative. u8, u64, usize, ...
    UInt(usize),
    /// Floating point primative. f32, f64, ...
    Float(usize),
    /// Algebraic data types. Structures, enumerations, ...
    Adt,
    /// Function
    Fn,
    /// Tuple. ie. (usize, T, f32), (bool, i32), ...
    Tuple,
    /// Slice of data. [T]
    Slice,
    /// Pointer *
    Ptr,
    /// Reference &
    Ref,
    /// Aliased type. Can be recursive.
    Alias,
    /// FFI type.
    Forign,
    /// Infered type. Should not be recursive.
    Infer,

    /// Type can not be determined.
    UNKNOWN,
}

/// A single entry in the [`SymbolTable`].
#[derive(Debug)]
pub struct Symbol<'a> {
    /// Name used in source code for the symbol.
    name: &'a str,
    /// Type
    kind: SymbolKind,
    /// Size of element in bytes.
    size: usize,
    /// Number of elements, aka dimentions.
    width: usize,
}

impl<'a> Symbol<'a> {
    pub fn new(name: &'a str, kind: SymbolKind, size: usize, width: usize) -> Self {
        Self {
            name,
            kind,
            size,
            width,
        }
    }
}

pub struct SymbolManifest<'a> {
    table_map: HashMap<&'a str, Box<SymbolTable<'a>>>,
}

impl<'a> SymbolManifest<'a> {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            table_map: Default::default(),
        }))
    }

    pub fn add_module(&mut self, module_name: &'a str, module_symbol_table: SymbolTable<'a>) {
        self.table_map
            .insert(module_name, Box::new(module_symbol_table));
    }
}

/// Stores [`Symbol`] as a structure of vectors.
/// Should have greater performance than vector of structures.
pub struct SymbolTable<'a> {
    name: Vec<&'a str>,
    kind: Vec<SymbolKind>,
    size: Vec<usize>,
    width: Vec<usize>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        Self {
            name: Vec::with_capacity(DEFAULT_TABLE_CAPACITY),
            kind: Vec::with_capacity(DEFAULT_TABLE_CAPACITY),
            size: Vec::with_capacity(DEFAULT_TABLE_CAPACITY),
            width: Vec::with_capacity(DEFAULT_TABLE_CAPACITY),
        }
    }

    pub fn add_symbol(&mut self, entry: Symbol<'a>) {
        self.name.push(entry.name);
        self.kind.push(entry.kind);
        self.size.push(entry.size);
        self.width.push(entry.width);
    }

    pub fn get_symbol(&self, index: usize) -> Option<Symbol> {
        let name = *self.name.get(index)?;
        let kind = *self.kind.get(index)?;
        let size = *self.size.get(index)?;
        let width = *self.width.get(index)?;
        Some(Symbol {
            name,
            kind,
            size,
            width,
        })
    }

    pub fn symbol_iter(&self) -> impl Iterator<Item = Symbol> + '_ {
        let mut names = self.name.iter();
        let mut kinds = self.kind.iter();
        let mut sizes = self.size.iter();
        let mut widths = self.width.iter();
        std::iter::from_fn(move || {
            Some(Symbol {
                name: *names.next()?,
                kind: *kinds.next()?,
                size: *sizes.next()?,
                width: *widths.next()?,
            })
        })
    }

    pub fn shrink_to_fit(&mut self) {
        self.name.shrink_to_fit();
        self.kind.shrink_to_fit();
        self.size.shrink_to_fit();
        self.width.shrink_to_fit();
    }
}
