//! Rage Bootstrap Lexer

use std::{fs, path::PathBuf, str::Chars};

use crate::token::{Lexeme, LexemeKind, Token, TokenKind};

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
        Ok(Self { path, input })
    }

    /// Executes the [`Tokenizer`] and runs all analysis passes.
    /// Returns an iterator over the lexed tokens.
    pub fn run(&mut self) -> impl Iterator<Item = Token> + '_ {
        let mut cursor = 0;
        let path = self.path.clone();
        let mut tokenized = Tokenizer::new(self.input.chars());
        std::iter::from_fn(move || {
            // TODO: add analysis passes
            if let Some(token) = tokenized.next() {
                if token.kind == TokenKind::UNKNOWN {
                    // TODO: err_manifest.lock().unwrap().push(CompError::new(CompErrorLevel::Error, path.clone(), cursor, token.length, "unknown token".to_string()));
                }
                cursor += token.length;
                return Some(token);
            }
            return None;
        })
    }

    pub fn tokenize(input: String) -> Vec<Token> {
        Tokenizer::new(input.chars()).collect()
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

pub struct Scanner<'a> {
    path: PathBuf,
    input: &'a str,
    chars: std::iter::Peekable<Chars<'a>>,
    buffer: Vec<char>,
}

impl<'a> Scanner<'a> {
    pub fn new(path: PathBuf) -> std::io::Result<Self> {
        let source = fs::read_to_string(&path)?;
        Ok(Self {
            path,
            input: &source,
            chars: source.chars().peekable(),
            buffer: Default::default(),
        })
    }

    fn consume_next(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn consume_while(&mut self, mut prediate: impl FnMut(char) -> bool) -> &str {
        while self.chars.peek().is_some_and(|c| prediate(*c)) {
            self.buffer.push(self.chars.next().unwrap())
        }
        todo!()
    }

    fn peek_first(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn peek_second(&self) -> Option<char> {
        let mut iter = self.chars.clone();
        iter.next()?;
        iter.next()
    }

    fn handle_whitespace(&mut self) -> Lexeme {
        self.chars.take_while(|c| c.is_ascii_whitespace());
        Lexeme::new(
            LexemeKind::Whitespace {
                length: take.len() + 1,
            },
            buf.concat(take.deref()),
            take.len() + 1,
        )
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Lexeme<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.chars.peek()?;

        return match first {
            c if c.is_ascii_whitespace() => Some(self.handle_whitespace()),
            c if c.is_ascii_digit() => Some(self.handle_digit()),
            c if c.is_ascii_alphabetic() => Some(self.handle_alphabetic()),
            c if c.is_ascii_punctuation() => Some(self.handle_punctuation()),
            _ => Some(self.handle_unknown()),
        };
    }
}
