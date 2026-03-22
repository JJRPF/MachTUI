//! The "Plume" Stylist
//!
//! A CSS-like parser and layout engine for .mtss (MachTUI Style Sheets).

pub mod lexer;

use std::collections::HashMap;
use lexer::{Lexer, Token};

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
    pub width: u16,
    pub height: u16,
    pub x: u16,
    pub y: u16,
    pub children: Vec<LayoutNode>,
}

impl LayoutNode {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            width: 0,
            height: 0,
            x: 0,
            y: 0,
            children: Vec::new(),
        }
    }
}

/// The main stylist engine that parses and applies MTSS.
pub struct Stylist {
    pub rules: Vec<StyleRule>,
}

impl Stylist {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
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

    /// A simple layout solver that assigns dimensions based on styles.
    /// This will eventually be a full Flexbox implementation.
    pub fn compute_layout(&self, node: &mut LayoutNode, parent_width: u16, parent_height: u16) {
        // Simple logic for now: use styles if available, otherwise fill parent.
        let width_str = self.get_property(&node.id, "width");
        let height_str = self.get_property(&node.id, "height");

        node.width = width_str.and_then(|s| s.parse::<u16>().ok()).unwrap_or(parent_width);
        node.height = height_str.and_then(|s| s.parse::<u16>().ok()).unwrap_or(parent_height);

        // Simple vertical layout for children
        let mut current_y = 0;
        for child in &mut node.children {
            child.x = node.x;
            child.y = node.y + current_y;
            self.compute_layout(child, node.width, node.height - current_y);
            current_y += child.height;
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
    fn test_simple_layout() {
        let mut stylist = Stylist::new();
        stylist.parse_mtss("container { width: 100; height: 50; }");
        
        let mut root = LayoutNode::new("container");
        stylist.compute_layout(&mut root, 200, 200);

        assert_eq!(root.width, 100);
        assert_eq!(root.height, 50);
    }
}
