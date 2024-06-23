use std::os::fd::{AsRawFd, OwnedFd};

use eframe::egui;
use nix::errno::Errno;

pub struct Terminus {
    buffer: Vec<u8>,
    fd: OwnedFd,
}

impl Terminus {
    pub fn new(_cc: &eframe::CreationContext<'_>, fd: OwnedFd) -> Self {
        Self::enable_non_block(&fd);
        Self {
            buffer: Vec::new(),
            fd,
        }
    }

    fn enable_non_block(fd: &OwnedFd) {
        let flags = nix::fcntl::fcntl(fd.as_raw_fd(), nix::fcntl::FcntlArg::F_GETFL).unwrap();
        let mut flags =
            nix::fcntl::OFlag::from_bits(flags & nix::fcntl::OFlag::O_ACCMODE.bits()).unwrap();

        flags.set(nix::fcntl::OFlag::O_NONBLOCK, true);

        nix::fcntl::fcntl(fd.as_raw_fd(), nix::fcntl::FcntlArg::F_SETFL(flags)).unwrap();
    }
}

impl eframe::App for Terminus {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut buf = vec![0u8; 4096];

        match nix::unistd::read(self.fd.as_raw_fd(), &mut buf) {
            Ok(read_size) => {
                self.buffer.extend_from_slice(&buf[0..read_size]);
            }
            Err(e) => {
                if e != Errno::EAGAIN {
                    println!("Failed to read from stdout: {e}");
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.input(|input_state| {
                for event in &input_state.events {
                    let text = match event {
                        egui::Event::Text(text) => text,
                        egui::Event::Key { key: egui::Key::Enter, pressed: true, .. } => {
                            "\n"
                        },
                        _ => "",
                    };

                    let bytes = text.as_bytes();
                    let mut to_write: &[u8] = bytes;

                    while to_write.len() > 0 {
                        let written = nix::unistd::write(&self.fd, to_write).unwrap();
                        to_write = &to_write[written..];
                    }
                }
            });

            unsafe {
                ui.label(std::str::from_utf8_unchecked(&self.buffer))
            }
        });
    }
}
