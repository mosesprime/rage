//! Rage Bootstrap
//! Parser

use std::fmt::Display;

use crate::{syntax::token::Token};

use self::{scanner::Scanner, lexeme::Lexeme};

pub mod lexeme;
pub mod scanner;
pub mod tree;

#[derive(Debug)]
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

/// Front-end of the compiler.
/// Performs lexical analysis, syntax analysis, and semantic analysis on the source code.
pub struct Parser<'a> {
    source: &'a str,
    scanner: Scanner<'a>,
    errors: Vec<ParserError<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            scanner: Scanner::new(source),
            errors: Vec::default(),
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

    pub fn run(mut self) -> Vec<Lexeme> {
        self.scanner.collect()
    }
}
