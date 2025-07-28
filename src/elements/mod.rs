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

// Type-erased element trait
pub(crate) trait DynElement: Send + Sync + std::fmt::Debug {
    fn content(&self, context: &dyn std::any::Any, global: &GlobalContext) -> String;
    fn styles(&self) -> Styles;
}

// Wrapper to implement DynElement for any Element
pub(crate) struct ElementWrapper<E: Element>(pub(crate) E);

impl<E: Element + Send + Sync> DynElement for ElementWrapper<E>
where
    E::Context: 'static,
{
    fn content(&self, context: &dyn std::any::Any, global: &GlobalContext) -> String {
        let ctx = context
            .downcast_ref::<E::Context>()
            .expect("Context type mismatch");
        self.0.content(ctx, global)
    }

    fn styles(&self) -> Styles {
        self.0.styles()
    }
}

impl<E: Element + Send + Sync> std::fmt::Debug for ElementWrapper<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ElementWrapper").finish()
    }
}
