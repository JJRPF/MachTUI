//! HTML to MachTUI Converter for Plume.
//! Maps standard HTML tags and inline styles to LayoutNode and MTSS.

use crate::plume::LayoutNode;
use taffy::prelude::*;

pub struct HtmlConverter;

impl HtmlConverter {
    /// Converts an HTML string into a MachTUI LayoutNode tree.
    pub fn convert(html: &str) -> Result<LayoutNode, String> {
        let dom = tl::parse(html, tl::ParserOptions::default()).map_err(|e| e.to_string())?;
        
        // Find the body tag or use the first child
        let body_node = dom.nodes()
            .iter()
            .find(|n| n.as_tag().map(|t| t.name().as_utf8_str() == "body").unwrap_or(false));

        if let Some(node) = body_node {
            Self::convert_tl_node(node, &dom)
        } else if let Some(root) = dom.children().first() {
            let node = root.get(dom.parser()).ok_or("Root node not found")?;
            Self::convert_tl_node(node, &dom)
        } else {
            Err("No nodes found in HTML".into())
        }
    }

    fn convert_tl_node(node: &tl::Node, dom: &tl::VDom) -> Result<LayoutNode, String> {
        match node {
            tl::Node::Tag(tag) => {
                let tag_name = tag.name().as_utf8_str();
                let mut layout_node = LayoutNode::new(&tag_name);

                // Map attributes (id, class)
                if let Some(id) = tag.attributes().id() {
                    layout_node = layout_node.with_id(&id.as_utf8_str());
                }
                if let Some(classes) = tag.attributes().get("class").flatten() {
                    for class in classes.as_utf8_str().split_whitespace() {
                        layout_node = layout_node.with_class(class);
                    }
                }

                // Map inline styles (very basic for now)
                if let Some(styles) = tag.attributes().get("style").flatten() {
                    let style_str = styles.as_utf8_str();
                    for part in style_str.split(';') {
                        let kv: Vec<&str> = part.split(':').collect();
                        if kv.len() == 2 {
                            let key = kv[0].trim();
                            let val = kv[1].trim();
                            // In a real impl, we'd store these in the node's style
                            // For now, we'll rely on the Stylist to handle MTSS.
                        }
                    }
                }

                // Recursively convert children using their handles
                for child_handle in tag.children().top().iter() {
                    if let Some(child_node) = child_handle.get(dom.parser()) {
                        if let Ok(converted) = Self::convert_tl_node(child_node, dom) {
                            layout_node.children.push(converted);
                        }
                    }
                }

                Ok(layout_node)
            }
            tl::Node::Raw(bytes) => {
                let text = bytes.as_utf8_str().trim().to_string();
                if text.is_empty() {
                    Err("Empty text".into())
                } else {
                    Ok(LayoutNode::new("text").with_id(&text))
                }
            }
            _ => Err("Unsupported node type".into()),
        }
    }
}
