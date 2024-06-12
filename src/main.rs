use eframe::egui;

const APP_NAME: &str = "Terminus";

#[derive(Default)]
struct Terminus {}

impl Terminus {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for Terminus {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]) // TODO: load default size from the terminus.json config.
            .with_min_inner_size([400.0, 300.0]), // TODO: also load frome the config file
        ..Default::default()
    };
    let _ = eframe::run_native(
        APP_NAME,
        native_options,
        Box::new(|cc| Box::new(Terminus::new(cc))),
    );
}
