//! Rage Bootstrap

use std::path::PathBuf;

use blake3::Hash;

pub struct Source {
    pub path: PathBuf,
    pub hash: Option<Hash>,
    pub contents: Option<String>,
}
