use std::str::Chars;

use super::lexeme::{Lexeme, LexemeKind};

pub struct Scanner<'a> {
    count: u32,
    chars: std::iter::Peekable<Chars<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(chars: Chars<'a>) -> Self {
        Self {
            count: 0,
            chars: chars.peekable(),
        }
    }

    /// Increment the index counter by a length and return the previous count.
    fn index(&mut self, length: u32) -> u32 {
        let i = self.count;
        self.count += length;
        i
    }

    fn consume(&mut self, mut prediate: impl FnMut(&char) -> bool) -> u32 {
        let mut len = 0;
        while self.chars.next_if(prediate).is_some() {
            len += 1;
        }
        len
    }

    fn handle_whitespace(&mut self, c: &char) -> Lexeme {
        match c {
            '\n' => {
                let length = self.consume(|c| c == &'\n');
                Lexeme::new(LexemeKind::NewLine, self.index(length), length)
            }
            _ => {
                let length = self.consume(|c| c.is_ascii_whitespace() && c != &'\n');
                Lexeme::new(LexemeKind::Whitespace, self.index(length), length)
            }
        }
    }

    fn handle_digit(&mut self) -> Lexeme {
        let length = self.consume(|c| c.is_ascii_digit());
        Lexeme::new(LexemeKind::Number, self.index(length), length)
    }

    fn handle_alphabetic(&mut self) -> Lexeme {
        let length = self.consume(|c| c.is_ascii_alphanumeric() || c == &'_');
        Lexeme::new(LexemeKind::Term, self.index(length), length)
    }

    fn handle_punctuation(&mut self) -> Lexeme {
        let length = self.consume(|c| c.is_ascii_punctuation());
        Lexeme::new(LexemeKind::Symbol, self.index(length), length)
    }

    fn handle_unknown(&mut self) -> Lexeme {
        let length = self.consume(|c| {
            !c.is_ascii_whitespace()
                && !c.is_ascii_digit()
                && !c.is_ascii_alphabetic()
                && !c.is_ascii_punctuation()
        });
        Lexeme::new(LexemeKind::UNKNOWN, self.index(length), length)
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Lexeme;

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.chars.peek()?;

        return match first {
            c if c.is_ascii_whitespace() => Some(self.handle_whitespace(c)),
            c if c.is_ascii_digit() => Some(self.handle_digit()),
            c if c.is_ascii_alphabetic() => Some(self.handle_alphabetic()),
            c if c.is_ascii_punctuation() => Some(self.handle_punctuation()),
            _ => Some(self.handle_unknown()),
        };
    }
}
