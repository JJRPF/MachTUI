//! Tab Manager Demo: Multi-tab organizational UI.

use machtui::core::Renderer;
use machtui::core::widgets::Tabs;
use machtui::core::components::{Component, BoxComponent};
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let mut tabs = Tabs::new(vec![
        "Overview".into(),
        "Resources".into(),
        "Networking".into(),
        "Settings".into(),
    ]);

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(16))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Right | KeyCode::Tab => tabs.next(),
                    KeyCode::Left => tabs.prev(),
                    _ => {}
                }
            }
        }

        let canvas = renderer.canvas_mut();
        canvas.clear();

        // --- HEADER ---
        canvas.draw_gradient_text(2, 1, "MACHTUI CONTROL PANEL", (0, 255, 255), (0, 100, 255));
        
        // --- TABS ---
        tabs.render(canvas, 2, 3, canvas.width - 4, 1);

        // --- CONTENT AREA ---
        let content_box = BoxComponent::new(&format!(" {} ", tabs.titles[tabs.selected_idx].to_uppercase()));
        content_box.render(canvas, 2, 5, canvas.width - 4, 15);

        match tabs.selected_idx {
            0 => {
                canvas.draw_text(4, 7, "System Status: OPTIMAL", Some(Color::Green));
                canvas.draw_text(4, 9, "All engines running at high efficiency.", Some(Color::White));
            }
            1 => {
                canvas.draw_text(4, 7, "CPU: 12% | RAM: 4.2GB", Some(Color::Cyan));
            }
            2 => {
                canvas.draw_text(4, 7, "Inbound: 1.2 MB/s | Outbound: 0.5 MB/s", Some(Color::Yellow));
            }
            _ => {
                canvas.draw_text(4, 7, "Configuration settings placeholder...", Some(Color::Grey));
            }
        }

        canvas.draw_text(2, 21, "Arrows/Tab: Switch Tabs | Q: Quit", Some(Color::DarkGrey));

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
