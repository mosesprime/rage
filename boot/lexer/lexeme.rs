use std::{fmt, ops::Range};

use ecow::EcoString;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Span {
    pub start: u32,
    pub length: u32,
}

impl Span {
    pub fn new(start: u32, length: u32) -> Self {
        Self { start, length }
    }

    pub fn as_range(&self) -> Range<u32> {
        self.start..(self.start + self.length)
    }
}

/// A lexical token.
#[derive(Debug, PartialEq)]
pub enum Lexeme {
    /* Special */
    Label(EcoString),
    BlockComment(EcoString),
    DocComment(EcoString),
    InlineComment(EcoString),

    /* Literals */
    Integer(EcoString),
    Float(EcoString),
    Hex(EcoString),
    Binary(EcoString),
    String(EcoString),
    
    /* Punctuation & Operators */
    /// !
    Bang,
    /// "
    Quote, // TODO: clean unused lexemes
    /// #
    Pound,
    /// $
    Dollar,
    /// %
    Percent,
    /// &
    Ampersand,
    /// '
    Apostrophe,
    /// (
    LeftParenthesis,
    /// )
    RightParenthesis,
    /// *
    Asterisk,
    /// +
    Plus,
    /// ,
    Comma,
    /// -
    Hyphen,
    /// .
    Period,
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
    LeftSquare,
    /// \
    BackSlash,
    /// ]
    RightSquare,
    /// ^
    Caret,
    /// _
    Underscore,
    /// `
    Grave,
    /// {
    LeftBrace,
    /// |
    Pipe,
    /// }
    RightBrace,
    /// ~
    Tilde,
    /// !=
    NotEqual,
    /// ==
    EqualEqual,
    /// +=
    PlusEqual,
    /// -=
    MinusEqual,
    /// <<
    ShiftLeft,
    /// >>
    ShiftRight,
    /// ...
    Ellipsis,
    /// ..
    Range,

    /* Keywords */
    Const,
    Else,
    Fn,
    If,
    Match,
    Mut,
    Pub,
    Return,
    Type,
    Use,
}

impl fmt::Display for Lexeme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::Label(s)
            | Self::BlockComment(s)
            | Self::DocComment(s)
            | Self::InlineComment(s)
            | Self::Integer(s) 
            | Self::Hex(s) 
            | Self::Binary(s)
            | Self::Float(s) 
            | Self::String(s) => s.as_str(), 
            Self::Bang => "!",
            Self::Quote => "\"",
            Self::Pound => "#",
            Self::Dollar => "$",
            Self::Percent => "%",
            Self::Ampersand => "&",
            Self::Apostrophe => "'",
            Self::LeftParenthesis => "(",
            Self::RightParenthesis => ")",
            Self::Asterisk => "*",
            Self::Plus => "+",
            Self::Comma => ",",
            Self::Hyphen => "-",
            Self::Period => ".",
            Self::Slash => "/",
            Self::Colon => ":",
            Self::Semicolon => ";",
            Self::Lesser => "<",
            Self::Equal => "=",
            Self::Greater => ">",
            Self::Question => "?",
            Self::At => "@",
            Self::LeftSquare => "[",
            Self::BackSlash => "\\",
            Self::RightSquare => "]",
            Self::Caret => "^",
            Self::Underscore => "_",
            Self::Grave => "`",
            Self::LeftBrace => "{",
            Self::Pipe => "|",
            Self::RightBrace => "}",
            Self::Tilde => "~",
            Self::NotEqual => "!=",
            Self::EqualEqual => "==",
            Self::PlusEqual => "+=",
            Self::MinusEqual => "-=",
            Self::ShiftLeft => "<<",
            Self::ShiftRight => ">>",
            Self::Ellipsis => "...",
            Self::Range => "..",
            Self::Const => "const",
            Self::Else => "else",
            Self::Fn => "fn",
            Self::If => "if",
            Self::Match => "match",
            Self::Mut => "mut",
            Self::Pub => "pub",
            Self::Return => "return",
            Self::Type => "type",
            Self::Use => "use",
        };
        write!(f, "{}", msg)
    }
}

pub fn str_to_keyword(query: &str) -> Option<Lexeme> {
    match query {
        "const" => Some(Lexeme::Const),
        "else" => Some(Lexeme::Else),
        "fn" => Some(Lexeme::Fn),
        "if" => Some(Lexeme::If),
        "match" => Some(Lexeme::Match),
        "mut" => Some(Lexeme::Mut),
        "pub" => Some(Lexeme::Pub),
        "return" => Some(Lexeme::Return),
        "type" => Some(Lexeme::Type),
        "use" => Some(Lexeme::Use),
        _ => None,
    }
}
