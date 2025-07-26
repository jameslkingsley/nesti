use std::io::Error;
use std::{sync::LazyLock, thread::sleep, time::Duration};

mod color;
mod content;
mod tree;

pub use crate::color::*;
pub use crate::content::*;
pub use crate::tree::*;

static GLOBAL_NESTI: LazyLock<Nesti> = LazyLock::new(Nesti::default);

pub fn nesti<T>(path: &str, content: T)
where
    T: Into<StyledContent>,
{
    GLOBAL_NESTI.put(path, content);
}

pub fn nesti_pop(path: &str) {
    GLOBAL_NESTI.pop(path);
}

pub fn output(opt: OutputOptions) -> Result<(), Error> {
    GLOBAL_NESTI.write_to_buffer(&opt)
}

pub fn output_task(opt: OutputOptions) -> impl FnOnce() {
    move || loop {
        GLOBAL_NESTI.write_to_buffer(&opt).unwrap();
        sleep(Duration::from_millis(opt.refresh_rate as u64));
    }
}
