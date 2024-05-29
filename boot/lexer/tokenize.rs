use std::str::Chars;
use super::{error::{LexicalError, LexicalErrorKind}, lexeme::{str_to_keyword, Lexeme, Span}};

pub struct Tokenizer<'a> {
    source: &'a str,
    chars: Chars<'a>,
    current: Option<char>,
    peeked: Option<char>,
    index: u32,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut tokenizer = Self { 
            source,
            chars: source.chars(),
            current: None,
            peeked: None,
            index: 0,
        };
        tokenizer.current = tokenizer.chars.next();
        tokenizer.peeked = tokenizer.chars.next();
        tokenizer
    }

    fn jump(&mut self, index: u32) {
        self.chars = self.source.chars();
        self.current = None;
        self.peeked = None;
        self.index = 0;
        for _ in 0..index {
            self.bump();
        }
    } 

    /// Helper to advance to the next char.
    fn bump(&mut self)  {
        self.current = self.peeked;
        self.peeked = self.chars.next();
        self.index += 1;
    }

    fn consume(&mut self, mut predicate: impl FnMut(char) -> bool) -> u32 {
        let mut n = 0;
        while let Some(c) = self.current {
            if !predicate(c) { break; }
            self.bump();
            n += 1;
        }
        n
    }

    pub fn get_span(&self, span: &Span) -> Result<&str, LexicalErrorKind> {
        let range = (span.start as usize)..((span.start + span.length) as usize);
        self.source.get(range).ok_or_else(|| LexicalErrorKind::MissingLexeme)
    }

    fn handle_comment(&mut self) -> Result<(Span, Lexeme), LexicalError> {
        let start = self.index;
        self.bump();
        self.bump();
        let mut len = 2;
        match self.current {
            Some('/') => todo!("tokenize doc comment"),
            Some('*') => {
                let mut prev = Some('/');
                loop {
                    match (prev, self.current) {
                        (Some('*'), Some('/')) => {
                            self.bump(); // eat closing /
                            let span = Span::new(start, len + 1);
                            let raw = self.get_span(&span).map_err(|e| LexicalError::new(e, span))?;
                            return Ok((span, Lexeme::BlockComment(raw.into())));
                        },
                        (Some(_), Some(_)) => {
                            prev = self.current;
                            self.bump();
                            len += 1;
                        },
                        (_, None) => return Err(LexicalError::new(LexicalErrorKind::DanglingLexeme, Span::new(start, len))),
                        _ => unreachable!(),
                    }
                }
            },
            _ => {
                len += self.consume(|c| c != '\n' || c != '\r');
                let span = Span::new(start, len);
                let raw = self.get_span(&span).map_err(|e| LexicalError::new(e, span))?;
                Ok((span, Lexeme::InlineComment(raw.into())))
            },
        }
    }

    fn handle_punctuation(&mut self) -> Result<(Span, Lexeme), LexicalError> {
        let start = self.index;
        match self.current.ok_or_else(|| LexicalError::new(LexicalErrorKind::MissingLexeme, Span::new(start, 0)))? {
            '!' => match self.peeked {
                Some('=') => {
                    self.bump();
                    self.bump();
                    Ok((Span::new(start, 2), Lexeme::NotEqual))
                },
                Some(_) => {
                    self.bump();
                    Ok((Span::new(start, 1), Lexeme::Bang))
                },
                None => Err(LexicalError::new(LexicalErrorKind::DanglingLexeme, Span::new(start, 1))),
            },
            '#' => { self.bump(); Ok((Span::new(start, 1), Lexeme::Pound)) },
            '{' => { self.bump(); Ok((Span::new(start, 1), Lexeme::LeftBrace)) },
            '}' => { self.bump(); Ok((Span::new(start, 1), Lexeme::RightBrace)) },
            '(' => { self.bump(); Ok((Span::new(start, 1), Lexeme::LeftParenthesis)) },
            ')' => { self.bump(); Ok((Span::new(start, 1), Lexeme::RightParenthesis)) },
            c => todo!("handle punc {}", c),
        }
    }

    fn handle_string(&mut self) -> Result<(Span, Lexeme), LexicalError> {
        let start = self.index;
        self.bump(); // eat opening quote
        self.consume(|c| c != '"');
        self.bump(); // eat closing quote
        let span = Span::new(start, self.index - start);
        let raw = self.get_span(&span).map_err(|e| LexicalError::new(e, span))?;
        Ok((span, Lexeme::String(raw.into())))
    }

    fn handle_label(&mut self) -> Result<(Span, Lexeme), LexicalError> {
        let start = self.index;
        let len = self.consume(|c| c.is_alphanumeric() || c == '_');
        let span = Span::new(start, len);
        let raw = self.get_span(&span).map_err(|e| LexicalError::new(e, span))?;
        if let Some(lex) = str_to_keyword(raw) {
            Ok((span, lex))
        } else {
            Ok((span, Lexeme::Label(raw.into())))
        }
    }

    fn handle_hex(&mut self) -> Result<(Span, Lexeme), LexicalError> {
        let start = self.index;
        self.bump();
        self.bump();
        let mut len = 2;
        len += self.consume(|c| c.is_ascii_hexdigit());
        let span = Span::new(start, len);
        let raw = self.get_span(&span).map_err(|e| LexicalError::new(e, span))?;
        Ok((span, Lexeme::Hex(raw.into())))
    }

    fn handle_binary(&mut self) -> Result<(Span, Lexeme), LexicalError> {
        // TODO: impliment visual spacing (ie 0b0101_0101)
        let start = self.index;
        self.bump();
        self.bump();
        let mut len = 2;
        len += self.consume(|c| c == '0' || c == '1');
        let span = Span::new(start, len);
        let raw = self.get_span(&span).map_err(|e| LexicalError::new(e, span))?;
        Ok((span, Lexeme::Binary(raw.into())))
    }

    fn handle_number(&mut self) -> Result<(Span, Lexeme), LexicalError> {
        // TODO: impliment visual spacing (ie 1_000_000)
        todo!("tokenize integer/float")        
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<(Span, Lexeme), LexicalError>; 

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.current? {
            // standard whitespace
            ' ' | '\t' => {
                self.consume(|c| c == ' ' || c == '\t');
                self.next()?
            },
            // new line whitespace
            '\n' | '\r' => {
                self.consume(|c| c == '\n' || c == '\r');
                self.next()?
            },
            '/' => match self.peeked {
                Some('/') => self.handle_comment(),
                Some(_) => self.handle_punctuation(),
                None => Err(LexicalError::new(LexicalErrorKind::DanglingLexeme, Span::new(self.index, 1)))
            },
            '"' => self.handle_string(),
            '0' => match self.peeked {
                Some('x') => self.handle_hex(),
                Some('b') => self.handle_binary(),
                _ => self.handle_number(),
            },
            '1'..='9' => self.handle_number(),
            c if c.is_ascii_punctuation() => self.handle_punctuation(),
            c if c.is_alphabetic() => self.handle_label(),
            _ => Err(LexicalError::new(LexicalErrorKind::InvalidLexeme, Span::new(self.index, 1))), // TODO: add support for multi-char unknown lexemes
        })
    }
}
