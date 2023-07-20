mod si_data;

use std::{
    sync::{Arc, Mutex},
    thread,
};

use eframe::egui;

use egui::{ProgressBar, Ui};
use egui_dock::{DockArea, Style, TabViewer, Tree};
use si_data::SysInfoData;
use sysinfo::{CpuExt, SystemExt};

pub struct SysInfoGuiApp {
    tree: Tree<String>,
    pub tab_viewer: SysInfoGuiTabViewer,
}

impl SysInfoGuiApp {
    pub fn new() -> Self {
        let mut tree = Tree::new(vec!["System Info".to_owned(), "CPU performance".to_owned()]);
        let tab_viewer = SysInfoGuiTabViewer {
            sys_info_data: Arc::new(Mutex::new(SysInfoData::new())),
        };
        Self { tree, tab_viewer }
    }
}

impl eframe::App for SysInfoGuiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            DockArea::new(&mut self.tree)
                .style(Style::from_egui(ctx.style().as_ref()))
                .show_close_buttons(false)
                .show_add_buttons(false)
                .draggable_tabs(false)
                .show(ctx, &mut self.tab_viewer);
        });
        ctx.request_repaint();
    }
}

pub struct SysInfoGuiTabViewer {
    pub sys_info_data: Arc<Mutex<SysInfoData>>,
}

impl SysInfoGuiTabViewer {
    fn display_basic_info(&mut self, ui: &mut Ui) {
        ui.heading("Hello world!");
        ui.label(format!(
            "System name: {}",
            self.sys_info_data.lock().unwrap().sys_info.name().unwrap()
        ));
        ui.label(format!(
            "System kernel version: {}",
            self.sys_info_data
                .lock()
                .unwrap()
                .sys_info
                .kernel_version()
                .unwrap()
        ));
        ui.label(format!(
            "System OS version:: {}",
            self.sys_info_data
                .lock()
                .unwrap()
                .sys_info
                .os_version()
                .unwrap()
        ));
        ui.label(format!(
            "System host name: {}",
            self.sys_info_data
                .lock()
                .unwrap()
                .sys_info
                .host_name()
                .unwrap()
        ));
    }

    fn display_cpu_performance(&mut self, ui: &mut Ui) {
        ui.heading("CPU");
        let mg_sys_info = self.sys_info_data.lock().unwrap();
        for (_, cpu) in mg_sys_info.sys_info.cpus().iter().enumerate() {
            ui.add(ProgressBar::new(cpu.cpu_usage() / 100.0).show_percentage());
            // ui.pro(format!("CPU {}: {}% ", i, cpu.cpu_usage()));
        }
    }
}

impl TabViewer for SysInfoGuiTabViewer {
    type Tab = String;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab.as_str() {
            "System Info" => self.display_basic_info(ui),
            "CPU performance" => self.display_cpu_performance(ui),
            _ => {
                ui.label(tab.as_str());
            }
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }
}
