//! Rage Bootstrap
//! keywork token kind

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Keyword {
    If,
    Mut,
    Return,

    UNKNOWN,
}

impl Keyword {
    pub fn match_keyword(str: &str) -> Option<Self> {
        let k = match str {
            "if" => Keyword::If,
            "mut" => Keyword::Mut,
            "return" => Keyword::Return,
            _ => Keyword::UNKNOWN,
        };
        if k == Keyword::UNKNOWN {
            return None;
        }
        Some(k)
    }
}
