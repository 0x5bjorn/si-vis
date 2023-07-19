mod si_data;

use eframe::egui;

use egui::Ui;
use egui_dock::{DockArea, Style, TabViewer, Tree};
use si_data::SysInfoData;
use sysinfo::SystemExt;

pub struct SysInfoGuiApp {
    tree: Tree<String>,
    tab_viewer: SysInfoGuiTabViewer,
}

impl SysInfoGuiApp {
    pub fn new() -> Self {
        let mut tree = Tree::new(vec!["System Info".to_owned(), "tab2".to_owned()]);

        let tab_viewer = SysInfoGuiTabViewer {
            sys_info_data: SysInfoData::new(),
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
    }
}

struct SysInfoGuiTabViewer {
    sys_info_data: SysInfoData,
}

impl SysInfoGuiTabViewer {
    fn display_basic_info(&mut self, ui: &mut Ui) {
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
    }
}

impl TabViewer for SysInfoGuiTabViewer {
    type Tab = String;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab.as_str() {
            "System Info" => self.display_basic_info(ui),
            _ => {
                ui.label(tab.as_str());
            }
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }
}
