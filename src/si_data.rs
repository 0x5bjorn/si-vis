use sysinfo::{CpuExt, NetworkExt, ProcessExt, System, SystemExt};

pub struct SysInfoData {
    pub sys_info: System,
}

impl SysInfoData {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        Self { sys_info: sys }
    }

    pub fn update_cpu_performmance(&mut self) {
        self.sys_info.refresh_all(); // Refreshing CPU information.
    }
}
