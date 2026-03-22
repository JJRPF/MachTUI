//! Vision Demo: Animated sine waves using sub-pixel Braille rendering.

use machtui::core::Renderer;
use machtui::vision::SubPixelCanvas;
use std::io;
use std::time::Instant;

fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let start_time = Instant::now();

    renderer.run(|canvas, _| {
        let mut subpixels = SubPixelCanvas::new(canvas.width, canvas.height);
        let t = start_time.elapsed().as_secs_f32();

        // Draw a sine wave
        for x in 0..(canvas.width * 2) {
            let y_off = (t * 2.0 + (x as f32 * 0.1)).sin() * 10.0 + 20.0;
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
        
        canvas.draw_text(2, 2, "MachTUI Vision: Sub-pixel Waves", None);
        Ok(true) // Run forever until 'q'
    })
}
