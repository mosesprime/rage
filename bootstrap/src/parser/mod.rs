//! Rage Bootstrap
//! Parser

use std::{collections::VecDeque, iter::Peekable};

use anyhow::{Context, Ok};

use crate::{builder::source::SourceId, syntax::{token::{ArithmeticOp, AssignmentOp, DelimiterKind, LiteralKind, LogicalOp, OperatorKind, RelationalOp, Token, TokenKind}, FuncDecl, Span, Statement}};

use self::{scanner::Scanner, lexeme::{Lexeme, LexemeKind, WhitespaceKind}, tree::ParseTree};

pub mod lexeme;
pub mod scanner;
pub mod tree;

/// Front-end of the compiler.
/// Performs lexical analysis, syntax analysis, and semantic analysis on the source code.
pub struct Parser<'a> {
    source_id: SourceId,
    parse_buffer: ParseBuffer<'a>,
    parse_tree: ParseTree,
}

impl<'a> Parser<'a> {
    pub fn new(source_text: &'a str, source_id: &SourceId) -> Self {
        Self {
            source_id: source_id.clone(),
            parse_buffer: ParseBuffer::new(source_text),
            parse_tree: ParseTree::new(source_id),
        }
    }
    
    

    pub fn run(mut self) {
        // TODO: do parsing
        log::error!("parser is a work in process");
        let mut ast = ParseTree::new(&self.source_id);
        while !self.parse_buffer.is_empty() {
            let stmt = Statement::parse(&self.parse_buffer);
        }
        println!("{ast}");
    }
}

pub trait Parse: Sized {
    fn parse(input: &ParseBuffer) -> Result<Option<Self>, ParseError>;
}

pub enum ParseError {
    /// Failed to parse because the pattern does not match.
    PaternMatch,
}

pub struct ParseBuffer<'a> {
    source_text: &'a str,
    /// [Lexeme] iterator.
    lexemes: Peekable<Scanner<'a>>,
    /// Lookback buffer of [Token]s.
    lookback: Vec<&'a Token>,
    stashed: VecDeque<Lexeme>,
    lookforward: VecDeque<Token>,
    ///
    cursor: u32,
}

impl<'a> ParseBuffer<'a> {
    pub fn new(source_text: &'a str) -> Self {
        Self {
            source_text,
            lexemes: Scanner::new(source_text).peekable(),
            lookback: Vec::default(),
            stashed: VecDeque::default(),
            lookforward: VecDeque::default(),
            cursor: 0,
        }
    }

    fn get_span(&self, span: &Span) -> Option<&str> {
        self.source_text.get(span.as_range())
    }

    fn get_value(&self, start: usize, end: usize) -> Option<&str> {
        self.source_text.get(start..end)
    }

    /// Get the line of source code where the index is located.
    fn get_line_from_index(&self, index: usize) -> Option<&str> {
        self.source_text.lines().find(|line| {
            let mut indcies = line.char_indices();
            if let Some((n, _)) = indcies.next() {
                n == index
            } else {
                false
            }
        })
    }

    /// Gets a single line of the source if able.
    fn get_line_number(&self, line_num: usize) -> Option<&str> {
        self.source_text.lines().nth(line_num)
    }

    pub fn is_empty(&mut self) -> bool {
        self.lexemes.peek().is_none()
    }

    fn prev_token(&mut self) -> Option<&Token> {
        let x = self.lookback.last()?;
        Some(*x)
    }

    fn next_lexeme(&mut self) -> Option<Lexeme> {
        if self.stashed.len() > 0 {
            return self.stashed.pop_front();
        }
        self.lexemes.next()
    }

    fn peek_lexeme(&mut self) -> Option<&Lexeme> {
        if self.stashed.len() > 0 {
            return self.stashed.front()
        }
        self.lexemes.peek()
    }

    fn handle_literal(&mut self, lit_kind: LiteralKind, length: u32) -> Token {
        let start = self.cursor;
        self.cursor += length; 
        return Token::new(TokenKind::Literal(lit_kind), Span::new(start, self.cursor));
    }

    fn handle_term(&mut self, length: u32) -> Token {
        let start = self.cursor;
        self.cursor += length;
        let span = Span::new(start, self.cursor);
        // TODO: idk if I need to lookback yet
        return Token::new(TokenKind::Identifier, span);
    }

    fn handle_unknown(&mut self, length: u32) -> Token {
        let start = self.cursor;
        self.cursor += length;
        return Token::new(TokenKind::UNKNOWN, Span::new(start, self.cursor));
    }

    fn handle_other(&mut self, lexeme: Lexeme) -> Token {
        let start = self.cursor;
        self.cursor += lexeme.length;
        match lexeme.kind {
            // LogicalNot, NotEqual
            LexemeKind::Exclamation => match self.lexemes.peek() {
                Some(l2) => match l2.kind {
                    // NotEqual
                    LexemeKind::Equal => {
                        self.next_lexeme();
                        self.cursor += l2.length;
                        return Token::new(TokenKind::Operator(OperatorKind::Relational(RelationalOp::NotEqual)), Span::new(start, self.cursor));
                    },
                    // LogicalNot
                    _ => {
                        return Token::new(TokenKind::Operator(OperatorKind::Logical(LogicalOp::LogicNOT)), Span::new(start, self.cursor));
                    },
                }, 
                // Dangling !
                None => {
                    return Token::new(TokenKind::VERBATIM(lexeme.kind), Span::new(start, self.cursor)); 
                },
            },
            // 
            LexemeKind::Quotation => todo!(),
            // Meta tag
            LexemeKind::Number => {
                return Token::new(TokenKind::Meta, Span::new(start, self.cursor));
            },
            // Borrow
            LexemeKind::Dollar => {
                return Token::new(TokenKind::Borrow, Span::new(start, self.cursor));
            },
            // Modulo, ModuloAssign
            LexemeKind::Percent => match self.lexemes.peek() {
                Some(l2) => match l2.kind {
                    // ModuloAssign
                    LexemeKind::Equal => {
                        self.lexemes.next();
                        self.cursor += l2.length;
                        return Token::new(TokenKind::Operator(OperatorKind::Assignment(AssignmentOp::ModuloAssign)), Span::new(start, self.cursor));
                    },
                    // Modulo
                    _ => {
                        return Token::new(TokenKind::Operator(OperatorKind::Arithmetic(ArithmeticOp::Modulo)), Span::new(start, self.cursor));
                    },
                },
                // Dangling %
                None => {
                    return Token::new(TokenKind::VERBATIM(lexeme.kind), Span::new(start, self.cursor));
                },
            },
            LexemeKind::Ampersand => todo!(),
            LexemeKind::Apostrophe => todo!(),
            // Parenthesis delimiter
            LexemeKind::LParen => match self.peek_lexeme() {
                Some(l2) => match l2.kind {
                    // paired delimiter ()
                    LexemeKind::RParen => {
                        self.next_lexeme();
                        self.cursor += l2.length;
                        return Token::new(TokenKind::Delimiter(DelimiterKind::Paren), Span::new(start, self.cursor));
                    },
                    // with contents ( ... )
                    _ => {
                        let mut width = 0;
                        let mut counter: u32 = 1; 
                        todo!()
                    },
                },
                // Dangling (
                None => {
                    return Token::new(TokenKind::VERBATIM(lexeme.kind), Span::new(start, self.cursor));
                },
            },
            LexemeKind::RParen => todo!(), // closing delimiter should be handled already, so skip
            LexemeKind::Asterisk => todo!(),
            LexemeKind::Plus => todo!(),
            LexemeKind::Comma => todo!(),
            LexemeKind::Hyphen => todo!(),
            LexemeKind::Dot => todo!(),
            LexemeKind::Slash => todo!(),
            LexemeKind::Colon => todo!(),
            LexemeKind::Semicolon => todo!(),
            LexemeKind::Lesser => todo!(),
            LexemeKind::Equal => todo!(),
            LexemeKind::Greater => todo!(),
            LexemeKind::Question => todo!(),
            LexemeKind::At => todo!(),
            LexemeKind::LSquare => todo!(),
            LexemeKind::Backslash => todo!(),
            LexemeKind::RSquare => todo!(),
            LexemeKind::Caret => todo!(),
            LexemeKind::Underscore => todo!(),
            LexemeKind::Accent => todo!(),
            LexemeKind::LCurly => todo!(),
            LexemeKind::Pipe => todo!(),
            LexemeKind::RCurly => todo!(),
            LexemeKind::Tilde => todo!(),
            LexemeKind::Whitespace(_) | 
                LexemeKind::Comment(_) |
                LexemeKind::Literal(_) |
                LexemeKind::Term |
                LexemeKind::UNKNOWN => unreachable!(),
        }
    }
}

impl Iterator for ParseBuffer<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        /*if self.lookforward.len() > 0 {
            return self.lookforward.pop_front();
        }*/
        match self.next_lexeme() {
            Some(lexeme) => match lexeme.kind {
                LexemeKind::Whitespace(_) => {
                    self.cursor += lexeme.length;
                    self.next()
                },
                LexemeKind::Comment(_) => {
                    self.cursor += lexeme.length;
                    self.next()
                },
                LexemeKind::Literal(lit_kind) => Some(self.handle_literal(lit_kind, lexeme.length)),
                LexemeKind::Term => Some(self.handle_term(lexeme.length)),
                LexemeKind::UNKNOWN => Some(self.handle_unknown(lexeme.length)),
                _ => Some(self.handle_other(lexeme)),
            },
            None => return None,
        }
    }
}
