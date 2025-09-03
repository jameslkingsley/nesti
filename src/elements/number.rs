use std::fmt::Display;

use bevy_ecs::{component::Component, world::EntityWorldMut};
use num_format::{Locale, ToFormattedString};

use super::{Content, Element, Style, Styles};

/// Component that stores the raw numeric value for delta operations
#[derive(Component, Debug, Clone)]
pub enum NumericValue {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    F32(f32),
    F64(f64),
}

/// Display integer.
#[derive(Debug, Clone)]
pub struct Integer<T: IntegerLike>(pub T);

impl<T: IntegerLike> Integer<T> {
    pub fn add(self) -> IntegerDelta<T> {
        IntegerDelta::Add(self.0)
    }
    
    pub fn sub(self) -> IntegerDelta<T> {
        IntegerDelta::Sub(self.0)
    }
    
    pub fn mul(self) -> IntegerDelta<T> {
        IntegerDelta::Mul(self.0)
    }
    
    pub fn div(self) -> IntegerDelta<T> {
        IntegerDelta::Div(self.0)
    }
}

/// Display integer with a specified unit.
#[derive(Debug)]
pub struct IntegerUnit<T: IntegerLike, U: Display>(pub T, pub U);

/// Display decimal.
#[derive(Debug, Clone)]
pub struct Decimal<T: FloatLike>(pub T);

impl<T: FloatLike> Decimal<T> {
    pub fn add(self) -> DecimalDelta<T> {
        DecimalDelta::Add(self.0)
    }
    
    pub fn sub(self) -> DecimalDelta<T> {
        DecimalDelta::Sub(self.0)
    }
    
    pub fn mul(self) -> DecimalDelta<T> {
        DecimalDelta::Mul(self.0)
    }
    
    pub fn div(self) -> DecimalDelta<T> {
        DecimalDelta::Div(self.0)
    }
}

/// Display decimal with a specified unit.
#[derive(Debug)]
pub struct DecimalUnit<T: FloatLike, U: Display>(pub T, pub U);

/// Delta operation for integer values
#[derive(Debug)]
pub enum IntegerDelta<T: IntegerLike> {
    Add(T),
    Sub(T),
    Mul(T),
    Div(T),
}

/// Delta operation for decimal values
#[derive(Debug)]
pub enum DecimalDelta<T: FloatLike> {
    Add(T),
    Sub(T),
    Mul(T),
    Div(T),
}

pub trait IntegerLike: Copy + Clone {
    type Primitive: ToFormattedString;

    fn primitive(self) -> Self::Primitive;
    fn from_numeric_value(value: &NumericValue) -> Option<Self>;
    fn to_numeric_value(self) -> NumericValue;
    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
    fn mul(self, other: Self) -> Self;
    fn div(self, other: Self) -> Self;
}

impl IntegerLike for i8 {
    type Primitive = i8;
    fn primitive(self) -> Self::Primitive {
        self
    }
    fn from_numeric_value(value: &NumericValue) -> Option<Self> {
        match value {
            NumericValue::I8(v) => Some(*v),
            _ => None,
        }
    }
    fn to_numeric_value(self) -> NumericValue {
        NumericValue::I8(self)
    }
    fn add(self, other: Self) -> Self { self + other }
    fn sub(self, other: Self) -> Self { self - other }
    fn mul(self, other: Self) -> Self { self * other }
    fn div(self, other: Self) -> Self { self / other }
}
impl IntegerLike for i16 {
    type Primitive = i16;
    fn primitive(self) -> Self::Primitive {
        self
    }
    fn from_numeric_value(value: &NumericValue) -> Option<Self> {
        match value {
            NumericValue::I16(v) => Some(*v),
            _ => None,
        }
    }
    fn to_numeric_value(self) -> NumericValue {
        NumericValue::I16(self)
    }
    fn add(self, other: Self) -> Self { self + other }
    fn sub(self, other: Self) -> Self { self - other }
    fn mul(self, other: Self) -> Self { self * other }
    fn div(self, other: Self) -> Self { self / other }
}
impl IntegerLike for i32 {
    type Primitive = i32;
    fn primitive(self) -> Self::Primitive {
        self
    }
    fn from_numeric_value(value: &NumericValue) -> Option<Self> {
        match value {
            NumericValue::I32(v) => Some(*v),
            _ => None,
        }
    }
    fn to_numeric_value(self) -> NumericValue {
        NumericValue::I32(self)
    }
    fn add(self, other: Self) -> Self { self + other }
    fn sub(self, other: Self) -> Self { self - other }
    fn mul(self, other: Self) -> Self { self * other }
    fn div(self, other: Self) -> Self { self / other }
}
impl IntegerLike for i64 {
    type Primitive = i64;
    fn primitive(self) -> Self::Primitive {
        self
    }
    fn from_numeric_value(value: &NumericValue) -> Option<Self> {
        match value {
            NumericValue::I64(v) => Some(*v),
            _ => None,
        }
    }
    fn to_numeric_value(self) -> NumericValue {
        NumericValue::I64(self)
    }
    fn add(self, other: Self) -> Self { self + other }
    fn sub(self, other: Self) -> Self { self - other }
    fn mul(self, other: Self) -> Self { self * other }
    fn div(self, other: Self) -> Self { self / other }
}
impl IntegerLike for i128 {
    type Primitive = i128;
    fn primitive(self) -> Self::Primitive {
        self
    }
    fn from_numeric_value(value: &NumericValue) -> Option<Self> {
        match value {
            NumericValue::I128(v) => Some(*v),
            _ => None,
        }
    }
    fn to_numeric_value(self) -> NumericValue {
        NumericValue::I128(self)
    }
    fn add(self, other: Self) -> Self { self + other }
    fn sub(self, other: Self) -> Self { self - other }
    fn mul(self, other: Self) -> Self { self * other }
    fn div(self, other: Self) -> Self { self / other }
}
impl IntegerLike for isize {
    type Primitive = isize;
    fn primitive(self) -> Self::Primitive {
        self
    }
    fn from_numeric_value(value: &NumericValue) -> Option<Self> {
        match value {
            NumericValue::Isize(v) => Some(*v),
            _ => None,
        }
    }
    fn to_numeric_value(self) -> NumericValue {
        NumericValue::Isize(self)
    }
    fn add(self, other: Self) -> Self { self + other }
    fn sub(self, other: Self) -> Self { self - other }
    fn mul(self, other: Self) -> Self { self * other }
    fn div(self, other: Self) -> Self { self / other }
}
impl IntegerLike for u8 {
    type Primitive = u8;
    fn primitive(self) -> Self::Primitive {
        self
    }
    fn from_numeric_value(value: &NumericValue) -> Option<Self> {
        match value {
            NumericValue::U8(v) => Some(*v),
            _ => None,
        }
    }
    fn to_numeric_value(self) -> NumericValue {
        NumericValue::U8(self)
    }
    fn add(self, other: Self) -> Self { self + other }
    fn sub(self, other: Self) -> Self { self - other }
    fn mul(self, other: Self) -> Self { self * other }
    fn div(self, other: Self) -> Self { self / other }
}
impl IntegerLike for u16 {
    type Primitive = u16;
    fn primitive(self) -> Self::Primitive {
        self
    }
    fn from_numeric_value(value: &NumericValue) -> Option<Self> {
        match value {
            NumericValue::U16(v) => Some(*v),
            _ => None,
        }
    }
    fn to_numeric_value(self) -> NumericValue {
        NumericValue::U16(self)
    }
    fn add(self, other: Self) -> Self { self + other }
    fn sub(self, other: Self) -> Self { self - other }
    fn mul(self, other: Self) -> Self { self * other }
    fn div(self, other: Self) -> Self { self / other }
}
impl IntegerLike for u32 {
    type Primitive = u32;
    fn primitive(self) -> Self::Primitive {
        self
    }
    fn from_numeric_value(value: &NumericValue) -> Option<Self> {
        match value {
            NumericValue::U32(v) => Some(*v),
            _ => None,
        }
    }
    fn to_numeric_value(self) -> NumericValue {
        NumericValue::U32(self)
    }
    fn add(self, other: Self) -> Self { self + other }
    fn sub(self, other: Self) -> Self { self - other }
    fn mul(self, other: Self) -> Self { self * other }
    fn div(self, other: Self) -> Self { self / other }
}
impl IntegerLike for u64 {
    type Primitive = u64;
    fn primitive(self) -> Self::Primitive {
        self
    }
    fn from_numeric_value(value: &NumericValue) -> Option<Self> {
        match value {
            NumericValue::U64(v) => Some(*v),
            _ => None,
        }
    }
    fn to_numeric_value(self) -> NumericValue {
        NumericValue::U64(self)
    }
    fn add(self, other: Self) -> Self { self + other }
    fn sub(self, other: Self) -> Self { self - other }
    fn mul(self, other: Self) -> Self { self * other }
    fn div(self, other: Self) -> Self { self / other }
}
impl IntegerLike for u128 {
    type Primitive = u128;
    fn primitive(self) -> Self::Primitive {
        self
    }
    fn from_numeric_value(value: &NumericValue) -> Option<Self> {
        match value {
            NumericValue::U128(v) => Some(*v),
            _ => None,
        }
    }
    fn to_numeric_value(self) -> NumericValue {
        NumericValue::U128(self)
    }
    fn add(self, other: Self) -> Self { self + other }
    fn sub(self, other: Self) -> Self { self - other }
    fn mul(self, other: Self) -> Self { self * other }
    fn div(self, other: Self) -> Self { self / other }
}
impl IntegerLike for usize {
    type Primitive = usize;
    fn primitive(self) -> Self::Primitive {
        self
    }
    fn from_numeric_value(value: &NumericValue) -> Option<Self> {
        match value {
            NumericValue::Usize(v) => Some(*v),
            _ => None,
        }
    }
    fn to_numeric_value(self) -> NumericValue {
        NumericValue::Usize(self)
    }
    fn add(self, other: Self) -> Self { self + other }
    fn sub(self, other: Self) -> Self { self - other }
    fn mul(self, other: Self) -> Self { self * other }
    fn div(self, other: Self) -> Self { self / other }
}


pub trait FloatLike: std::fmt::Display + Copy + Clone {
    fn from_numeric_value(value: &NumericValue) -> Option<Self>;
    fn to_numeric_value(self) -> NumericValue;
    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
    fn mul(self, other: Self) -> Self;
    fn div(self, other: Self) -> Self;
}

impl FloatLike for f32 {
    fn from_numeric_value(value: &NumericValue) -> Option<Self> {
        match value {
            NumericValue::F32(v) => Some(*v),
            _ => None,
        }
    }
    fn to_numeric_value(self) -> NumericValue {
        NumericValue::F32(self)
    }
    fn add(self, other: Self) -> Self { self + other }
    fn sub(self, other: Self) -> Self { self - other }
    fn mul(self, other: Self) -> Self { self * other }
    fn div(self, other: Self) -> Self { self / other }
}

impl FloatLike for f64 {
    fn from_numeric_value(value: &NumericValue) -> Option<Self> {
        match value {
            NumericValue::F64(v) => Some(*v),
            _ => None,
        }
    }
    fn to_numeric_value(self) -> NumericValue {
        NumericValue::F64(self)
    }
    fn add(self, other: Self) -> Self { self + other }
    fn sub(self, other: Self) -> Self { self - other }
    fn mul(self, other: Self) -> Self { self * other }
    fn div(self, other: Self) -> Self { self / other }
}

impl<T: IntegerLike> Element for Integer<T> {
    fn spawn(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        entity.insert(Content(self.0.primitive().to_formatted_string(&Locale::en)));
        entity.insert(NumericValue::from(self.0.to_numeric_value()));
        if let Some(style) = style_override {
            entity.insert(Style(style));
        }
    }
    
    fn tick(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        entity.insert(Content(self.0.primitive().to_formatted_string(&Locale::en)));
        entity.insert(NumericValue::from(self.0.to_numeric_value()));
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
        entity.insert(NumericValue::from(self.0.to_numeric_value()));
        if let Some(style) = style_override {
            entity.insert(Style(style));
        }
    }
    
    fn tick(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        entity.insert(Content(format!("{:.2}", self.0)));
        entity.insert(NumericValue::from(self.0.to_numeric_value()));
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

impl<T: IntegerLike> Element for IntegerDelta<T> {
    fn spawn(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        // Get the existing numeric value if it exists
        let current_value = if let Some(num_value) = entity.get::<NumericValue>() {
            T::from_numeric_value(num_value)
        } else {
            None
        };
        
        // Calculate the new value based on the delta operation
        let new_value = if let Some(current) = current_value {
            match self {
                IntegerDelta::Add(delta) => current.add(*delta),
                IntegerDelta::Sub(delta) => current.sub(*delta),
                IntegerDelta::Mul(delta) => current.mul(*delta),
                IntegerDelta::Div(delta) => current.div(*delta),
            }
        } else {
            // If no existing value, treat as starting from zero and add the delta
            match self {
                IntegerDelta::Add(delta) => *delta,
                IntegerDelta::Sub(_) => panic!("Cannot subtract from non-existent value"),
                IntegerDelta::Mul(_) => panic!("Cannot multiply non-existent value"),
                IntegerDelta::Div(_) => panic!("Cannot divide non-existent value"),
            }
        };
        
        // Store the new value
        entity.insert(NumericValue::from(new_value.to_numeric_value()));
        entity.insert(Content(new_value.primitive().to_formatted_string(&Locale::en)));
        
        if let Some(style) = style_override {
            entity.insert(Style(style));
        }
    }
    
    fn tick(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        self.spawn(entity, style_override);
    }
}

impl<T: FloatLike> Element for DecimalDelta<T> {
    fn spawn(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        // Get the existing numeric value if it exists
        let current_value = if let Some(num_value) = entity.get::<NumericValue>() {
            T::from_numeric_value(num_value)
        } else {
            None
        };
        
        // Calculate the new value based on the delta operation
        let new_value = if let Some(current) = current_value {
            match self {
                DecimalDelta::Add(delta) => current.add(*delta),
                DecimalDelta::Sub(delta) => current.sub(*delta),
                DecimalDelta::Mul(delta) => current.mul(*delta),
                DecimalDelta::Div(delta) => current.div(*delta),
            }
        } else {
            // If no existing value, treat as starting from zero and add the delta
            match self {
                DecimalDelta::Add(delta) => *delta,
                DecimalDelta::Sub(_) => panic!("Cannot subtract from non-existent value"),
                DecimalDelta::Mul(_) => panic!("Cannot multiply non-existent value"),
                DecimalDelta::Div(_) => panic!("Cannot divide non-existent value"),
            }
        };
        
        // Store the new value
        entity.insert(NumericValue::from(new_value.to_numeric_value()));
        entity.insert(Content(format!("{:.2}", new_value)));
        
        if let Some(style) = style_override {
            entity.insert(Style(style));
        }
    }
    
    fn tick(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        self.spawn(entity, style_override);
    }
}
