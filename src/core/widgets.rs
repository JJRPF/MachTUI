//! Advanced widgets for MachTUI.

use crate::core::Canvas;
use crate::core::components::Component;
use crossterm::style::Color;
use crate::oracle::{OracleProvider, SemanticNode};

/// A single-line text input component.
pub struct TextInput {
    pub content: String,
    pub cursor_pos: usize,
    pub focused: bool,
    pub placeholder: String,
}

impl TextInput {
    pub fn new(placeholder: &str) -> Self {
        Self {
            content: String::new(),
            cursor_pos: 0,
            focused: false,
            placeholder: placeholder.to_string(),
        }
    }

    pub fn handle_char(&mut self, c: char) {
        self.content.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
    }

    pub fn handle_backspace(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.content.remove(self.cursor_pos);
        }
    }
}

impl Component for TextInput {
    fn render(&self, canvas: &mut Canvas, x: u16, y: u16, width: u16, _height: u16) {
        let border_color = if self.focused { Color::Cyan } else { Color::DarkGrey };
        
        // Draw input box
        canvas.draw_text(x, y, &"━".repeat(width as usize), Some(border_color));
        canvas.draw_text(x, y + 2, &"━".repeat(width as usize), Some(border_color));
        canvas.set_cell(x, y + 1, '┃', Some(border_color));
        canvas.set_cell(x + width - 1, y + 1, '┃', Some(border_color));

        if self.content.is_empty() {
            canvas.draw_text(x + 1, y + 1, &self.placeholder, Some(Color::DarkGrey));
        } else {
            canvas.draw_text(x + 1, y + 1, &self.content, Some(Color::White));
        }

        if self.focused {
            canvas.set_cell(x + 1 + self.cursor_pos as u16, y + 1, '█', Some(Color::White));
        }
    }
}

impl OracleProvider for TextInput {
    fn to_semantic(&self) -> SemanticNode {
        SemanticNode::new("text_input")
            .with_content(&self.content)
            .with_metadata("focused", &self.focused.to_string())
            .with_metadata("placeholder", &self.placeholder)
    }
}

/// A scrollable list component.
pub struct ListSelection {
    pub items: Vec<String>,
    pub selected_idx: usize,
}

impl ListSelection {
    pub fn new(items: Vec<String>) -> Self {
        Self { items, selected_idx: 0 }
    }

    pub fn move_up(&mut self) {
        if self.selected_idx > 0 { self.selected_idx -= 1; }
    }

    pub fn move_down(&mut self) {
        if self.selected_idx < self.items.len() - 1 { self.selected_idx += 1; }
    }
}

impl Component for ListSelection {
    fn render(&self, canvas: &mut Canvas, x: u16, y: u16, _width: u16, height: u16) {
        for i in 0..(height as usize).min(self.items.len()) {
            let is_sel = i == self.selected_idx;
            let prefix = if is_sel { "> " } else { "  " };
            let color = if is_sel { Color::Green } else { Color::White };
            canvas.draw_text(x, y + i as u16, &format!("{}{}", prefix, self.items[i]), Some(color));
        }
    }
}

impl OracleProvider for ListSelection {
    fn to_semantic(&self) -> SemanticNode {
        let mut root = SemanticNode::new("list");
        root = root.with_metadata("selected_idx", &self.selected_idx.to_string());
        for (i, item) in self.items.iter().enumerate() {
            root.add_child(SemanticNode::new("list_item")
                .with_content(item)
                .with_metadata("index", &i.to_string()));
        }
        root
    }
}

/// Tab system for MachTUI.
pub struct Tabs {
    pub titles: Vec<String>,
    pub selected_idx: usize,
}

impl Tabs {
    pub fn new(titles: Vec<String>) -> Self {
        Self { titles, selected_idx: 0 }
    }

    pub fn next(&mut self) {
        self.selected_idx = (self.selected_idx + 1) % self.titles.len();
    }

    pub fn prev(&mut self) {
        if self.selected_idx == 0 {
            self.selected_idx = self.titles.len() - 1;
        } else {
            self.selected_idx -= 1;
        }
    }
}

impl Component for Tabs {
    fn render(&self, canvas: &mut Canvas, x: u16, y: u16, _width: u16, _height: u16) {
        let mut current_x = x;
        for (i, title) in self.titles.iter().enumerate() {
            let is_sel = i == self.selected_idx;
            let color = if is_sel { Color::Cyan } else { Color::Grey };
            let text = if is_sel { format!("[ {} ]", title) } else { format!("  {}  ", title) };
            
            canvas.draw_text(current_x, y, &text, Some(color));
            current_x += text.len() as u16 + 1;
        }
    }
}
