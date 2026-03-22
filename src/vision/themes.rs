//! High-end Theming system for MachTUI.
//! Provides global color schemes and visual presets.

use crossterm::style::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub background: Color,
    pub foreground: Color,
    pub primary: Color,
    pub secondary: Color,
    pub success: Color,
    pub error: Color,
    pub warning: Color,
}

impl Theme {
    pub fn dracula() -> Self {
        Self {
            name: "Dracula".into(),
            background: Color::Rgb {
                r: 40,
                g: 42,
                b: 54,
            },
            foreground: Color::Rgb {
                r: 248,
                g: 248,
                b: 242,
            },
            primary: Color::Rgb {
                r: 189,
                g: 147,
                b: 249,
            }, // Purple
            secondary: Color::Rgb {
                r: 255,
                g: 121,
                b: 198,
            }, // Pink
            success: Color::Rgb {
                r: 80,
                g: 250,
                b: 123,
            }, // Green
            error: Color::Rgb {
                r: 255,
                g: 85,
                b: 85,
            }, // Red
            warning: Color::Rgb {
                r: 241,
                g: 250,
                b: 140,
            }, // Yellow
        }
    }

    pub fn nord() -> Self {
        Self {
            name: "Nord".into(),
            background: Color::Rgb {
                r: 46,
                g: 52,
                b: 64,
            },
            foreground: Color::Rgb {
                r: 216,
                g: 222,
                b: 233,
            },
            primary: Color::Rgb {
                r: 136,
                g: 192,
                b: 208,
            }, // Frost Blue
            secondary: Color::Rgb {
                r: 129,
                g: 161,
                b: 193,
            }, // Polar Night Blue
            success: Color::Rgb {
                r: 163,
                g: 190,
                b: 140,
            }, // Green
            error: Color::Rgb {
                r: 191,
                g: 97,
                b: 106,
            }, // Red
            warning: Color::Rgb {
                r: 235,
                g: 203,
                b: 139,
            }, // Yellow
        }
    }
}
