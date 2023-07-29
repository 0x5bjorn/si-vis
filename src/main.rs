mod si_gui;

use si_gui::SysInfoGuiApp;

fn main() -> eframe::Result<()> {
    let gui_app = SysInfoGuiApp::new();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1100.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native("EGUI app", options, Box::new(|cc| Box::new(gui_app)))
}
