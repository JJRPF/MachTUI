//! Keyboard shortcut manager for MachTUI.
//! Provides a centralized way to handle global hotkeys.

use crossterm::event::{KeyCode, KeyModifiers};
use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Shortcut {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

pub struct ShortcutManager {
    pub shortcuts: HashMap<Shortcut, Box<dyn Fn() + Send + Sync>>,
}

impl ShortcutManager {
    pub fn new() -> Self {
        Self {
            shortcuts: HashMap::new(),
        }
    }

    pub fn register<F>(&mut self, code: KeyCode, modifiers: KeyModifiers, action: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.shortcuts
            .insert(Shortcut { code, modifiers }, Box::new(action));
    }

    pub fn handle_event(&self, code: KeyCode, modifiers: KeyModifiers) -> bool {
        if let Some(action) = self.shortcuts.get(&Shortcut { code, modifiers }) {
            action();
            true
        } else {
            false
        }
    }
}
