use std::{
    fmt::Display,
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    },
};

use bevy_ecs::world::EntityWorldMut;
use num_format::{Locale, ToFormattedString};

use super::{Content, Element, Style, Styles};

/// Display integer.
#[derive(Debug)]
pub struct Integer<T: IntegerLike>(pub T);

/// Display integer with a specified unit.
#[derive(Debug)]
pub struct IntegerUnit<T: IntegerLike, U: Display>(pub T, pub U);

/// Display decimal.
#[derive(Debug)]
pub struct Decimal<T: FloatLike>(pub T);

/// Display decimal with a specified unit.
#[derive(Debug)]
pub struct DecimalUnit<T: FloatLike, U: Display>(pub T, pub U);

pub trait IntegerLike: Copy {
    type Primitive: ToFormattedString;

    fn primitive(self) -> Self::Primitive;
}

impl IntegerLike for i8 {
    type Primitive = i8;
    fn primitive(self) -> Self::Primitive {
        self
    }
}
impl IntegerLike for i16 {
    type Primitive = i16;
    fn primitive(self) -> Self::Primitive {
        self
    }
}
impl IntegerLike for i32 {
    type Primitive = i32;
    fn primitive(self) -> Self::Primitive {
        self
    }
}
impl IntegerLike for i64 {
    type Primitive = i64;
    fn primitive(self) -> Self::Primitive {
        self
    }
}
impl IntegerLike for i128 {
    type Primitive = i128;
    fn primitive(self) -> Self::Primitive {
        self
    }
}
impl IntegerLike for isize {
    type Primitive = isize;
    fn primitive(self) -> Self::Primitive {
        self
    }
}
impl IntegerLike for u8 {
    type Primitive = u8;
    fn primitive(self) -> Self::Primitive {
        self
    }
}
impl IntegerLike for u16 {
    type Primitive = u16;
    fn primitive(self) -> Self::Primitive {
        self
    }
}
impl IntegerLike for u32 {
    type Primitive = u32;
    fn primitive(self) -> Self::Primitive {
        self
    }
}
impl IntegerLike for u64 {
    type Primitive = u64;
    fn primitive(self) -> Self::Primitive {
        self
    }
}
impl IntegerLike for u128 {
    type Primitive = u128;
    fn primitive(self) -> Self::Primitive {
        self
    }
}
impl IntegerLike for usize {
    type Primitive = usize;
    fn primitive(self) -> Self::Primitive {
        self
    }
}

impl IntegerLike for NonZeroI8 {
    type Primitive = i8;
    fn primitive(self) -> Self::Primitive {
        self.get()
    }
}
impl IntegerLike for NonZeroI16 {
    type Primitive = i16;
    fn primitive(self) -> Self::Primitive {
        self.get()
    }
}
impl IntegerLike for NonZeroI32 {
    type Primitive = i32;
    fn primitive(self) -> Self::Primitive {
        self.get()
    }
}

impl IntegerLike for NonZeroI64 {
    type Primitive = i64;
    fn primitive(self) -> Self::Primitive {
        self.get()
    }
}
impl IntegerLike for NonZeroI128 {
    type Primitive = i128;
    fn primitive(self) -> Self::Primitive {
        self.get()
    }
}
impl IntegerLike for NonZeroIsize {
    type Primitive = isize;
    fn primitive(self) -> Self::Primitive {
        self.get()
    }
}
impl IntegerLike for NonZeroU8 {
    type Primitive = u8;
    fn primitive(self) -> Self::Primitive {
        self.get()
    }
}
impl IntegerLike for NonZeroU16 {
    type Primitive = u16;
    fn primitive(self) -> Self::Primitive {
        self.get()
    }
}
impl IntegerLike for NonZeroU32 {
    type Primitive = u32;
    fn primitive(self) -> Self::Primitive {
        self.get()
    }
}
impl IntegerLike for NonZeroU64 {
    type Primitive = u64;
    fn primitive(self) -> Self::Primitive {
        self.get()
    }
}
impl IntegerLike for NonZeroU128 {
    type Primitive = u128;
    fn primitive(self) -> Self::Primitive {
        self.get()
    }
}
impl IntegerLike for NonZeroUsize {
    type Primitive = usize;
    fn primitive(self) -> Self::Primitive {
        self.get()
    }
}

pub trait FloatLike: std::fmt::Display {}
impl FloatLike for f32 {}
impl FloatLike for f64 {}

impl<T: IntegerLike> Element for Integer<T> {
    fn spawn(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        entity.insert(Content(self.0.primitive().to_formatted_string(&Locale::en)));
        if let Some(style) = style_override {
            entity.insert(Style(style));
        }
    }
}

impl<T: IntegerLike, U: Display> Element for IntegerUnit<T, U> {
    fn spawn(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        entity.insert(Content(format!(
            "{} {}",
            self.0.primitive().to_formatted_string(&Locale::en),
            self.1
        )));

        if let Some(style) = style_override {
            entity.insert(Style(style));
        }
    }
}

impl<T: FloatLike> Element for Decimal<T> {
    fn spawn(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        entity.insert(Content(format!("{:.2}", self.0)));
        if let Some(style) = style_override {
            entity.insert(Style(style));
        }
    }
}

impl<T: FloatLike, U: Display> Element for DecimalUnit<T, U> {
    fn spawn(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        entity.insert(Content(format!("{:.2} {}", self.0, self.1)));
        if let Some(style) = style_override {
            entity.insert(Style(style));
        }
    }
}
