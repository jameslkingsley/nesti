use std::{
    ops::Deref,
    time::{Duration, Instant},
};

use bevy_ecs::{component::Component, world::EntityWorldMut};
use humanize_duration::{prelude::DurationExt, types::DurationParts, Formatter, Truncate, Unit};

use super::{Content, Element, Style, Styles};

macro_rules! unit {
    ($unit_name:tt, $one:expr) => {
        pub struct $unit_name;
        impl humanize_duration::Unit for $unit_name {
            fn one(&self) -> &'static str {
                $one
            }

            fn many(&self) -> &'static str {
                $one
            }

            fn format(
                &self,
                f: &mut std::fmt::Formatter<'_>,
                value: u64,
                allow_zero: bool,
                started: &mut bool,
            ) -> std::fmt::Result {
                if value != 0 || (allow_zero && !*started) {
                    if *started {
                        f.write_str(" ")?;
                    }
                    write!(f, "{:>3}{}", value, $one)?;
                    *started = true;
                }
                Ok(())
            }
        }
    };
    ($unit_name:tt, $one:expr, $many:expr) => {
        pub struct $unit_name;
        impl humanize_duration::Unit for $unit_name {
            fn one(&self) -> &'static str {
                $one
            }

            fn many(&self) -> &'static str {
                $many
            }

            fn format(
                &self,
                f: &mut std::fmt::Formatter<'_>,
                value: u64,
                allow_zero: bool,
                started: &mut bool,
            ) -> std::fmt::Result {
                if value != 0 || (allow_zero && !*started) {
                    if *started {
                        f.write_str(" ")?;
                    }
                    if value > 1 || value == 0 {
                        write!(f, "{:>3}{}", value, $many)?;
                    } else {
                        write!(f, "{:>3}{}", value, $one)?;
                    }
                    *started = true;
                }
                Ok(())
            }
        }
    };
}

unit!(YearFormat, "year", "years");
unit!(MonthFormat, "month", "months");
unit!(DayFormat, "day", "days");
unit!(HourFormat, "h");
unit!(MinuteFormat, "m");
unit!(SecondFormat, "s");
unit!(MillisFormat, "ms");
unit!(MicroFormat, "µs");
unit!(NanoFormat, "ns");

#[derive(Debug)]
pub struct Timer(pub Truncate);

#[derive(Debug)]
pub struct EndTimer;

pub(crate) struct TimerFormatter;

#[derive(Component, Debug)]
pub struct TimeComponent(pub Instant);

#[derive(Component, Debug)]
pub struct StoppedTimer {
    pub elapsed: Duration,
}

impl Deref for TimeComponent {
    type Target = Instant;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Formatter for TimerFormatter {
    fn get(&self, truncate: Truncate) -> Box<dyn Unit> {
        match truncate {
            Truncate::Nano => Box::new(NanoFormat),
            Truncate::Micro => Box::new(MicroFormat),
            Truncate::Millis => Box::new(MillisFormat),
            Truncate::Second => Box::new(SecondFormat),
            Truncate::Minute => Box::new(MinuteFormat),
            Truncate::Hour => Box::new(HourFormat),
            Truncate::Day => Box::new(DayFormat),
            Truncate::Month => Box::new(MonthFormat),
            Truncate::Year => Box::new(YearFormat),
        }
    }

    fn format(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        parts: DurationParts,
        truncate: Truncate,
    ) -> core::fmt::Result {
        self.format_default(f, parts, truncate)
    }
}

impl Element for Timer {
    fn spawn(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        // If the timer was stopped, restart it
        if entity.contains::<StoppedTimer>() {
            entity.remove::<StoppedTimer>();
            entity.insert(TimeComponent(Instant::now()));
        } else {
            entity.insert_if_new(TimeComponent(Instant::now()));
        }

        entity.insert(Content(
            Duration::ZERO
                .human_with_format(self.0, TimerFormatter)
                .to_string(),
        ));

        if let Some(style) = style_override {
            entity.insert(Style(style));
        }
    }

    fn tick(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        // If the timer was stopped, restart it
        if entity.contains::<StoppedTimer>() {
            entity.remove::<StoppedTimer>();
            entity.insert(TimeComponent(Instant::now()));
        }
        
        // Display the current elapsed time
        if let Some(time_component) = entity.get::<TimeComponent>() {
            let uptime = time_component.elapsed();
            let content = uptime.human_with_format(self.0, TimerFormatter).to_string();
            entity.insert(Content(content));
        }

        if let Some(style) = style_override {
            entity.insert(Style(style));
        }
    }
}

impl Element for EndTimer {
    fn spawn(&self, entity: &mut EntityWorldMut, _style_override: Option<Styles>) {
        // If there's an active timer, stop it and freeze the elapsed time
        if let Some(time_component) = entity.get::<TimeComponent>() {
            let elapsed = time_component.elapsed();
            entity.insert(StoppedTimer { elapsed });
            entity.remove::<TimeComponent>();
        }
        // If already stopped, do nothing
    }
}
