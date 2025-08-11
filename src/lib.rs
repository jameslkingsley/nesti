use std::io::Error;
use std::{sync::LazyLock, thread::sleep, time::Duration};

pub mod style;

mod core;
mod elements;

pub use crate::core::*;
pub use crate::elements::*;
pub use humanize_duration::Truncate::*;

static GLOBAL_NESTI: LazyLock<Nesti> = LazyLock::new(Nesti::default);

pub fn nesti<P, E>(path: P, element: E)
where
    P: Into<String>,
    E: Element + Send + Sync + 'static,
{
    GLOBAL_NESTI.put(path, element);
}

pub fn nesti_pop(path: &str) {
    GLOBAL_NESTI.pop(path);
}

pub fn nesti_flush() -> Result<(), Error> {
    GLOBAL_NESTI.flush()
}

pub fn nesti_task() {
    loop {
        GLOBAL_NESTI.flush().unwrap();
        sleep(Duration::from_millis(32));
    }
}
