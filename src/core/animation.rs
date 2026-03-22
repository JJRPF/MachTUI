//! Animation engine for MachTUI.
//! Provides tweening and easing functions for smooth UI transitions.

use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub enum Easing {
    Linear,
    QuadIn,
    QuadOut,
    QuadInOut,
    CubicIn,
    CubicOut,
}

impl Easing {
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            Easing::Linear => t,
            Easing::QuadIn => t * t,
            Easing::QuadOut => t * (2.0 - t),
            Easing::QuadInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            Easing::CubicIn => t * t * t,
            Easing::CubicOut => {
                let f = t - 1.0;
                f * f * f + 1.0
            }
        }
    }
}

pub struct Tween {
    start_value: f32,
    end_value: f32,
    duration: Duration,
    start_time: Instant,
    easing: Easing,
}

impl Tween {
    pub fn new(start: f32, end: f32, duration: Duration, easing: Easing) -> Self {
        Self {
            start_value: start,
            end_value: end,
            duration,
            start_time: Instant::now(),
            easing,
        }
    }

    pub fn value(&self) -> f32 {
        let elapsed = self.start_time.elapsed();
        let t = (elapsed.as_secs_f32() / self.duration.as_secs_f32()).min(1.0);
        let eased_t = self.easing.apply(t);
        self.start_value + (self.end_value - self.start_value) * eased_t
    }

    pub fn is_finished(&self) -> bool {
        self.start_time.elapsed() >= self.duration
    }
}
