use crate::si_data;

use ratatui::widgets::TableState;
use std::sync::{Arc, Mutex};
use sysinfo::SystemExt;

pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct App<'a> {
    pub running: bool,
    pub si_data: Arc<Mutex<si_data::SysInfoData>>,

    pub tab_index: usize,
    pub tab_titles: Vec<&'a str>,
    pub table_state: TableState,
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
            table_state: TableState::default(),
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next_tab_index(&mut self) {
        self.tab_index = (self.tab_index + 1) % self.tab_titles.len();
    }

    pub fn prev_tab_index(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = self.tab_titles.len() - 1;
        }
    }

    pub fn next_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.si_data.lock().unwrap().sys_info.processes().len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn prev_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.si_data.lock().unwrap().sys_info.processes().len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }
}

fn init_si_data_updater(si_data_ref: Arc<Mutex<si_data::SysInfoData>>) {
    loop {
        std::thread::sleep(std::time::Duration::from_millis(800));
        si_data_ref.lock().unwrap().update_si_data()
    }
}
