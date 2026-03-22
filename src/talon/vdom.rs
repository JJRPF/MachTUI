//! MachTUI Virtual DOM (VDom) for optimized UI diffing.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum VNode {
    Element {
        tag: String,
        id: Option<String>,
        classes: Vec<String>,
        props: HashMap<String, String>,
        children: Vec<VNode>,
    },
    Text(String),
}

pub struct VDom {
    pub root: VNode,
}

impl VDom {
    pub fn new(root: VNode) -> Self {
        Self { root }
    }

    /// Computes the patch required to transform one VDom tree into another.
    /// Simplified for now: just checks equality.
    pub fn diff(&self, other: &VDom) -> bool {
        self.root == other.root
    }
}
