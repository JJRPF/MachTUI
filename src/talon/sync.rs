//! State Synchronization for Talon.
//! Allows multiple MachTUI instances to share and sync their Model state.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StateDelta {
    pub key: String,
    pub value: serde_json::Value,
    pub timestamp: u64,
}

pub struct StateSyncManager {
    pub deltas: Vec<StateDelta>,
}

impl StateSyncManager {
    pub fn new() -> Self {
        Self { deltas: Vec::new() }
    }

    /// Records a change to the model state for synchronization.
    pub fn record_change(&mut self, key: &str, value: serde_json::Value) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        self.deltas.push(StateDelta {
            key: key.to_string(),
            value,
            timestamp: now,
        });
    }

    /// Applies a delta from a remote instance to the local state.
    pub fn apply_delta(&mut self, delta: StateDelta) {
        // Simplified: just store it. In a real impl, this would update the Model.
        self.deltas.push(delta);
    }
}
