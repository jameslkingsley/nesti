use bytesize::ByteSize;
use humanize_duration::{prelude::DurationExt, Truncate};
use num_format::{Locale, ToFormattedString};
use std::time::{Duration, Instant};

use crate::{stopwatch::StopwatchFormatter, style::Styles};

pub(crate) mod numbers;

pub trait Element {
    type Context;

    fn content(&self, ctx: Self::Context) -> String;

    fn styles(&self, _ctx: Self::Context) -> Styles {
        Styles::new()
    }
}

pub struct Text(pub String);

impl Element for Text {
    type Context = ();

    fn content(&self, _ctx: Self::Context) -> String {
        self.0.clone()
    }
}

impl From<&str> for Text {
    fn from(s: &str) -> Self {
        Text(s.to_string())
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        Text(s)
    }
}

pub struct Stopwatch {
    pub truncate: Truncate,
}

impl Stopwatch {
    pub fn new(truncate: Truncate) -> Self {
        Self { truncate }
    }
}

impl Element for Stopwatch {
    type Context = Option<Instant>;

    fn content(&self, ctx: Self::Context) -> String {
        let uptime = match ctx {
            Some(start) => start.elapsed(),
            None => Duration::ZERO,
        };

        uptime
            .human_with_format(self.truncate, StopwatchFormatter)
            .to_string()
    }
}

pub struct Bytes(pub u64);
pub struct Kilobytes(pub u64);
pub struct Megabytes(pub u64);
pub struct Gigabytes(pub u64);
pub struct Terabytes(pub u64);
pub struct Petabytes(pub u64);

impl Element for Bytes {
    type Context = ();

    fn content(&self, _ctx: Self::Context) -> String {
        ByteSize::b(self.0).to_string()
    }
}

impl Element for Kilobytes {
    type Context = ();

    fn content(&self, _ctx: Self::Context) -> String {
        ByteSize::kb(self.0).to_string()
    }
}

impl Element for Megabytes {
    type Context = ();

    fn content(&self, _ctx: Self::Context) -> String {
        ByteSize::mb(self.0).to_string()
    }
}

impl Element for Gigabytes {
    type Context = ();

    fn content(&self, _ctx: Self::Context) -> String {
        ByteSize::gb(self.0).to_string()
    }
}

impl Element for Terabytes {
    type Context = ();

    fn content(&self, _ctx: Self::Context) -> String {
        ByteSize::tb(self.0).to_string()
    }
}

impl Element for Petabytes {
    type Context = ();

    fn content(&self, _ctx: Self::Context) -> String {
        ByteSize::pb(self.0).to_string()
    }
}

pub struct Progress {
    pub current: u64,
    pub maximum: u64,
    pub show_percent: bool,
    pub show_values: bool,
    pub show_rate: bool,
}

impl Element for Progress {
    type Context = Duration;

    fn content(&self, uptime: Self::Context) -> String {
        let percentage = if self.maximum > 0 {
            (self.current as f64 / self.maximum as f64) * 100.0
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

        if self.show_percent {
            bar.push_str(&format!(" {:.1}%", percentage));
        }

        if self.show_values {
            bar.push_str(&format!(
                " ({} / {})",
                self.current.to_formatted_string(&Locale::en),
                self.maximum.to_formatted_string(&Locale::en)
            ));
        }

        if self.show_rate {
            bar.push_str(&format!(" {:.2}/s", {
                let elapsed_secs = uptime.as_secs_f64();

                if elapsed_secs == 0.0 {
                    0.0
                } else {
                    self.current as f64 / elapsed_secs
                }
            }));
        }

        bar
    }
}

impl Element for &str {
    type Context = ();

    fn content(&self, _ctx: Self::Context) -> String {
        self.to_string()
    }
}

impl Element for String {
    type Context = ();

    fn content(&self, _ctx: Self::Context) -> String {
        self.clone()
    }
}
