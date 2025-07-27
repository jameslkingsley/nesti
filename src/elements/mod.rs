use crate::style::Styles;

pub(crate) mod numbers;

pub trait Element {
    type Context;

    fn content(&self, ctx: Self::Context) -> String;

    fn styles(&self, ctx: Self::Context) -> Styles {
        Styles::new()
    }
}
