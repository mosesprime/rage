//! Rage Bootstrap 
//! Scanner

use std::str::Chars;

use crate::syntax::token::{Comment, Literal};

use super::lexeme::{Lexeme, LexemeKind, Whitespace};

/// Lexiacal Tokenizer.
pub struct Scanner<'a> {
    chars: Chars<'a>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars(),
        }
    }

    fn peek_first(&mut self) -> Option<char> {
        self.chars.clone().next()
    }

    fn peek_second(&self) -> Option<char> {
        let mut iter = self.chars.clone();
        iter.next()?;
        iter.next()
    }

    /// Consumes while the predicate is true. Returns number of [`char`] consumed.
    fn consume(&mut self, mut predicate: impl FnMut(&char) -> bool) -> u32 {
        let mut len = 0;
        let mut peekable = self.chars.clone().peekable();
        while peekable.next_if(&mut predicate).is_some() {
            len += 1;
            self.chars.next();
        }
        return len;
    }

    /// Handle ASCII whitespace.
    fn whitespace(&mut self, c: char) -> Lexeme {
        let mut length = 1;
        match c {
            '\n' => {
                length += self.consume(|c| c == &'\n');
                return Lexeme::new(LexemeKind::Whitespace(Whitespace::NewLine), length);
            },
            _ => {
                length += self.consume(|c| c.is_ascii_whitespace() && c != &'\n');
                return Lexeme::new(LexemeKind::Whitespace(Whitespace::Blank), length)
            }
        }
    }

    /// Handle comments.
    fn comment(&mut self) -> Lexeme {
        let mut length = 1;
        match self.peek_second() {
            Some('*') => {
                let mut prev = '_';
                loop {
                    length += 1;
                    if let Some(new) = self.chars.next() {
                        if prev == '*' && new == '/' {
                            break;
                        } else {
                            prev = new;
                        }
                    } else {
                        break;
                    }
                }
                return Lexeme::new(LexemeKind::Comment(Comment::Block), length);
            }
            Some('/') => {
                length += self.consume(|c| c != &'\n');
                return Lexeme::new(LexemeKind::Comment(Comment::Documentation), length);
            }
            _ => {
                length += self.consume(|c| c != &'\n');
                return Lexeme::new(LexemeKind::Comment(Comment::Line), length);
            }
        }
    }

    /// Handle alphabetic terms. Yields keywords, identifiers, bool-literal, etc.
    fn term(&mut self) -> Lexeme {
        let mut length = 1;
        length += self.consume(|c| c.is_ascii_alphanumeric() || c == &'_');
        return Lexeme::new(LexemeKind::Term, length);
    }

    fn number(&mut self) -> Lexeme {
        let mut length = 1;
        length += self.consume(|c| c.is_ascii_digit());
        // TODO: add other number literal kinds
        return Lexeme::new(LexemeKind::Literal(Literal::Integer), length);
    }

    fn string(&mut self) -> Lexeme {
        let mut length = 1;
        length += self.consume(|c| c != &'"');
        self.chars.next().unwrap(); // consume closing quote
        length += 1; // include closing quote
        return Lexeme::new(LexemeKind::Literal(Literal::String), length);
    }

    fn character(&mut self) -> Lexeme {
        let length = self.consume(|c| c != &'\'');
        return Lexeme::new(LexemeKind::Literal(Literal::Char), length);
    }

    fn symbol(&mut self, c: char) -> Lexeme {
        let length = 1;
        let kind = match c {
            '!' => LexemeKind::Exclamation,
            '"' => LexemeKind::Quotation,
            '#' => LexemeKind::Number,
            '$' => LexemeKind::Dollar,
            '%' => LexemeKind::Percent,
            '&' => LexemeKind::Ampersand,
            '\'' => LexemeKind::Apostrophe,
            '(' => LexemeKind::LParen,
            ')' => LexemeKind::RParen,
            '*' => LexemeKind::Asterisk,
            '+' => LexemeKind::Plus,
            ',' => LexemeKind::Comma,
            '-' => LexemeKind::Hyphen,
            '.' => LexemeKind::Dot,
            '/' => LexemeKind::Slash,
            ':' => LexemeKind::Colon,
            ';' => LexemeKind::Semicolon,
            '<' => LexemeKind::Lesser,
            '=' => LexemeKind::Equal,
            '>' => LexemeKind::Greater,
            '?' => LexemeKind::Question,
            '@' => LexemeKind::At,
            '[' => LexemeKind::LSquare,
            '\\' => LexemeKind::Backslash,
            ']' => LexemeKind::RSquare,
            '^' => LexemeKind::Caret,
            '_' => LexemeKind::Underscore,
            '`' => LexemeKind::Accent,
            '{' => LexemeKind::LCurly,
            '|' => LexemeKind::Pipe,
            '}' => LexemeKind::RCurly,
            '~' => LexemeKind::Tilde,
            _ => LexemeKind::UNKNOWN,
        };
        return Lexeme::new(kind, length);
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Lexeme;

    fn next(&mut self) -> Option<Self::Item> {
        let first = match self.chars.next() {
            Some(c) => c,
            None => return None,
        };

        return match first {
            // whitespace
            c if c.is_ascii_whitespace() => Some(self.whitespace(c)),

            // comments, or slash
            '/' => match self.peek_first() {
                Some('/') => Some(self.comment()),
                _ => Some(Lexeme::new(LexemeKind::Slash, 1)),
            },

            // alphabetic
            c if c.is_ascii_alphabetic() => Some(self.term()),

            // numeric
            c if c.is_ascii_digit() => Some(self.number()),

            // string
            '"' => Some(self.string()),

            // character
            '\'' => Some(self.character()),

            c if c.is_ascii_punctuation() => Some(self.symbol(c)),

            _ => Some(Lexeme::new(LexemeKind::UNKNOWN, 1)),
        };
    }
}
