//! Project Pro Demo: Task Management and Checklists.

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use machtui::core::components::{BoxComponent, Component};
use machtui::core::widgets::{Checklist, Tabs};
use machtui::core::Renderer;
use machtui::vision::icons::Icons;
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let mut tasks = Checklist::new(vec![
        "Implement SSH Serving".into(),
        "Refactor Plume Layout".into(),
        "Add Kitty image support".into(),
        "Optimize VDom diffing".into(),
    ]);

    let mut tabs = Tabs::new(vec!["Active".into(), "Completed".into(), "Settings".into()]);

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(16))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => tasks.move_up(),
                    KeyCode::Down => tasks.move_down(),
                    KeyCode::Char(' ') => tasks.toggle(),
                    KeyCode::Tab => tabs.next(),
                    _ => {}
                }
            }
        }

        let canvas = renderer.canvas_mut();
        canvas.clear();

        // --- HEADER ---
        canvas.draw_gradient_text(
            2,
            1,
            &format!("{} MACH PROJECT PRO", Icons::GEAR),
            (0, 255, 100),
            (0, 200, 255),
        );

        // --- TABS ---
        tabs.render(canvas, 2, 3, 40, 1);

        // --- MAIN TASK AREA ---
        let main_box = BoxComponent::new(&format!(
            " {} ",
            tabs.titles[tabs.selected_idx].to_uppercase()
        ));
        main_box.render(canvas, 2, 5, 50, 15);

        if tabs.selected_idx == 0 {
            tasks.render(canvas, 4, 7, 46, 12);
        } else {
            canvas.draw_text(4, 7, "No items here yet.", Some(Color::Grey));
        }

        // --- STATS PANEL ---
        let stats_box = BoxComponent::new(" STATUS ");
        stats_box.render(canvas, 54, 5, 24, 8);
        let done = tasks.items.iter().filter(|i| i.1).count();
        canvas.draw_text(
            56,
            7,
            &format!("Done: {}/{}", done, tasks.items.len()),
            Some(Color::Green),
        );
        canvas.draw_text(56, 9, "Health: OPTIMAL", Some(Color::Cyan));

        canvas.draw_text(
            2,
            21,
            "Arrows: Nav | Space: Toggle | Tab: Tabs | Q: Quit",
            Some(Color::DarkGrey),
        );

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
