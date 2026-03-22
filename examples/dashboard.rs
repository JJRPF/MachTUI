//! Dashboard Demo: Advanced Components, FPS Tracking, and Layout.

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use machtui::core::components::{BoxComponent, Component, ProgressBar};
use machtui::core::Renderer;
use std::io;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let start_time = Instant::now();

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(16))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                if code == KeyCode::Char('q') {
                    break;
                }
                if code == KeyCode::Char('f') {
                    renderer.show_fps = !renderer.show_fps;
                }
            }
        }

        let canvas = renderer.canvas_mut();
        canvas.clear();

        let elapsed = start_time.elapsed().as_secs_f32();

        // 1. Draw Main Outer Container
        let main_box = BoxComponent::new(" MACHTUI SYSTEM DASHBOARD ");
        main_box.render(canvas, 2, 1, 60, 20);

        // 2. Draw Sub-sections
        let cpu_box = BoxComponent {
            title: "CPU Usage".into(),
            border_color: Color::Green,
        };
        cpu_box.render(canvas, 4, 3, 26, 6);

        let cpu_progress = (elapsed * 0.5).sin().abs();
        let cpu_bar = ProgressBar::new("Core 0", cpu_progress);
        cpu_bar.render(canvas, 6, 5, 20, 2);

        let ram_box = BoxComponent {
            title: "Memory".into(),
            border_color: Color::Yellow,
        };
        ram_box.render(canvas, 32, 3, 26, 6);

        let ram_progress = (elapsed * 0.2).cos().abs();
        let ram_bar = ProgressBar {
            progress: ram_progress,
            label: "RAM".into(),
            start_color: (255, 200, 0),
            end_color: (255, 50, 0),
        };
        ram_bar.render(canvas, 34, 5, 20, 2);

        let gpu_box = BoxComponent {
            title: "GPU Rendering".into(),
            border_color: Color::Magenta,
        };
        gpu_box.render(canvas, 4, 10, 54, 8);

        let gpu_progress = (elapsed * 0.8).sin().abs();
        let gpu_bar = ProgressBar::new("VRAM Usage", gpu_progress);
        gpu_bar.render(canvas, 6, 12, 50, 2);

        canvas.draw_text(
            6,
            15,
            "Press 'f' to toggle FPS overlay",
            Some(Color::DarkGrey),
        );
        canvas.draw_text(6, 16, "Press 'q' to exit", Some(Color::DarkRed));

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
