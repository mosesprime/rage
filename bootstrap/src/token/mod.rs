//! Rage Bootstrap Token

use self::{symbol::Symbol, keyword::Keyword};

pub mod keyword;
pub mod symbol;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub length: usize,
}

impl Token {
    pub fn new(kind: TokenKind, length: usize) -> Self {
        Self { kind, length }
    }
}

#[derive(Debug, PartialEq)]
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

    /// Unknown. Likely an error.
    UNKNOWN,
}

#[derive(Debug, PartialEq)]
pub enum Whitespace {
    Blank,
    NewLine,
}

#[derive(Debug, PartialEq)]
pub enum Comment {
    Line,
    Block,
    Document,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    /// ie. "The qucik brown fox jumps over the lazy dog."
    String,
    /// ie. 1_000, -32, 3.14, 0_u8, 3.7E-7
    Numeric,
    /// ie. 0b01010101
    Binanry,
    /// ie. 'a', '\U+00B5'
    Char,
    /// ie. 0x55AA, 0x000f
    Hex,
}

