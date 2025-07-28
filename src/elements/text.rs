use bevy_ecs::world::EntityWorldMut;

use super::{Content, Element, Style, Styles};

#[derive(Debug)]
pub struct Text<T: Into<String>>(pub T);

impl<T: Into<String> + Clone> Element for Text<T> {
    fn spawn(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        entity.insert(Content(self.0.clone().into()));
        if let Some(style) = style_override {
            entity.insert(Style(style));
        }
    }
}

impl Element for &str {
    fn spawn(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        entity.insert(Content(self.to_string()));
        if let Some(style) = style_override {
            entity.insert(Style(style));
        }
    }
}

impl Element for String {
    fn spawn(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        entity.insert(Content(self.to_owned()));
        if let Some(style) = style_override {
            entity.insert(Style(style));
        }
    }
}
