//! Rage Bootstrap
//! Parser

use std::{fmt::{Display, Debug}, collections::VecDeque};

use crate::syntax::token::{Token, TokenKind};

use self::{scanner::Scanner, lexeme::{Lexeme, LexemeKind}, tree::ParseTree};

pub mod lexeme;
pub mod scanner;
pub mod tree;

/// Front-end of the compiler.
/// Performs lexical analysis, syntax analysis, and semantic analysis on the source code.
pub struct Parser<'a> {
    source: String,
    tokens: Vec<Token>,
    parse_tree: ParseTree<'a>,
}

impl<'a> Debug for Parser<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: change Parser display()
        write!(f, "work in progress")
    }
}

impl<'a> Parser<'a> {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::default(),
            parse_tree: ParseTree { declarations: Vec::default() },
        }
    }
    
    /// Gets a slice of the source if able.
    fn get_value(&self, index: usize, length: usize) -> Option<&str> {
        self.source.get(index..(index + length))
    }

    /// Get the line of source code where the index is located.
    fn get_line_from_index(&self, index: usize) -> Option<&str> {
        self.source.lines().find(|line| {
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
        self.source.lines().nth(line_num)
    }

    fn tokenize(&mut self) {
        let mut cursor = 0;
        let mut lexemes = Scanner::new(self.source.as_str()).peekable();
        while let Some(lexeme) = lexemes.next() {
            match lexeme.kind {
                LexemeKind::Whitespace(w) => {
                    cursor += lexeme.length as usize;
                },
                LexemeKind::Comment(c) => {
                    cursor += lexeme.length as usize;
                },
                LexemeKind::Literal(l) => {
                    cursor += lexeme.length as usize;
                },
                LexemeKind::Term => {
                    let peeked = lexemes.peek();
                    todo!();
                    /*match peeked.kind {
                        LexemeKind::Colon => {}, // path
                        LexemeKind::Dot => {}, // member 
                        _ => todo!(),
                    }*/
                },
                //
                LexemeKind::Number => {
                    if let Some(l2) = lexemes.peek() {
                        match l2.kind {
                            LexemeKind::Term => todo!(), // directive
                            _ => todo!()
                        }
                    }
                },
                _ => todo!(),
            }
        }
    }

    pub fn run(mut self) -> ParseTree<'a> {
        // TODO: do parsing
        log::error!("parser is a work in process");
        self.parse_tree
    }
}
