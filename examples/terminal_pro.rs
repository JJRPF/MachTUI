//! Terminal Pro Demo: Embedded Shells and Multi-session management.

use machtui::core::Renderer;
use machtui::core::shell::ShellComponent;
use machtui::core::components::{Component, BoxComponent};
use machtui::vision::icons::Icons;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let term = ShellComponent::new();
    let mut command_input = String::new();

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(30))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Enter => {
                        term.send_command(&command_input);
                        command_input.clear();
                    }
                    KeyCode::Char(c) => command_input.push(c),
                    KeyCode::Backspace => { command_input.pop(); }
                    _ => {}
                }
            }
        }

        let canvas = renderer.canvas_mut();
        canvas.clear();

        // --- HEADER ---
        canvas.draw_gradient_text(2, 1, &format!("{} MACH TERMINAL PRO", Icons::GEAR), (0, 255, 255), (255, 255, 255));

        // --- TERMINAL VIEWPORT ---
        let b = BoxComponent::new(" BASH SESSION ");
        b.render(canvas, 2, 3, canvas.width - 4, canvas.height - 8);
        term.render(canvas, 4, 5, canvas.width - 8, canvas.height - 12);

        // --- INPUT BAR ---
        let input_box = BoxComponent::new(" COMMAND ");
        input_box.render(canvas, 2, canvas.height - 5, canvas.width - 4, 3);
        canvas.draw_text(4, canvas.height - 4, &format!("$ {}█", command_input), Some(Color::Green));

        canvas.draw_text(2, canvas.height - 1, "Type Command + Enter | Q: Quit | Real-time PTY integration", Some(Color::DarkGrey));

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
