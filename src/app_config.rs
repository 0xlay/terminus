use egui::Vec2;

pub const APP_NAME: &str = "Terminus";

pub const APP_ICON: &[u8] = include_bytes!("../assets/icons/terminus-256.png");

pub const APP_DEFAULT_SIZE: Vec2 = Vec2 { x: 800.0, y: 600.0 };
pub const APP_DEFAULT_MIN_SIZE: Vec2 = Vec2 { x: 400.0, y: 300.0 };
