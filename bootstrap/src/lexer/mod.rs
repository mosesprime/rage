//! Rage Bootstrap Lexer

use std::{sync::{mpsc::Sender, Mutex, Arc}, path::PathBuf, fs};

use crate::{token::{Token, TokenKind}, errors::{CompError, CompErrorLevel, ErrorManifest}};

use self::tokenizer::Tokenizer;

mod tokenizer;

/// Compilation unit that tokenizes the source code.
pub struct Lexer {
    path: PathBuf,
    input: String,
    output: Vec<Token>,
}

impl Lexer {
    /// Generate a new [`Lexer`] and pre-load with source text.
    pub fn new(path: PathBuf) -> std::io::Result<Self> {
        let input = fs::read_to_string(&path)?;
        Ok(Self {
            path,
            input,
            output: Default::default(),
        })
    }

    /// 
    pub fn run(&mut self, err_manifest: Arc<Mutex<ErrorManifest>>) -> Result<(), Box<dyn std::error::Error>> {
        self.output = self.tokenize().collect();
        let mut cursor = 0;
        for token in &self.output {
            if token.kind == TokenKind::UNKNOWN {
                let msg = format!("unknown token: {}", self.get_value(cursor, token.length).unwrap()); // should not error as length should be valid during tokenize 
                err_manifest.lock().unwrap().push(CompError::new(CompErrorLevel::Error, self.path.clone(), cursor, token.length, msg));
            }
            cursor += token.length;
        }
        Ok(())
    }

    ///
    pub fn report(&self) -> &Vec<Token> {
        &self.output
    }

    /// Generate an [`Iterator`] over the input that produces [`Token`].
    pub fn tokenize(&self) -> impl Iterator<Item = Token> + '_ {
        Tokenizer::new(self.input.chars())
    }

    /// Gets a slice of the input if able.
    pub fn get_value(&self, position: usize, length: usize) -> Option<&str> {
        self.input.get(position..(position + length))
    }

    /// Gets a single line of the input if able.
    pub fn get_line(&self, line_num: usize) -> Option<&str> {
        self.input.lines().nth(line_num)
    }
}
