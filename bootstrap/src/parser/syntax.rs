//! Rage Bootstrap
//! Parser Syntax Analysis

use anyhow::anyhow;

use crate::syntax::token::{Token, TokenKind, Comment, Literal};

use super::{lexeme::{Lexeme, LexemeKind}, scanner::Scanner};

pub struct SyntaxAnalyzer<'a> {
    buffer: Vec<Token>,
    lexemes: std::iter::Peekable<Scanner<'a>>
}

impl<'a> SyntaxAnalyzer<'a> {
    pub fn new(scanner: Scanner<'a>) -> Self {
        Self { 
            buffer: Vec::default(),
            lexemes: scanner.peekable()
        }
    }
    
    fn handle_number(&mut self, first: Lexeme) -> Token {
        //todo!()
        Token::new(TokenKind::Literal(Literal::Integer), Some((first.index, first.index + first.length)))
    }

    fn handle_identifer(&mut self, first: Lexeme) -> Token {
        // TODO:
        Token::new(TokenKind::Identifier, Some((first.index, first.index + first.length)))
    }

    fn handle_symbol(&mut self, first: Lexeme) -> Token {
        //todo!()
        Token::new(TokenKind::UNKNOWN, Some((first.index, first.index + first.length)))
    }

    pub fn run(&mut self) -> Vec<Token> {//impl Iterator<Item = &Token> + '_ {
        while let Some(first) = self.lexemes.next() {
            match first.kind {
                LexemeKind::Space => {},
                LexemeKind::NewLine => {},
                LexemeKind::LineComment => {
                    self.buffer.push(Token::new(TokenKind::Comment(Comment::Line), Some((first.index, first.index + first.length))))
                },
                LexemeKind::BlockComment => {
                    self.buffer.push(Token::new(TokenKind::Comment(Comment::Block), Some((first.index, first.index + first.length))))
                },
                LexemeKind::Documentation => {
                    self.buffer.push(Token::new(TokenKind::Comment(Comment::Documentation), Some((first.index, first.index + first.length))))
                },
                LexemeKind::StringLiteral => {
                    self.buffer.push(Token::new(TokenKind::Literal(Literal::String), Some((first.index, first.index + first.length))))
                },
                LexemeKind::NumericLiteral => {
                    let tok = self.handle_number(first);
                    self.buffer.push(tok);
                },
                LexemeKind::BooleanLiteral => {
                    self.buffer.push(Token::new(TokenKind::Literal(Literal::Bool), Some((first.index, first.index + first.length))))
                },
                LexemeKind::CharLiteral => {
                    self.buffer.push(Token::new(TokenKind::Literal(Literal::Char), Some((first.index, first.index + first.length))))
                },
                LexemeKind::Identifier => {
                    let tok = self.handle_identifer(first);
                    self.buffer.push(tok);
                },
                LexemeKind::UNKNOWN => {
                    self.buffer.push(Token::new(TokenKind::UNKNOWN, Some((first.index, first.index + first.length))))
                }
                _ => {
                    let tok = self.handle_symbol(first);
                    self.buffer.push(tok);
                },
            }; 
        }
        return self.buffer.clone();
    }
}
