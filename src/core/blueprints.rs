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

use crate::core::components::{Component, BoxComponent, ProgressBar};

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

    /// Spawns a list of active components from a blueprint node.
    pub fn spawn_components(node: &BlueprintNode) -> Vec<Box<dyn Component>> {
        let mut components = Vec::new();
        
        match node.tag.as_str() {
            "box" => {
                let title = node.properties.get("title").cloned().unwrap_or_default();
                components.push(Box::new(BoxComponent::new(&title)) as Box<dyn Component>);
            }
            "progress" => {
                let label = node.properties.get("label").cloned().unwrap_or_default();
                let progress = node.properties.get("value")
                    .and_then(|v| v.parse::<f32>().ok())
                    .unwrap_or(0.0);
                components.push(Box::new(ProgressBar::new(&label, progress)) as Box<dyn Component>);
            }
            _ => {}
        }

        for child in &node.children {
            components.extend(Self::spawn_components(child));
        }

        components
    }
}
