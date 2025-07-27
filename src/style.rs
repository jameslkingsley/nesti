use stanza::{
    style::{Styles as StanzaStyles},
    table::{Cell, Content as TableContent},
};

pub use stanza::style::blink::*;
pub use stanza::style::bold::*;
pub use stanza::style::border_bg::*;
pub use stanza::style::border_fg::*;
pub use stanza::style::fill_bg::*;
pub use stanza::style::fill_invert::*;
pub use stanza::style::halign::*;
pub use stanza::style::header::*;
pub use stanza::style::italic::*;
pub use stanza::style::max_width::*;
pub use stanza::style::min_width::*;
pub use stanza::style::palette_16::*;
pub use stanza::style::separator::*;
pub use stanza::style::strikethrough::*;
pub use stanza::style::text_bg::*;
pub use stanza::style::text_fg::TextFg;
pub use stanza::style::text_invert::*;
pub use stanza::style::underline::*;
pub use stanza::style::Style;

#[derive(Clone)]
pub struct Styles(StanzaStyles);

#[derive(Clone)]
pub struct StyledContent {
    pub content: String,
    pub styles: Styles,
}

impl Styles {
    pub fn new() -> Self {
        Self(StanzaStyles::default())
    }

    pub fn with(mut self, style: impl Style) -> Self {
        self.0 = self.0.with(style);
        self
    }
}

impl StyledContent {
    pub(crate) fn to_cell(&self) -> Cell {
        let style = self.styles.0.clone();
        Cell::new(style, TableContent::Label(self.content.clone()))
    }
}

impl std::fmt::Debug for StyledContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StyledContent")
            .field("content", &self.content)
            .finish()
    }
}

unsafe impl Send for Styles {}
unsafe impl Sync for Styles {}
