//! Rage Bootstrap 
//! Scanner

use std::str::Chars;

use crate::syntax::{keywords::KeywordKind, lexeme::{Lexeme, LexemeKind}, CommentKind, LiteralKind, WhitespaceKind};

/// Lexiacal Tokenizer.
pub struct Scanner<'a> {
    cursor: usize,
    content: &'a str,
    chars: std::iter::Peekable<Chars<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            cursor: 0,
            content,
            chars: content.chars().peekable(),
        }
    }

    /// # Safety
    /// Panics if asked to get value out of bounds.
    fn get_value(&self, length: usize) -> &str {
        unsafe {
            self.content.get_unchecked(self.cursor .. self.cursor + length)
        }
    }

    /// Consumes while the predicate is true. Returns number of [`char`] consumed.
    fn consume(&mut self, mut predicate: impl FnMut(&char) -> bool) -> usize {
        let mut len = 0;
        while self.chars.next_if(&mut predicate).is_some() {
            len += 1;
        }
        return len;
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn next_char(&mut self) -> Option<char> {
        self.chars.next()
    }

}

impl Iterator for Scanner<'_> {
    type Item = Lexeme;
    fn next(&mut self) -> Option<Self::Item> {
        let first = match self.chars.next() {
            Some(c) => c,
            None => return None,
        };
        let lexeme = match first {
            // whitespace
            c if c.is_ascii_whitespace() => whitespace(self, c),

            // comments, or slash
            /*'/' => match self.chars.peek() {
                Some('/') => {
                    self.chars.next();
                    comment(self)
                },
                _ => symbol(self, '/'),
            },*/

            // alphabetic
            c if c.is_ascii_alphabetic() => term(self),

            // numeric
            c if c.is_ascii_digit() => number(self, c), 

            // string
            '"' => string(self),

            // character
            '\'' => character(self),

            c if c.is_ascii_punctuation() => symbol(self, c),

            _ => Lexeme::with_length(LexemeKind::UNKNOWN, 1),
        };
        self.cursor += lexeme.count();
        return Some(lexeme);
    }
}

    /// Handle ASCII whitespace.
    fn whitespace<'a>(scanner: &mut Scanner<'a>, c: char) -> Lexeme {
        let mut length = 1;
        match c {
            '\n' => {
                length += scanner.consume(|c| c == &'\n');
                return Lexeme::with_length(LexemeKind::Whitespace(WhitespaceKind::NewLine), length);
            },
            _ => {
                length += scanner.consume(|c| c.is_ascii_whitespace() && c != &'\n');
                return Lexeme::with_length(LexemeKind::Whitespace(WhitespaceKind::Blank), length);
            },
        }
    }

    /// Handle comments.
    fn comment<'a>(scanner: &mut Scanner<'a>) -> Lexeme {
        let mut length = 2;
        match scanner.peek_char() {
            Some('*') => {
                let mut prev = '_';
                loop {
                    length += 1;
                    if let Some(new) = scanner.next_char() {
                        if prev == '*' && new == '/' {
                            break;
                        } else {
                            prev = new;
                        }
                    } else {
                        break;
                    }
                }
                return Lexeme::with_value(LexemeKind::Comment(CommentKind::Block), scanner.get_value(length));
            },
            Some('/') => {
                length += scanner.consume(|c| c != &'\n');
                return Lexeme::with_value(LexemeKind::Comment(CommentKind::Documentation), scanner.get_value(length));
            }
            _ => {
                length += scanner.consume(|c| c != &'\n');
                return Lexeme::with_value(LexemeKind::Comment(CommentKind::Line), scanner.get_value(length));
            }
        }
    }

    /// Handle alphabetic terms. Yields keywords, identifiers, bool-literal, etc.
    fn term<'a>(scanner: &mut Scanner<'a>) -> Lexeme {
        let mut length = 1;
        length += scanner.consume(|c| c.is_ascii_alphanumeric() || c == &'_');
        let text = scanner.get_value(length);
        return match text {
            "true" => Lexeme::with_value(LexemeKind::Literal(LiteralKind::Bool), text),
            "false" => Lexeme::with_value(LexemeKind::Literal(LiteralKind::Bool), text),
            t => match KeywordKind::from(t) {
                KeywordKind::UNKNOWN => Lexeme::with_value(LexemeKind::Term, t),
                k => Lexeme::with_length(LexemeKind::Keyword(k), length),
            },
        };
    }

    fn number<'a>(scanner: &mut Scanner<'a>, c: char) -> Lexeme {
        let mut length = 1;
        let kind = match c {
            '0' => match scanner.peek_char() {
                Some('x') => { 
                    scanner.next_char();
                    length += scanner.consume(|c| c.is_ascii_hexdigit()) + 1;
                    Some(LiteralKind::Hex)
                },
                Some('b') => {
                    scanner.next_char();
                    length += scanner.consume(|c| c == &'0'|| c == &'1') + 1;
                    Some(LiteralKind::Binary)
                },
                Some('o') => { 
                    scanner.next_char();
                    // TODO: finish implimenting octal lexing
                    length += scanner.consume(|c| c.is_ascii_digit()) + 1;
                    Some(LiteralKind::Octal)
                },
                Some(c) => {
                    if c.is_ascii_digit() {
                        length += scanner.consume(|c| c.is_ascii_digit());
                    }
                    None
                },
                None => {
                    Some(LiteralKind::Integer)
                },
            },
            _ => {
                length += scanner.consume(|c| c.is_ascii_digit());
                None
            },
        };
        match kind {
            Some(k) => { return Lexeme::with_value(LexemeKind::Literal(k), scanner.get_value(length)); },
            None => match scanner.peek_char() {
                Some('.') => {
                    scanner.next_char();
                    length += scanner.consume(|c| c.is_ascii_digit()) + 1; // include '.'
                    return Lexeme::with_value(LexemeKind::Literal(LiteralKind::Float), scanner.get_value(length));
                },
                _ => {
                    return Lexeme::with_value(LexemeKind::Literal(LiteralKind::Integer), scanner.get_value(length));
                },
            }
        }
    }

    fn string<'a>(scanner: &mut Scanner<'a>) -> Lexeme {
        let mut length = 1;
        length += scanner.consume(|c| c != &'"');
        scanner.next_char(); // consume closing quote
        length += 1; // include closing quote
        return Lexeme::with_value(LexemeKind::Literal(LiteralKind::String), scanner.get_value(length));
    }

    fn character<'a>(scanner: &mut Scanner<'a>) -> Lexeme {
        scanner.next_char(); // consume opening
        let length = scanner.consume(|c| c != &'\'') + 1; // include opening
        return Lexeme::with_value(LexemeKind::Literal(LiteralKind::Char), scanner.get_value(length));
    }

    fn symbol<'a>(scanner: &mut Scanner<'a>, c: char) -> Lexeme {
        match c {
            '!' => match scanner.peek_char() {
                Some('=') => {
                    scanner.next_char();
                    return Lexeme::with_length(LexemeKind::NotEqual, 2);
                },
                _ => return Lexeme::with_length(LexemeKind::Exclamation, 1),
            },
            //'"' => LexemeKind::Quotation,
            '#' => return Lexeme::with_length(LexemeKind::Number, 1),
            '$' => return Lexeme::with_length(LexemeKind::Dollar, 1),
            '%' => match scanner.peek_char() {
                Some('=') => {
                    scanner.next_char();
                    return Lexeme::with_length(LexemeKind::ModuloEqual, 2);
                },
                _ => return Lexeme::with_length(LexemeKind::Percent, 1),
            },
            '&' => match scanner.peek_char() {
                Some('&') => {
                    scanner.next_char();
                    return Lexeme::with_length(LexemeKind::AndAnd, 2);
                },
                Some('=') => {
                    scanner.next_char();
                    return Lexeme::with_length(LexemeKind::AndEqual, 2);
                },
                _ => return Lexeme::with_length(LexemeKind::Ampersand, 1),
            },
            //'\'' => LexemeKind::Apostrophe,
            '(' => return Lexeme::with_length(LexemeKind::LParen, 1),
            ')' => return Lexeme::with_length(LexemeKind::RParen, 1),
            '*' => match scanner.peek_char() {
                Some('=') => {
                    scanner.next_char();
                    return Lexeme::with_length(LexemeKind::MultiplyEqual, 2);
                },
                _ => return Lexeme::with_length(LexemeKind::Asterisk, 1),
            },
            '+' => match scanner.peek_char() {
                Some('+') => {
                    scanner.next_char();
                    return Lexeme::with_length(LexemeKind::Incriment, 2);
                },
                Some('=') => {
                    scanner.next_char();
                    return Lexeme::with_length(LexemeKind::PlusEqual, 2);
                },
                _ => return Lexeme::with_length(LexemeKind::Plus, 1),
            },
            ',' => return Lexeme::with_length(LexemeKind::Comma, 1),
            '-' => match scanner.peek_char() {
                Some('-') => {
                    scanner.next_char();
                    return Lexeme::with_length(LexemeKind::Decriment, 2);
                },
                Some('=') => {
                    scanner.next_char();
                    return Lexeme::with_length(LexemeKind::MinusEqual, 2);
                },
                _ => return Lexeme::with_length(LexemeKind::Hyphen, 1),
            },
            '.' => match scanner.peek_char() {
                Some('.') => {
                    scanner.next_char();
                    match scanner.peek_char() {
                        Some('.') => {
                            scanner.next_char();
                            return Lexeme::with_length(LexemeKind::Ellipsis, 3);
                        },
                        Some('=') => {
                            scanner.next_char();
                            return Lexeme::with_length(LexemeKind::InclusiveRange, 3); 
                        },
                        _ => return Lexeme::with_length(LexemeKind::ExclusiveRange, 2),
                    }
                },
                _ => return Lexeme::with_length(LexemeKind::Dot, 1),
            },
            '/' => match scanner.peek_char() {
                Some('=') => {
                    scanner.next_char();
                    return Lexeme::with_length(LexemeKind::DivideEqual, 2);
                },
                Some('/') => {
                    scanner.next_char();
                    return comment(scanner);
                },
                _ => return Lexeme::with_length(LexemeKind::Slash, 1),
            },
            ':' => return Lexeme::with_length(LexemeKind::Colon, 1),
            ';' => return Lexeme::with_length(LexemeKind::Semicolon, 1),
            '<' => match scanner.peek_char() {
                Some('<') => {
                    scanner.next_char();
                    match scanner.peek_char() {
                        Some('<') => {
                            scanner.next_char();
                            return Lexeme::with_length(LexemeKind::LeftRotate, 3);
                        },
                        Some('=') => {
                            scanner.next_char();
                            return Lexeme::with_length(LexemeKind::LeftShiftEqual, 3);
                        },
                        _ => return Lexeme::with_length(LexemeKind::LeftShift, 2),
                    }
                },
                Some('=') => {
                    scanner.next_char();
                    return Lexeme::with_length(LexemeKind::LesserOrEqual, 2);
                },
                _ => return Lexeme::with_length(LexemeKind::Lesser, 1),
            },
            '=' => match scanner.peek_char() {
                Some('=') => {
                    scanner.next_char();
                    return Lexeme::with_length(LexemeKind::EqualEqual, 2);
                },
                _ => return Lexeme::with_length(LexemeKind::Equal, 1),
            },
            '>' => match scanner.peek_char() {
                Some('>') => {
                    scanner.next_char();
                    match scanner.peek_char() {
                        Some('>') => {
                            scanner.next_char();
                            return Lexeme::with_length(LexemeKind::RightRotate, 3);
                        },
                        Some('=') => {
                            scanner.next_char();
                            return Lexeme::with_length(LexemeKind::RightShiftEqual, 3);
                        },
                        _ => return Lexeme::with_length(LexemeKind::RightShift, 2),
                    }
                },
                Some('=') => {
                    scanner.next_char();
                    return Lexeme::with_length(LexemeKind::GreaterOrEqual, 2);
                },
                _ => return Lexeme::with_length(LexemeKind::Greater, 1),
            },
            //'?' => LexemeKind::Question,
            //'@' => LexemeKind::At,
            '[' => return Lexeme::with_length(LexemeKind::LSquare, 1),
            //'\\' => LexemeKind::Backslash,
            ']' => return Lexeme::with_length(LexemeKind::RSquare, 1),
            '^' => match scanner.peek_char() {
                Some('=') => {
                    scanner.next_char();
                    return Lexeme::with_length(LexemeKind::XorEqual, 2);
                },
                _ => return Lexeme::with_length(LexemeKind::Caret, 1),
            },
            //'_' => LexemeKind::Underscore,
            //'`' => LexemeKind::Accent,
            '{' => return Lexeme::with_length(LexemeKind::LCurly, 1),
            '|' => match scanner.peek_char() {
                Some('|') => {
                    scanner.next_char();
                    return Lexeme::with_length(LexemeKind::OrOr, 2);
                },
                Some('=') => {
                    scanner.next_char();
                    return Lexeme::with_length(LexemeKind::OrEqual, 2);
                },
                _ => return Lexeme::with_length(LexemeKind::Pipe, 1),
            },
            '}' => return Lexeme::with_length(LexemeKind::RCurly, 1),
            '~' => return Lexeme::with_length(LexemeKind::Tilde, 1),
            _ => return Lexeme::with_length(LexemeKind::UNKNOWN, 1),
        }
    }
