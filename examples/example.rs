#[cfg(feature = "example")]
mod system_info {
    use nesti::{nesti, output, Color::*, Content::*, OutputOptions};
    use std::{
        thread::{self, sleep},
        time::Duration,
    };
    use sysinfo::System;

    pub fn main() -> Result<(), std::io::Error> {
        let _printer = thread::spawn(output(OutputOptions::default()));

        let mut sys = System::new_all();

        loop {
            sys.refresh_all();
            sys.refresh_cpu();

            nesti("system/arch", "amd64");
            nesti(
                "system/cpu/cores",
                Magenta(IntegerUnit(
                    sys.physical_core_count().unwrap() as u64,
                    "cores",
                )),
            );
            nesti(
                "system/memory",
                Cyan(Progress {
                    current: sys.used_memory(),
                    maximum: sys.total_memory(),
                    show_percent: true,
                    show_values: false,
                }),
            );
            nesti("system/memory/total", Bytes(sys.total_memory()));
            nesti("system/memory/used", Bytes(sys.used_memory()));
            nesti("system/memory/free", Bytes(sys.available_memory()));

            let process_count = sys.processes().len() as u64;
            nesti("processes/total", Integer(process_count));

            sleep(Duration::from_millis(100));
        }
    }
}

pub fn main() {
    #[cfg(feature = "example")]
    let _ = system_info::main();

    #[cfg(not(feature = "example"))]
    println!("Enable the `example` feature")
}
