use crate::si_data;

use std::sync::{Arc, Mutex};

pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct App<'a> {
    pub running: bool,
    pub si_data: Arc<Mutex<si_data::SysInfoData>>,

    pub tab_index: usize,
    pub tab_titles: Vec<&'a str>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let sys_info_data = Arc::new(Mutex::new(si_data::SysInfoData::new()));

        let si_data_ref = sys_info_data.clone();
        std::thread::spawn(move || init_si_data_updater(si_data_ref));

        Self {
            running: true,
            si_data: sys_info_data,
            tab_index: 0,
            tab_titles: vec!["System Info", "Processes"],
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next_tab_index(&mut self) {
        self.tab_index = (self.tab_index + 1) % self.tab_titles.len();
    }

    pub fn previous_tab_index(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = self.tab_titles.len() - 1;
        }
    }
}

fn init_si_data_updater(si_data_ref: Arc<Mutex<si_data::SysInfoData>>) {
    loop {
        std::thread::sleep(std::time::Duration::from_millis(800));
        si_data_ref.lock().unwrap().update_cpu_performmance()
    }
}
