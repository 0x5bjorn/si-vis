mod si_gui;

use si_gui::SysInfoGuiApp;

fn main() {
    let gui_app = SysInfoGuiApp::new();

    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native("EGUI app", options, Box::new(|cc| Box::new(gui_app)));
}
