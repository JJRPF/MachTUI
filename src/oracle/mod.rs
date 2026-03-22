//! The "Oracle" Protocol
//!
//! Provides semantic tree generation and a headless JSON API for AI agents.

use serde::{Serialize, Deserialize};

/// A node in the semantic UI tree.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SemanticNode {
    pub role: String,
    pub content: Option<String>,
    pub children: Vec<SemanticNode>,
}

impl SemanticNode {
    pub fn new(role: &str) -> Self {
        Self {
            role: role.to_string(),
            content: None,
            children: Vec::new(),
        }
    }

    pub fn with_content(mut self, content: &str) -> Self {
        self.content = Some(content.to_string());
        self
    }

    pub fn add_child(&mut self, child: SemanticNode) {
        self.children.push(child);
    }
}

/// The Oracle engine for headless AI interaction.
pub struct Oracle {
    pub tree: SemanticNode,
}

impl Oracle {
    pub fn new(root_role: &str) -> Self {
        Self {
            tree: SemanticNode::new(root_role),
        }
    }

    /// Serializes the semantic tree to a JSON string for AI consumption.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self.tree).unwrap_or_else(|_| "{}".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle_json() {
        let mut oracle = Oracle::new("window");
        let btn = SemanticNode::new("button").with_content("Submit");
        oracle.tree.add_child(btn);
        
        let json = oracle.to_json();
        assert!(json.contains(r#""role": "window""#));
        assert!(json.contains(r#""role": "button""#));
        assert!(json.contains(r#""content": "Submit""#));
    }
}
