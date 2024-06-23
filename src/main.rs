#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use nix::pty::ForkptyResult;
use std::ffi::CStr;

mod app;
mod app_config;

fn main() -> eframe::Result<()> {
    let res = unsafe { nix::pty::forkpty(None, None).expect("The forkpty failed") };

    match res {
        ForkptyResult::Parent { child, master } => {
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
                Box::new(|cc| Box::new(app::Terminus::new(cc, master))),
            )?;
        },
        ForkptyResult::Child => {
            let shell_name =
                CStr::from_bytes_with_nul(b"sh\0").expect("Should always have null terminator");
            let args: &[&[u8]] = &[b"sh\0", b"--noprofile\0", b"--norc\0", b"--posix\0"];

            let args: Vec<&'static CStr> = args
                .iter()
                .map(|v| CStr::from_bytes_with_nul(v).expect("Should always have null terminator"))
                .collect::<Vec<_>>();

            std::env::remove_var("PROMPT_COMMAND");
            std::env::set_var("PS1", "$ ");
            std::env::set_var("TERM", "xterm-256color");
            nix::unistd::execvp(shell_name, &args).unwrap();
        }
    }

    Ok(())
}
