use bevy_ecs::world::EntityWorldMut;
use num_format::{Locale, ToFormattedString};

use super::{Content, Element, Style, Styles};

#[derive(Debug)]
pub struct Progress {
    pub current: u64,
    pub maximum: u64,
    pub show_percent: bool,
    pub show_values: bool,
    pub show_rate: bool,
}

impl Element for Progress {
    fn spawn(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
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
            bar.push_str(&format!(" {percentage:.1}%"));
        }

        if self.show_values {
            bar.push_str(&format!(
                " ({} / {})",
                self.current.to_formatted_string(&Locale::en),
                self.maximum.to_formatted_string(&Locale::en)
            ));
        }

        // if self.show_rate {
        //     let rate = if let Some(started) = ctx.started {
        //         let elapsed = started.elapsed().as_secs_f64();
        //         if elapsed > 0.0 {
        //             (self.current - ctx.initial_value) as f64 / elapsed
        //         } else {
        //             0.0
        //         }
        //     } else {
        //         0.0
        //     };
        //     bar.push_str(&format!(" {rate:.2}/s"));
        // }

        entity.insert(Content(bar));
        if let Some(style) = style_override {
            entity.insert(Style(style));
        }
    }
}
