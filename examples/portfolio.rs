//! Portfolio Demo: High-End Aesthetics, Layout, and Hover effects.

use machtui::core::Renderer;
use machtui::core::components::{Component, BoxComponent};
use machtui::vision::utils::get_ascii_art;
use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent};
use crossterm::style::Color;
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let mut mouse_pos = (0, 0);

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(16))? {
            match event {
                Event::Key(KeyEvent { code, .. }) => {
                    if code == KeyCode::Char('q') { break; }
                }
                Event::Mouse(MouseEvent { column, row, .. }) => {
                    mouse_pos = (column, row);
                }
                _ => {}
            }
        }

        let canvas = renderer.canvas_mut();
        canvas.clear();

        // --- BACKGROUND DECORATION ---
        for i in 0..canvas.width {
            let color = Color::Rgb { r: 15, g: 23, b: (i % 40 + 20) as u8 };
            for j in 0..canvas.height {
                canvas.set_cell(i, j, ' ', Some(color));
            }
        }

        // --- HEADER ---
        let name_art = get_ascii_art("MACH TUI");
        for (i, line) in name_art.iter().enumerate() {
            canvas.draw_gradient_text(4, 2 + i as u16, line, (0, 255, 255), (255, 0, 255));
        }
        canvas.draw_text(4, 8, "v1.0.0-rc1 | THE AI-NATIVE ENGINE", Some(Color::Grey));

        // --- PORTFOLIO CARDS ---
        let cards = ["Performance", "Architecture", "Multimedia", "AI-Native"];
        for (i, title) in cards.iter().enumerate() {
            let x = 4 + (i as u16 * 18);
            let y = 10;
            let w = 16;
            let h = 6;

            let is_hovered = mouse_pos.0 >= x && mouse_pos.0 < x + w && mouse_pos.1 >= y && mouse_pos.1 < y + h;
            let border_color = if is_hovered { Color::Yellow } else { Color::DarkGrey };
            let title_color = if is_hovered { Color::White } else { Color::Grey };

            let card_box = BoxComponent { title: "".into(), border_color };
            card_box.render(canvas, x, y, w, h);
            
            canvas.draw_text(x + 2, y + 2, title, Some(title_color));
            if is_hovered {
                canvas.draw_text(x + 2, y + 4, "CLICK TO VIEW", Some(Color::Cyan));
            }
        }

        // --- FOOTER ---
        canvas.draw_text(4, 18, "Press 'q' to exit | Use mouse to explore", Some(Color::DarkGrey));

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
