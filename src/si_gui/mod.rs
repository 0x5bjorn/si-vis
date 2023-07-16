mod si_data;

use eframe::egui;

use si_data::SysInfoData;
use sysinfo::SystemExt;

pub struct SysInfoGuiApp {
    sys_info_data: SysInfoData,
}

impl SysInfoGuiApp {
    pub fn new() -> Self {
        Self {
            sys_info_data: SysInfoData::new(),
        }
    }
}

impl eframe::App for SysInfoGuiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello world!");
            ui.label(format!(
                "System name: {}",
                self.sys_info_data.sys_info.name().unwrap()
            ));
            ui.label(format!(
                "System kernel version: {}",
                self.sys_info_data.sys_info.kernel_version().unwrap()
            ));
            ui.label(format!(
                "System OS version:: {}",
                self.sys_info_data.sys_info.os_version().unwrap()
            ));
            ui.label(format!(
                "System host name: {}",
                self.sys_info_data.sys_info.host_name().unwrap()
            ));
        });
    }
}
