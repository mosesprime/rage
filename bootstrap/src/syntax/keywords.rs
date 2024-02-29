//! Rage Bootstrap Keywords

#[derive(Debug, Clone, Copy)]
pub enum KeywordKind {
    Pub,
    Mod,
    Use,
    Mut,
    Defer,
    Return,
    Inline,

    /// Likely not a keyword.
    UNKNOWN,
}

impl<'a> Into<&'a str> for KeywordKind {
    fn into(self) -> &'a str {
        match self {
            Self::Pub => "pub",
            Self::Mod => "mod",
            Self::Use => "use",
            Self::Mut => "mut",
            Self::Defer => "defer",
            Self::Return => "return",
            Self::Inline => "inline",
            Self::UNKNOWN => unreachable!(),
        }
    }
}

impl From<&str> for KeywordKind {
    fn from(value: &str) -> Self {
        match value {
            "pub" => Self::Pub,
            "mod" => Self::Mod,
            "use" => Self::Use,
            "mut" => Self::Mut,
            "defer" => Self::Defer,
            "return" => Self::Return,
            "inline" => Self::Inline,
            _ => Self::UNKNOWN,
        }
    }
}
