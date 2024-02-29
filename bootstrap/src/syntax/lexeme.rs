//! Rage Bootstrap
//! Lexeme

use crate::common::Either;

use super::{keywords::KeywordKind, CommentKind, LiteralKind, WhitespaceKind};

/// A lexical token.
#[derive(Debug)]
pub struct Lexeme {
    pub kind: LexemeKind,
    /// Either a Box<str> or a length
    pub contents: Either<Box<str>, usize>,
}

impl Lexeme {
    /// Return a new Lexeme which contains a certain value.
    pub fn with_value(kind: LexemeKind, value: &str) -> Self {
        Self {
            kind,
            contents: Either::Left(value.into()), // PERF: IDK if this is performant
        }
    }

    /// Return a new Lexeme which is of a certain char length.
    pub fn with_length(kind: LexemeKind, length: usize) -> Self {
        Self { 
            kind,
            contents: Either::Right(length)
        }
    }

    /// Number of [char] in [Lexeme].
    pub fn count(&self) -> usize {
        match &self.contents {
            Either::Left(s) => s.chars().count(),
            Either::Right(l) => *l,
        }
    }

    pub fn value(&self) -> Option<&str> {
        match &self.contents {
            Either::Left(v) => Some(&v),
            Either::Right(_) => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LexemeKind {
    /// `my_func`, `MyStruct`
    Term,
    /// blank space, \n
    Whitespace(WhitespaceKind),
    /// `// hi mom`
    Comment(CommentKind),
    /// `5`
    Literal(LiteralKind),
    ///
    Keyword(KeywordKind),

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
    
    /// `==`
    EqualEqual,
    /// `!=`
    NotEqual,
    /// `>=`
    GreaterOrEqual,
    /// `<=`
    LesserOrEqual,

    /// `&&`
    AndAnd,
    /// `||`
    OrOr,

    /// `<<`
    LeftShift,
    /// `>>`
    RightShift,
    /// `<<<`
    LeftRotate,
    /// `>>>`
    RightRotate,

    /// `+=`
    PlusEqual,
    /// `-=`
    MinusEqual,
    /// `*=`
    MultiplyEqual,
    /// `/=`
    DivideEqual,
    /// `%=`
    ModuloEqual,
    /// `&=`
    AndEqual,
    /// `|=`
    OrEqual,
    /// `^=`
    XorEqual,
    /// `<<=`
    LeftShiftEqual,
    /// `>>=`
    RightShiftEqual,

    /// `++`
    Incriment,
    /// `--`
    Decriment,

    /// `..`
    ExclusiveRange,
    /// `..=`
    InclusiveRange,
    /// `...`
    Ellipsis,

    /// Unknown. Likely an error.
    UNKNOWN,
}
