//! Explorer Demo: Advanced File Explorer with Widgets and Animation.

use machtui::core::Renderer;
use machtui::core::widgets::{ListSelection, TextInput};
use machtui::core::components::{Component, BoxComponent};
use machtui::core::animation::{Tween, Easing};
use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent, MouseEventKind};
use crossterm::style::Color;
use std::io;
use std::fs;
use std::time::Duration;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    
    // Get files in current dir
    let mut items = fs::read_dir(".")?
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    items.sort();

    let mut list = ListSelection::new(items);
    let mut search = TextInput::new("Type to search...");
    search.focused = true;

    // Animation for sidebar width
    let mut sidebar_tween = Tween::new(0.0, 30.0, Duration::from_millis(500), Easing::CubicOut);

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(10))? {
            match event {
                Event::Key(KeyEvent { code, .. }) => {
                    match code {
                        KeyCode::Char('q') => break,
                        KeyCode::Up => list.move_up(),
                        KeyCode::Down => list.move_down(),
                        KeyCode::Char(c) => search.handle_char(c),
                        KeyCode::Backspace => search.handle_backspace(),
                        _ => {}
                    }
                }
                Event::Mouse(MouseEvent { kind, column, row, .. }) => {
                    if let MouseEventKind::Down(_) = kind {
                        // Simple toggle focus
                        search.focused = row == 2;
                    }
                }
                _ => {}
            }
        }

        let canvas = renderer.canvas_mut();
        canvas.clear();

        let sidebar_w = sidebar_tween.value() as u16;

        // Draw Sidebar
        let side_box = BoxComponent::new(" EXPLORER ");
        side_box.render(canvas, 1, 1, sidebar_w, 20);
        
        if sidebar_w > 5 {
            search.render(canvas, 2, 2, sidebar_w - 2, 3);
            list.render(canvas, 2, 5, sidebar_w - 2, 15);
        }

        // Draw Content Area
        let content_box = BoxComponent::new(" PREVIEW ");
        content_box.render(canvas, sidebar_w + 1, 1, canvas.width - sidebar_w - 2, 20);
        
        let selected_file = &list.items[list.selected_idx];
        canvas.draw_gradient_text(sidebar_w + 3, 3, &format!("File: {}", selected_file), (255, 255, 255), (100, 100, 100));
        
        canvas.draw_text(2, 22, "Arrows: Nav | Type: Search | Q: Quit", Some(Color::DarkGrey));

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
