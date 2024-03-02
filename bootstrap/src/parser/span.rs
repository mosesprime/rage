//! Rage Bootstrap Span

/// Range of char indecies in source code.
#[derive(Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

///
#[derive(Debug)]
pub struct Location {
    pub line: usize,
    pub char: usize,
}

impl Location {
    pub fn new(line: usize, char: usize) -> Self {
        Self { line, char }
    }
}
