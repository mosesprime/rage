//! Rage Bootstrap
//! Parser

use std::fmt::Display;

use crate::parser::tree::SymbolKind;

use self::{scanner::Scanner, lexeme::{LexemeKind, Lexeme}, tree::{ParseTree, Symbol}};

pub mod lexeme;
pub mod scanner;
pub mod tree;

pub struct ParserError<'a> {
    msg: &'a str,
    index: Option<usize>,
    line: Option<&'a str>,
}

impl<'a> Display for ParserError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.index, self.line) {
            (Some(index), Some(line)) => write!(f, "{} : {}\n\t{}\n", self.msg, index, line),
            (None, Some(line)) => write!(f, "{}\n\t{}\n", self.msg, line),
            (Some(index),  None) => write!(f, "{} : {}\n", self.msg, index),
            (None, None) => write!(f, "{}", self.msg),
        }
    }
}

impl<'a> ParserError<'a> {
    fn new(msg: &'a str, index: Option<usize>, line: Option<&'a str>) -> Self {
        Self { msg, index, line }
    }
}

/// 
pub struct Parser<'a> {
    source: &'a str,
    scanner: Scanner<'a>,
    errors: Vec<ParserError<'a>>,
    parse_tree: ParseTree<'a>,
    lookback_buffer: Vec<Lexeme>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            scanner: Scanner::new(source),
            errors: Vec::default(),
            parse_tree: ParseTree::default(),
            lookback_buffer: Vec::default(),
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

    fn parse_string_literal(&mut self, lexeme: Lexeme) {
        if let Some(value) = self.get_value(lexeme.index as usize, lexeme.length as usize) {
            self.parse_tree.add_symbol(Symbol::new(value, SymbolKind::StringLiteral, Some(value.len() as u32), Some(1)));
        } else {
            self.errors.push(ParserError::new("unable to read value", Some(lexeme.index as usize), None));
        }
    }

    pub fn run(mut self) -> Result<ParseTree<'a>, Vec<ParserError<'a>>> {
        let mut lexemes = self.scanner.peekable();
        while let Some(lexeme) = lexemes.next() {
            match lexeme.kind {
                LexemeKind::Space => {},
                LexemeKind::NewLine => {},
                LexemeKind::LineComment => {},
                LexemeKind::BlockComment => {},
                LexemeKind::Documentation => {},
                LexemeKind::StringLiteral => {
                    self.parse_string_literal(lexeme);
                },
                LexemeKind::NumericLiteral => {
                    let first = lexemes.pee
                },
            }
        };
        if self.errors.len() > 0 {
            return Err(self.errors);
        } else {
            return Ok(self.parse_tree);
        }
    }
}
