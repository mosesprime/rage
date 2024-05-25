use crate::parse::lexeme::{Lexeme, Span};

use super::{error::{LexicalError, LexicalErrorKind}, tokenize::Tokenizer};

#[test]
fn tokenize_dangling_block_comment() {
    if let Some(Err(e)) = Tokenizer::new("//*").next() {
        assert_eq!(e, LexicalError { kind: LexicalErrorKind::DanglingLexeme, span: Span { start: 0, length: 3} })
    } else {
        assert!(false)
    }
}
    
#[test]
fn tokenize_label_snake_case() {
    if let Some(Ok((_, l))) = Tokenizer::new("snake_case").next() {
        assert_eq!(l, Lexeme::Label("snake_case".into()))
    } else {
        assert!(false)
    }
}

#[test]
fn tokenize_label_pascal_case() {
    if let Some(Ok((_, l))) = Tokenizer::new("PascalCase").next() {
        assert_eq!(l, Lexeme::Label("PascalCase".into()))
    } else {
        assert!(false)
    }
}

#[test]
fn tokenize_lable_with_digit() {
    if let Some(Ok((_, l))) = Tokenizer::new("test0").next() {
        assert_eq!(l, Lexeme::Label("test0".into()))
    } else {
        assert!(false)
    }
}

#[test]
fn tokenize_hex() {
    if let Some(Ok((_, l))) = Tokenizer::new("0x0f7f").next() {
        assert_eq!(l, Lexeme::Hex("0x0f7f".into()))
    } else {
        assert!(false)
    }
}

#[test]
fn tokenize_binary() {
    if let Some(Ok((_, l))) = Tokenizer::new("0b01010101").next() {
        assert_eq!(l, Lexeme::Binary("0b01010101".into()))
    } else {
        assert!(false)
    }
}
