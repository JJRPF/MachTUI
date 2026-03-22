//! Search Pro Demo: High-End Search Interface with AI-Native Oracle support.

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use machtui::core::components::{BoxComponent, Component};
use machtui::core::widgets::{ListSelection, TextInput};
use machtui::core::Renderer;
use machtui::vision::icons::Icons;
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let mut search_input = TextInput::new("Search repositories...");
    search_input.focused = true;

    let all_items = vec![
        "machtui-engine".to_string(),
        "machtop-monitor".to_string(),
        "mach-chat-client".to_string(),
        "vision-graphics".to_string(),
        "oracle-protocol".to_string(),
        "plume-stylist".to_string(),
    ];
    let mut results = ListSelection::new(all_items.clone());

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(16))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(c) => {
                        search_input.handle_char(c);
                        let filtered: Vec<String> = all_items
                            .iter()
                            .filter(|s| s.contains(&search_input.content))
                            .cloned()
                            .collect();
                        results = ListSelection::new(filtered);
                    }
                    KeyCode::Backspace => {
                        search_input.handle_backspace();
                        let filtered: Vec<String> = all_items
                            .iter()
                            .filter(|s| s.contains(&search_input.content))
                            .cloned()
                            .collect();
                        results = ListSelection::new(filtered);
                    }
                    KeyCode::Up => results.move_up(),
                    KeyCode::Down => results.move_down(),
                    _ => {}
                }
            }
        }

        let canvas = renderer.canvas_mut();
        canvas.clear();

        // --- HEADER ---
        canvas.draw_gradient_text(
            2,
            1,
            &format!("{} MACH SEARCH PRO", Icons::CHART),
            (0, 255, 255),
            (255, 255, 0),
        );

        // --- SEARCH INPUT ---
        search_input.render(canvas, 2, 3, canvas.width - 4, 3);

        // --- RESULTS ---
        let res_box = BoxComponent::new(&format!(" RESULTS ({}) ", results.items.len()));
        res_box.render(canvas, 2, 7, canvas.width - 4, canvas.height - 10);

        if results.items.is_empty() {
            canvas.draw_text(4, 9, "No repositories found.", Some(Color::DarkGrey));
        } else {
            results.render(canvas, 4, 9, canvas.width - 8, canvas.height - 14);
        }

        canvas.draw_text(
            2,
            canvas.height - 1,
            "Type to filter | Arrows: Select | Q: Quit",
            Some(Color::DarkGrey),
        );

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
