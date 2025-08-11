use std::time::Instant;

use bevy_ecs::{component::Component, world::EntityWorldMut};
use num_format::{Locale, ToFormattedString};

use super::{Content, Element, Style, Styles, TimeComponent};

#[derive(Component, Debug)]
pub struct ProgressState {
    pub initial_value: u64,
    pub last_value: u64,
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
    fn spawn(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        // Initialize tracking components for rate calculation
        if self.show_rate {
            entity.insert_if_new(TimeComponent(Instant::now()));
            entity.insert_if_new(ProgressState {
                initial_value: self.current,
                last_value: self.current,
            });
        }

        // Generate the progress bar content
        self.render_bar(entity, style_override);
    }

    fn tick(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        // Update the tracking state if we're showing rate
        if self.show_rate {
            if let Some(mut state) = entity.get_mut::<ProgressState>() {
                // Reset tracking if progress went backwards (e.g., started over)
                if self.current < state.last_value {
                    // Reset the initial value and time when progress resets
                    state.initial_value = self.current;
                    state.last_value = self.current;
                    entity.insert(TimeComponent(Instant::now()));
                } else {
                    state.last_value = self.current;
                }
            }
        }

        // Re-render the bar with updated values
        self.render_bar(entity, style_override);
    }
}

impl Progress {
    fn render_bar(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
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

        if self.show_rate {
            let rate = if let (Some(time), Some(state)) =
                (entity.get::<TimeComponent>(), entity.get::<ProgressState>())
            {
                let elapsed = time.elapsed().as_secs_f64();
                if elapsed > 0.0 && self.current >= state.initial_value {
                    (self.current - state.initial_value) as f64 / elapsed
                } else {
                    0.0
                }
            } else {
                0.0
            };

            if rate > 0.0 {
                bar.push_str(&format!(" {rate:.1}/s"));
            } else {
                bar.push_str(" 0.0/s");
            }
        }

        entity.insert(Content(bar));
        if let Some(style) = style_override {
            entity.insert(Style(style));
        }
    }
}
