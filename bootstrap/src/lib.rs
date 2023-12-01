//! Rage Bootstrap

pub mod errors;
pub mod lexer;
pub mod parser;
pub mod symbol;
pub mod token;

pub enum TextColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Grey,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl TextColor {
    pub fn wrap_text(s: String, fg_color: TextColor) -> String {
        let fg_code = match fg_color {
            TextColor::Black => 30,
            TextColor::Red => 31,
            TextColor::Green => 32,
            TextColor::Yellow => 33,
            TextColor::Blue => 34,
            TextColor::Magenta => 35,
            TextColor::Cyan => 36,
            TextColor::White => 37,
            TextColor::Grey => 90,
            TextColor::BrightRed => 91,
            TextColor::BrightGreen => 92,
            TextColor::BrightYellow => 93,
            TextColor::BrightBlue => 94,
            TextColor::BrightMagenta => 95,
            TextColor::BrightCyan => 96,
            TextColor::BrightWhite => 97,
        };
        format!("\x1b[0;{}m{}\x1b[0m", fg_code, s)
    }
}

