//! Rage Bootstrap
//! Parser

use anyhow::anyhow;

use crate::{builder::source::SourceId, common::span::Span, syntax::{token::{DelimiterKind, Literal, LiteralKind, LiteralRepr, LogicalOpKind, OperatorKind, RelationalOpKind, Token, TokenKind, UnpairedDelimiter, DelimiterSide}, Expression, Identifier, LiteralExpr, Statement}};

use self::{lexeme::{Lexeme, LexemeKind, WhitespaceKind}, scanner::Scanner, tree::ParseTree };//buffer::ParseBuffer};

pub mod lexeme;
pub mod scanner;
pub mod tree;

/// Front-end of the compiler.
/// Performs lexical analysis, syntax analysis, and semantic analysis on the source code.
pub struct Parser<'a> {
    source_id: &'a SourceId,
    source_text: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(source_text: &'a str, source_id: &'a SourceId) -> Self {
        Self {
            source_id,
            source_text,
        }
    }
    
    pub fn get_span(&self, span: &Span) -> Result<&str, ParseError> {
        self.source_text.get(span.as_range()).ok_or_else(|| ParseError::UnobtainableSpan)
    }

    pub fn get_value(&self, start: usize, end: usize) -> Option<&str> {
        self.source_text.get(start..end)
    }

    /// Get the line of source code where the index is located.
    pub fn get_line_from_index(&self, index: usize) -> Option<&str> {
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
    pub fn get_line_number(&self, line_num: usize) -> Option<&str> {
        self.source_text.lines().nth(line_num)
    }

    pub fn run(mut self) -> Result<ParseTree<'a>, ParseError> {
        let mut scanner = Scanner::new(self.source_text).peekable();
        let mut buffer: Vec<Token> = vec![];
        let mut start = 0;
        let mut end = 0;
        while let Some(lexeme) = scanner.next() {
            end += lexeme.length;
            match lexeme.kind {
                LexemeKind::Whitespace(w) => match w {
                    WhitespaceKind::Blank => {}, //ignore
                    WhitespaceKind::NewLine => { }, // TODO: 
                }, 
                LexemeKind::Comment(c) => {}, // ignore
                LexemeKind::Literal(l) => match l {
                    LiteralKind::Bool => unreachable!("boolean lexemes should be parsed via terms at this point"),
                    LiteralKind::Integer => {
                        let span = Span::new(start, end);
                        let value = self.get_span(&span)?;
                        buffer.push(Token::new(TokenKind::Literal(LiteralRepr::Integer(value.into())), span));
                    },
                    _ => todo!()
                },
                LexemeKind::Term => {
                    let span = Span::new(start, end);
                    let value = self.get_span(&span)?; 
                    match value {
                        "true" => buffer.push(Token::new(TokenKind::Literal(LiteralRepr::Bool(true)), span)),
                        "false" => buffer.push(Token::new(TokenKind::Literal(LiteralRepr::Bool(false)), span)),
                        other => buffer.push(Token::new(TokenKind::Identifier(Identifier(other.into())), span)),
                    };
                },
                // != !a
                LexemeKind::Exclamation => match scanner.peek() {
                    Some(l) => match l.kind {
                        // !=
                        LexemeKind::Equal => {
                            end += l.length;
                            scanner.next();
                            buffer.push(Token::new(TokenKind::Operator(OperatorKind::Relational(RelationalOpKind::NotEqual)), Span::new(start, end)));
                        },
                        // !a
                        _ => buffer.push(Token::new(TokenKind::Operator(OperatorKind::Logical(LogicalOpKind::LogicNOT)), Span::new(start, end))),
                    },
                    None => return Err(ParseError::BadSyntax(anyhow!("hanging '!' at {:?}", Span::new(start, end)))),
                },
                LexemeKind::Quotation => todo!(),
                LexemeKind::Number => todo!(),
                LexemeKind::Dollar => todo!(),
                LexemeKind::Percent => todo!(),
                LexemeKind::Ampersand => todo!(),
                LexemeKind::Apostrophe => todo!(),
                LexemeKind::LParen => match scanner.peek() {
                    Some(l) => match l.kind {
                        LexemeKind::RParen => {
                            end += l.length;
                            scanner.next();
                            buffer.push(Token::new(TokenKind::Delimiter(DelimiterKind::Paren), Span::new(start, end)));
                        },
                        _ => buffer.push(Token::new(TokenKind::UnpairedDelimiter(UnpairedDelimiter { kind: DelimiterKind::Paren, side: DelimiterSide::Open }), Span::new(start, end))),
                    },
                    None => return Err(ParseError::BadSyntax(anyhow!("hanging '(' at {:?}", Span::new(start, end)))),
                },
                LexemeKind::RParen => {
                    // TODO: close unpaired LPAREN
                    let mut rev_buf_iter = buffer.iter_mut().rev();
                    let mut counter = 1;
                    todo!();
                },
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
                LexemeKind::UNKNOWN => return Err(ParseError::UnknownLexeme),
            }
            start = end;
        }
        let mut parse_tree = ParseTree::new(self.source_id);

        return Ok(parse_tree); 
    }
}

pub trait Parse: Sized {
    fn parse() -> Result<Option<Self>, ParseError>;
}

#[derive(Debug)]
pub enum ParseError {
    BadSyntax(anyhow::Error),
    /// Failed to parse because the pattern does not match.
    PaternMatch,
    UnexpetedValue,
    UnobtainableSpan,
    UnknownLexeme,
}


