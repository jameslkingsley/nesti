use bevy_ecs::world::EntityWorldMut;
use bytesize::ByteSize;

use super::{Content, Element, Style, Styles};

#[derive(Debug)]
pub struct Bytes(pub u64);

#[derive(Debug)]
pub struct Kilobytes(pub u64);

#[derive(Debug)]
pub struct Megabytes(pub u64);

#[derive(Debug)]
pub struct Gigabytes(pub u64);

#[derive(Debug)]
pub struct Terabytes(pub u64);

#[derive(Debug)]
pub struct Petabytes(pub u64);

impl Element for Bytes {
    fn spawn(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        entity.insert(Content(ByteSize::b(self.0).to_string()));
        if let Some(style) = style_override {
            entity.insert(Style(style));
        }
    }
}

impl Element for Kilobytes {
    fn spawn(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        entity.insert(Content(ByteSize::kb(self.0).to_string()));
        if let Some(style) = style_override {
            entity.insert(Style(style));
        }
    }
}

impl Element for Megabytes {
    fn spawn(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        entity.insert(Content(ByteSize::mb(self.0).to_string()));
        if let Some(style) = style_override {
            entity.insert(Style(style));
        }
    }
}

impl Element for Gigabytes {
    fn spawn(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        entity.insert(Content(ByteSize::gb(self.0).to_string()));
        if let Some(style) = style_override {
            entity.insert(Style(style));
        }
    }
}

impl Element for Terabytes {
    fn spawn(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        entity.insert(Content(ByteSize::tb(self.0).to_string()));
        if let Some(style) = style_override {
            entity.insert(Style(style));
        }
    }
}

impl Element for Petabytes {
    fn spawn(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        entity.insert(Content(ByteSize::pb(self.0).to_string()));
        if let Some(style) = style_override {
            entity.insert(Style(style));
        }
    }
}
