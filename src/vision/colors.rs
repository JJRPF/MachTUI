//! High-Resolution Color Palette system for MachTUI.
//! Provides Tailwind-inspired color presets for professional UI design.

use crossterm::style::Color;

pub struct Palette;

impl Palette {
    pub const SLATE_50: Color = Color::Rgb { r: 248, g: 250, b: 252 };
    pub const SLATE_500: Color = Color::Rgb { r: 100, g: 116, b: 139 };
    pub const SLATE_900: Color = Color::Rgb { r: 15, g: 23, b: 42 };

    pub const BLUE_500: Color = Color::Rgb { r: 59, g: 130, b: 246 };
    pub const BLUE_700: Color = Color::Rgb { r: 29, g: 78, b: 216 };

    pub const EMERALD_500: Color = Color::Rgb { r: 16, g: 185, b: 129 };
    pub const ROSE_500: Color = Color::Rgb { r: 244, g: 63, b: 94 };
    
    pub const AMBER_500: Color = Color::Rgb { r: 245, g: 158, b: 11 };
}
