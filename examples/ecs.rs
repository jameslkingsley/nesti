#[cfg(feature = "example")]
mod system_info {
    use std::{thread::sleep, time::Duration};

    use nesti::core::Nesti;
    use sysinfo::System;

    pub fn main() {
        let mut sys = System::new_all();
        sys.refresh_all();
        sys.refresh_cpu();

        let nesti = Nesti::default();

        loop {
            nesti.put("system/uptime", "system/uptime");
            nesti.put("system/online", "system/online");
            nesti.put("system/offline", "system/offline");
            nesti.put("system/cpu", "system/cpu");
            nesti.put("system/cpu/arch", "system/cpu/arch");
            nesti.put("system/cpu/cores", "system/cpu/cores");
            nesti.put("system/memory", "system/memory");
            nesti.put("system/memory/total", "system/memory/total");
            nesti.put("system/memory/used", "system/memory/used");
            nesti.put("system/memory/free", "system/memory/free");
            nesti.flush().unwrap();
            sleep(Duration::from_millis(32));
        }
    }
}

pub fn main() {
    #[cfg(feature = "example")]
    let _ = system_info::main();

    #[cfg(not(feature = "example"))]
    println!("Enable the `example` feature")
}
