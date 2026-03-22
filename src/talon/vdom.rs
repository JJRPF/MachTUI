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

#[derive(Debug)]
pub enum Patch {
    Replace(VNode),
    UpdateProps(HashMap<String, String>),
    UpdateText(String),
    Children(Vec<Patch>),
    Noop,
}

impl VDom {
    pub fn new(root: VNode) -> Self {
        Self { root }
    }

    /// Computes the granular patch required to transform this VDom into another.
    pub fn diff(&self, other: &VNode) -> Patch {
        Self::diff_nodes(&self.root, other)
    }

    fn diff_nodes(old: &VNode, new: &VNode) -> Patch {
        match (old, new) {
            (VNode::Text(old_text), VNode::Text(new_text)) => {
                if old_text == new_text { Patch::Noop }
                else { Patch::UpdateText(new_text.clone()) }
            }
            (VNode::Element { tag: old_tag, props: old_props, children: old_children, .. },
             VNode::Element { tag: new_tag, props: new_props, children: new_children, .. }) => {
                if old_tag != new_tag {
                    return Patch::Replace(new.clone());
                }

                // Diff props
                let mut prop_patch = HashMap::new();
                for (k, v) in new_props {
                    if old_props.get(k) != Some(v) {
                        prop_patch.insert(k.clone(), v.clone());
                    }
                }

                // Diff children (simplified)
                let mut child_patches = Vec::new();
                for (old_c, new_c) in old_children.iter().zip(new_children.iter()) {
                    child_patches.push(Self::diff_nodes(old_c, new_c));
                }

                Patch::Children(child_patches)
            }
            _ => Patch::Replace(new.clone()),
        }
    }
}
