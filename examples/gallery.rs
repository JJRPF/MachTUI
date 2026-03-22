//! Image Gallery Demo: High-Fidelity Image-to-Braille rendering.

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use image::DynamicImage;
use machtui::core::components::{BoxComponent, Component};
use machtui::core::Renderer;
use machtui::vision::icons::Icons;
use machtui::vision::images::ImageRenderer;
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;

    // Create a procedural placeholder image (Gradient)
    let mut img = DynamicImage::new_rgb8(100, 100);
    if let Some(rgb) = img.as_mut_rgb8() {
        for (x, y, pixel) in rgb.enumerate_pixels_mut() {
            *pixel = image::Rgb([
                (x as f32 / 100.0 * 255.0) as u8,
                (y as f32 / 100.0 * 255.0) as u8,
                150,
            ]);
        }
    }

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(16))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                if code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        let canvas = renderer.canvas_mut();
        canvas.clear();

        // --- HEADER ---
        canvas.draw_gradient_text(
            2,
            1,
            &format!("{} MACHTUI IMAGE GALLERY", Icons::FILE),
            (255, 255, 0),
            (0, 255, 255),
        );

        // --- IMAGE VIEWPORT ---
        let view_w = 60;
        let view_h = 20;
        let b = BoxComponent::new(" IMAGE PREVIEW ");
        b.render(canvas, 2, 3, view_w + 4, view_h + 2);

        ImageRenderer::render_to_canvas(&img, canvas, 4, 4, view_w, view_h);

        // --- INFO PANEL ---
        let info_box = BoxComponent::new(" METADATA ");
        info_box.render(canvas, view_w + 8, 3, 30, 10);
        canvas.draw_text(view_w + 10, 5, "Type: DynamicProcedural", Some(Color::Cyan));
        canvas.draw_text(view_w + 10, 6, "Res: 100x100", Some(Color::White));
        canvas.draw_text(view_w + 10, 7, "Filter: Lanczos3", Some(Color::Green));

        canvas.draw_text(
            2,
            canvas.height - 1,
            "Press 'q' to exit | High-Fidelity Braille + RGB fallback",
            Some(Color::DarkGrey),
        );

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
