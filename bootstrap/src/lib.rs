//! Rage Bootstrap

use std::sync::{Arc, Mutex};

pub mod errors;
pub mod lexer;
pub mod parser;
pub mod token;

const DEFAULT_TABLE_CAPACITY: usize = 1000;

pub struct SymbolManifestEntry {
    name: String,
    kind: Kind,
    size: usize
}

/// Stores [`SymbolManifestEntry`] as a structure of vectors.
/// Should have greater performance than vector of structures.
pub struct SymbolManifest {
    name: Vec<String>,
    kind: Vec<Kind>,
    size: Vec<usize>,
}

impl Default for SymbolManifest {
    fn default() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self { 
            name: Vec::with_capacity(DEFAULT_TABLE_CAPACITY),
            kind: Vec::with_capacity(DEFAULT_TABLE_CAPACITY),
            size: Vec::with_capacity(DEFAULT_TABLE_CAPACITY),
        }))
    }
}

impl SymbolManifest {
    pub fn add_entry(&mut self, entry: SymbolManifestEntry) {
        self.name.push(entry.name);
        self.kind.push(entry.kind);
        self.size.push(entry.size);
    }

    pub fn get_entry(&self, index: usize) -> Option<SymbolManifestEntry> {
        let name = self.name.get(index)?;
        let kind = self.kind.get(index)?;
        let size = self.size.get(index)?;
        Some(SymbolManifestEntry { name, kind, size })
    }

    pub fn get_entries(&self) -> impl Iterator<Item = SymbolManifestEntry> {
        let names = self.name.iter();
        let kinds = self.kind.iter();
        let sizes = self.size.iter();
        std::iter::from_fn(move || {
            if let Some(name) = names.next() {
                if let Some(kind) = kinds.next() {
                    if let Some(size) = sizes.next() {
                        Some(SymbolManifestEntry { name, kind, size })
                    }
                    None    
                }
                None
            }
            None
        })
    }

    pub fn shrink_to_fit(&mut self) {
        self.name.shrink_to_fit();
        self.kind.shrink_to_fit();
        self.size.shrink_to_fit();
    }
}
