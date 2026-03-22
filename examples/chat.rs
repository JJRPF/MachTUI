//! Chat Demo: Advanced Layout and Widgets.

use machtui::core::Renderer;
use machtui::core::widgets::{TextInput, ListSelection};
use machtui::core::components::{Component, BoxComponent};
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let mut input = TextInput::new("Type message...");
    input.focused = true;

    let mut messages = ListSelection::new(vec![
        "System: Welcome to MachChat!".to_string(),
        "User1: Hello world!".to_string(),
        "User2: MachTUI is so fast!".to_string(),
    ]);

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(10))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Enter => {
                        if !input.content.is_empty() {
                            messages.items.push(format!("Me: {}", input.content));
                            input.content.clear();
                            input.cursor_pos = 0;
                            messages.selected_idx = messages.items.len() - 1;
                        }
                    }
                    KeyCode::Char(c) => input.handle_char(c),
                    KeyCode::Backspace => input.handle_backspace(),
                    KeyCode::Up => messages.move_up(),
                    KeyCode::Down => messages.move_down(),
                    _ => {}
                }
            }
        }

        let canvas = renderer.canvas_mut();
        canvas.clear();

        // --- LAYOUT ---
        let chat_w = (canvas.width as f32 * 0.8) as u16;
        let chat_h = (canvas.height as f32 * 0.8) as u16;
        let start_x = (canvas.width - chat_w) / 2;
        let start_y = (canvas.height - chat_h) / 2;

        // 1. Chat Container
        let chat_box = BoxComponent::new(" MACH CHAT ");
        chat_box.render(canvas, start_x, start_y, chat_w, chat_h);

        // 2. Message List
        messages.render(canvas, start_x + 2, start_y + 2, chat_w - 4, chat_h - 7);

        // 3. Input Field (Bottom)
        input.render(canvas, start_x + 2, start_y + chat_h - 4, chat_w - 4, 3);

        canvas.draw_text(start_x + 2, start_y + chat_h + 1, "Press Enter to Send | Q to Quit", Some(Color::DarkGrey));

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
