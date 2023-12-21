//! Rage Bootstrap Logging

/**
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        LogLevel::Info.print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        LogLevel::Debug.print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        LogLevel::Warn.print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        LogLevel::Error.print(format_args!($($arg)*));
    };
}

pub(crate) use debug;
pub(crate) use error;
pub(crate) use info;
pub(crate) use warn;
*/ // TODO: get macros working?

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
    pub fn wrap_text(&self, s: impl ToString) -> String {
        let fg_code = match self {
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
            TextColor::DEFAULT => return s.to_string(),
        };
        format!("\x1b[0;{}m{}\x1b[0m", fg_code, s.to_string())
    }
}

#[derive(PartialEq)]
pub enum LogLevel {
    Info,
    Debug,
    Warn,
    Error,
}

impl LogLevel {
    pub fn println(&self, s: impl ToString) {
        let level = match self {
            LogLevel::Info => TextColor::Green.wrap_text("INFO"),
            LogLevel::Debug => TextColor::Blue.wrap_text("DEBUG"),
            LogLevel::Warn => TextColor::Yellow.wrap_text("WARN"),
            LogLevel::Error => TextColor::Red.wrap_text("ERROR"),
        };
        println!("[{}] {}", level, s.to_string());
    }
}
