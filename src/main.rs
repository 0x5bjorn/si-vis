use eframe::egui;

fn main() {
    let options: eframe::NativeOptions = eframe::NativeOptions::default();
    eframe::run_native(
        "EGUI app",
        options,
        Box::new(|_cc| Box::<EguiApp>::default()),
    );
}

#[derive(Default)]
struct EguiApp {}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello world!");
            ui.label(format!("Hello '{}', age {}", "test", 123));
            ui.button("Click each year");
        });
    }
}
