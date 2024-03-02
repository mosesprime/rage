//! Rage Bootstrap
//! Common 

use anyhow::{Context, Ok};

use crate::{parser::Parse, syntax::{keywords::KeywordKind, lexeme::LexemeKind}};

#[derive(Debug, Clone)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

///
#[derive(Debug)]
pub enum Safety {
    /// Determined to be safe-mode.
    Safe,
    /// Explicitly allow unsafe-mode.
    Unsafe,
    /// Undetermined safety. Assumed safe-mode.
    Unknown,
}

///
#[derive(Debug)]
pub enum Mutability {
    /// Explicity mutable.
    Mutable,
    /// Determined to be immutable.
    Immutable,
    /// Undetermined mutability. Assumed immutable.
    Unknown,
}

impl Parse for Mutability {
    fn parse(parser: &mut crate::parser::Parser<'_>) -> Result<Self, anyhow::Error> {
        if let Some(lexeme) = parser.peek_lexeme() {
            if lexeme.kind == LexemeKind::Keyword(KeywordKind::Mut) {
                parser.next_lexeme().unwrap();
                return Ok(Mutability::Mutable);
            }
        }
        Ok(Mutability::Unknown)
    }
}

///
#[derive(Debug)]
pub enum Visability {
    /// Explicitly visable.
    Public,
    /// Determined to not be visable.
    Private,
    /// Undetermined visability. Assumed private.
    Unknown,
}

impl Parse for Visability {
    fn parse(parser: &mut crate::parser::Parser<'_>) -> Result<Self, anyhow::Error> {
        if let Some(lexeme) = parser.peek_lexeme() {
            if lexeme.kind == LexemeKind::Keyword(KeywordKind::Pub) {
                parser.next_lexeme().unwrap();
                return Ok(Visability::Public);
            }
        }
        Ok(Visability::Unknown)
    }
}

/// Core Rage directives.
#[derive(Debug)]
pub enum Directive {
    Run,
    Use,
    Build,
    Define,
    UNKNOWN,
}

impl Directive {
    pub fn map_to(s: &str) -> Self {
        match s {
            "run" => Self::Run,
            "use" => Self::Use,
            "build" => Self::Build,
            "define" => Self::Define,
            _ => Self::UNKNOWN,
        }
    }

    pub fn map_from(&self) -> &str {
        match self {
            Self::Run => "run",
            Self::Use => "use",
            Self::Build => "build",
            Self::Define => "define",
            Self::UNKNOWN => unreachable!(),
        }
    }
}

/// Core Rage attributes.
#[derive(Debug)]
pub enum Attribute {
    Constant,
    Mutable,
    Public,
    Inline(Inlinedness),
}

/// 
#[derive(Debug)]
pub enum Inlinedness {
    /// Prevent inlining, equivalent to:
    /// ```
    /// #[inline(false)]
    /// ```
    Disable,
    /// Suggest inlining, equivalent to:
    /// ```
    /// #[inline]
    /// ```
    Weak,
    /// Force inlining, equivalent to:
    /// ```
    /// #[inline(true)]
    /// ```
    Strong,
}
