use crate::style::{Palette16, TextFg};
use bevy_ecs::world::EntityWorldMut;

use super::{Element, Styles};

macro_rules! color {
    ($name:ident, $palette:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name<T: Element>(pub T);

        impl<T: Element> Element for $name<T> {
            fn spawn(&self, entity: &mut EntityWorldMut, _style_override: Option<Styles>) {
                self.0
                    .spawn(entity, Some(Styles::new().with(TextFg($palette))));
            }

            fn tick(&self, entity: &mut EntityWorldMut, _style_override: Option<Styles>) {
                self.0
                    .tick(entity, Some(Styles::new().with(TextFg($palette))));
            }
        }
    };
}

color!(Black, Palette16::Black);
color!(Red, Palette16::Red);
color!(Green, Palette16::Green);
color!(Yellow, Palette16::Yellow);
color!(Blue, Palette16::Blue);
color!(Magenta, Palette16::Magenta);
color!(Cyan, Palette16::Cyan);
color!(White, Palette16::White);
color!(BrightBlack, Palette16::BrightBlack);
color!(BrightRed, Palette16::BrightRed);
color!(BrightGreen, Palette16::BrightGreen);
color!(BrightYellow, Palette16::BrightYellow);
color!(BrightBlue, Palette16::BrightBlue);
color!(BrightMagenta, Palette16::BrightMagenta);
color!(BrightCyan, Palette16::BrightCyan);
color!(BrightWhite, Palette16::BrightWhite);
