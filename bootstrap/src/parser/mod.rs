//! Rage Bootstrap
//! Parser

use anyhow::Ok;

use crate::syntax::{lexeme::{Lexeme, LexemeKind}, Statement};

use self::{scanner::Scanner, tree::ParseTree};

mod scanner;
pub mod span;
pub mod tree;

pub trait Parse: Sized {
    fn parse(parser: &mut Parser<'_>) -> Result<Self, anyhow::Error>;
}

pub struct Parser<'a> {
    cursor: usize,
    buffer: Vec<(Lexeme, usize)>,
    content: &'a str,
    lexemes: std::iter::Peekable<Scanner<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a str) -> Self {
        Self { 
            cursor: 0,
            buffer: Vec::default(),
            content,
            lexemes: Scanner::new(content).peekable(),
        }  
    }

    pub fn start(mut self) -> anyhow::Result<ParseTree> {
        let mut tree = ParseTree::new();
        while let Some(first) = self.peek_lexeme() {
            match first.kind {
                LexemeKind::Whitespace(_) | LexemeKind::Comment(_) => {
                    self.consume_lexeme(); // should contain some already peeked lexeme
                },
                _ => { 
                    tree.add(Statement::parse(&mut self)?);
                },
            }
        }
        Ok(tree)
    }

    pub fn get_value(&self, start: usize, end: usize) -> Option<&str> {
        self.content.get(start..end)
    }

    // 
    pub fn consume_lexeme(&mut self) -> Option<Lexeme> {
        if let Some(lexeme) = self.lexemes.next() {
            self.cursor += lexeme.count();
            return Some(lexeme);
        }
        None
    }

    pub fn peek_lexeme(&mut self) -> Option<&Lexeme> {
        self.lexemes.peek()
    }

    /// Returns the next non-whitespace & non-comment [Lexeme] if able.
    /// Use consume_lexeme() if you want these tokens to be included in iteration.
    pub fn next_lexeme(&mut self) -> Option<Lexeme> {
        if let Some(lexeme) = self.consume_lexeme() {
            return match lexeme.kind {
                LexemeKind::Whitespace(_) | LexemeKind::Comment(_) => self.next_lexeme(),
                _ => Some(lexeme),
            }
        }
        None
    }

    /// Returns the char cursor index.
    pub fn get_cursor(&self) -> usize {
        self.cursor
    }

    pub fn push_buffer(&mut self, lexeme: Lexeme, start_index: usize) {
        self.buffer.push((lexeme, start_index))
    }

    pub fn clear_buffer(&mut self) {
        self.buffer.clear()
    }

    pub fn get_buffer(&self) -> &'_ [(Lexeme, usize)] {
        self.buffer.as_slice()
    }
}

