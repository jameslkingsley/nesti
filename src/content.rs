use bytesize::ByteSize;
use num_format::{Locale, ToFormattedString};
use stanza::{
    style::{Palette16, Styles, TextFg},
    table::{Cell, Content as TableContent},
};

#[derive(Debug, Clone)]
pub enum Content {
    Text(&'static str),
    Integer(u64),
    IntegerUnit(u64, &'static str),
    Decimal(f64),
    DecimalUnit(f64, &'static str),
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
    Terabytes(u64),
    Petabytes(u64),
    Progress {
        current: u64,
        maximum: u64,
        show_percent: bool,
        show_values: bool,
    },
}

#[derive(Debug, Clone)]
pub struct StyledContent(pub Content, pub Palette16);

impl From<Content> for StyledContent {
    fn from(content: Content) -> Self {
        StyledContent(content, Palette16::Default)
    }
}

impl From<&'static str> for StyledContent {
    fn from(content: &'static str) -> Self {
        StyledContent(Content::Text(content), Palette16::Default)
    }
}

impl StyledContent {
    pub(crate) fn to_cell(&self) -> Cell {
        let (content, color) = (self.0.clone(), self.1.clone());

        let style = Styles::default().with(TextFg(color));

        let content = match content {
            Content::Text(text) => TableContent::Label(text.into()),
            Content::Integer(value) => TableContent::Label(value.to_formatted_string(&Locale::en)),
            Content::IntegerUnit(value, unit) => TableContent::Label(format!(
                "{} {}",
                value.to_formatted_string(&Locale::en),
                unit
            )),
            Content::Decimal(value) => TableContent::Label(format!("{:.2}", value)),
            Content::DecimalUnit(value, unit) => {
                TableContent::Label(format!("{:.2} {}", value, unit))
            }
            Content::Bytes(value) => TableContent::Label(ByteSize::b(value).to_string()),
            Content::Kilobytes(value) => TableContent::Label(ByteSize::kb(value).to_string()),
            Content::Megabytes(value) => TableContent::Label(ByteSize::mb(value).to_string()),
            Content::Gigabytes(value) => TableContent::Label(ByteSize::gb(value).to_string()),
            Content::Terabytes(value) => TableContent::Label(ByteSize::tb(value).to_string()),
            Content::Petabytes(value) => TableContent::Label(ByteSize::pb(value).to_string()),
            Content::Progress {
                current,
                maximum,
                show_percent,
                show_values,
            } => {
                let percentage = if maximum > 0 {
                    (current as f64 / maximum as f64) * 100.0
                } else {
                    0.0
                };

                let bar_width = 20;
                let filled = ((percentage / 100.0) * bar_width as f64) as usize;

                let filled_chars = "━".repeat(filled);
                let empty_chars = "┄".repeat(bar_width - filled);

                let mut bar = String::new();

                if filled > 0 {
                    bar.push_str(&filled_chars);
                }

                if bar_width - filled > 0 {
                    bar.push_str(&empty_chars);
                }

                if show_percent {
                    bar.push_str(&format!(" {:.1}%", percentage));
                }

                if show_values {
                    bar.push_str(&format!(
                        " ({} / {})",
                        current.to_formatted_string(&Locale::en),
                        maximum.to_formatted_string(&Locale::en)
                    ));
                }

                TableContent::Label(bar)
            }
        };

        Cell::new(style, content)
    }
}
