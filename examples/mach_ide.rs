//! MachIDE Demo: High-End Code Editor with Syntax Highlighting and Layout.

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use machtui::core::components::{BoxComponent, Component};
use machtui::core::widgets::ListSelection;
use machtui::core::Renderer;
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let mut files = ListSelection::new(vec![
        "src/lib.rs".to_string(),
        "src/core/mod.rs".to_string(),
        "Cargo.toml".to_string(),
        "README.md".to_string(),
    ]);

    let code_lines = vec![
        "fn main() {".to_string(),
        "    println!(\"Hello MachTUI!\");".to_string(),
        "}".to_string(),
    ];

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(16))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => files.move_up(),
                    KeyCode::Down => files.move_down(),
                    _ => {}
                }
            }
        }

        let canvas = renderer.canvas_mut();
        canvas.clear();

        // --- LAYOUT ---
        let sidebar_w = 25;
        let header_h = 3;

        // 1. Header
        canvas.draw_gradient_text(2, 1, "MACH IDE v1.0", (0, 255, 255), (255, 0, 255));

        // 2. Sidebar (Files)
        let side_box = BoxComponent::new(" FILES ");
        side_box.render(canvas, 1, header_h, sidebar_w, canvas.height - header_h - 2);
        files.render(canvas, 3, header_h + 2, sidebar_w - 4, 10);

        // 3. Editor Area
        let edit_box = BoxComponent::new(" EDITOR ");
        edit_box.render(
            canvas,
            sidebar_w + 1,
            header_h,
            canvas.width - sidebar_w - 2,
            canvas.height - header_h - 2,
        );

        for (i, line) in code_lines.iter().enumerate() {
            let y = header_h + 2 + i as u16;
            canvas.draw_text(
                sidebar_w + 3,
                y,
                &format!("{:>2} | ", i + 1),
                Some(Color::DarkGrey),
            );
            canvas.draw_text(sidebar_w + 8, y, line, Some(Color::White));
        }

        canvas.draw_text(
            2,
            canvas.height - 1,
            "Arrows: Navigate Files | Q: Quit",
            Some(Color::DarkGrey),
        );

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
