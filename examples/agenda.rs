//! Agenda Demo: Calendar and Task Management with Notifications.

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use machtui::core::components::{BoxComponent, Component};
use machtui::core::notifications::{NotificationLevel, NotificationManager};
use machtui::core::Renderer;
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let mut notifications = NotificationManager::new();

    let mut tasks = vec![
        "Finish MachTUI v1.0".to_string(),
        "Add Sixel support".to_string(),
        "AI Oracle optimization".to_string(),
    ];

    notifications.notify(
        "Welcome to MachAgenda!",
        NotificationLevel::Info,
        Duration::from_secs(3),
    );

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(16))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('a') => {
                        tasks.push("New Task".to_string());
                        notifications.notify(
                            "Task added!",
                            NotificationLevel::Success,
                            Duration::from_secs(2),
                        );
                    }
                    KeyCode::Char('d') => {
                        if !tasks.is_empty() {
                            tasks.pop();
                            notifications.notify(
                                "Task removed.",
                                NotificationLevel::Warning,
                                Duration::from_secs(2),
                            );
                        }
                    }
                    _ => {}
                }
            }
        }

        notifications.update();

        let canvas = renderer.canvas_mut();
        canvas.clear();

        // --- DRAW CALENDAR VIEW ---
        let cal_box = BoxComponent::new(" CALENDAR: MARCH 2026 ");
        cal_box.render(canvas, 2, 1, 40, 15);

        let days = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];
        for (i, day) in days.iter().enumerate() {
            canvas.draw_text(4 + (i as u16 * 5), 3, day, Some(Color::Cyan));
        }

        for day in 1..=31 {
            let x = 4 + (((day - 1) % 7) as u16 * 5);
            let y = 5 + (((day - 1) / 7) as u16 * 2);
            let color = if day == 22 {
                Color::Yellow
            } else {
                Color::White
            };
            canvas.draw_text(x, y, &format!("{:02}", day), Some(color));
        }

        // --- DRAW TASKS ---
        let task_box = BoxComponent::new(" AGENDA ");
        task_box.render(canvas, 44, 1, 30, 15);
        for (i, task) in tasks.iter().enumerate() {
            canvas.draw_text(46, 3 + i as u16, &format!("• {}", task), Some(Color::Green));
        }

        canvas.draw_text(
            2,
            17,
            "Press 'a' to add task | 'd' to delete | 'q' to quit",
            Some(Color::DarkGrey),
        );

        // --- RENDER NOTIFICATIONS OVERLAY ---
        notifications.render(canvas);

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
