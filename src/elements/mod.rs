use super::style::Styles;
use super::tree::GlobalContext;

mod bool;
mod color;
mod map;
mod number;
mod progress;
mod size;
mod text;
mod timer;
mod vector;

// pub use self::bool::*;
pub use self::color::*;
// pub use self::map::*;
pub use self::number::*;
pub use self::progress::*;
pub use self::size::*;
pub use self::text::*;
pub use self::timer::*;
// pub use self::vector::*;

pub trait Element {
    type Context: Default + Send + Sync + 'static;

    fn content(&self, ctx: &Self::Context, global: &GlobalContext) -> String;

    fn styles(&self) -> Styles {
        Styles::new()
    }
}
