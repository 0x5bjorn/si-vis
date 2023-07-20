mod si_gui;

use std::thread;

use si_gui::SysInfoGuiApp;

fn main() -> eframe::Result<()> {
    let gui_app = SysInfoGuiApp::new();

    let sys_info_data_ref = gui_app.tab_viewer.sys_info_data.clone();
    thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(800));
        sys_info_data_ref.lock().unwrap().update_cpu_performmance()
    });

    let options = eframe::NativeOptions::default();
    eframe::run_native("EGUI app", options, Box::new(|cc| Box::new(gui_app)))
}
