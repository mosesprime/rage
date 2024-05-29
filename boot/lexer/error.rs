use super::lexeme::Span;

#[derive(Debug, PartialEq)]
pub struct LexicalError {
    pub kind: LexicalErrorKind,
    pub span: Span,
}    

#[derive(Debug, PartialEq)]
pub enum LexicalErrorKind {
    /// Lexeme has an invalid position or is incomplete.
    DanglingLexeme,
    /// Lexeme contains invalid characters.
    InvalidLexeme,
    /// Lexeme has an invalid span.
    MissingLexeme,
}

impl LexicalError {
    pub fn new(kind: LexicalErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}
