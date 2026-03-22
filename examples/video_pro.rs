//! Video Pro Demo: High-Resolution Procedural Animation with Sixel Support.

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use image::{DynamicImage, Rgb, RgbImage};
use machtui::core::components::{BoxComponent, Component};
use machtui::core::Renderer;
use machtui::vision::animations::ImageSequence;
use machtui::vision::icons::Icons;
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;

    // Generate procedural "Loading" animation frames
    let mut frames = Vec::new();
    for i in 0..10 {
        let mut img = RgbImage::new(60, 60);
        for x in 0..60 {
            for y in 0..60 {
                let dx = x as f32 - 30.0;
                let dy = y as f32 - 30.0;
                let dist = (dx * dx + dy * dy).sqrt();
                let angle = (i as f32 * 36.0).to_radians();
                let target_angle = (dy).atan2(dx);

                let val = if (dist > 20.0 && dist < 25.0) && (target_angle - angle).abs() < 0.5 {
                    255
                } else {
                    50
                };
                img.put_pixel(x, y, Rgb([val, (val / 2), 255]));
            }
        }
        frames.push(DynamicImage::ImageRgb8(img));
    }

    let sequence = ImageSequence::new(frames, Duration::from_millis(100));

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(30))? {
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
            &format!("{} MACHTUI VIDEO PRO", Icons::ROCKET),
            (0, 255, 255),
            (255, 0, 255),
        );

        // --- ANIMATION VIEWPORT ---
        let b = BoxComponent::new(" LIVE STREAM ");
        b.render(canvas, 4, 3, 44, 22);

        // Render using Braille fallback by default
        sequence.render(canvas, 6, 4, 40, 20);

        // --- CONTROLS ---
        let ctrl_box = BoxComponent::new(" CONTROLS ");
        ctrl_box.render(canvas, 50, 3, 20, 5);
        canvas.draw_text(52, 5, "Status: PLAYING", Some(Color::Green));

        canvas.draw_text(
            2,
            26,
            "Press 'q' to stop | Procedural Sixel-ready Frames",
            Some(Color::DarkGrey),
        );

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
