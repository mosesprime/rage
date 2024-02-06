//! Rage Bootstrap

use std::{io, path::PathBuf};

use super::incrimental::Incrimental;

pub type SourceId = blake3::Hash;

#[derive(PartialEq)]
pub struct Source {
    unique_id: SourceId,
    full_path: PathBuf,
    pub incrimental: Incrimental,
}

impl Source {
    ///
    pub fn from_path(path: PathBuf) -> io::Result<Self> {
        let text = std::fs::read_to_string(path.clone())?;
        let hash = blake3::hash(text.as_bytes());
        Ok(Self {
            unique_id: hash,
            full_path: path.canonicalize()?,
            incrimental: Incrimental::RawText(text),
        })
    }

    pub fn id(&self) -> &SourceId {
        &self.unique_id
    }
}
