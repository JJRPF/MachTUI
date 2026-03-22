//! High-end notification system for MachTUI.
//! Provides "Toast" notifications with automatic expiry and animation.

use crate::core::Canvas;
use crossterm::style::Color;
use std::time::{Duration, Instant};

pub struct Notification {
    pub message: String,
    pub level: NotificationLevel,
    pub expiry: Instant,
}

pub enum NotificationLevel {
    Info,
    Success,
    Warning,
    Error,
}

impl NotificationLevel {
    pub fn color(&self) -> Color {
        match self {
            NotificationLevel::Info => Color::Cyan,
            NotificationLevel::Success => Color::Green,
            NotificationLevel::Warning => Color::Yellow,
            NotificationLevel::Error => Color::Red,
        }
    }
}

pub struct NotificationManager {
    pub queue: Vec<Notification>,
}

impl NotificationManager {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    pub fn notify(&mut self, message: &str, level: NotificationLevel, duration: Duration) {
        self.queue.push(Notification {
            message: message.to_string(),
            level,
            expiry: Instant::now() + duration,
        });
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.queue.retain(|n| n.expiry > now);
    }

    pub fn render(&self, canvas: &mut Canvas) {
        for (i, notification) in self.queue.iter().enumerate() {
            let x = canvas.width.saturating_sub(35);
            let y = 1 + (i as u16 * 3);

            // Draw Toast Box
            let color = notification.level.color();
            canvas.draw_text(x, y, "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓", Some(color));
            canvas.draw_text(
                x,
                y + 1,
                &format!("┃ {:<28} ┃", notification.message),
                Some(Color::White),
            );
            canvas.draw_text(x, y + 2, "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛", Some(color));
        }
    }
}
