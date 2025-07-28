use super::Element;

#[derive(Debug)]
pub struct Text<T: Into<String>>(pub T);

impl<T: Into<String> + Clone> Element for Text<T> {
    type Context = ();

    fn content(&self, _ctx: &Self::Context, _global: &super::GlobalContext) -> String {
        self.0.clone().into()
    }
}

impl Element for &str {
    type Context = ();

    fn content(&self, _ctx: &Self::Context, _global: &super::GlobalContext) -> String {
        self.to_string()
    }
}

impl Element for String {
    type Context = ();

    fn content(&self, _ctx: &Self::Context, _global: &super::GlobalContext) -> String {
        self.to_owned()
    }
}
