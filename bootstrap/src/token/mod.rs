//! Rage Bootstrap Token

use self::{symbol::Symbol, keyword::Keyword};

mod keyword;
mod symbol;

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    length: usize,
}

impl Token {
    pub fn new(kind: TokenKind, length: usize) -> Self {
        Self { kind, length }
    }
}

#[derive(Debug)]
pub enum TokenKind {
    /// 
    Whitespace(Whitespace),
    /// 
    Comment(Comment),
    ///
    Literal(Literal),
    ///
    Keyword(Keyword),
    ///
    Symbol(Symbol),
    ///
    Identifier,

    /// End of file.
    EOF,
    /// Unknown. Likely an error.
    UNKNOWN,
}

#[derive(Debug)]
pub enum Whitespace {
    Blank,
    NewLine,
}

#[derive(Debug)]
pub enum Comment {
    Line,
    Block,
    // Document,
}

#[derive(Debug)]
pub enum Literal {
    String,
    Numeric,
    Binanry,
    Char,
    Hex,
}

