//! Rage Bootstrap

use std::cmp::{max, min};

/// Represents the index of starting and ending chars.
/// # Safety
/// Uses [u32] for [char] indecies so errors will occur if 
/// file contains more the u32::MAX chars (about 4 billion).
#[derive(Debug)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn as_range(&self) -> std::ops::Range<usize> {
        (self.start as usize)..(self.end as usize)
    }

    pub fn sum(&self, other: &Span) -> Self {
        Self {
            start: min(self.start, other.start),
            end: max(self.end, other.end),
        }
    }
}
