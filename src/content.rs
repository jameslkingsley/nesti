use std::{fmt::Debug, ops::Deref, time::Duration};

use bytesize::ByteSize;
use humanize_duration::{prelude::DurationExt, Truncate};
use num_format::{Locale, ToFormattedString};
use stanza::{
    style::{Palette16, Style, Styles as StanzaStyles, TextFg},
    table::{Cell, Content as TableContent},
};

use crate::stopwatch::StopwatchFormatter;

/// The content to display.
#[derive(Debug, Clone)]
pub enum Content {
    /// Display text.
    Text(String),

    /// Display a stopwatch.
    Stopwatch(Truncate),

    /// Display integer.
    Integer(u64),

    /// Display integer with a specified unit.
    IntegerUnit(u64, &'static str),

    /// Display decimal.
    Decimal(f64),

    /// Display decimal with a specified unit.
    DecimalUnit(f64, &'static str),

    /// Display bytes.
    Bytes(u64),

    /// Display kilobytes.
    Kilobytes(u64),

    /// Display megabytes.
    Megabytes(u64),

    /// Display gigabytes.
    Gigabytes(u64),

    /// Display terabytes.
    Terabytes(u64),

    /// Display petabytes.
    Petabytes(u64),

    /// Display a progress bar.
    Progress {
        /// The current progress value.
        current: u64,

        /// The maximum progress value.
        maximum: u64,

        /// Whether to show the percentage of progress.
        show_percent: bool,

        /// Whether to show the current and maximum values.
        show_values: bool,

        /// Whether to show the average rate of progress.
        show_rate: bool,
    },
}

impl StyledContent {
    pub(crate) fn to_cell(&self, uptime: Duration, delta: Duration) -> Cell {
        let (content, color) = (self.0.clone(), self.1.clone());

        let style = Styles::default().with(TextFg(color));

        let content = match content {
            Content::Text(text) => TableContent::Label(text.into()),
            Content::Stopwatch(trunc) => build_stopwatch(uptime, delta, trunc),
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
                show_rate,
            } => build_progress(
                uptime,
                delta,
                current,
                maximum,
                show_percent,
                show_values,
                show_rate,
            ),
        };

        Cell::new(style, content)
    }
}

impl Debug for StyledContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("StyledContent").field(&self.0).finish()
    }
}

impl Element for Content {
    fn content(&self) -> Content {
        self.to_owned()
    }

    fn styles(&self) -> Styles {
        Styles::new()
    }
}

impl Element for &'static str {
    fn content(&self) -> Content {
        Content::Text(self.to_string())
    }

    fn styles(&self) -> Styles {
        Styles::new()
    }
}

impl From<Content> for StyledContent {
    fn from(content: Content) -> Self {
        StyledContent(content, Palette16::Default)
    }
}

impl From<&'static str> for StyledContent {
    fn from(content: &'static str) -> Self {
        StyledContent(Content::Text(content.into()), Palette16::Default)
    }
}

fn build_stopwatch(uptime: Duration, _delta: Duration, trunc: Truncate) -> TableContent {
    TableContent::Label({
        uptime
            .human_with_format(trunc, StopwatchFormatter)
            .to_string()
    })
}

fn build_progress(
    uptime: Duration,
    _delta: Duration,
    current: u64,
    maximum: u64,
    show_percent: bool,
    show_values: bool,
    show_rate: bool,
) -> TableContent {
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

    if show_rate {
        bar.push_str(&format!(" {:.2}/s", {
            let elapsed_secs = uptime.as_secs_f64();

            if elapsed_secs == 0.0 {
                0.0
            } else {
                current as f64 / elapsed_secs
            }
        }));
    }

    TableContent::Label(bar)
}
