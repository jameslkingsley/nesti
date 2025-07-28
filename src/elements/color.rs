use crate::{
    elements::Element,
    style::{Palette16, Styles, TextFg},
};

macro_rules! color_struct {
    ($name:ident, $palette:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name<T: Element>(pub T);

        impl<T: Element> Element for $name<T> {
            type Context = T::Context;

            fn content(&self, ctx: &Self::Context) -> String {
                self.0.content(ctx)
            }

            fn styles(&self) -> Styles {
                self.0.styles().with(TextFg($palette))
            }
        }
    };
}

color_struct!(Black, Palette16::Black);
color_struct!(Red, Palette16::Red);
color_struct!(Green, Palette16::Green);
color_struct!(Yellow, Palette16::Yellow);
color_struct!(Blue, Palette16::Blue);
color_struct!(Magenta, Palette16::Magenta);
color_struct!(Cyan, Palette16::Cyan);
color_struct!(White, Palette16::White);
color_struct!(BrightBlack, Palette16::BrightBlack);
color_struct!(BrightRed, Palette16::BrightRed);
color_struct!(BrightGreen, Palette16::BrightGreen);
color_struct!(BrightYellow, Palette16::BrightYellow);
color_struct!(BrightBlue, Palette16::BrightBlue);
color_struct!(BrightMagenta, Palette16::BrightMagenta);
color_struct!(BrightCyan, Palette16::BrightCyan);
color_struct!(BrightWhite, Palette16::BrightWhite);
