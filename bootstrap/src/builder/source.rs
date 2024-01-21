//! Rage Bootstrap

use std::path::PathBuf;

use blake3::Hash;

#[derive(Debug, PartialEq)]
pub struct SourceRecord {
    path: PathBuf,
    hash: Hash,
}

impl SourceRecord {
    pub fn new(path: PathBuf, hash: Hash) -> Self {
        Self { path, hash }
    }
}

