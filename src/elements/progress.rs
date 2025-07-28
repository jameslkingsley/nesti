use std::time::Instant;

use num_format::{Locale, ToFormattedString};

use super::Element;

#[derive(Debug)]
pub struct ProgressContext {
    pub started: Option<Instant>,
    pub initial_value: u64,
}

impl Default for ProgressContext {
    fn default() -> Self {
        Self {
            started: Some(Instant::now()),
            initial_value: 0,
        }
    }
}

#[derive(Debug)]
pub struct Progress {
    pub current: u64,
    pub maximum: u64,
    pub show_percent: bool,
    pub show_values: bool,
    pub show_rate: bool,
}

impl Element for Progress {
    type Context = ProgressContext;

    fn content(&self, ctx: &Self::Context) -> String {
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
            let rate = if let Some(started) = ctx.started {
                let elapsed = started.elapsed().as_secs_f64();
                if elapsed > 0.0 {
                    (self.current - ctx.initial_value) as f64 / elapsed
                } else {
                    0.0
                }
            } else {
                0.0
            };
            bar.push_str(&format!(" {:.2}/s", rate));
        }

        bar
    }
}
