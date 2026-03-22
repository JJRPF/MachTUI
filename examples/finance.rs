//! Finance Demo: High-Resolution Real-time Data Visualization.

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use machtui::core::components::{BoxComponent, Component};
use machtui::core::Renderer;
use machtui::vision::charts::Sparkline;
use rand::RngExt;
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let mut data = vec![50.0; 100];
    let mut rng = rand::rng();

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                if code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        // Simulate market volatility
        let last = *data.last().unwrap();
        let next = last + (rng.random_range(-5.0..5.0));
        data.remove(0);
        data.push(next);

        let canvas = renderer.canvas_mut();
        canvas.clear();

        // --- HEADER ---
        canvas.draw_gradient_text(
            2,
            1,
            "MACHTUI terminal Terminal",
            (0, 255, 100),
            (0, 255, 255),
        );

        // --- TICKER CARDS ---
        let cards = [
            ("BTC/USD", Color::Yellow),
            ("ETH/USD", Color::Cyan),
            ("MCH/USD", Color::Magenta),
        ];
        for (i, (name, color)) in cards.iter().enumerate() {
            let x = 2 + (i as u16 * 22);
            let b = BoxComponent {
                title: name.to_string(),
                border_color: *color,
            };
            b.render(canvas, x, 3, 20, 5);
            canvas.draw_text(
                x + 2,
                5,
                &format!("${:.2}", last + i as f32 * 100.0),
                Some(Color::White),
            );
        }

        // --- MAIN CHART ---
        let main_box = BoxComponent::new(" MARKET PERFORMANCE (REAL-TIME) ");
        main_box.render(canvas, 2, 9, 66, 12);

        let spark = Sparkline::new(data.clone(), Color::Green);
        spark.render(canvas, 4, 11, 62, 8);

        canvas.draw_text(
            2,
            22,
            "Press 'q' to exit | Visualizing 100 data points via Braille sub-pixels",
            Some(Color::DarkGrey),
        );

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
