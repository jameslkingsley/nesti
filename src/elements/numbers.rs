use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

use num_format::{Locale, ToFormattedString};

use super::Element;

/// Display integer.
pub struct Integer<T: IntegerLike>(T);

/// Display integer with a specified unit.
pub struct IntegerUnit<T: IntegerLike, U: Into<String>>(T, U);

/// Display decimal.
pub struct Decimal<T: FloatLike>(T);

/// Display decimal with a specified unit.
pub struct DecimalUnit<T: FloatLike, U: Into<String>>(T, U);

trait IntegerLike {
    fn primitive<T>(self) -> T {
        self
    }
}

impl IntegerLike for i8 {}
impl IntegerLike for i16 {}
impl IntegerLike for i32 {}
impl IntegerLike for i64 {}
impl IntegerLike for i128 {}
impl IntegerLike for isize {}
impl IntegerLike for u8 {}
impl IntegerLike for u16 {}
impl IntegerLike for u32 {}
impl IntegerLike for u64 {}
impl IntegerLike for u128 {}
impl IntegerLike for usize {}

impl IntegerLike for NonZeroI8 {
    fn primitive<T>(self) -> T {
        self.get()
    }
}
impl IntegerLike for NonZeroI16 {
    fn primitive<T>(self) -> T {
        self.get()
    }
}
impl IntegerLike for NonZeroI32 {
    fn primitive<T>(self) -> T {
        self.get()
    }
}
impl IntegerLike for NonZeroI64 {
    fn primitive<T>(self) -> T {
        self.get()
    }
}
impl IntegerLike for NonZeroI128 {
    fn primitive<T>(self) -> T {
        self.get()
    }
}
impl IntegerLike for NonZeroIsize {
    fn primitive<T>(self) -> T {
        self.get()
    }
}
impl IntegerLike for NonZeroU8 {
    fn primitive<T>(self) -> T {
        self.get()
    }
}
impl IntegerLike for NonZeroU16 {
    fn primitive<T>(self) -> T {
        self.get()
    }
}
impl IntegerLike for NonZeroU32 {
    fn primitive<T>(self) -> T {
        self.get()
    }
}
impl IntegerLike for NonZeroU64 {
    fn primitive<T>(self) -> T {
        self.get()
    }
}
impl IntegerLike for NonZeroU128 {
    fn primitive<T>(self) -> T {
        self.get()
    }
}
impl IntegerLike for NonZeroUsize {
    fn primitive<T>(self) -> T {
        self.get()
    }
}

trait FloatLike {}
impl FloatLike for f32 {}
impl FloatLike for f64 {}

impl<T: IntegerLike> Element for Integer<T> {
    type Context = ();

    fn content(&self, ctx: Self::Context) -> String {
        self.0.primitive().to_formatted_string(&Locale::en)
    }
}

impl<T: IntegerLike, U: Into<String>> Element for IntegerUnit<T, U> {
    type Context = ();

    fn content(&self, ctx: Self::Context) -> String {
        format!(
            "{} {}",
            self.0.primitive().to_formatted_string(&Locale::en),
            self.1
        )
    }
}

impl<T: FloatLike> Element for Decimal<T> {
    type Context = ();

    fn content(&self, ctx: Self::Context) -> String {
        format!("{:.2}", self.0)
    }
}

impl<T: FloatLike, U: Into<String>> Element for DecimalUnit<T, U> {
    type Context = ();

    fn content(&self, ctx: Self::Context) -> String {
        format!("{:.2} {}", self.0, self.1)
    }
}
