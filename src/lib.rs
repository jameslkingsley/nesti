use std::io::Error;
use std::{sync::LazyLock, thread::sleep, time::Duration};

pub mod style;

mod elements;
mod tree;

pub use crate::elements::*;
pub use crate::tree::*;

pub use humanize_duration::Truncate;

static GLOBAL_NESTI: LazyLock<Nesti> = LazyLock::new(Nesti::default);

pub fn nesti<T>(path: &str, content: T)
where
    T: Element + 'static + Send + Sync + std::fmt::Debug,
    T::Context: Default + Send + Sync + 'static,
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
