//! Shell component for MachTUI.
//! Provides an embedded PTY for running shell commands within the TUI.

use crate::core::Canvas;
use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtySize};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ShellComponent {
    pub output: Arc<Mutex<Vec<String>>>,
    pub master: Arc<Mutex<Box<dyn MasterPty + Send>>>,
}

impl ShellComponent {
    pub fn new() -> Self {
        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            })
            .expect("Failed to open PTY");

        let cmd = CommandBuilder::new("bash");
        let _child = pair.slave.spawn_command(cmd).expect("Failed to spawn bash");

        let mut reader = pair
            .master
            .try_clone_reader()
            .expect("Failed to clone reader");
        let output = Arc::new(Mutex::new(vec!["Shell Started...".to_string()]));
        let output_clone = Arc::clone(&output);

        thread::spawn(move || {
            let mut buf = [0u8; 1024];
            while let Ok(n) = reader.read(&mut buf) {
                if n == 0 {
                    break;
                }
                let s = String::from_utf8_lossy(&buf[..n]);
                let mut out = output_clone.lock().unwrap();
                for line in s.lines() {
                    out.push(line.to_string());
                    if out.len() > 100 {
                        out.remove(0);
                    }
                }
            }
        });

        Self {
            output,
            master: Arc::new(Mutex::new(pair.master)),
        }
    }

    pub fn send_command(&self, cmd: &str) {
        let master = self.master.lock().unwrap();
        let mut writer = master.take_writer().expect("Failed to take writer");
        let _ = write!(writer, "{}\n", cmd);
    }

    pub fn render(&self, canvas: &mut Canvas, x: u16, y: u16, width: u16, height: u16) {
        let lines = self.output.lock().unwrap();
        let start = lines.len().saturating_sub(height as usize);
        for (i, line) in lines[start..].iter().enumerate() {
            let truncated = if line.len() > width as usize {
                &line[..width as usize]
            } else {
                line
            };
            canvas.draw_text(
                x,
                y + i as u16,
                truncated,
                Some(crossterm::style::Color::White),
            );
        }
    }
}
