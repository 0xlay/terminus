mod app;
mod app_config;

fn main() -> eframe::Result<()> {
    // TODO: add the terminus.json config file and load settings instead the default settings.

    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title(app_config::APP_NAME)
            .with_inner_size(app_config::APP_DEFAULT_SIZE)
            .with_min_inner_size(app_config::APP_DEFAULT_MIN_SIZE)
            .with_icon(eframe::icon_data::from_png_bytes(app_config::APP_ICON).unwrap())
            .with_drag_and_drop(true),
        follow_system_theme: false,
        default_theme: eframe::Theme::Dark,
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        app_config::APP_NAME,
        native_options,
        Box::new(|cc| Box::new(app::Terminus::new(cc))),
    )?;

    Ok(())
}
