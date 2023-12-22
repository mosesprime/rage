//! Rage Bootstrap
//! symbol token kind

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Symbol {
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

    /// &&
    And,
    /// ||
    Or,
    /// ==
    Equivalent,
    /// !=
    NotEquivalent,
    /// <<
    LeftShift,
    /// <<<
    LeftRotate,
    /// >>
    RightShift,
    /// >>>
    RightRotate,
    /// +=
    Incriment,
    /// -=
    Decriment,
    /// ..
    ExclusiveRange,
    /// ..=
    InclusiveRange,

    UNKNOWN,
}

impl Symbol {
    pub fn match_symbol(chars: &[char]) -> Option<Self> {
        let k = match chars {
            ['!'] => Symbol::Exclamation,
            ['"'] => Symbol::Quotation,
            ['#'] => Symbol::Number,
            ['$'] => Symbol::Dollar,
            ['%'] => Symbol::Percent,
            ['&'] => Symbol::Ampersand,
            ['\''] => Symbol::Apostrophe,
            ['('] => Symbol::LParen,
            [')'] => Symbol::RParen,
            ['*'] => Symbol::Asterisk,
            ['+'] => Symbol::Plus,
            [','] => Symbol::Comma,
            ['-'] => Symbol::Hyphen,
            ['.'] => Symbol::Dot,
            ['/'] => Symbol::Slash,
            [':'] => Symbol::Colon,
            [';'] => Symbol::Semicolon,
            ['<'] => Symbol::Lesser,
            ['='] => Symbol::Equal,
            ['>'] => Symbol::Greater,
            ['?'] => Symbol::Question,
            ['@'] => Symbol::At,
            ['['] => Symbol::LSquare,
            ['\\'] => Symbol::Backslash,
            [']'] => Symbol::RSquare,
            ['^'] => Symbol::Caret,
            ['_'] => Symbol::Underscore,
            ['`'] => Symbol::Accent,
            ['{'] => Symbol::LCurly,
            ['|'] => Symbol::Pipe,
            ['}'] => Symbol::RCurly,
            ['~'] => Symbol::Tilde,
            ['!', '='] => Symbol::NotEquivalent,
            ['&', '&'] => Symbol::And,
            ['+', '='] => Symbol::Incriment,
            ['-', '='] => Symbol::Decriment,
            ['.', '.'] => Symbol::ExclusiveRange,
            ['.', '.', '.'] => Symbol::InclusiveRange,
            ['<', '<'] => Symbol::LeftShift,
            ['<', '<', '<'] => Symbol::LeftRotate,
            ['>', '>'] => Symbol::RightShift,
            ['>', '>', '>'] => Symbol::RightRotate,
            ['=', '='] => Symbol::Equivalent,
            ['|', '|'] => Symbol::Or,
            _ => Symbol::UNKNOWN,
        };
        if k == Symbol::UNKNOWN {
            return None;
        }
        Some(k)
    }
}
