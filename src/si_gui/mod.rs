mod si_data;

use byte_unit::Byte;
use eframe::egui;
use std::{
    sync::{Arc, Mutex},
    thread,
};

use egui::{
    plot::{Line, Plot, PlotPoints},
    ProgressBar, Ui,
};
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

        let sys_info_data_ref = tab_viewer.sys_info_data.clone();
        thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_millis(800));
            sys_info_data_ref.lock().unwrap().update_cpu_performmance()
        });

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
        let mg_sys_info = self.sys_info_data.lock().unwrap();

        ui.heading("System info");
        ui.label(format!(
            "System name: {}",
            mg_sys_info.sys_info.name().unwrap()
        ));
        ui.label(format!(
            "System kernel version: {}",
            mg_sys_info.sys_info.kernel_version().unwrap()
        ));
        ui.label(format!(
            "System OS version:: {}",
            mg_sys_info.sys_info.os_version().unwrap()
        ));
        ui.label(format!(
            "System host name: {}",
            mg_sys_info.sys_info.host_name().unwrap()
        ));

        ui.separator();
        ui.heading("Memory");
        ui.horizontal(|ui| {
            ui.label(format!(
                "Total: {}",
                to_gb(mg_sys_info.sys_info.total_memory() as u128)
            ));
            ui.label(format!(
                "Used: {}",
                to_gb(mg_sys_info.sys_info.used_memory() as u128)
            ));
            ui.label(format!(
                "Available: {}",
                to_gb(mg_sys_info.sys_info.available_memory() as u128)
            ));
            ui.add(
                ProgressBar::new(
                    to_gb(mg_sys_info.sys_info.used_memory() as u128)
                        .split(" ")
                        .next()
                        .unwrap()
                        .parse::<f32>()
                        .unwrap()
                        / 24.0,
                )
                .text(mg_sys_info.sys_info.used_memory().to_string() + " in bytes"),
            );
        });
        ui.horizontal(|ui| {
            ui.label(format!(
                "Total swap:: {}",
                to_gb(mg_sys_info.sys_info.free_memory() as u128)
            ));
            ui.label(format!(
                "Used swap: {}",
                to_gb(mg_sys_info.sys_info.used_swap() as u128)
            ));
        });
    }

    fn display_cpu_performance(&mut self, ui: &mut Ui) {
        let mg_sys_info = self.sys_info_data.lock().unwrap();
        ui.heading(format!(
            "CPU: {} {}",
            mg_sys_info.sys_info.global_cpu_info().brand(),
            mg_sys_info.sys_info.global_cpu_info().name(),
        ));
        ui.separator();
        for (i, cpu) in mg_sys_info.sys_info.cpus().iter().enumerate() {
            ui.horizontal(|ui| {
                ui.label(format!("CPU {}:   ", i));
                ui.add(
                    ProgressBar::new(cpu.cpu_usage() / 100.0)
                        .desired_width(550.0)
                        .show_percentage(),
                );
            });
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

fn to_gb(data_in_bytes: u128) -> String {
    Byte::from_bytes(data_in_bytes)
        .get_appropriate_unit(true)
        .format(1)
}
