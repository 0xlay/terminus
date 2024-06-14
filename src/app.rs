use eframe::egui;

#[derive(Default)]
pub struct Terminus {}

impl Terminus {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for Terminus {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}
