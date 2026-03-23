//! Drag and Drop system foundation for MachTUI.
//! Provides tracking for mouse-based component movement.

use crate::core::Canvas;
use crossterm::event::{MouseEvent, MouseEventKind};

pub struct DragState {
    pub is_dragging: bool,
    pub start_pos: (u16, u16),
    pub current_pos: (u16, u16),
    pub payload: Option<String>,
}

impl DragState {
    pub fn new() -> Self {
        Self {
            is_dragging: false,
            start_pos: (0, 0),
            current_pos: (0, 0),
            payload: None,
        }
    }

    pub fn handle_mouse(&mut self, event: &MouseEvent) {
        match event.kind {
            MouseEventKind::Down(_) => {
                self.is_dragging = true;
                self.start_pos = (event.column, event.row);
                self.current_pos = (event.column, event.row);
            }
            MouseEventKind::Drag(_) => {
                if self.is_dragging {
                    self.current_pos = (event.column, event.row);
                }
            }
            MouseEventKind::Up(_) => {
                self.is_dragging = false;
            }
            _ => {}
        }
    }

    pub fn render_ghost(&self, canvas: &mut Canvas) {
        if self.is_dragging {
            if let Some(ref text) = self.payload {
                canvas.draw_text_z(self.current_pos.0, self.current_pos.1, text, Some(crossterm::style::Color::DarkGrey), 200);
            }
        }
    }
}
