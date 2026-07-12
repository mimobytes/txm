#![allow(unused)]

use std::fmt;

#[cfg(feature = "fancy")]
use crate::ParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    attrs: u16,
    fg: Color,
    bg: Color,
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

const EMPTY: Style = Style {
    attrs: 0,
    fg: Color::None,
    bg: Color::None,
};

const BOLD: u16 = 1 << 0;
const ITALIC: u16 = 1 << 1;
const UNDERLINE: u16 = 1 << 2;
const DIM: u16 = 1 << 3;

impl Style {
    pub const fn new() -> Self {
        EMPTY
    }

    pub fn bold(mut self) -> Self {
        self.attrs |= BOLD;
        self
    }

    pub fn italic(mut self) -> Self {
        self.attrs |= ITALIC;
        self
    }

    pub fn underline(mut self) -> Self {
        self.attrs |= UNDERLINE;
        self
    }

    pub fn dim(mut self) -> Self {
        self.attrs |= DIM;
        self
    }

    pub fn fg(mut self, color: Color) -> Self {
        self.fg = color;
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    pub fn is_bold(self) -> bool {
        self.attrs & BOLD != 0
    }

    pub fn is_italic(self) -> bool {
        self.attrs & ITALIC != 0
    }

    pub fn is_underline(self) -> bool {
        self.attrs & UNDERLINE != 0
    }

    pub fn is_dim(self) -> bool {
        self.attrs & DIM != 0
    }

    pub fn fg_color(self) -> Color {
        self.fg
    }

    pub fn bg_color(self) -> Color {
        self.bg
    }

    pub fn merge(self, other: Style) -> Style {
        let mut style = self;
        style.attrs |= other.attrs;

        if other.fg != Color::None {
            style.fg = other.fg;
        }

        if other.bg != Color::None {
            style.bg = other.bg;
        }

        style
    }

    pub fn is_empty(self) -> bool {
        self.attrs == 0 && self.fg == Color::None && self.bg == Color::None
    }

    pub fn write_ansi_prefix(&self, f: &mut impl fmt::Write) -> fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        write!(f, "\x1b[")?;
        let mut first = true;

        if self.is_bold() {
            write!(f, "1")?;
            first = false;
        }
        if self.is_italic() {
            write!(f, "{}{}", if first { "" } else { ";" }, "3")?;
            first = false;
        }
        if self.is_underline() {
            write!(f, "{}{}", if first { "" } else { ";" }, "4")?;
            first = false;
        }
        if self.is_dim() {
            write!(f, "{}{}", if first { "" } else { ";" }, "2")?;
            first = false;
        }

        Self::write_color_code(f, &mut first, self.fg_color(), false)?;
        Self::write_color_code(f, &mut first, self.bg_color(), true)?;

        write!(f, "m")?;
        Ok(())
    }

    fn write_color_code(
        f: &mut impl fmt::Write,
        first: &mut bool,
        color: Color,
        is_background: bool,
    ) -> fmt::Result {
        match color {
            Color::None => Ok(()),
            Color::Black => Self::write_color_code_value(f, first, if is_background { 40 } else { 30 }),
            Color::Red => Self::write_color_code_value(f, first, if is_background { 41 } else { 31 }),
            Color::Green => Self::write_color_code_value(f, first, if is_background { 42 } else { 32 }),
            Color::Yellow => Self::write_color_code_value(f, first, if is_background { 43 } else { 33 }),
            Color::Blue => Self::write_color_code_value(f, first, if is_background { 44 } else { 34 }),
            Color::Magenta => Self::write_color_code_value(f, first, if is_background { 45 } else { 35 }),
            Color::Cyan => Self::write_color_code_value(f, first, if is_background { 46 } else { 36 }),
            Color::White => Self::write_color_code_value(f, first, if is_background { 47 } else { 37 }),
            Color::BrightBlack => Self::write_color_code_value(f, first, if is_background { 100 } else { 90 }),
            Color::BrightRed => Self::write_color_code_value(f, first, if is_background { 101 } else { 91 }),
            Color::BrightGreen => Self::write_color_code_value(f, first, if is_background { 102 } else { 92 }),
            Color::BrightYellow => Self::write_color_code_value(f, first, if is_background { 103 } else { 93 }),
            Color::BrightBlue => Self::write_color_code_value(f, first, if is_background { 104 } else { 94 }),
            Color::BrightMagenta => Self::write_color_code_value(f, first, if is_background { 105 } else { 95 }),
            Color::BrightCyan => Self::write_color_code_value(f, first, if is_background { 106 } else { 96 }),
            Color::BrightWhite => Self::write_color_code_value(f, first, if is_background { 107 } else { 97 }),
            Color::Ansi256(index) => {
                write!(f, "{}{};5;{index}", if *first { "" } else { ";" }, if is_background { 48 } else { 38 })?;
                *first = false;
                Ok(())
            }
            Color::Rgb { r, g, b } => {
                write!(f, "{}{};2;{r};{g};{b}", if *first { "" } else { ";" }, if is_background { 48 } else { 38 })?;
                *first = false;
                Ok(())
            }
        }
    }

    fn write_color_code_value(
        f: &mut impl fmt::Write,
        first: &mut bool,
        code: u16,
    ) -> fmt::Result {
        write!(f, "{}{}", if *first { "" } else { ";" }, code)?;
        *first = false;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    None,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Ansi256(u8),
    Rgb { r: u8, g: u8, b: u8 },
}

impl Color {
    pub const fn from_u16(v: u16) -> Color {
        use Color::*;
        match v {
            1 => Black,
            2 => Red,
            3 => Green,
            4 => Yellow,
            5 => Blue,
            6 => Magenta,
            7 => Cyan,
            8 => White,
            9 => BrightBlack,
            10 => BrightRed,
            11 => BrightGreen,
            12 => BrightYellow,
            13 => BrightBlue,
            14 => BrightMagenta,
            15 => BrightCyan,
            16 => BrightWhite,
            _ => Color::None,
        }
    }

    pub fn from_hex(s: &str) -> Option<Self> {
        let value = s.strip_prefix('#').unwrap_or(s);
        let hex = match value.len() {
            3 | 4 => value
                .chars()
                .flat_map(|c| [c, c])
                .collect::<String>(),
            6 | 8 => value.to_string(),
            _ => return None,
        };

        let value = u32::from_str_radix(&hex, 16).ok()?;
        Some(Self::Rgb {
            r: ((value >> 16) & 0xFF) as u8,
            g: ((value >> 8) & 0xFF) as u8,
            b: (value & 0xFF) as u8,
        })
    }

    pub fn from_name(s: &str) -> Option<Self> {
        Some(match s.to_ascii_lowercase().as_str() {
            "none" => Self::None,
            "black" => Self::Black,
            "red" => Self::Red,
            "green" => Self::Green,
            "yellow" => Self::Yellow,
            "blue" => Self::Blue,
            "magenta" => Self::Magenta,
            "cyan" => Self::Cyan,
            "white" => Self::White,
            "brightblack" | "gray" | "grey" => Self::BrightBlack,
            "brightred" => Self::BrightRed,
            "brightgreen" => Self::BrightGreen,
            "brightyellow" => Self::BrightYellow,
            "brightblue" => Self::BrightBlue,
            "brightmagenta" => Self::BrightMagenta,
            "brightcyan" => Self::BrightCyan,
            "brightwhite" => Self::BrightWhite,
            "orange" => Self::Rgb { r: 255, g: 165, b: 0 },
            "purple" => Self::Rgb { r: 128, g: 0, b: 128 },
            "pink" => Self::Rgb { r: 255, g: 105, b: 180 },
            "brown" => Self::Rgb { r: 165, g: 42, b: 42 },
            "olive" => Self::Rgb { r: 128, g: 128, b: 0 },
            "teal" => Self::Rgb { r: 0, g: 128, b: 128 },
            "navy" => Self::Rgb { r: 0, g: 0, b: 128 },
            "maroon" => Self::Rgb { r: 128, g: 0, b: 0 },
            "silver" => Self::Rgb { r: 192, g: 192, b: 192 },
            "gold" => Self::Rgb { r: 255, g: 215, b: 0 },
            _ => return None,
        })
    }
}

#[cfg(feature = "fancy")]
pub fn parse_color(s: &str) -> Result<Color, ParseError> {
    let trimmed = s.trim();
    if let Some(color) = Color::from_name(trimmed) {
        return Ok(color);
    }
    if let Some(color) = Color::from_hex(trimmed) {
        return Ok(color);
    }

    Err(ParseError(format!("invalid color name: {s}")))
}
