//! High-resolution charting utilities for MachTUI.
//! Uses Braille sub-pixels for smooth line and area charts.

use crate::core::Canvas;
use crate::vision::SubPixelCanvas;
use crossterm::style::Color;

pub struct Sparkline {
    pub data: Vec<f32>,
    pub color: Color,
}

impl Sparkline {
    pub fn new(data: Vec<f32>, color: Color) -> Self {
        Self { data, color }
    }

    pub fn render(&self, canvas: &mut Canvas, x: u16, y: u16, width: u16, height: u16) {
        if self.data.is_empty() {
            return;
        }

        let mut subpixels = SubPixelCanvas::new(width, height);
        let max = self.data.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let min = self.data.iter().cloned().fold(f32::INFINITY, f32::min);
        let range = (max - min).max(1.0);

        let sub_w = width * 2;
        let sub_h = height * 4;

        for (i, &val) in self.data.iter().enumerate() {
            let px = (i as f32 / self.data.len() as f32 * sub_w as f32) as u16;
            let normalized = (val - min) / range;
            let py = (sub_h as f32 - (normalized * (sub_h - 1) as f32)) as u16;

            if px < sub_w && py < sub_h {
                subpixels.set_pixel(px, py, true);
            }
        }

        let cells = subpixels.render_to_cells();
        for cy in 0..height {
            for cx in 0..width {
                let idx = (cy as usize) * (width as usize) + (cx as usize);
                if cells[idx] != '\u{2800}' {
                    canvas.set_cell(x + cx, y + cy, cells[idx], Some(self.color));
                }
            }
        }
    }
}

pub struct AreaChart {
    pub data: Vec<f32>,
    pub primary_color: Color,
    pub fill_color: (u8, u8, u8),
}

impl AreaChart {
    pub fn new(data: Vec<f32>, primary: Color, fill: (u8, u8, u8)) -> Self {
        Self {
            data,
            primary_color: primary,
            fill_color: fill,
        }
    }

    pub fn render(&self, canvas: &mut Canvas, x: u16, y: u16, width: u16, height: u16) {
        if self.data.is_empty() {
            return;
        }

        let mut subpixels = SubPixelCanvas::new(width, height);
        let max = self.data.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let min = self.data.iter().cloned().fold(f32::INFINITY, f32::min);
        let range = (max - min).max(1.0);

        let sub_w = width * 2;
        let sub_h = height * 4;

        for (i, &val) in self.data.iter().enumerate() {
            let px = (i as f32 / self.data.len() as f32 * sub_w as f32) as u16;
            let normalized = (val - min) / range;
            let py_top = (sub_h as f32 - (normalized * (sub_h - 1) as f32)) as u16;

            if px < sub_w {
                for py in py_top..sub_h {
                    subpixels.set_pixel(px, py, true);
                }
            }
        }

        let cells = subpixels.render_to_cells();
        for cy in 0..height {
            for cx in 0..width {
                let idx = (cy as usize) * (width as usize) + (cx as usize);
                if cells[idx] != '\u{2800}' {
                    let t = cy as f32 / height as f32;
                    let r = (self.fill_color.0 as f32 * (1.0 - t * 0.5)) as u8;
                    let g = (self.fill_color.1 as f32 * (1.0 - t * 0.5)) as u8;
                    let b = (self.fill_color.2 as f32 * (1.0 - t * 0.5)) as u8;
                    canvas.set_cell(x + cx, y + cy, cells[idx], Some(Color::Rgb { r, g, b }));
                }
            }
        }
    }
}
