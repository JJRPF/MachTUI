//! Game Demo: High-speed sprite animation and physics.

use machtui::core::Renderer;
use machtui::vision::sprites::Sprite;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use std::io;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let start_time = Instant::now();
    let mut last_tick = Instant::now();

    // Create a simple "Ship" sprite
    let ship_frames = vec![
        vec![
            "  /\\  ".to_string(),
            " /  \\ ".to_string(),
            "|MACH |".to_string(),
            " \\__/ ".to_string(),
        ]
    ];
    let mut ship = Sprite::new(ship_frames);
    ship.x = 10.0;
    ship.y = 10.0;

    loop {
        let dt = last_tick.elapsed().as_secs_f32();
        last_tick = Instant::now();

        if let Some(event) = renderer.poll_event(Duration::from_millis(1))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => ship.velocity_y = -20.0,
                    KeyCode::Down => ship.velocity_y = 20.0,
                    KeyCode::Left => ship.velocity_x = -40.0,
                    KeyCode::Right => ship.velocity_x = 40.0,
                    _ => {}
                }
            }
        } else {
            // Apply friction
            ship.velocity_x *= 0.95;
            ship.velocity_y *= 0.95;
        }

        ship.update(dt);

        let canvas = renderer.canvas_mut();
        canvas.clear();

        // Draw some "Stars" in the background
        let t = start_time.elapsed().as_secs_f32();
        for i in 0..20 {
            let x = ((i as f32 * 137.5).sin() * 0.5 + 0.5) * canvas.width as f32;
            let y = ((i as f32 * 55.2).cos() * 0.5 + 0.5) * canvas.height as f32;
            let star_char = if (t * 5.0 + i as f32).sin() > 0.0 { '*' } else { '.' };
            canvas.set_cell(x as u16, y as u16, star_char, Some(Color::DarkGrey));
        }

        ship.render(canvas);

        canvas.draw_text(2, 1, "--- MACHTUI GAME ENGINE DEMO ---", Some(Color::Yellow));
        canvas.draw_text(2, 2, "Use Arrows to Fly | Q to Quit", Some(Color::Grey));

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
