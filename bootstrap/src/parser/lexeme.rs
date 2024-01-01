//! Rage Bootstrap
//! Lexeme

/// Pre-parsed lexical tokens.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Lexeme {
    pub kind: LexemeKind,
    /// Index of the [char] at the start of the lexeme.
    pub index: u32,
    /// Number of [char]s in lexeme.
    pub length: u16,
}

impl Lexeme {
    pub fn new(kind: LexemeKind, index: u32, length: u16) -> Self {
        Self {
            kind,
            index,
            length,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LexemeKind {
    /// ' '
    Space,
    /// \n
    NewLine,
    /// Inline comment ie '//'
    LineComment,
    /// Mulitline comment ie '//* */'
    BlockComment,
    /// Documentation comment ie '///'
    Documentation,
    /// "the quick brown fox jumps over the jazy dog"
    StringLiteral,
    /// 0, 7, 10000
    NumericLiteral,
    /// true, false
    BooleanLiteral,
    /// 'a'
    CharLiteral,
    /// my_variable, MyColor
    Identifier,
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
