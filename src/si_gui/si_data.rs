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

fn print_info() {
    // Please note that we use "new_all" to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    // We display all disks' information:
    println!("=> disks:");
    for disk in sys.disks() {
        println!("{:?}", disk);
    }

    // Network interfaces name, data received and data transmitted:
    println!("=> networks:");
    for (interface_name, data) in sys.networks() {
        println!(
            "{}: {}/{} B",
            interface_name,
            data.received(),
            data.transmitted()
        );
    }

    // Components temperature:
    println!("=> components:");
    for component in sys.components() {
        println!("{:?}", component);
    }

    loop {
        sys.refresh_all();

        println!("=> system:");
        // RAM and swap information:
        println!("total memory: {} bytes", sys.total_memory());
        println!("used memory : {} bytes", sys.used_memory());
        println!("total swap  : {} bytes", sys.total_swap());
        println!("used swap   : {} bytes", sys.used_swap());

        // Display system information:
    }

    // // Number of CPUs:
    // println!("NB CPUs: {}", self.sys_info.cpus().len());

    // // Display processes ID, name na disk usage:
    // for (pid, process) in self.sys_info.processes() {
    //     println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
    // }
}
