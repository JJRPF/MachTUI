//! Vector graphics utilities for MachTUI.
//! Provides high-level path and shape drawing using Braille sub-pixels.

use crate::vision::SubPixelCanvas;

pub struct VectorCanvas {
    pub canvas: SubPixelCanvas,
}

impl VectorCanvas {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            canvas: SubPixelCanvas::new(width, height),
        }
    }

    /// Draws a line using Bresenham's algorithm on the sub-pixel grid.
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = x0;
        let mut y = y0;

        loop {
            self.canvas.set_pixel(x as u16, y as u16, true);
            if x == x1 && y == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    /// Draws a simple rectangle on the sub-pixel grid.
    pub fn draw_rect(&mut self, x: u16, y: u16, w: u16, h: u16) {
        let x1 = x as i32;
        let y1 = y as i32;
        let x2 = (x + w) as i32;
        let y2 = (y + h) as i32;

        self.draw_line(x1, y1, x2, y1);
        self.draw_line(x2, y1, x2, y2);
        self.draw_line(x2, y2, x1, y2);
        self.draw_line(x1, y2, x1, y1);
    }
}
