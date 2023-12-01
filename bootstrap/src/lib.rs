//! Rage Bootstrap

use std::fmt::Display;

pub mod errors;
pub mod lexer;
pub mod parser;
//pub mod symbol;
pub mod token;

pub enum LogLevel {
    Info,
    Debug,
    Warn,
    Error,
    Panic,
}
/**
impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            LogLevel::Info => TextColor::wrap_text("INFO", TextColor::BrightCyan),
            LogLevel::Debug => TextColor::wrap_text("DEBUG", TextColor::BrightMagenta),
            LogLevel::Warn => TextColor::wrap_text("WARN", TextColor::BrightYellow),
            LogLevel::Error => TextColor::wrap_text("ERROR", TextColor::BrightRed),
            LogLevel::Panic => TextColor::wrap_text("PANIC", TextColor::BrightRed),
        };
        write!(f, "{}", msg)
    }
}
*/
impl LogLevel {
    pub fn println(&self, msg: impl ToString) {
        let log_level = match self {
            LogLevel::Info => TextColor::wrap_text("INFO", TextColor::BrightCyan),
            LogLevel::Debug => TextColor::wrap_text("DEBUG", TextColor::BrightMagenta),
            LogLevel::Warn => TextColor::wrap_text("WARN", TextColor::BrightYellow),
            LogLevel::Error => TextColor::wrap_text("ERROR", TextColor::BrightRed),
            LogLevel::Panic => TextColor::wrap_text("PANIC", TextColor::BrightRed),
        };
        println!("[Bootstrap {}] {}", log_level, msg.to_string())
    }
}

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
    DEFAULT,
}

impl TextColor {
    pub fn wrap_text(s: impl ToString, fg_color: TextColor) -> String {
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
            TextColor::DEFAULT => 97,
        };
        /**let bg_code = match bg_color {
            TextColor::Black => 40,
            TextColor::Red => 41,
            TextColor::Green => 42,
            TextColor::Yellow => 43,
            TextColor::Blue => 44,
            TextColor::Magenta => 45,
            TextColor::Cyan => 46,
            TextColor::White => 47,
            TextColor::Grey => 100,
            TextColor::BrightRed => 101,
            TextColor::BrightGreen => 102,
            TextColor::BrightYellow => 103,
            TextColor::BrightBlue => 104,
            TextColor::BrightMagenta => 105,
            TextColor::BrightCyan => 106,
            TextColor::BrightWhite => 107,
            TextColor::DEFAULT => 40,
        };*/
        format!("\x1b[0;{}m{}\x1b[0m", fg_code, s.to_string())
    }
}

