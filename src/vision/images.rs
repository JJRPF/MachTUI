//! High-fidelity image rendering for MachTUI.
//! Converts standard images into Braille-based terminal graphics with RGB colors.

use crate::core::Canvas;
use crate::vision::SubPixelCanvas;
use image::{GenericImageView, DynamicImage};
use crossterm::style::Color;

pub struct ImageRenderer;

impl ImageRenderer {
    /// Renders an image into a MachTUI Canvas using Braille sub-pixels and RGB colors.
    pub fn render_to_canvas(img: &DynamicImage, canvas: &mut Canvas, x: u16, y: u16, width: u16, height: u16) {
        let resized = img.resize_exact(width as u32 * 2, height as u32 * 4, image::imageops::FilterType::Lanczos3);
        let mut subpixels = SubPixelCanvas::new(width, height);

        for py in 0..(height * 4) {
            for px in 0..(width * 2) {
                let pixel = resized.get_pixel(px as u32, py as u32);
                let luma = 0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32;
                if luma > 128.0 {
                    subpixels.set_pixel(px, py, true);
                }
            }
        }

        let cells = subpixels.render_to_cells();
        for cy in 0..height {
            for cx in 0..width {
                let idx = (cy as usize) * (width as usize) + (cx as usize);
                if cells[idx] != '\u{2800}' {
                    // Sample color from the center of the 2x4 subpixel block
                    let pixel = resized.get_pixel(cx as u32 * 2, cy as u32 * 4);
                    let color = Color::Rgb { r: pixel[0], g: pixel[1], b: pixel[2] };
                    canvas.set_cell(x + cx, y + cy, cells[idx], Some(color));
                }
            }
        }
    }
}
