//! Rage Bootstrap Builder Unit

use std::{fs, io, path::PathBuf};

use crate::{errors::CompError, lexer::Lexer, logging::LogLevel, parser::Parser, token::Token};

#[derive(Default)]
enum BuilderPhase {
    #[default]
    Idle,
}

/// Single compilation worker.
#[derive(Default)]
pub struct Builder {
    phase: BuilderPhase,
    path_buf: PathBuf,
    error_buf: Vec<CompError>,
    source: String,
    tokens: Vec<Token>,
}

impl Builder {
    pub fn source(&mut self, path_buf: PathBuf) -> io::Result<()> {
        self.source = fs::read_to_string(&path_buf)?;
        Ok(())
    }

    pub fn run(&mut self) {
        LogLevel::Debug.println("running builder");
        let lexer = Lexer::new(self.source.as_str());
        self.tokens = lexer.scan().unwrap();
        self.tokens.iter().for_each(|t| println!("{t:?}"));
        let parser = Parser::new();
        parser.run(&self.tokens);
    }
}
