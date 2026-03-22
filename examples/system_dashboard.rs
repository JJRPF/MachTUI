//! System Dashboard Demo: Synthesis of all MachTUI features.

use machtui::core::Renderer;
use machtui::core::components::{Component, ProgressBar, BoxComponent};
use machtui::vision::icons::Icons;
use machtui::vision::charts::Sparkline;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let mut cpu_data = vec![0.0; 50];

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                if code == KeyCode::Char('q') { break; }
            }
        }

        // Simulating data
        let next_val = (rand::random::<f32>() * 100.0);
        cpu_data.remove(0);
        cpu_data.push(next_val);

        let canvas = renderer.canvas_mut();
        canvas.clear();

        // --- MASTER CONTAINER ---
        let main_box = BoxComponent::new(&format!("{} MACHTUI COMMAND CENTER ", Icons::ROCKET));
        main_box.render(canvas, 1, 1, canvas.width - 2, canvas.height - 2);

        // --- TOP ROW: STATUS CARDS ---
        let row1_y = 3;
        let card_w = 20;
        
        let cpu_card = BoxComponent { title: format!("{} CPU", Icons::CPU), border_color: Color::Green };
        cpu_card.render(canvas, 3, row1_y, card_w, 5);
        canvas.draw_text(5, row1_y + 2, &format!("{:.1}% Load", next_val), Some(Color::White));

        let ram_card = BoxComponent { title: format!("{} RAM", Icons::RAM), border_color: Color::Cyan };
        ram_card.render(canvas, 3 + card_w + 2, row1_y, card_w, 5);
        canvas.draw_text(5 + card_w + 2, row1_y + 2, "4.2 GB / 16 GB", Some(Color::White));

        // --- MIDDLE ROW: CHARTS ---
        let chart_box = BoxComponent::new(&format!("{} PERFORMANCE TELEMETRY ", Icons::CHART));
        chart_box.render(canvas, 3, 9, canvas.width - 6, 10);
        
        let spark = Sparkline::new(cpu_data.clone(), Color::Magenta);
        spark.render(canvas, 5, 11, canvas.width - 10, 6);

        // --- FOOTER ---
        canvas.draw_text(3, canvas.height - 2, &format!("{} System Stable | Press 'q' to disconnect", Icons::SUCCESS), Some(Color::DarkGrey));

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
