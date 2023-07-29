mod si_data;

use byte_unit::Byte;
use chrono::{NaiveDate, NaiveDateTime};
use eframe::egui;
use egui::{
    plot::{Line, Plot, PlotPoints},
    ProgressBar, Ui,
};
use egui_dock::{DockArea, Style, TabViewer, Tree};
use egui_extras::{Column, TableBuilder};
use si_data::SysInfoData;
use std::{
    sync::{Arc, Mutex},
    thread,
};
use sysinfo::{CpuExt, NetworksExt, Pid, ProcessExt, SystemExt};

pub struct SysInfoGuiApp {
    tree: Tree<String>,
    pub tab_viewer: SysInfoGuiTabViewer,
}

impl SysInfoGuiApp {
    pub fn new() -> Self {
        let mut tree = Tree::new(vec![
            "System Info".to_owned(),
            "CPU performance".to_owned(),
            "Processes".to_owned(),
        ]);
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
            let mut style = Style::from_egui(ctx.style().as_ref());
            style.tabs.fill_tab_bar = true;

            DockArea::new(&mut self.tree)
                .style(style)
                .show_close_buttons(false)
                .show_add_buttons(false)
                .draggable_tabs(true)
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
        ui.label(format!(
            "Boot time: {}",
            NaiveDateTime::from_timestamp_opt(mg_sys_info.sys_info.boot_time() as i64, 0).unwrap()
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

        ui.separator();
        ui.heading("Network Data");
        for (i, net) in mg_sys_info.sys_info.networks().iter() {
            ui.label(format!("{i}: {:?}", net));
        }
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

    fn display_process_info(&mut self, ui: &mut Ui) {
        let mg_sys_info = self.sys_info_data.lock().unwrap();

        let process_table = TableBuilder::new(ui)
            .striped(true)
            .column(Column::auto().resizable(true))
            .column(Column::auto().resizable(true))
            .column(Column::auto().resizable(true))
            .column(Column::auto().resizable(true))
            .column(Column::auto().resizable(true))
            .column(Column::auto().resizable(true))
            .column(Column::auto().resizable(true))
            .column(Column::remainder())
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.button("PID");
                });
                header.col(|ui| {
                    ui.button("Name");
                });
                header.col(|ui| {
                    ui.button("CPU usage");
                });
                header.col(|ui| {
                    ui.button("Memory");
                });
                header.col(|ui| {
                    ui.button("Virtual memory");
                });
                header.col(|ui| {
                    ui.button("Parent PID");
                });
                header.col(|ui| {
                    ui.button("Runtime");
                });
                header.col(|ui| {
                    ui.button("Disk usage");
                });
            })
            .body(|mut body| {
                for (pid, process) in mg_sys_info.sys_info.processes() {
                    body.row(30.0, |mut row| {
                        row.col(|ui| {
                            ui.label(pid.to_string());
                        });
                        row.col(|ui| {
                            ui.label(process.name());
                        });
                        row.col(|ui| {
                            ui.label(process.cpu_usage().to_string());
                        });
                        row.col(|ui| {
                            ui.label(to_gb(process.memory() as u128));
                        });
                        row.col(|ui| {
                            ui.label(to_gb(process.virtual_memory() as u128));
                        });
                        row.col(|ui| {
                            ui.label(match process.parent() {
                                Some(pid) => pid.to_string(),
                                None => "None".to_owned(),
                            });
                        });
                        row.col(|ui| {
                            ui.label(process.run_time().to_string());
                        });
                        row.col(|ui| {
                            ui.label(format!("{:?}", process.disk_usage()));
                        });
                    });
                }
            });
    }
}

impl TabViewer for SysInfoGuiTabViewer {
    type Tab = String;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab.as_str() {
            "System Info" => self.display_basic_info(ui),
            "CPU performance" => self.display_cpu_performance(ui),
            "Processes" => self.display_process_info(ui),
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
