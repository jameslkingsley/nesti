use stanza::style::Palette16;

use crate::{content::Content, StyledContent};

#[derive(Debug, Clone)]
pub enum Color {
    None(Content),
    Black(Content),
    Red(Content),
    Green(Content),
    Yellow(Content),
    Blue(Content),
    Magenta(Content),
    Cyan(Content),
    White(Content),
    BrightBlack(Content),
    BrightRed(Content),
    BrightGreen(Content),
    BrightYellow(Content),
    BrightBlue(Content),
    BrightMagenta(Content),
    BrightCyan(Content),
    BrightWhite(Content),
}

impl Color {
    pub fn to_palette(&self) -> Palette16 {
        match self {
            Color::None(_) => Palette16::Default,
            Color::Black(_) => Palette16::Black,
            Color::Red(_) => Palette16::Red,
            Color::Green(_) => Palette16::Green,
            Color::Yellow(_) => Palette16::Yellow,
            Color::Blue(_) => Palette16::Blue,
            Color::Magenta(_) => Palette16::Magenta,
            Color::Cyan(_) => Palette16::Cyan,
            Color::White(_) => Palette16::White,
            Color::BrightBlack(_) => Palette16::BrightBlack,
            Color::BrightRed(_) => Palette16::BrightRed,
            Color::BrightGreen(_) => Palette16::BrightGreen,
            Color::BrightYellow(_) => Palette16::BrightYellow,
            Color::BrightBlue(_) => Palette16::BrightBlue,
            Color::BrightMagenta(_) => Palette16::BrightMagenta,
            Color::BrightCyan(_) => Palette16::BrightCyan,
            Color::BrightWhite(_) => Palette16::BrightWhite,
        }
    }
}

impl From<Color> for StyledContent {
    fn from(color: Color) -> Self {
        let palette = color.to_palette();
        match color {
            Color::None(content) => StyledContent(content, palette),
            Color::Black(content) => StyledContent(content, palette),
            Color::Red(content) => StyledContent(content, palette),
            Color::Green(content) => StyledContent(content, palette),
            Color::Yellow(content) => StyledContent(content, palette),
            Color::Blue(content) => StyledContent(content, palette),
            Color::Magenta(content) => StyledContent(content, palette),
            Color::Cyan(content) => StyledContent(content, palette),
            Color::White(content) => StyledContent(content, palette),
            Color::BrightBlack(content) => StyledContent(content, palette),
            Color::BrightRed(content) => StyledContent(content, palette),
            Color::BrightGreen(content) => StyledContent(content, palette),
            Color::BrightYellow(content) => StyledContent(content, palette),
            Color::BrightBlue(content) => StyledContent(content, palette),
            Color::BrightMagenta(content) => StyledContent(content, palette),
            Color::BrightCyan(content) => StyledContent(content, palette),
            Color::BrightWhite(content) => StyledContent(content, palette),
        }
    }
}
