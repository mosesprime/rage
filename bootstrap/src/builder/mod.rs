//! Rage Bootstrap Builder Unit

use std::{sync::{Arc, Mutex}, path::PathBuf, error::Error, fs};

use crate::{errors::{ErrorManifest, CompError, CompErrorLevel}, token::Token, lexer::Lexer, parser::Parser};

/// Single compilation worker.
pub struct Builder {
    error_manifest: Arc<Mutex<ErrorManifest>>,
    path: PathBuf,
    source: String,
    tokens: Vec<Token>,
}

impl Builder {
    pub fn new(error_manifest: Arc<Mutex<ErrorManifest>>, path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let source = fs::read_to_string(&path)?;
        Ok(Self { 
            error_manifest, 
            path,
            source,
            tokens: Default::default(),
        })  
    }

    pub fn run(&mut self) {
        log::debug!("starting builder on {}", self.path.display());
        self.error_manifest.lock().unwrap().push(CompError::new(CompErrorLevel::Error, self.path.clone(), 0, 0, "test error".to_string()));
        self.tokens = Lexer::tokenize(self.source.clone());
        self.tokens.iter().for_each(|t| println!("{t:?}"));
        let parser = Parser::new();
        parser.run(&self.tokens);
    }
}
