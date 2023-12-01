//! Rage Bootstrap Lexer

use std::{sync::{mpsc::Sender, Mutex, Arc}, path::PathBuf, fs};

use crate::{token::{Token, TokenKind}, errors::{CompError, CompErrorLevel, ErrorManifest}};

use self::tokenizer::Tokenizer;

mod tokenizer;

/// Compilation unit that tokenizes the source code.
pub struct Lexer {
    path: PathBuf,
    input: String,
}

impl Lexer {
    /// Generate a new [`Lexer`] and pre-load with source text.
    pub fn new(path: PathBuf) -> std::io::Result<Self> {
        let input = fs::read_to_string(&path)?;
        Ok(Self {
            path,
            input,
        })
    }

    /// Executes the [`Tokenizer`] and runs all analysis passes.
    /// Returns an iterator over the lexed tokens.
    pub fn run(&mut self, err_manifest: Arc<Mutex<ErrorManifest>>) -> impl Iterator<Item = Token> + '_ {
        let mut cursor = 0;
        let path = self.path.clone();
        let mut tokenized = Tokenizer::new(self.input.chars());
        std::iter::from_fn(move || {
            // TODO: add analysis passes
            if let Some(token) = tokenized.next() {
                if token.kind == TokenKind::UNKNOWN {
                    err_manifest.lock().unwrap().push(CompError::new(CompErrorLevel::Error, path.clone(), cursor, token.length, "unknown token".to_string()));
                }
                cursor += token.length;
                return Some(token);
            }
            return None;
        })
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
