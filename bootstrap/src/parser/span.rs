//! Rage Bootstrap Span

/// Position within source code.
pub struct Location {
    /// Source 0-indexed line position.
    pub line: u32,
    /// Source 0-indexed character offset.
    pub offset: u32,
}

/// Range of positions in source code.
pub struct Span {
    pub start: Location,
    pub end: Location,
}

