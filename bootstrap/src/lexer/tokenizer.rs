//! Rage Bootstrap
//! tokenizer

use std::str::Chars;

use crate::token::{Token, TokenKind, symbol::Symbol, Whitespace, Comment, keyword::Keyword, Literal};

pub struct Tokenizer<'a> {
    chars: Chars<'a>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(chars: Chars<'a>) -> Self {
        Self { chars }
    }

    fn peek_first(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn peek_second(&self) -> Option<char> {
        let mut iter = self.chars.clone();
        iter.next()?;
        iter.next()
    }

    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    /// Consumes the next [`char`] if able.
    fn consume_next(&mut self) -> Option<char> {
        self.chars.next()
    }

    /// Consumes while the predicate is true. Returns number of [`char`] consumed.
    fn consume_while(&mut self, mut predicate: impl FnMut(char)->bool) -> usize {
        let mut len = 0;
        while self.peek_first().is_some_and(|c| predicate(c)) && !self.is_eof() {
            len += 1;
            let _ = self.consume_next();
        }
        return len;
    }

    /// Handle ASCII whitespace.
    fn whitespace(&mut self, c: char) -> Token {
        let mut length = 1;
        if c == '\n' {
            length += self.consume_while(|c| c == '\n');
            return Token::new(TokenKind::Whitespace(Whitespace::NewLine), length); 
        }
        length += self.consume_while(|c| c.is_ascii_whitespace() && c != '\n' );
        return Token::new(TokenKind::Whitespace(Whitespace::Blank), length);
    }

    /// Handle comments.
    fn comment(&mut self) -> Token {
        let mut length = 1;
        match self.peek_second() {
            Some('*') => {
                let mut prev = '_';
                loop {
                    length += 1;
                    if let Some(new) = self.consume_next() {
                        if prev == '*' && new == '/' { 
                            break; 
                        } else {
                            prev = new;
                        }
                    } else {
                        break;
                    }
                }
                return Token::new(TokenKind::Comment(Comment::Block), length);
            },
            Some('/') => {
                length += self.consume_while(|c| c != '\n');
                return Token::new(TokenKind::Comment(Comment::Document), length);
            },
            _ => {
                length += self.consume_while(|c| c != '\n');
                return Token::new(TokenKind::Comment(Comment::Line), length);
            },
        }
    }

    /// Handle alphabetic terms. Yields keywords, identifiers, etc.
    fn term(&mut self, c: char) -> Token {
        let mut length = 1;
        let chars = self.chars.as_str();
        length += self.consume_while(|c| c.is_ascii_alphanumeric() || c == '_');
        let s = chars.get(..(length - 1)).unwrap();
        if let Some(keyword) = Keyword::match_keyword(s) {
            return Token::new(TokenKind::Keyword(keyword), length);
        }
        return Token::new(TokenKind::Identifier, length);
    } 

    fn number(&mut self, c: char) -> Token {
        let mut length = 1;
        if c == '0' {
            match self.peek_first() {
                Some('x') => todo!(),
                Some('b') => todo!(),
                _ => {},
            }
        }
        length += self.consume_while(|c| c.is_ascii_digit());
        return Token::new(TokenKind::Literal(Literal::Numeric), length);
    }

    fn string(&mut self) -> Token {
        let mut length = 1;
        length += self.consume_while(|c| c != '"');
        return Token::new(TokenKind::Literal(Literal::String), length);
    }

    fn character(&mut self) -> Token {
        let length = self.consume_while(|c| c != '\'');
        return Token::new(TokenKind::Literal(Literal::Char), length);
    }

    fn symbol(&mut self, c: char) -> Token {
        if let Some(s) = Symbol::match_symbol(&[c]) {
            if let Some(c2) = self.peek_first() {
               if let Some(s2) = Symbol::match_symbol(&[c, c2]) {
                    if let Some(c3) = self.peek_second() {
                        if let Some(s3) = Symbol::match_symbol(&[c, c2, c3]) {
                            self.consume_next();
                            self.consume_next();
                            return Token::new(TokenKind::Symbol(s3), 3);
                        }
                    }
                    self.consume_next();
                    return Token::new(TokenKind::Symbol(s2), 2);
               } 
            }
            return Token::new(TokenKind::Symbol(s), 1);
        }
        return Token::new(TokenKind::UNKNOWN, 1);
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let first = match self.consume_next() {
            Some(c) => c,
            None => return Some(Token::new(TokenKind::EOF, 0)),
        };

        return match first {
            // whitespace
            c if c.is_ascii_whitespace() => Some(self.whitespace(c)),

            // comments, or slash
            '/' => match self.peek_first() {
                Some('/') => Some(self.comment()),
                _ => Some(Token::new(TokenKind::Symbol(Symbol::Slash), 1)),
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

            _ => Some(Token::new(TokenKind::UNKNOWN, 1)),
        };
    }
}
