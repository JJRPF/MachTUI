//! High-fidelity image sequence and animation support for MachTUI.

use crate::core::Canvas;
use crate::vision::images::ImageRenderer;
use image::DynamicImage;
use std::time::{Duration, Instant};

pub struct ImageSequence {
    pub frames: Vec<DynamicImage>,
    pub frame_duration: Duration,
    pub start_time: Instant,
    pub loop_count: usize,
}

impl ImageSequence {
    pub fn new(frames: Vec<DynamicImage>, frame_duration: Duration) -> Self {
        Self {
            frames,
            frame_duration,
            start_time: Instant::now(),
            loop_count: 0,
        }
    }

    pub fn current_frame(&self) -> &DynamicImage {
        let elapsed = self.start_time.elapsed();
        let index = (elapsed.as_millis() / self.frame_duration.as_millis()) as usize % self.frames.len();
        &self.frames[index]
    }


    pub fn render(&self, canvas: &mut Canvas, x: u16, y: u16, width: u16, height: u16) {
        if self.frames.is_empty() {
            return;
        }
        ImageRenderer::render_to_canvas(self.current_frame(), canvas, x, y, width, height);
    }
}
