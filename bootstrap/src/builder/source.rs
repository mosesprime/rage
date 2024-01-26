//! Rage Bootstrap

use std::{io, path::PathBuf, time::SystemTime};

pub type SourceId = blake3::Hash;

#[derive(PartialEq)]
pub struct Source {
    id: SourceId,
    last_built: Option<SystemTime>,
    last_modified: Option<SystemTime>,
    source_path: PathBuf,
    cached_path: Option<PathBuf>,
    source_text: Option<String>,
}

impl Source {
    ///
    pub fn from_source(path: PathBuf) -> io::Result<Self> {
        let text = std::fs::read_to_string(path.clone())?;
        let hash = blake3::hash(text.as_bytes());
        Ok(Self {
            id: hash,
            last_built: None,
            last_modified: None,
            source_path: path,
            cached_path: None,
            source_text: Some(text),
        })
    }

    /// 
    pub fn from_cache(path: PathBuf) -> io::Result<Self> {
        todo!()
    }

    pub fn id(&self) -> &SourceId {
        &self.id
    }

    pub fn source_text(&self) -> Option<&str> {
        if let Some(text) = &self.source_text {
            return Some(text.as_str());
        }
        None
    }

    /// Drop the source text to free up memory.
    pub fn strip_source_text(&mut self) {
        self.source_text = None
    }
}
