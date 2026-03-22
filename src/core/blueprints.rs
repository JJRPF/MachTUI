//! Blueprint system for MachTUI.
//! Allows defining UI structures in YAML/JSON.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlueprintNode {
    pub tag: String,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub children: Vec<BlueprintNode>,
    pub properties: HashMap<String, String>,
}

pub struct BlueprintEngine;

impl BlueprintEngine {
    /// Loads a blueprint from a YAML string.
    pub fn from_yaml(yaml: &str) -> Result<BlueprintNode, String> {
        serde_yaml::from_str(yaml).map_err(|e| e.to_string())
    }

    /// Loads a blueprint from a JSON string.
    pub fn from_json(json: &str) -> Result<BlueprintNode, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }
}
