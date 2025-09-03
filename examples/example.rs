#[cfg(feature = "example")]
mod system_info {
    use humanize_duration::Truncate::Millis;
    use nesti::{
        nesti, nesti_task, Bytes, Cyan, Integer, IntegerUnit, Magenta, Nano, Progress, Timer,
        Yellow,
    };
    use rand::{rng, Rng};
    use std::{
        thread::{self, sleep},
        time::Duration,
    };
    use sysinfo::System;

    pub fn main() -> Result<(), std::io::Error> {
        let _task = thread::spawn(nesti_task);

        let mut sys = System::new_all();
        let mut rng = rng();
        let mut timers = 0;

        loop {
            sys.refresh_all();
            sys.refresh_cpu();

            if timers < 5 && rng.random_bool(0.2) {
                let key: u16 = rng.random();
                nesti(format!("system/timers/{}", key), Timer(Millis));
                timers += 1;
                sleep(Duration::from_millis(1000));
            }

            nesti("system/uptime", Timer(Nano));

            nesti("system/online", rng.random_bool(0.5));
            nesti("system/offline", rng.random_bool(0.5));

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

            // Demonstrate delta updates - process count changes over time
            let process_delta = (sys.processes().len() as i32) - 100;
            if process_delta > 0 {
                // If we have more than 100 processes, show it as incremental addition
                nesti("processes/total", Integer(100u64)); // Initial value
                nesti("processes/total", Integer(process_delta.abs() as u64).add());
            // Delta update
            } else {
                // Otherwise show the actual count
                let process_count = sys.processes().len() as u64;
                nesti("processes/total", Integer(process_count));
            }

            // Demonstrate other delta operations
            nesti("processes/adder", Integer(1).add());

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
