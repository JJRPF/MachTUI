//! The "Vision" Layer
//! 
//! Provides advanced terminal graphics, including sub-pixel rendering using Braille characters.

pub mod utils;
pub mod images;
pub mod sprites;
pub mod colors;
pub mod sound;
pub mod charts;
pub mod icons;
pub mod animations;

/// A canvas for drawing sub-pixels (2x4 per terminal cell).
pub struct SubPixelCanvas {
    pub width: u16,  // in actual terminal cells
    pub height: u16, // in actual terminal cells
    pixels: Vec<bool>, // 2x4 subpixels per cell
}

impl SubPixelCanvas {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            pixels: vec![false; (width as usize * 2) * (height as usize * 4)],
        }
    }

    /// Sets a sub-pixel at the given (x, y) coordinate, where x goes up to width * 2 and y up to height * 4.
    pub fn set_pixel(&mut self, x: u16, y: u16, on: bool) {
        if x < self.width * 2 && y < self.height * 4 {
            let idx = (y as usize) * (self.width as usize * 2) + (x as usize);
            self.pixels[idx] = on;
        }
    }

    /// Converts the sub-pixel buffer into a grid of Braille characters.
    pub fn render_to_cells(&self) -> Vec<char> {
        let mut cells = Vec::with_capacity((self.width * self.height) as usize);
        for cell_y in 0..self.height {
            for cell_x in 0..self.width {
                let mut braille_char = 0x2800; // Base braille offset
                // Braille dot matrix offsets:
                // 1 4
                // 2 5
                // 3 6
                // 7 8
                let dot_offsets = [
                    (0, 0, 0x1), (0, 1, 0x2), (0, 2, 0x4), (0, 3, 0x40),
                    (1, 0, 0x8), (1, 1, 0x10), (1, 2, 0x20), (1, 3, 0x80)
                ];

                for (dx, dy, mask) in dot_offsets.iter() {
                    let px = cell_x * 2 + dx;
                    let py = cell_y * 4 + dy;
                    let idx = (py as usize) * (self.width as usize * 2) + (px as usize);
                    if self.pixels[idx] {
                        braille_char |= mask;
                    }
                }
                cells.push(std::char::from_u32(braille_char).unwrap_or(' '));
            }
        }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_braille_render() {
        let mut canvas = SubPixelCanvas::new(1, 1);
        canvas.set_pixel(0, 0, true);
        canvas.set_pixel(1, 0, true);
        let cells = canvas.render_to_cells();
        assert_eq!(cells.len(), 1);
        // dot 1 (0x1) + dot 4 (0x8) = 0x9 -> 0x2809
        assert_eq!(cells[0], '\u{2809}');
    }
}
