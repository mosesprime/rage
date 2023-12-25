//! Rage Bootstrap Builder Unit

use std::{fs, io, path::PathBuf};

use crate::{errors::CompError, lexer::Lexer, parser::Parser, token::Token};

/// Single compilation worker.
#[derive(Default)]
pub struct Builder {
    path: PathBuf,
    errors: Vec<CompError>,
    source: String,
}

impl Builder {
    pub fn source(&mut self, path: PathBuf) -> io::Result<()> {
        self.path = path.clone();
        self.source = fs::read_to_string(path)?;
        Ok(())
    }

    pub fn run(&mut self) {
        log::debug!("starting builder for {}", self.path.display());
        let lexer = Lexer::new(self.source.as_str());
        let parser = Parser::new();
        parser.run(lexer.tokenize());
    }
}
