//! Live MTSS Inspector for MachTUI.
//! Provides real-time debugging and inspection of component styles.

use crate::plume::{Stylist, LayoutNode};
use std::collections::HashMap;

pub struct StylistInspector;

impl StylistInspector {
    /// Generates a report of all properties applied to a node by a stylist.
    pub fn inspect_node(stylist: &Stylist, node: &LayoutNode) -> HashMap<String, String> {
        let mut report = HashMap::new();
        let keys = ["color", "background", "width", "height", "justify-content", "align-items", "flex-direction"];
        
        for key in keys {
            if let Some(val) = stylist.get_property(node, key) {
                report.insert(key.to_string(), val);
            }
        }
        report
    }
}
