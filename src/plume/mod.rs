//! The "Plume" Stylist
//!
//! A CSS-like parser and layout engine for .mtss (MachTUI Style Sheets).
//! Powered by Taffy for high-performance Flexbox/Grid layout.

pub mod lexer;
pub mod layout;

use std::collections::HashMap;
use lexer::{Lexer, Token};
use layout::*;
use taffy::prelude::*;

/// A MachTUI Style Sheet (MTSS) rule.
#[derive(Debug, Clone, Default)]
pub struct StyleRule {
    pub selector: String, // e.g., "button", ".primary", "#header"
    pub properties: HashMap<String, String>,
}

/// A node in the layout tree.
#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub tag: String,
    pub style: Style,
    pub children: Vec<LayoutNode>,
    pub layout: Layout,
}

impl LayoutNode {
    pub fn new(tag: &str) -> Self {
        Self {
            id: None,
            classes: Vec::new(),
            tag: tag.to_string(),
            style: Style::default(),
            children: Vec::new(),
            layout: Layout::new(),
        }
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn with_class(mut self, class: &str) -> Self {
        self.classes.push(class.to_string());
        self
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

    pub fn parse_mtss(&mut self, input: &str) {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokens();
        let mut i = 0;

        while i < tokens.len() {
            let mut selector = String::new();
            
            // Handle complex selectors like .class, #id, or tag
            while i < tokens.len() && tokens[i] != Token::OpenBrace {
                match &tokens[i] {
                    Token::Ident(s) => selector.push_str(s),
                    Token::Dot => selector.push('.'),
                    Token::Hash => selector.push('#'),
                    _ => {}
                }
                i += 1;
            }

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
                self.rules.push(StyleRule { selector, properties });
            }
            i += 1;
        }
    }

    pub fn get_property(&self, node: &LayoutNode, key: &str) -> Option<&String> {
        // Selector precedence: ID > Class > Tag
        if let Some(id) = &node.id {
            let id_sel = format!("#{}", id);
            if let Some(prop) = self.rules.iter().find(|r| r.selector == id_sel).and_then(|r| r.properties.get(key)) {
                return Some(prop);
            }
        }

        for class in &node.classes {
            let class_sel = format!(".{}", class);
            if let Some(prop) = self.rules.iter().find(|r| r.selector == class_sel).and_then(|r| r.properties.get(key)) {
                return Some(prop);
            }
        }

        self.rules.iter()
            .find(|r| r.selector == node.tag)
            .and_then(|r| r.properties.get(key))
    }

    fn build_taffy_tree(&mut self, node: &LayoutNode) -> NodeId {
        let mut style = node.style.clone();
        
        // Apply MTSS properties to Taffy style
        if let Some(jc) = self.get_property(node, "justify-content") {
            style.justify_content = Some(map_justify_content(jc));
        }
        if let Some(ai) = self.get_property(node, "align-items") {
            style.align_items = Some(map_align_items(ai));
        }
        if let Some(fd) = self.get_property(node, "flex-direction") {
            style.flex_direction = map_flex_direction(fd);
        }

        let mut taffy_children = Vec::new();
        for child in &node.children {
            taffy_children.push(self.build_taffy_tree(child));
        }
        self.taffy.new_with_children(style, &taffy_children).unwrap()
    }

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
    fn test_parse_advanced_selectors() {
        let mut stylist = Stylist::new();
        stylist.parse_mtss(".btn { color: red; } #hdr { color: blue; } div { color: green; }");
        
        let btn = LayoutNode::new("div").with_class("btn");
        let hdr = LayoutNode::new("div").with_id("hdr");
        let plain = LayoutNode::new("div");

        assert_eq!(stylist.get_property(&btn, "color"), Some(&"red".to_string()));
        assert_eq!(stylist.get_property(&hdr, "color"), Some(&"blue".to_string()));
        assert_eq!(stylist.get_property(&plain, "color"), Some(&"green".to_string()));
    }

    #[test]
    fn test_parse_simple_mtss() {
        let mut stylist = Stylist::new();
        stylist.parse_mtss("button { color: red; background: blue; }");
        
        assert_eq!(stylist.rules.len(), 1);
        let btn = LayoutNode::new("button");
        assert_eq!(stylist.get_property(&btn, "color"), Some(&"red".to_string()));
    }
    #[test]
    fn test_taffy_layout() {
        let mut stylist = Stylist::new();
        let mut root = LayoutNode::new("div").with_id("container");
        root.style = Style {
            size: Size { width: length(100.0), height: length(50.0) },
            ..Default::default()
        };
        
        stylist.compute_layout(&mut root, 200.0, 200.0);

        assert_eq!(root.layout.size.width, 100.0);
        assert_eq!(root.layout.size.height, 50.0);
    }
}
