use super::Element;

impl Element for bool {
    type Context = ();

    fn content(&self, _ctx: &Self::Context, _global: &super::GlobalContext) -> String {
        match self {
            true => "✓".to_string(), // ✔
            false => "🗙".to_string(),
        }
    }
}
