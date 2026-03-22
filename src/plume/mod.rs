//! The "Plume" Stylist
//!
//! A CSS-like parser and layout engine for .mtss (MachTUI Style Sheets).
//! Powered by Taffy for high-performance Flexbox/Grid layout.

pub mod converter;
pub mod inspector;
pub mod layout;
pub mod lexer;

use layout::*;
use lexer::{Lexer, Token};
use std::collections::HashMap;
use taffy::prelude::*;

/// A MachTUI Style Sheet (MTSS) rule.
#[derive(Debug, Clone, Default)]
pub struct StyleRule {
    pub selector: String,             // e.g., "button", ".primary", "#header"
    pub pseudo_class: Option<String>, // e.g., "hover", "active"
    pub properties: HashMap<String, String>,
}

/// A node in the layout tree.
#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub tag: String,
    pub is_hovered: bool,
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
            is_hovered: false,
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
    pub variables: HashMap<String, String>,
    pub taffy: TaffyTree<()>,
}

impl Stylist {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            variables: HashMap::new(),
            taffy: TaffyTree::new(),
        }
    }

    pub fn parse_mtss(&mut self, input: &str) {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokens();
        let mut i = 0;

        while i < tokens.len() {
            let mut selector = String::new();
            let mut pseudo_class = None;

            while i < tokens.len() && tokens[i] != Token::OpenBrace {
                match &tokens[i] {
                    Token::Ident(s) => selector.push_str(s),
                    Token::Dot => selector.push('.'),
                    Token::Hash => selector.push('#'),
                    Token::Colon => {
                        i += 1;
                        if let Some(Token::Ident(p)) = tokens.get(i) {
                            pseudo_class = Some(p.clone());
                        }
                    }
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

                            let mut value = String::new();
                            while i < tokens.len()
                                && tokens[i] != Token::Semicolon
                                && tokens[i] != Token::CloseBrace
                            {
                                match &tokens[i] {
                                    Token::Ident(s) => value.push_str(s),
                                    Token::OpenParen => value.push('('),
                                    Token::CloseParen => value.push(')'),
                                    _ => {}
                                }
                                i += 1;
                            }

                            if key.starts_with("--") {
                                self.variables.insert(key.to_string(), value);
                            } else {
                                properties.insert(key.to_string(), value);
                            }

                            if i < tokens.len() && tokens[i] == Token::Semicolon {
                                i += 1;
                            }
                        }
                    } else {
                        i += 1;
                    }
                }
                if !selector.is_empty() {
                    self.rules.push(StyleRule {
                        selector,
                        pseudo_class,
                        properties,
                    });
                }
            }
            i += 1;
        }
    }

    pub fn get_property(&self, node: &LayoutNode, key: &str) -> Option<String> {
        let raw_val = if node.is_hovered {
            self.find_rule(node, Some("hover"), key)
                .or_else(|| self.find_rule(node, None, key))
        } else {
            self.find_rule(node, None, key)
        }?;

        // Resolve variables: var(--name)
        if raw_val.starts_with("var(") && raw_val.ends_with(")") {
            let var_name = &raw_val[4..raw_val.len() - 1];
            self.variables.get(var_name).cloned()
        } else {
            Some(raw_val.clone())
        }
    }

    fn find_rule(&self, node: &LayoutNode, pseudo: Option<&str>, key: &str) -> Option<&String> {
        if let Some(id) = &node.id {
            let id_sel = format!("#{}", id);
            if let Some(prop) = self
                .rules
                .iter()
                .find(|r| r.selector == id_sel && r.pseudo_class.as_deref() == pseudo)
                .and_then(|r| r.properties.get(key))
            {
                return Some(prop);
            }
        }

        for class in &node.classes {
            let class_sel = format!(".{}", class);
            if let Some(prop) = self
                .rules
                .iter()
                .find(|r| r.selector == class_sel && r.pseudo_class.as_deref() == pseudo)
                .and_then(|r| r.properties.get(key))
            {
                return Some(prop);
            }
        }

        self.rules
            .iter()
            .find(|r| r.selector == node.tag && r.pseudo_class.as_deref() == pseudo)
            .and_then(|r| r.properties.get(key))
    }

    fn build_taffy_tree(&mut self, node: &LayoutNode) -> NodeId {
        let mut style = node.style.clone();

        if let Some(jc) = self.get_property(node, "justify-content") {
            style.justify_content = Some(map_justify_content(&jc));
        }
        if let Some(ai) = self.get_property(node, "align-items") {
            style.align_items = Some(map_align_items(&ai));
        }
        if let Some(fd) = self.get_property(node, "flex-direction") {
            style.flex_direction = map_flex_direction(&fd);
        }

        let mut taffy_children = Vec::new();
        for child in &node.children {
            taffy_children.push(self.build_taffy_tree(child));
        }
        self.taffy
            .new_with_children(style, &taffy_children)
            .unwrap()
    }

    pub fn compute_layout(&mut self, root: &mut LayoutNode, width: f32, height: f32) {
        self.taffy.clear();
        let root_id = self.build_taffy_tree(root);
        self.taffy
            .compute_layout(
                root_id,
                Size {
                    width: AvailableSpace::Definite(width),
                    height: AvailableSpace::Definite(height),
                },
            )
            .unwrap();
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
    fn test_variables() {
        let mut stylist = Stylist::new();
        stylist.parse_mtss(":root { --main-bg: blue; } button { color: var(--main-bg); }");
        let btn = LayoutNode::new("button");
        assert_eq!(
            stylist.get_property(&btn, "color"),
            Some("blue".to_string())
        );
    }

    #[test]
    fn test_pseudo_classes() {
        let mut stylist = Stylist::new();
        stylist.parse_mtss("button { color: white; } button:hover { color: yellow; }");

        let mut btn = LayoutNode::new("button");
        assert_eq!(
            stylist.get_property(&btn, "color"),
            Some("white".to_string())
        );

        btn.is_hovered = true;
        assert_eq!(
            stylist.get_property(&btn, "color"),
            Some("yellow".to_string())
        );
    }
}
