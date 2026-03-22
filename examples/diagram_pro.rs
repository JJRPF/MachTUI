//! Diagram Pro Demo: Vector Graphics and Shapes.

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use machtui::core::components::Component;
use machtui::core::Renderer;
use machtui::vision::icons::Icons;
use machtui::vision::vector::VectorCanvas;
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let mut rotation = 0.0;

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(16))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                if code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        rotation += 0.1;

        let canvas = renderer.canvas_mut();
        canvas.clear();

        // --- HEADER ---
        canvas.draw_gradient_text(
            2,
            1,
            &format!("{} MACH DIAGRAM PRO", Icons::GEAR),
            (255, 255, 0),
            (255, 0, 255),
        );

        // --- VECTOR VIEWPORT ---
        let mut vc = VectorCanvas::new(40, 15);

        // Draw some static shapes
        vc.draw_rect(5, 5, 30, 20);
        vc.draw_line(5, 5, 35, 25);
        vc.draw_line(35, 5, 5, 25);

        // Draw an animated star-like shape (simplified)
        let cx = 40;
        let cy = 30;
        for i in 0..8 {
            let angle = rotation + (i as f32 * std::f32::consts::PI / 4.0);
            let rx = (cx as f32 + angle.cos() * 20.0) as i32;
            let ry = (cy as f32 + angle.sin() * 20.0) as i32;
            vc.draw_line(cx as i32, cy as i32, rx, ry);
        }

        let cells = vc.canvas.render_to_cells();
        for y in 0..15 {
            for x in 0..40 {
                let idx = (y as usize * 40) + x as usize;
                if cells[idx] != '\u{2800}' {
                    canvas.set_cell(x + 4, y + 4, cells[idx], Some(Color::Cyan));
                }
            }
        }

        canvas.draw_text(
            2,
            22,
            "Press 'q' to exit | Real-time Vector Bresenham Line Rendering",
            Some(Color::DarkGrey),
        );

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
