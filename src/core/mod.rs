//! The "Mach" Core
//!
//! Immediate-mode renderer internally, exposed via a Reactive Component layer.
//! Supports Mouse events, RGB Gradients, Double-Buffered diffing, and Z-Index Layering.

pub mod components;
pub mod animation;
pub mod widgets;
pub mod keys;
pub mod notifications;
pub mod testing;
pub mod http;
pub mod db;
pub mod shell;
pub mod bridge;
pub mod modals;
pub mod events;
pub mod plugins;
pub mod cloud;

use crossterm::{
    cursor,
    event::{self, Event, EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    style::{self, Color, ContentStyle, Colors},
    QueueableCommand,
};
use std::io::{self, Stdout, Write};
use std::time::{Duration, Instant};

/// A single cell in the terminal grid.
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub content: char,
    pub style: ContentStyle,
    pub z_index: i32,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            content: ' ',
            style: ContentStyle::default(),
            z_index: 0,
        }
    }
}

/// A buffer representing the terminal screen state.
pub struct Canvas {
    pub width: u16,
    pub height: u16,
    pub cells: Vec<Cell>,
}

impl Canvas {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::default(); (width as usize) * (height as usize)],
        }
    }

    /// Sets a cell only if the provided z_index is >= current cell's z_index.
    pub fn set_cell_z(&mut self, x: u16, y: u16, content: char, color: Option<Color>, z: i32) {
        if x < self.width && y < self.height {
            let idx = (y as usize) * (self.width as usize) + (x as usize);
            if z >= self.cells[idx].z_index {
                self.cells[idx].content = content;
                self.cells[idx].z_index = z;
                if let Some(c) = color {
                    self.cells[idx].style.foreground_color = Some(c);
                }
            }
        }
    }

    pub fn set_cell(&mut self, x: u16, y: u16, content: char, color: Option<Color>) {
        self.set_cell_z(x, y, content, color, 0);
    }

    pub fn draw_text_z(&mut self, x: u16, y: u16, text: &str, color: Option<Color>, z: i32) {
        for (i, c) in text.chars().enumerate() {
            self.set_cell_z(x + i as u16, y, c, color, z);
        }
    }

    pub fn draw_text(&mut self, x: u16, y: u16, text: &str, color: Option<Color>) {
        self.draw_text_z(x, y, text, color, 0);
    }

    pub fn draw_gradient_text_z(&mut self, x: u16, y: u16, text: &str, start_rgb: (u8, u8, u8), end_rgb: (u8, u8, u8), z: i32) {
        let len = text.chars().count();
        for (i, c) in text.chars().enumerate() {
            let t = i as f32 / (len.max(1) as f32);
            let r = (start_rgb.0 as f32 * (1.0 - t) + end_rgb.0 as f32 * t) as u8;
            let g = (start_rgb.1 as f32 * (1.0 - t) + end_rgb.1 as f32 * t) as u8;
            let b = (start_rgb.2 as f32 * (1.0 - t) + end_rgb.2 as f32 * t) as u8;
            self.set_cell_z(x + i as u16, y, c, Some(Color::Rgb { r, g, b }), z);
        }
    }

    pub fn draw_gradient_text(&mut self, x: u16, y: u16, text: &str, start_rgb: (u8, u8, u8), end_rgb: (u8, u8, u8)) {
        self.draw_gradient_text_z(x, y, text, start_rgb, end_rgb, 0);
    }

    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = Cell::default();
        }
    }
}

pub struct Renderer {
    stdout: Stdout,
    current_canvas: Canvas,
    last_canvas: Canvas,
    sync_output: bool,
    last_frame_time: Instant,
    pub show_fps: bool,
}

impl Renderer {
    pub fn new() -> io::Result<Self> {
        let (width, height) = terminal::size()?;
        let mut stdout = io::stdout();
        terminal::enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen, cursor::Hide, EnableMouseCapture)?;
        Ok(Self {
            stdout,
            current_canvas: Canvas::new(width, height),
            last_canvas: Canvas::new(width, height),
            sync_output: true,
            last_frame_time: Instant::now(),
            show_fps: true,
        })
    }

    pub fn shutdown(&mut self) -> io::Result<()> {
        terminal::disable_raw_mode()?;
        execute!(self.stdout, LeaveAlternateScreen, cursor::Show, DisableMouseCapture)?;
        Ok(())
    }

    fn begin_sync(&mut self) -> io::Result<()> {
        if self.sync_output {
            self.stdout.queue(style::Print("\x1b[?2026h"))?;
        }
        Ok(())
    }

    fn end_sync(&mut self) -> io::Result<()> {
        if self.sync_output {
            self.stdout.queue(style::Print("\x1b[?2026l"))?;
        }
        Ok(())
    }

    pub fn render(&mut self) -> io::Result<()> {
        let now = Instant::now();
        let fps = 1.0 / now.duration_since(self.last_frame_time).as_secs_f32();
        self.last_frame_time = now;

        if self.show_fps {
            let fps_str = format!(" FPS: {:.1} ", fps);
            self.current_canvas.draw_text_z(self.current_canvas.width - fps_str.len() as u16 - 1, 0, &fps_str, Some(Color::DarkGrey), 100);
        }

        self.begin_sync()?;
        for y in 0..self.current_canvas.height {
            for x in 0..self.current_canvas.width {
                let idx = (y as usize) * (self.current_canvas.width as usize) + (x as usize);
                if self.current_canvas.cells[idx] != self.last_canvas.cells[idx] {
                    let cell = &self.current_canvas.cells[idx];
                    self.stdout.queue(cursor::MoveTo(x, y))?;
                    
                    let fg = cell.style.foreground_color.unwrap_or(Color::Reset);
                    let bg = cell.style.background_color.unwrap_or(Color::Reset);
                    self.stdout.queue(style::SetColors(Colors::new(fg, bg)))?;
                    
                    self.stdout.queue(style::Print(cell.content))?;
                }
            }
        }
        self.end_sync()?;
        self.stdout.flush()?;
        self.last_canvas.cells.clone_from(&self.current_canvas.cells);
        Ok(())
    }

    pub fn poll_event(&self, timeout: Duration) -> io::Result<Option<Event>> {
        if event::poll(timeout)? {
            Ok(Some(event::read()?))
        } else {
            Ok(None)
        }
    }

    pub fn canvas_mut(&mut self) -> &mut Canvas {
        &mut self.current_canvas
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}
