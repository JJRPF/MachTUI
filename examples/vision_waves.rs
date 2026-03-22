//! Vision Demo: High-end animated sine waves using sub-pixel Braille rendering.

use machtui::core::Renderer;
use machtui::vision::SubPixelCanvas;
use std::io;
use std::time::{Instant, Duration};
use crossterm::event::{Event, KeyCode, KeyEvent};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let start_time = Instant::now();

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(5))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                if code == KeyCode::Char('q') { break; }
            }
        }

        let canvas = renderer.canvas_mut();
        canvas.clear();
        
        let mut subpixels = SubPixelCanvas::new(canvas.width, canvas.height);
        let t = start_time.elapsed().as_secs_f32();

        for x in 0..(canvas.width * 2) {
            let y_off = (t * 3.0 + (x as f32 * 0.05)).sin() * 15.0 + 30.0;
            let y = y_off as u16;
            if y < canvas.height * 4 {
                subpixels.set_pixel(x, y, true);
            }
        }

        let cells = subpixels.render_to_cells();
        for y in 0..canvas.height {
            for x in 0..canvas.width {
                let idx = (y as usize) * (canvas.width as usize) + (x as usize);
                canvas.set_cell(x, y, cells[idx], None);
            }
        }
        
        canvas.draw_text(2, 2, "MachTUI Vision: Ultra-Smooth Sub-pixel Waves", None);
        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
