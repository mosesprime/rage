#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
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
}

