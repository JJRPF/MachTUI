//! Event propagation system for MachTUI.
//! Supports Bubbling and Capture phases for complex UI interaction.

use crossterm::event::Event;

pub enum EventPhase {
    Capture,
    Bubble,
}

pub enum EventResponse {
    Handled,
    Ignored,
    StopPropagation,
}

pub trait EventHandler {
    fn handle_event(&mut self, event: &Event, phase: &EventPhase) -> EventResponse;
}

pub struct EventDispatcher;

impl EventDispatcher {
    /// Dispatches an event through a hierarchy of handlers.
    pub fn dispatch<H: EventHandler>(handlers: &mut [H], event: &Event) {
        // 1. Capture Phase (Root to Leaf)
        for handler in handlers.iter_mut() {
            if let EventResponse::StopPropagation =
                handler.handle_event(event, &EventPhase::Capture)
            {
                return;
            }
        }

        // 2. Bubble Phase (Leaf to Root)
        for handler in handlers.iter_mut().rev() {
            if let EventResponse::StopPropagation = handler.handle_event(event, &EventPhase::Bubble)
            {
                return;
            }
        }
    }
}
