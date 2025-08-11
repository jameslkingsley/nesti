use bevy_ecs::world::EntityWorldMut;
use stanza::style::{Palette16, TextFg};

use super::{Content, Element, Style, Styles};

impl Element for bool {
    fn spawn(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        entity.insert(Content(String::from(match self {
            true => " TRUE ●",
            false => "FALSE ○",
        })));

        entity.insert(Style(style_override.unwrap_or_else(|| {
            Styles::new().with(TextFg(match self {
                true => Palette16::Green,
                false => Palette16::Red,
            }))
        })));
    }
}
