use crate::{
    content::Content,
    style::{Palette16, TextFg},
    Element, Styles,
};

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

impl Element for Color {
    fn content(&self) -> Content {
        match self {
            Color::None(content)
            | Color::Black(content)
            | Color::Red(content)
            | Color::Green(content)
            | Color::Yellow(content)
            | Color::Blue(content)
            | Color::Magenta(content)
            | Color::Cyan(content)
            | Color::White(content)
            | Color::BrightBlack(content)
            | Color::BrightRed(content)
            | Color::BrightGreen(content)
            | Color::BrightYellow(content)
            | Color::BrightBlue(content)
            | Color::BrightMagenta(content)
            | Color::BrightCyan(content)
            | Color::BrightWhite(content) => content.to_owned(),
        }
    }

    fn styles(&self) -> Styles {
        Styles::new().with(TextFg(self.to_palette()))
    }
}
