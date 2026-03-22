//! Cloud Gateway for MachTUI.
//! Provides streaming support for MachTUI canvases over the network.

use crate::core::Canvas;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CanvasFrame {
    pub width: u16,
    pub height: u16,
    pub cells: Vec<(u16, u16, char, (u8, u8, u8))>, // Sparse update format
}

pub struct RemoteGateway;

impl RemoteGateway {
    /// Serializes a canvas into a sparse frame for cloud streaming.
    pub fn serialize_frame(canvas: &Canvas) -> CanvasFrame {
        let mut cells = Vec::new();
        for y in 0..canvas.height {
            for x in 0..canvas.width {
                let idx = (y as usize * canvas.width as usize) + x as usize;
                let cell = &canvas.cells[idx];
                if cell.content != ' ' {
                    let fg = cell.style.foreground_color.unwrap_or(crossterm::style::Color::Reset);
                    let rgb = match fg {
                        crossterm::style::Color::Rgb { r, g, b } => (r, g, b),
                        _ => (255, 255, 255),
                    };
                    cells.push((x, y, cell.content, rgb));
                }
            }
        }
        CanvasFrame {
            width: canvas.width,
            height: canvas.height,
            cells,
        }
    }
}
