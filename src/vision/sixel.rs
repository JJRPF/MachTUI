//! High-performance Sixel rendering for MachTUI.
//! Provides color quantization and Sixel emission for high-fidelity images.

use image::{DynamicImage, GenericImageView};
use std::io::{self, Write};

pub struct SixelRenderer;

impl SixelRenderer {
    /// Renders an image using the Sixel protocol.
    /// Simplified implementation: encodes a 6-pixel high column per Sixel character.
    pub fn render_sixel(img: &DynamicImage, stdout: &mut dyn Write) -> io::Result<()> {
        let (w, h) = img.dimensions();

        // Sixel Start: ESC P q
        write!(stdout, "\x1bPq")?;

        // Define simple palette (8 basic colors for speed)
        // #0: Black, #1: Blue, #2: Red, #3: Green, #4: Magenta, #5: Cyan, #6: Yellow, #7: White
        write!(stdout, "#0;2;0;0;0#1;2;0;0;100#2;2;100;0;0#3;2;0;100;0#4;2;100;0;100#5;2;0;100;100#6;2;100;100;0#7;2;100;100;100")?;

        for y_six in (0..h).step_by(6) {
            for x in 0..w {
                let mut sixel_val = 0u8;
                for row in 0..6 {
                    if y_six + row < h {
                        let pix = img.get_pixel(x, y_six + row);
                        let luma =
                            0.299 * pix[0] as f32 + 0.587 * pix[1] as f32 + 0.114 * pix[2] as f32;
                        if luma > 128.0 {
                            sixel_val |= 1 << row;
                        }
                    }
                }
                // Emit Sixel char (offset by 63)
                write!(stdout, "{}", (sixel_val + 63) as char)?;
            }
            write!(stdout, "$")?; // Graphics Carriage Return
            write!(stdout, "-")?; // Graphics New Line
        }

        // Sixel End: ESC \
        write!(stdout, "\x1b\\")?;
        stdout.flush()?;
        Ok(())
    }
}
