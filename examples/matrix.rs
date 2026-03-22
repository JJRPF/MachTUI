//! Matrix Rain Demo: High-Speed Character Streaming and Color Fading.

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use machtui::core::Renderer;
use rand::RngExt;
use std::io;
use std::time::Duration;

struct Drop {
    x: u16,
    y: f32,
    speed: f32,
    chars: Vec<char>,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let mut rng = rand::rng();
    let mut drops: Vec<Drop> = (0..50)
        .map(|_| Drop {
            x: rng.random_range(0..renderer.canvas_mut().width),
            y: rng.random_range(-20.0..0.0),
            speed: rng.random_range(5.0..15.0),
            chars: (0..10)
                .map(|_| rng.random_range(33..126) as u8 as char)
                .collect(),
        })
        .collect();

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

        for drop in &mut drops {
            drop.y += drop.speed * 0.16;
            if drop.y > canvas.height as f32 {
                drop.y = -10.0;
                drop.x = rng.random_range(0..canvas.width);
            }

            for i in 0..drop.chars.len() {
                let py = drop.y as i32 - i as i32;
                if py >= 0 && py < canvas.height as i32 {
                    let alpha = 1.0 - (i as f32 / drop.chars.len() as f32);
                    let color = if i == 0 {
                        Color::White
                    } else {
                        Color::Rgb {
                            r: 0,
                            g: (255.0 * alpha) as u8,
                            b: 0,
                        }
                    };
                    canvas.set_cell(drop.x, py as u16, drop.chars[i], Some(color));
                }
            }
            // Randomly flip a char
            if rng.random_bool(0.1) {
                drop.chars[0] = rng.random_range(33..126) as u8 as char;
            }
        }

        canvas.draw_gradient_text_z(2, 1, "MACHTUI DIGITAL RAIN", (0, 255, 0), (0, 100, 0), 10);
        canvas.draw_text_z(
            2,
            2,
            "High-Speed Streaming Engine | Press 'q' to exit",
            Some(Color::DarkGrey),
            10,
        );

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
