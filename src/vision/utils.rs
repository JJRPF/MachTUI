//! Vision Utilities for high-end TUI effects.

/// A simple 5x5 ASCII font for high-impact headers.
pub fn get_ascii_art(text: &str) -> Vec<String> {
    let mut lines = vec![String::new(); 5];
    for c in text.to_uppercase().chars() {
        let char_lines = match c {
            'A' => [" ### ", "A   A", "#####", "A   A", "A   A"],
            'B' => ["#### ", "B   B", "#### ", "B   B", "#### "],
            'C' => [" ####", "C    ", "C    ", "C    ", " ####"],
            'D' => ["#### ", "D   D", "D   D", "D   D", "#### "],
            'E' => ["#####", "E    ", "#### ", "E    ", "#####"],
            'M' => ["M   M", "MM MM", "M M M", "M   M", "M   M"],
            'T' => ["#####", "  T  ", "  T  ", "  T  ", "  T  "],
            'U' => ["U   U", "U   U", "U   U", "U   U", " ### "],
            'I' => [" ### ", "  I  ", "  I  ", "  I  ", " ### "],
            'H' => ["H   H", "H   H", "#####", "H   H", "H   H"],
            ' ' => ["     ", "     ", "     ", "     ", "     "],
            _ => ["#####", "#   #", "#####", "#   #", "#####"],
        };
        for i in 0..5 {
            lines[i].push_str(char_lines[i]);
            lines[i].push(' ');
        }
    }
    lines
}
