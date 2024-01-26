//! Rage Bootstrap
//! Lexeme

use crate::syntax::token::{CommentKind, LiteralKind};

#[derive(Debug)]
pub struct Lexeme {
    pub kind: LexemeKind,
    /// Number of [char]s in lexeme.
    pub length: u32,
}

impl Lexeme {
    pub fn new(kind: LexemeKind, length: u32) -> Self {
        Self {
            kind,
            length,
        }
    }
}

#[derive(Debug)]
pub enum WhitespaceKind {
    Blank,
    NewLine,
}

#[derive(Debug)]
pub enum LexemeKind {
    Whitespace(WhitespaceKind),
    Comment(CommentKind),
    Literal(LiteralKind),
    /// my_variable, MyColor
    Term,
    /// !
    Exclamation,
    /// "
    Quotation,
    /// #
    Number,
    /// $
    Dollar,
    /// %
    Percent,
    /// &
    Ampersand,
    /// '
    Apostrophe,
    /// (
    LParen,
    /// )
    RParen,
    /// *
    Asterisk,
    /// +
    Plus,
    /// ,
    Comma,
    /// -
    Hyphen,
    /// .
    Dot,
    /// /
    Slash,
    /// :
    Colon,
    /// ;
    Semicolon,
    /// <
    Lesser,
    /// =
    Equal,
    /// >
    Greater,
    /// ?
    Question,
    /// @
    At,
    /// [
    LSquare,
    /// \
    Backslash,
    /// ]
    RSquare,
    /// ^
    Caret,
    /// _
    Underscore,
    /// `
    Accent,
    /// {
    LCurly,
    /// |
    Pipe,
    /// }
    RCurly,
    /// ~
    Tilde,
    
    /// Unknown. Likely an error.
    UNKNOWN,
}
