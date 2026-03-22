//! Paint Demo: Mouse-based Canvas Drawing and High-Speed Rendering.

use machtui::core::Renderer;
use machtui::core::components::{Component, BoxComponent};
use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent, MouseEventKind};
use crossterm::style::Color;
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let mut active_color = Color::Green;
    let mut is_drawing = false;

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(5))? {
            match event {
                Event::Key(KeyEvent { code, .. }) => {
                    match code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('c') => {
                            renderer.canvas_mut().clear();
                        }
                        KeyCode::Char('1') => active_color = Color::Red,
                        KeyCode::Char('2') => active_color = Color::Green,
                        KeyCode::Char('3') => active_color = Color::Blue,
                        KeyCode::Char('4') => active_color = Color::Yellow,
                        _ => {}
                    }
                }
                Event::Mouse(MouseEvent { kind, column, row, .. }) => {
                    match kind {
                        MouseEventKind::Down(_) => is_drawing = true,
                        MouseEventKind::Up(_) => is_drawing = false,
                        MouseEventKind::Drag(_) => {
                            if is_drawing {
                                renderer.canvas_mut().set_cell(column, row, '█', Some(active_color));
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // We don't clear the canvas every frame to preserve drawing!
        let canvas = renderer.canvas_mut();
        
        // --- OVERLAY: UI ---
        canvas.draw_gradient_text_z(2, 1, "MACHTUI CANVAS PAINT", (255, 0, 255), (0, 255, 255), 10);
        canvas.draw_text_z(2, 2, "Colors: 1:Red 2:Grn 3:Blu 4:Yel | C:Clear | Q:Quit", Some(Color::White), 10);
        
        // Draw a brush indicator (Layer 10)
        let color_name = match active_color {
            Color::Red => "RED",
            Color::Green => "GREEN",
            Color::Blue => "BLUE",
            Color::Yellow => "YELLOW",
            _ => "UNKNOWN",
        };
        canvas.draw_text_z(2, 4, &format!("Active: {}", color_name), Some(active_color), 10);

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
