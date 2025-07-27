use humanize_duration::{types::DurationParts, Formatter, Truncate, Unit};

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

unit!(Year, "year", "years");
unit!(Month, "month", "months");
unit!(Day, "day", "days");
unit!(Hour, "h");
unit!(Minute, "m");
unit!(Second, "s");
unit!(Millis, "ms");
unit!(Micro, "µs");
unit!(Nano, "ns");

pub(crate) struct StopwatchFormatter;

impl Formatter for StopwatchFormatter {
    fn get(&self, truncate: Truncate) -> Box<dyn Unit> {
        match truncate {
            Truncate::Nano => Box::new(Nano),
            Truncate::Micro => Box::new(Micro),
            Truncate::Millis => Box::new(Millis),
            Truncate::Second => Box::new(Second),
            Truncate::Minute => Box::new(Minute),
            Truncate::Hour => Box::new(Hour),
            Truncate::Day => Box::new(Day),
            Truncate::Month => Box::new(Month),
            Truncate::Year => Box::new(Year),
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
