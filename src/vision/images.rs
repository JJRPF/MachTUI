//! High-fidelity image rendering for MachTUI.
//! Supports Kitty Graphics Protocol and Braille fallbacks.

use crate::core::Canvas;
use crate::vision::SubPixelCanvas;
use image::{GenericImageView, DynamicImage};
use crossterm::style::Color;
use base64::{Engine as _, engine::general_purpose};
use std::io::{self, Write};

pub struct ImageRenderer;

impl ImageRenderer {
    /// Renders an image using the Kitty graphics protocol if supported.
    pub fn render_kitty(img: &DynamicImage, stdout: &mut dyn Write) -> io::Result<()> {
        let rgba = img.to_rgba8();
        let (w, h) = rgba.dimensions();
        let raw_data = rgba.into_raw();
        let encoded = general_purpose::STANDARD.encode(raw_data);

        // Kitty protocol: a=T (transfer), t=d (direct), f=32 (rgba), s=width, v=height
        write!(stdout, "\x1b_Ga=T,t=d,f=32,s={},v={};{}\x1b\\", w, h, encoded)?;
        stdout.flush()?;
        Ok(())
    }

    /// Fallback: Renders an image into a MachTUI Canvas using Braille sub-pixels.
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
                    let pixel = resized.get_pixel(cx as u32 * 2, cy as u32 * 4);
                    let color = Color::Rgb { r: pixel[0], g: pixel[1], b: pixel[2] };
                    canvas.set_cell(x + cx, y + cy, cells[idx], Some(color));
                }
            }
        }
    }
}
