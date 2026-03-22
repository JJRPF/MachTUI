//! DB Manager Demo: High-Performance SQLite Integration.

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use machtui::core::components::{BoxComponent, Component};
use machtui::core::db::Database;
use machtui::core::widgets::{ListSelection, TextInput};
use machtui::core::Renderer;
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let db = Database::open_in_memory().expect("Failed to open DB");
    db.init_kv_table().expect("Failed to init table");

    let mut input = TextInput::new("Enter key...");
    input.focused = true;
    let mut value_input = TextInput::new("Enter value...");

    let mut logs = ListSelection::new(vec!["DB Initialized".into()]);
    let mut active_input = 0; // 0 for key, 1 for value

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(16))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Tab => active_input = (active_input + 1) % 2,
                    KeyCode::Enter => {
                        if !input.content.is_empty() && !value_input.content.is_empty() {
                            db.set_kv(&input.content, &value_input.content)
                                .expect("Set failed");
                            logs.items
                                .push(format!("SET: {} -> {}", input.content, value_input.content));
                            input.content.clear();
                            value_input.content.clear();
                            input.cursor_pos = 0;
                            value_input.cursor_pos = 0;
                        }
                    }
                    KeyCode::Char(c) => {
                        if active_input == 0 {
                            input.handle_char(c);
                        } else {
                            value_input.handle_char(c);
                        }
                    }
                    KeyCode::Backspace => {
                        if active_input == 0 {
                            input.handle_backspace();
                        } else {
                            value_input.handle_backspace();
                        }
                    }
                    _ => {}
                }
            }
        }

        input.focused = active_input == 0;
        value_input.focused = active_input == 1;

        let canvas = renderer.canvas_mut();
        canvas.clear();

        // --- HEADER ---
        canvas.draw_gradient_text(
            2,
            1,
            "MACHTUI DATABASE ENGINE",
            (255, 100, 0),
            (255, 200, 0),
        );

        // --- INPUTS ---
        let input_box = BoxComponent::new(" KV STORAGE ");
        input_box.render(canvas, 2, 3, 40, 10);
        canvas.draw_text(4, 5, "Key:", Some(Color::Cyan));
        input.render(canvas, 10, 4, 30, 3);

        canvas.draw_text(4, 8, "Val:", Some(Color::Cyan));
        value_input.render(canvas, 10, 7, 30, 3);

        // --- LOGS ---
        let log_box = BoxComponent::new(" TRANSACTION LOG ");
        log_box.render(canvas, 44, 3, 34, 15);
        logs.render(canvas, 46, 5, 30, 12);

        canvas.draw_text(
            2,
            20,
            "TAB: Switch | ENTER: Save | Q: Quit",
            Some(Color::DarkGrey),
        );

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
