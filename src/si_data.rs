use byte_unit::Byte;
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

    pub fn update_si_data(&mut self) {
        self.sys_info.refresh_all(); // Refreshing CPU information.
    }
}

pub fn to_gb(data_in_bytes: u128) -> String {
    Byte::from_bytes(data_in_bytes)
        .get_appropriate_unit(true)
        .format(1)
}
