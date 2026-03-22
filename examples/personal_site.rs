//! Personal Site Demo: Z-Index Layering and High-End Typography.

use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent};
use crossterm::style::Color;
use machtui::core::components::{BoxComponent, Component};
use machtui::core::Renderer;
use machtui::vision::icons::Icons;
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let mut show_modal = false;
    let mut mouse_pos = (0, 0);

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(16))? {
            match event {
                Event::Key(KeyEvent { code, .. }) => {
                    if code == KeyCode::Char('q') {
                        break;
                    }
                    if code == KeyCode::Char('m') {
                        show_modal = !show_modal;
                    }
                }
                Event::Mouse(MouseEvent { column, row, .. }) => {
                    mouse_pos = (column, row);
                }
                _ => {}
            }
        }

        let canvas = renderer.canvas_mut();
        canvas.clear();

        // --- BACKGROUND (Layer 0) ---
        for x in 0..canvas.width {
            for y in 0..canvas.height {
                canvas.set_cell_z(
                    x,
                    y,
                    '.',
                    Some(Color::Rgb {
                        r: 20,
                        g: 20,
                        b: 30,
                    }),
                    0,
                );
            }
        }

        // --- MAIN CONTENT (Layer 1) ---
        canvas.draw_gradient_text_z(4, 2, "JJR PRO PORTFOLIO", (0, 255, 255), (255, 0, 255), 1);
        canvas.draw_text_z(
            4,
            4,
            "A display of high-end terminal engineering.",
            Some(Color::Grey),
            1,
        );

        let projects = ["MachTUI Engine", "AI Oracle Protocol", "Vision Graphics"];
        for (i, p) in projects.iter().enumerate() {
            let y = 7 + (i as u16 * 4);
            let b = BoxComponent {
                title: p.to_string(),
                border_color: Color::Green,
            };
            b.render(canvas, 4, y, 30, 3);
        }

        // --- MODAL OVERLAY (Layer 10) ---
        if show_modal {
            let mw = 40;
            let mh = 10;
            let mx = (canvas.width - mw) / 2;
            let my = (canvas.height - mh) / 2;

            // Dim the background (Layer 9)
            for x in 0..canvas.width {
                for y in 0..canvas.height {
                    canvas.set_cell_z(x, y, ' ', Some(Color::Black), 9);
                }
            }

            let modal_box = BoxComponent {
                title: format!("{} INFORMATION", Icons::GEAR),
                border_color: Color::Cyan,
            };
            // Draw modal on top (Layer 10)
            for x in mx..mx + mw {
                for y in my..my + mh {
                    canvas.set_cell_z(x, y, ' ', None, 10);
                }
            }
            modal_box.render(canvas, mx, my, mw, mh); // Note: Component needs z-index support, for now we manual draw
            canvas.draw_text_z(
                mx + 2,
                my + 2,
                "This is a high-z-index modal!",
                Some(Color::White),
                11,
            );
            canvas.draw_text_z(
                mx + 2,
                my + 4,
                "It overlaps all background content.",
                Some(Color::Grey),
                11,
            );
            canvas.draw_text_z(
                mx + 2,
                my + 8,
                "Press 'm' to close",
                Some(Color::Yellow),
                11,
            );
        }

        canvas.draw_text_z(
            2,
            canvas.height - 1,
            "Press 'm' for Modal | 'q' to Quit",
            Some(Color::DarkGrey),
            50,
        );

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
