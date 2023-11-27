//! Rage Bootstrap Lexer

use std::{sync::mpsc::Sender, path::PathBuf, fs};

use crate::{token::{Token, TokenKind}, errors::{CompError, CompErrorLevel}};

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

    /// 
    pub fn run(&self, err_tx: Sender<CompError>) -> Result<(), Box<dyn std::error::Error>> {
        let lexemes = self.tokenize();
        let mut cursor = 0;
        for token in lexemes {
            if token.kind == TokenKind::UNKNOWN {
                let msg = format!("unknown token: {}", self.get_str(cursor, token.length).unwrap());
                err_tx.send(CompError::new(CompErrorLevel::Error, self.path.clone(), cursor, token.length, msg))?;
            }
            cursor += token.length;
        }
        Ok(())
    }

    /// Generate an [`Iterator`] over the input that produces [`Token`].
    pub fn tokenize(&self) -> impl Iterator<Item = Token> + '_ {
        Tokenizer::new(self.input.chars())
    }

    /// Gets a single [`char`] from the input if able.
    pub fn get_char(&self, position: usize) -> Option<char> {
        self.input.chars().nth(position)
    }

    /// Gets a slice of the input if able.
    pub fn get_str(&self, position: usize, length: usize) -> Option<&str> {
        self.input.get(position..(position + length))
    }

    /// Gets a single line of the input if able.
    pub fn get_line(&self, line_num: usize) -> Option<&str> {
        self.input.lines().nth(line_num)
    }
}
