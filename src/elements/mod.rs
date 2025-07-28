use super::style::Styles;

mod bool;
mod map;
mod number;
mod progress;
mod size;
mod text;
mod timer;
mod vector;

// pub use self::bool::*;
// pub use self::map::*;
pub use self::number::*;
pub use self::progress::*;
pub use self::size::*;
pub use self::text::*;
pub use self::timer::*;
// pub use self::vector::*;

pub trait Element {
    type Context;

    fn content(&self, ctx: Self::Context) -> String;

    fn styles(&self, _ctx: Self::Context) -> Styles {
        Styles::new()
    }
}
