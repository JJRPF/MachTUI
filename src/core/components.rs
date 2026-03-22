//! Component system for MachTUI.
//! Provides reusable UI elements with built-in styling and event handling.

use crate::core::Canvas;
use crossterm::style::Color;
use crate::oracle::{OracleProvider, SemanticNode};

/// The base trait for all MachTUI components.
pub trait Component {
    fn render(&self, canvas: &mut Canvas, x: u16, y: u16, width: u16, height: u16);
}

/// A high-end Progress Bar component with gradient support.
pub struct ProgressBar {
    pub progress: f32, // 0.0 to 1.0
    pub label: String,
    pub start_color: (u8, u8, u8),
    pub end_color: (u8, u8, u8),
}

impl ProgressBar {
    pub fn new(label: &str, progress: f32) -> Self {
        Self {
            label: label.to_string(),
            progress: progress.clamp(0.0, 1.0),
            start_color: (0, 255, 0),
            end_color: (0, 100, 255),
        }
    }
}

impl Component for ProgressBar {
    fn render(&self, canvas: &mut Canvas, x: u16, y: u16, width: u16, _height: u16) {
        let filled_width = (width as f32 * self.progress) as u16;
        
        // Draw track
        for i in 0..width {
            canvas.set_cell(x + i, y, '░', Some(Color::DarkGrey));
        }
        
        // Draw fill with gradient
        let filled_text = "█".repeat(filled_width as usize);
        canvas.draw_gradient_text(x, y, &filled_text, self.start_color, self.end_color);
        
        // Draw label
        canvas.draw_text(x, y + 1, &format!("{} ({:.0}%)", self.label, self.progress * 100.0), Some(Color::White));
    }
}

impl OracleProvider for ProgressBar {
    fn to_semantic(&self) -> SemanticNode {
        SemanticNode::new("progress_bar")
            .with_content(&self.label)
            .with_metadata("progress", &format!("{:.2}", self.progress))
    }
}

/// A stylized Box component for grouping UI elements.
pub struct BoxComponent {
    pub title: String,
    pub border_color: Color,
}

impl BoxComponent {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            border_color: Color::Cyan,
        }
    }
}

impl Component for BoxComponent {
    fn render(&self, canvas: &mut Canvas, x: u16, y: u16, width: u16, height: u16) {
        // Draw corners
        canvas.set_cell(x, y, '┏', Some(self.border_color));
        canvas.set_cell(x + width - 1, y, '┓', Some(self.border_color));
        canvas.set_cell(x, y + height - 1, '┗', Some(self.border_color));
        canvas.set_cell(x + width - 1, y + height - 1, '┛', Some(self.border_color));

        // Draw horizontal lines
        for i in 1..width - 1 {
            canvas.set_cell(x + i, y, '━', Some(self.border_color));
            canvas.set_cell(x + i, y + height - 1, '━', Some(self.border_color));
        }

        // Draw vertical lines
        for i in 1..height - 1 {
            canvas.set_cell(x, y + i, '┃', Some(self.border_color));
            canvas.set_cell(x + width - 1, y + i, '┃', Some(self.border_color));
        }

        // Draw title
        if !self.title.is_empty() {
            let title_str = format!(" {} ", self.title);
            canvas.draw_text(x + 2, y, &title_str, Some(Color::White));
        }
    }
}

impl OracleProvider for BoxComponent {
    fn to_semantic(&self) -> SemanticNode {
        SemanticNode::new("box")
            .with_content(&self.title)
    }
}
