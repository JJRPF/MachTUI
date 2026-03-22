//! Game Engine layer for MachTUI.
//! Provides sprite management and high-speed collision detection.

use crate::core::Canvas;
use crossterm::style::Color;

pub struct Sprite {
    pub x: f32,
    pub y: f32,
    pub frames: Vec<Vec<String>>,
    pub current_frame: usize,
    pub velocity_x: f32,
    pub velocity_y: f32,
}

impl Sprite {
    pub fn new(frames: Vec<Vec<String>>) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            frames,
            current_frame: 0,
            velocity_x: 0.0,
            velocity_y: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.x += self.velocity_x * dt;
        self.y += self.velocity_y * dt;
    }

    pub fn render(&self, canvas: &mut Canvas) {
        if let Some(frame) = self.frames.get(self.current_frame) {
            for (i, line) in frame.iter().enumerate() {
                canvas.draw_text(self.x as u16, self.y as u16 + i as u16, line, Some(Color::White));
            }
        }
    }
}
