//! Remote Desktop Demo: Multi-host management and SSH Bridge.

use machtui::core::Renderer;
use machtui::core::components::{Component, BoxComponent};
use machtui::core::widgets::{TextInput, ListSelection};
use machtui::vision::icons::Icons;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let mut hosts = ListSelection::new(vec![
        "prod-web-01".into(),
        "prod-db-01".into(),
        "staging-api-01".into(),
    ]);
    
    let mut logs = ListSelection::new(vec!["Select a host to begin...".into()]);

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(16))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => hosts.move_up(),
                    KeyCode::Down => hosts.move_down(),
                    KeyCode::Enter => {
                        let host = &hosts.items[hosts.selected_idx];
                        logs.items.push(format!("Connecting to {}...", host));
                        // Simulated connection
                        logs.items.push(format!("{}: Authentication Success.", host));
                        logs.items.push(format!("{}: System load 0.12", host));
                    }
                    _ => {}
                }
            }
        }

        let canvas = renderer.canvas_mut();
        canvas.clear();

        // --- HEADER ---
        canvas.draw_gradient_text(2, 1, &format!("{} MACH REMOTE DESKTOP", Icons::GEAR), (255, 100, 0), (255, 255, 0));

        // --- HOST LIST ---
        let side_box = BoxComponent::new(" REMOTE HOSTS ");
        side_box.render(canvas, 2, 3, 25, 15);
        hosts.render(canvas, 4, 5, 21, 12);

        // --- REMOTE LOGS ---
        let main_box = BoxComponent::new(" CONNECTION CONSOLE ");
        main_box.render(canvas, 28, 3, canvas.width - 30, 15);
        logs.render(canvas, 30, 5, canvas.width - 34, 12);

        canvas.draw_text(2, 19, "Arrows: Select Host | Enter: Connect | Q: Quit", Some(Color::DarkGrey));

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
