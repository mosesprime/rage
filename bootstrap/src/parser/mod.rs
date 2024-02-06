//! Rage Bootstrap
//! Parser

use self::{scanner::Scanner, tree::ParseTree};

mod scanner;
pub mod tree;

pub trait Parse: Sized {
    fn parse(parser: &mut Parser<'_>) -> Result<Self, anyhow::Error>;
}

pub fn parse_file(content: &str) -> anyhow::Result<ParseTree> {
    let mut scanner = Scanner::new(content);
    while let Some(l) = scanner.next() {
        println!("{l:?}");
    }
    // TODO: let mut parser = Parser::new(content);
    //ParseTree::parse(&mut parser)
    Ok(ParseTree::new())
}

pub struct Parser<'a> {
    start: usize,
    end: usize,
    content: &'a str,
    scanner: Scanner<'a>,
}

impl<'a> Parser<'a> {
    fn new(content: &'a str) -> Self {
        Self { 
            start: 0,
            end: 0,
            content,
            scanner: Scanner::new(content),
        }  
    }

    fn get_value(&self, start: usize, end: usize) -> Option<&str> {
        self.content.get(start..end)
    }
}

