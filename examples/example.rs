#[cfg(feature = "example")]
mod system_info {
    use nesti::{nesti, output_task, Color::*, Content::*, OutputOptions};
    use std::{
        thread::{self, sleep},
        time::Duration,
    };
    use sysinfo::System;

    pub fn main() -> Result<(), std::io::Error> {
        let _printer = thread::spawn(output_task(OutputOptions::default()));

        let mut sys = System::new_all();
        let mut counter = 0;

        loop {
            sys.refresh_all();
            sys.refresh_cpu();

            nesti(
                "system/counter",
                Cyan(Progress {
                    current: counter,
                    maximum: 1000,
                    show_percent: false,
                    show_values: false,
                    show_rate: true,
                }),
            );

            nesti("system/uptime", Stopwatch(0b0001));
            nesti("system/cpu/arch", "amd64");
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
                    show_rate: false,
                }),
            );
            nesti("system/memory/total", Yellow(Bytes(sys.total_memory())));
            nesti("system/memory/used", Yellow(Bytes(sys.used_memory())));
            nesti("system/memory/free", Yellow(Bytes(sys.available_memory())));

            let process_count = sys.processes().len() as u64;
            nesti("processes/total", Integer(process_count));

            sleep(Duration::from_millis(100));

            counter += 1;
        }
    }
}

pub fn main() {
    #[cfg(feature = "example")]
    let _ = system_info::main();

    #[cfg(not(feature = "example"))]
    println!("Enable the `example` feature")
}
