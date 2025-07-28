use super::Element;

impl Element for bool {
    type Context = ();

    fn content(&self, _ctx: &Self::Context) -> String {
        match self {
            true => format!("YES"),
            false => format!("NO"),
        }
    }
}
