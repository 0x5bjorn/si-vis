mod si_gui;

use si_gui::SysInfoGuiApp;

fn main() -> eframe::Result<()> {
    let gui_app = SysInfoGuiApp::new();

    let options = eframe::NativeOptions::default();
    eframe::run_native("EGUI app", options, Box::new(|cc| Box::new(gui_app)))
}
