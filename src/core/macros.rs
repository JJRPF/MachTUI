//! Automation and Macro system for MachTUI.
//! Allows recording and playing back TUI interactions.

use crossterm::event::Event;
use serde::{Serialize, Deserialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InteractionStep {
    pub event: serde_json::Value, // Serialized crossterm::event::Event
    pub delay: Duration,
}

pub struct InteractionMacro {
    pub steps: Vec<InteractionStep>,
}

impl InteractionMacro {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    /// Records a step in the macro.
    pub fn record(&mut self, event: &Event, delay: Duration) {
        // Simplified serialization: just store as a string for now.
        // In a real impl, this would use a robust Event -> JSON mapper.
        let event_json = serde_json::to_value(format!("{:?}", event)).unwrap();
        self.steps.push(InteractionStep {
            event: event_json,
            delay,
        });
    }

    /// Plays back the macro by yielding steps.
    pub fn playback(&self) -> impl Iterator<Item = &InteractionStep> {
        self.steps.iter()
    }
}
