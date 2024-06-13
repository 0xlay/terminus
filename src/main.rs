mod app_config;

use eframe::egui;

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
            .with_inner_size(app_config::APP_DEFAULT_SIZE)
            .with_min_inner_size(app_config::APP_DEFAULT_MIN_SIZE)
            .with_icon(
                eframe::icon_data::from_png_bytes(app_config::APP_ICON)
                    .unwrap(),
            ),
        ..Default::default()
    };
    let _ = eframe::run_native(
        app_config::APP_NAME,
        native_options,
        Box::new(|cc| Box::new(Terminus::new(cc))),
    );
}
