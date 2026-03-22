//! Music Visualizer Demo: Synchronized Audio and Braille Graphics.

use machtui::core::Renderer;
use machtui::vision::SubPixelCanvas;
use machtui::vision::sound::SoundEngine;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use std::io;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let sound = SoundEngine::new();
    let start_time = Instant::now();

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(10))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                if code == KeyCode::Char('q') { break; }
                if code == KeyCode::Char('p') {
                    if let Some(ref s) = sound { s.play_tone(440.0); }
                }
            }
        }

        let canvas = renderer.canvas_mut();
        canvas.clear();

        let t = start_time.elapsed().as_secs_f32();
        
        // --- DRAW VISUALIZER BARS ---
        let mut subpixels = SubPixelCanvas::new(canvas.width, canvas.height);
        for x in 0..(canvas.width * 2) {
            // Simulated frequency data
            let h = (t * 2.0 + (x as f32 * 0.2)).sin().abs() * 20.0 + 
                    (t * 5.0 + (x as f32 * 0.5)).cos().abs() * 10.0;
            
            for y in 0..(h as u16) {
                if y < canvas.height * 4 {
                    subpixels.set_pixel(x, (canvas.height * 4) - 1 - y, true);
                }
            }
        }

        let cells = subpixels.render_to_cells();
        for y in 0..canvas.height {
            for x in 0..canvas.width {
                let idx = (y as usize) * (canvas.width as usize) + (x as usize);
                if cells[idx] != '\u{2800}' {
                    let r = (x as f32 / canvas.width as f32 * 255.0) as u8;
                    let g = (y as f32 / canvas.height as f32 * 255.0) as u8;
                    canvas.set_cell(x, y, cells[idx], Some(Color::Rgb { r, g, b: 255 }));
                }
            }
        }

        canvas.draw_gradient_text(2, 1, "MACHTUI AUDIO VISUALIZER", (255, 0, 255), (0, 255, 255));
        canvas.draw_text(2, 3, "Press 'p' to play test tone | 'q' to exit", Some(Color::White));

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
