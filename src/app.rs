use crate::sys_data::SysInfoData;

use std::sync::{Arc, Mutex};

pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct App {
    pub running: bool,
    pub si_data: Arc<Mutex<SysInfoData>>,
}

impl App {
    pub fn new() -> Self {
        let sys_info_data = Arc::new(Mutex::new(SysInfoData::new()));

        let si_data_ref = sys_info_data.clone();
        std::thread::spawn(move || init_si_data_updater(si_data_ref));

        Self {
            running: true,
            si_data: sys_info_data,
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}

fn init_si_data_updater(si_data_ref: Arc<Mutex<SysInfoData>>) {
    loop {
        std::thread::sleep(std::time::Duration::from_millis(800));
        si_data_ref.lock().unwrap().update_cpu_performmance()
    }
}