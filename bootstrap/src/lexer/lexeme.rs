const DEFAULT_LEXEME_STORE_CAPACITY: usize = 1_000;

/// A scanned yet unparsed token.
#[derive(Debug)]
pub struct Lexeme {
    pub kind: LexemeKind,
    pub index: u32,
    pub length: u32,
}

impl Lexeme {
    pub fn new(kind: LexemeKind, index: u32, length: u32) -> Self {
        Self {
            kind,
            index,
            length,
        }
    }
}

pub struct LexemeStore {
    kinds: Vec<LexemeKind>,
    indcies: Vec<u32>,
    lengths: Vec<u32>,
}

impl LexemeStore {
    pub fn new() -> Self {
        Self {
            kinds: Vec::with_capacity(DEFAULT_LEXEME_STORE_CAPACITY),
            indcies: Vec::with_capacity(DEFAULT_LEXEME_STORE_CAPACITY),
            lengths: Vec::with_capacity(DEFAULT_LEXEME_STORE_CAPACITY),
        }
    }

    pub fn push(&mut self, lexeme: Lexeme) {
        self.kinds.push(lexeme.kind);
        self.indcies.push(lexeme.index);
        self.lengths.push(lexeme.length);
    }

    pub fn get(&self, index: usize) -> Option<&Lexeme> {
        let _kind = self.kinds.get(index)?;
        let _index = self.indcies.get(index)?;
        let _length = self.lengths.get(index)?;
        Some(&Lexeme::new(*_kind, *_index, *_length))
    }

    pub fn get_mut(&mut self, index: usize) -> Option<Lexeme> {
        let _kind = self.kinds.get_mut(index)?;
        let _index = self.indcies.get_mut(index)?;
        let _length = self.lengths.get_mut(index)?;
        Some(Lexeme::new(*_kind, *_index, *_length))
    }
}

#[derive(Debug, PartialEq)]
pub enum LexemeKind {
    NewLine,
    Whitespace,
    Number,
    Term,
    Symbol,

    UNKNOWN,
}
