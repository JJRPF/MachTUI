//! Modal and Dialog system for MachTUI.

use crate::core::components::{BoxComponent, Component};
use crate::core::Canvas;
use crossterm::style::Color;

pub struct Modal {
    pub title: String,
    pub message: String,
    pub visible: bool,
}

impl Modal {
    pub fn new(title: &str, message: &str) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
            visible: false,
        }
    }

    pub fn render_on_top(&self, canvas: &mut Canvas) {
        if !self.visible {
            return;
        }

        let mw = 40;
        let mh = 8;
        let mx = (canvas.width - mw) / 2;
        let my = (canvas.height - mh) / 2;

        // Dim background (Layer 90)
        for x in 0..canvas.width {
            for y in 0..canvas.height {
                canvas.set_cell_z(x, y, ' ', Some(Color::Black), 90);
            }
        }

        // Draw Modal Box (Layer 100)
        let b = BoxComponent {
            title: self.title.clone(),
            border_color: Color::Cyan,
        };
        for x in mx..mx + mw {
            for y in my..my + mh {
                canvas.set_cell_z(x, y, ' ', None, 100);
            }
        }
        b.render(canvas, mx, my, mw, mh);
        canvas.draw_text_z(mx + 2, my + 2, &self.message, Some(Color::White), 101);
        canvas.draw_text_z(
            mx + 2,
            my + 6,
            "Press 'Esc' to close",
            Some(Color::DarkGrey),
            101,
        );
    }
}
