//! Rage Bootstrap
//! Parser

use crate::syntax::lexeme::Lexeme;

use self::{scanner::Scanner, tree::ParseTree};

mod scanner;
pub mod tree;

pub trait Parse: Sized {
    fn parse(parser: &mut Parser<'_>) -> Result<Self, anyhow::Error>;
}

pub fn parse_file(content: &str) -> anyhow::Result<ParseTree> {
    let mut parser = Parser::new(content);
    ParseTree::parse(&mut parser)
}

pub struct Parser<'a> {
    start: usize,
    end: usize,
    content: &'a str,
    lexemes: std::iter::Peekable<Scanner<'a>>,
}

impl<'a> Parser<'a> {
    fn new(content: &'a str) -> Self {
        Self { 
            start: 0,
            end: 0,
            content,
            lexemes: Scanner::new(content).peekable(),
        }  
    }

    fn get_value(&self, start: usize, end: usize) -> Option<&str> {
        self.content.get(start..end)
    }

    pub fn next_lexeme(&mut self) -> Option<Lexeme> {
        self.lexemes.next()
    }

    pub fn peek_lexeme(&mut self) -> Option<&Lexeme> {
        self.lexemes.peek()
    }
}

