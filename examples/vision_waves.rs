//! Vision Demo: High-End Animated Waves.

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use machtui::core::Renderer;
use machtui::vision::utils::get_ascii_art;
use machtui::vision::SubPixelCanvas;
use std::io;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let start_time = Instant::now();
    let header = get_ascii_art("VISION");

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(5))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                if code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        let canvas = renderer.canvas_mut();
        canvas.clear();

        let t = start_time.elapsed().as_secs_f32();

        for (i, line) in header.iter().enumerate() {
            canvas.draw_gradient_text(2, 1 + i as u16, line, (255, 255, 0), (255, 0, 0));
        }

        let mut subpixels = SubPixelCanvas::new(canvas.width, canvas.height);
        for x in 0..(canvas.width * 2) {
            let y_off = (t * 3.0 + (x as f32 * 0.05)).sin() * 15.0 + 30.0;
            let y = y_off as u16;
            if y < canvas.height * 4 {
                subpixels.set_pixel(x, y, true);
            }
        }

        let wave_color = Color::Rgb {
            r: ((t.sin() * 0.5 + 0.5) * 255.0) as u8,
            g: ((t.cos() * 0.5 + 0.5) * 255.0) as u8,
            b: 200,
        };

        let cells = subpixels.render_to_cells();
        for y in 0..canvas.height {
            for x in 0..canvas.width {
                let idx = (y as usize) * (canvas.width as usize) + (x as usize);
                if cells[idx] != '\u{2800}' {
                    canvas.set_cell(x, y, cells[idx], Some(wave_color));
                }
            }
        }

        canvas.draw_text(
            2,
            10,
            "MachTUI Ultra-Smooth Braille Waves",
            Some(Color::White),
        );
        canvas.draw_text(2, 11, "Press 'q' to exit", Some(Color::DarkGrey));
        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
