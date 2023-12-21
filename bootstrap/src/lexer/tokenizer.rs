//! Rage Bootstrap
//! Lexer Tokenizer

use std::str::Chars;

use crate::token::{
    keyword::Keyword, symbol::Symbol, Bool, Comment, Literal, Token, TokenKind, Whitespace,
};

pub struct Tokenizer<'a> {
    next_index: u32,
    chars: std::iter::Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(chars: Chars<'a>) -> Self {
        Self {
            next_index: 0,
            chars,
        }
    }

    fn next_index(&mut self, length: u16) -> u32 {
        let i = self.next_idex;
        self.index += length as u32;
        i
    }

    fn peek_second(&self) -> Option<char> {
        let mut iter = self.chars.clone();
        iter.next()?;
        iter.next()
    }

    /// Consumes while the predicate is true. Returns number of [`char`] consumed.
    fn consume(&mut self, mut predicate: impl FnMut(&char) -> bool) -> usize {
        let mut len = 0;
        while self.chars.next_if(predicate).is_some() {
            len += 1;
        }
        return len;
    }

    /// Handle ASCII whitespace.
    fn whitespace(&mut self, c: char) -> Token {
        let mut length = 1;
        if c == '\n' {
            length += self.consume(|c| c == '\n');
            return Token::new(
                TokenKind::Whitespace(Whitespace::NewLine),
                self.next_index(length),
                length,
            );
        }
        length += self.consume(|c| c.is_ascii_whitespace() && c != '\n');
        return Token::new(
            TokenKind::Whitespace(Whitespace::Blank),
            self.next_index(length),
            length,
        );
    }

    /// Handle comments.
    fn comment(&mut self) -> Token {
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
                return Token::new(
                    TokenKind::Comment(Comment::Block),
                    self.next_index(length),
                    length,
                );
            }
            Some('/') => {
                length += self.consume(|c| c != '\n');
                return Token::new(
                    TokenKind::Comment(Comment::Document),
                    self.next_index(length),
                    length,
                );
            }
            _ => {
                length += self.consume(|c| c != '\n');
                return Token::new(
                    TokenKind::Comment(Comment::Line),
                    self.next_index(length),
                    length,
                );
            }
        }
    }

    /// Handle alphabetic terms. Yields keywords, identifiers, bool-literal, etc.
    fn term(&mut self, c: char) -> Token {
        let mut length = 1;
        let chars = self.chars.as_str();
        length += self.consume(|c| c.is_ascii_alphanumeric() || c == '_');
        let mut s = String::from(c);
        s.push_str(chars.get(..(length - 1)).unwrap());
        let s = s.as_str();
        if let Some(bool) = Bool::match_bool(s) {
            return Token::new(
                TokenKind::Literal(Literal::Bool(bool)),
                self.next_index(length),
                length,
            );
        }
        if let Some(keyword) = Keyword::match_keyword(s) {
            return Token::new(TokenKind::Keyword(keyword), self.next_index(length), length);
        }
        return Token::new(TokenKind::Identifier, self.next_index(length), length);
    }

    fn number(&mut self, c: char) -> Token {
        let mut length = 1;
        if c == '0' {
            match self.chars.peek() {
                Some('x') => todo!(),
                Some('b') => todo!(),
                _ => {}
            }
        }
        length += self.consume(|c| c.is_ascii_digit());
        return Token::new(
            TokenKind::Literal(Literal::Numeric),
            self.next_index(length),
            length,
        );
    }

    fn string(&mut self) -> Token {
        let mut length = 1;
        length += self.consume(|c| c != '"');
        self.chars.next().unwrap(); // consume closing quote
        length += 1; // include closing quote
        return Token::new(
            TokenKind::Literal(Literal::String),
            self.next_ondex(length),
            length,
        );
    }

    fn character(&mut self) -> Token {
        let length = self.consume(|c| c != '\'');
        return Token::new(
            TokenKind::Literal(Literal::Char),
            self.next_index(length),
            length,
        );
    }

    fn symbol(&mut self, c: char) -> Token {
        let length = 1;
        if let Some(s) = Symbol::match_symbol(&[c]) {
            if let Some(c2) = self.chars.peek() {
                if let Some(s2) = Symbol::match_symbol(&[c, c2]) {
                    length += 1;
                    if let Some(c3) = self.peek_second() {
                        if let Some(s3) = Symbol::match_symbol(&[c, c2, c3]) {
                            length += 1;
                            self.chars.next();
                            self.chars.next();
                            return Token::new(
                                TokenKind::Symbol(s3),
                                self.next_index(length),
                                length,
                            );
                        }
                    }
                    self.chars.next();
                    return Token::new(TokenKind::Symbol(s2), self.next_index(length), length);
                }
            }
            return Token::new(TokenKind::Symbol(s), self.next_index(length), length);
        }
        return Token::new(TokenKind::UNKNOWN, self.next_index(length), length);
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let first = match self.chars.next() {
            Some(c) => c,
            None => return None,
        };

        return match first {
            // whitespace
            c if c.is_ascii_whitespace() => Some(self.whitespace(c)),

            // comments, or slash
            '/' => match self.chars.peek() {
                Some('/') => Some(self.comment()),
                _ => Some(Token::new(
                    TokenKind::Symbol(Symbol::Slash),
                    self.next_index(1),
                    1,
                )),
            },

            // alphabetic
            c if c.is_ascii_alphabetic() => Some(self.term(c)),

            // numeric
            c if c.is_ascii_digit() => Some(self.number(c)),

            // string
            '"' => Some(self.string()),

            // character
            '\'' => Some(self.character()),

            c if c.is_ascii_punctuation() => Some(self.symbol(c)),

            _ => Some(Token::new(TokenKind::UNKNOWN, self.next_index(1), 1)),
        };
    }
}
