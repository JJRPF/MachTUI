//! The "Plume" Stylist
//!
//! A CSS-like parser and layout engine for .mtss (MachTUI Style Sheets).
//! Powered by Taffy for high-performance Flexbox/Grid layout.

pub mod lexer;

use std::collections::HashMap;
use lexer::{Lexer, Token};
use taffy::prelude::*;

/// A MachTUI Style Sheet (MTSS) rule.
#[derive(Debug, Clone, Default)]
pub struct StyleRule {
    pub selector: String,
    pub properties: HashMap<String, String>,
}

/// A node in the layout tree.
#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub id: String,
    pub style: Style,
    pub children: Vec<LayoutNode>,
    pub layout: Layout,
}

impl LayoutNode {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            style: Style::default(),
            children: Vec::new(),
            layout: Layout::new(),
        }
    }
}

/// The main stylist engine that parses and applies MTSS.
pub struct Stylist {
    pub rules: Vec<StyleRule>,
    pub taffy: TaffyTree<()>,
}

impl Stylist {
    pub fn new() -> Self {
        Self { 
            rules: Vec::new(),
            taffy: TaffyTree::new(),
        }
    }

    /// Parses MTSS input using a lexer and a simple state machine.
    pub fn parse_mtss(&mut self, input: &str) {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokens();
        let mut i = 0;

        while i < tokens.len() {
            if let Token::Ident(selector) = &tokens[i] {
                i += 1;
                if i < tokens.len() && tokens[i] == Token::OpenBrace {
                    i += 1;
                    let mut properties = HashMap::new();
                    while i < tokens.len() && tokens[i] != Token::CloseBrace {
                        if let Token::Ident(key) = &tokens[i] {
                            i += 1;
                            if i < tokens.len() && tokens[i] == Token::Colon {
                                i += 1;
                                if let Token::Ident(value) = &tokens[i] {
                                    properties.insert(key.clone(), value.clone());
                                    i += 1;
                                    if i < tokens.len() && tokens[i] == Token::Semicolon {
                                        i += 1;
                                    }
                                }
                            }
                        } else {
                            i += 1;
                        }
                    }
                    self.rules.push(StyleRule { selector: selector.clone(), properties });
                }
            }
            i += 1;
        }
    }

    /// Retrieve a property value for a given selector and key.
    pub fn get_property(&self, selector: &str, key: &str) -> Option<&String> {
        self.rules.iter()
            .find(|r| r.selector == selector)
            .and_then(|r| r.properties.get(key))
    }

    /// Recursively builds a Taffy tree from our LayoutNode tree.
    fn build_taffy_tree(&mut self, node: &LayoutNode) -> NodeId {
        let mut taffy_children = Vec::new();
        for child in &node.children {
            taffy_children.push(self.build_taffy_tree(child));
        }
        self.taffy.new_with_children(node.style.clone(), &taffy_children).unwrap()
    }

    /// Computes the layout for a node tree given parent dimensions.
    pub fn compute_layout(&mut self, root: &mut LayoutNode, width: f32, height: f32) {
        self.taffy.clear();
        let root_id = self.build_taffy_tree(root);
        
        self.taffy.compute_layout(
            root_id,
            Size {
                width: AvailableSpace::Definite(width),
                height: AvailableSpace::Definite(height),
            },
        ).unwrap();

        self.apply_taffy_layout(root, root_id);
    }

    /// Recursively copies Taffy results back to our LayoutNode tree.
    fn apply_taffy_layout(&self, node: &mut LayoutNode, taffy_id: NodeId) {
        node.layout = *self.taffy.layout(taffy_id).unwrap();
        let taffy_children = self.taffy.children(taffy_id).unwrap();
        for (i, child) in node.children.iter_mut().enumerate() {
            self.apply_taffy_layout(child, taffy_children[i]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_mtss() {
        let mut stylist = Stylist::new();
        stylist.parse_mtss("button { color: red; background: blue; }");
        
        assert_eq!(stylist.rules.len(), 1);
        assert_eq!(stylist.get_property("button", "color"), Some(&"red".to_string()));
    }

    #[test]
    fn test_taffy_layout() {
        let mut stylist = Stylist::new();
        let mut root = LayoutNode::new("container");
        root.style = Style {
            size: Size { width: length(100.0), height: length(50.0) },
            ..Default::default()
        };
        
        stylist.compute_layout(&mut root, 200.0, 200.0);

        assert_eq!(root.layout.size.width, 100.0);
        assert_eq!(root.layout.size.height, 50.0);
    }
}
