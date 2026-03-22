//! SysTop Demo: High-End Real-time System Monitor.

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use machtui::core::components::{BoxComponent, Component, ProgressBar};
use machtui::core::Renderer;
use machtui::vision::colors::Palette;
use std::io;
use std::time::Duration;
use sysinfo::System;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    // new_all ensures CPU list is populated
    let mut sys = System::new_all();

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(200))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                if code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        // 2025 API: Refresh specific data
        sys.refresh_cpu_usage();
        sys.refresh_memory();

        let canvas = renderer.canvas_mut();
        canvas.clear();

        // --- HEADER ---
        canvas.draw_gradient_text(2, 1, "MACHTUI SYSTEM MONITOR", (0, 255, 255), (0, 100, 255));

        // --- CPU USAGE ---
        let cpu_box = BoxComponent {
            title: " CPU CORES ".into(),
            border_color: Palette::BLUE_500,
        };
        cpu_box.render(canvas, 2, 3, 40, 12);

        for (i, cpu) in sys.cpus().iter().take(8).enumerate() {
            let usage = cpu.cpu_usage() / 100.0;
            let bar = ProgressBar {
                progress: usage,
                label: format!("CPU {}", i),
                start_color: (16, 185, 129), // Emerald 500
                end_color: (244, 63, 94),    // Rose 500
            };
            bar.render(canvas, 4, 5 + i as u16, 30, 1);
        }

        // --- MEMORY ---
        let mem_box = BoxComponent {
            title: " MEMORY ".into(),
            border_color: Palette::AMBER_500,
        };
        mem_box.render(canvas, 44, 3, 30, 6);

        let total_mem = sys.total_memory() as f32;
        let used_mem = sys.used_memory() as f32;
        let mem_usage = if total_mem > 0.0 {
            used_mem / total_mem
        } else {
            0.0
        };

        let mem_bar = ProgressBar::new("RAM", mem_usage);
        mem_bar.render(canvas, 46, 5, 20, 1);

        canvas.draw_text(
            2,
            16,
            "Press 'q' to exit | Refreshing every 200ms",
            Some(Color::DarkGrey),
        );

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
