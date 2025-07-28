use stanza::style::{Palette16, TextFg};

use super::{Element, Styles};

impl Element for bool {
    type Context = ();

    fn content(&self, _ctx: &Self::Context, _global: &super::GlobalContext) -> String {
        match self {
            true => "●".to_string(),
            false => "○".to_string(),
        }
    }

    fn styles(&self) -> Styles {
        Styles::new().with(TextFg(match self {
            true => Palette16::Green,
            false => Palette16::Red,
        }))
    }
}
