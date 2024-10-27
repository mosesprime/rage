use std::{iter::Peekable, str::Chars};

/// Lexical scanner and analyzer
pub struct Lexer<'a> {
    input: &'a str,
    chars: Peekable<Chars<'a>>,
    count: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars().peekable(),
            count: 0,
        }
    }
    
    fn handle_whitespace(&mut self) -> Lexeme {
        let start = self.count;
        while let Some(_) = self.chars.next_if(|c| c.is_whitespace()) {
            self.count += 1;
        }
        Lexeme::Whitespace(self.count - start)
    }

    fn handle_alphabetic(&mut self) -> Lexeme {
        let start = self.count;
        while let Some(_) = self.chars.next_if(|c| c.is_alphabetic() || c == &'_') {
            self.count += 1;
        }
        Lexeme::Ident(Box::from(&self.input[start..self.count]))
    }

    fn handle_digit(&mut self) -> Lexeme {
        let start = self.count;
        while let Some(_) = self.chars.next_if(|c| c.is_ascii_alphanumeric() || c == &'.' || c == &'_' || c == &'-') {
            self.count += 1;
        }
        Lexeme::Number(Box::from(&self.input[start..self.count]))
    }

    fn handle_char(&mut self) -> Lexeme {
        let start = self.count;
        self.chars.next().expect("missing single quote");
        if let Some(c) = self.chars.next() {
            if self.chars.peek() != Some(&'\'') {
                self.count += 2;
                return Lexeme::UNKNOWN(Box::from(&self.input[start..self.count]));
            }
            self.count += 3;
            self.chars.next().expect("missing single quote");
            return Lexeme::Char(c);
        } else {
            self.count += 1;
            Lexeme::Apostrophe
        }
    }

    fn handle_string(&mut self) -> Lexeme {
        let start = self.count;
        self.chars.next().expect("missing double quote");
        self.count += 1;
        while let Some(c) = self.chars.next() {
            self.count += 1;
            if c == '\"' {
                break;
            }
        }
        Lexeme::String(Box::from(&self.input[start..self.count]))
    }

    fn handle_slash(&mut self) -> Lexeme {
        let start = self.count;
        self.chars.next().expect("missing slash");
        self.count += 1;
        // TODO: add other comment types
        match self.chars.peek() {
            Some('/') => {
                while let Some(_) = self.chars.next_if(|c| c != &'\n') {
                    self.count += 1;
                }
                Lexeme::Comment(self.count - start)
            },
            _ => Lexeme::Slash,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Lexeme;
    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.chars.peek()? {
            '\"' => self.handle_string(),
            '\'' => self.handle_char(),
            '/' => self.handle_slash(),
            c if c.is_whitespace() => self.handle_whitespace(),
            c if c.is_alphabetic() => self.handle_alphabetic(),
            c if c.is_ascii_digit() => self.handle_digit(),
            _ => {
                self.count += 1;
                match self.chars.next()? {
                    '!' => Lexeme::Bang,
                    '#' => Lexeme::Pound,
                    '$' => Lexeme::Dollar,
                    '%' => Lexeme::Percent,
                    '&' => Lexeme::And,
                    '(' => Lexeme::LParen,
                    ')' => Lexeme::RParen,
                    '*' => Lexeme::Asterisk,
                    '+' => Lexeme::Plus,
                    ',' => Lexeme::Comma,
                    '-' => Lexeme::Hyphen,
                    '.' => Lexeme::Period,
                    ':' => Lexeme::Colon,
                    ';' => Lexeme::Semicolon,
                    '<' => Lexeme::Lesser,
                    '=' => Lexeme::Equal,
                    '>' => Lexeme::Greater,
                    '?' => Lexeme::Question,
                    '@' => Lexeme::At,
                    '[' => Lexeme::LBrack,
                    '\\' => Lexeme::Backslash,
                    ']' => Lexeme::RBrack,
                    '^' => Lexeme::Caret,
                    '_' => Lexeme::Underscore,
                    '`' => Lexeme::Grave,
                    '{' => Lexeme::LBrace,
                    '|' => Lexeme::Pipe,
                    '}' => Lexeme::RBrace,
                    '~' => Lexeme::Tilde,
                    _ => Lexeme::UNKNOWN(Box::from(&self.input[self.count-1..self.count])),
                }
            }
        })
    }
}

/// Lexical tokens
#[derive(Debug, PartialEq)]
pub enum Lexeme {
    Whitespace(usize),
    Comment(usize),

    Ident(Box<str>),
    String(Box<str>),
    Number(Box<str>),
    Char(char),

    Bang,
    Quote,
    Pound,
    Dollar,
    Percent,
    And,
    Apostrophe,
    LParen,
    RParen,
    Asterisk,
    Plus,
    Comma,
    Hyphen,
    Period,
    Slash,
    Colon,
    Semicolon,
    Lesser,
    Equal,
    Greater,
    Question,
    At,
    LBrack,
    Backslash,
    RBrack,
    Caret,
    Underscore,
    Grave,
    LBrace,
    Pipe,
    RBrace,
    Tilde,

    UNKNOWN(Box<str>),
}

impl Lexeme {
    /// Length in [char]s of a [Lexeme]
    pub fn len(&self) -> usize {
        match self {
            Lexeme::Whitespace(n) | Lexeme::Comment(n) => *n,
            Lexeme::Ident(s) | Lexeme::String(s) | Lexeme::Number(s) | Lexeme::UNKNOWN(s) => s.len(),
            Lexeme::Char(_) => 3,
            _ => 1,
        }
    }
}

#[test]
fn lex_string() {
    let s = "\"banana\"";
    let mut lexer = Lexer::new(s);
    assert_eq!(lexer.next(), Some(Lexeme::String(Box::from(s))));
    assert_eq!(lexer.next(), None);
}

#[test]
fn lex_ident() {
    let i = "banana";
    let mut lexer = Lexer::new(i);
    assert_eq!(lexer.next(), Some(Lexeme::Ident(Box::from(i))));
    assert_eq!(lexer.next(), None);
}

#[test]
fn lex_char() {
    let c = "\'!\'";
    let mut lexer = Lexer::new(c);
    assert_eq!(lexer.next(), Some(Lexeme::Char('!')));
    assert_eq!(lexer.next(), None);
}
