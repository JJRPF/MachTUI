//! The "Mach" Core
//!
//! Immediate-mode renderer internally, exposed via a Reactive Component layer.

use crossterm::{
    cursor,
    event::{self, Event},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    style::{self, Color, ContentStyle},
    QueueableCommand,
};
use std::io::{self, Stdout, Write};
use std::time::Duration;

/// A single cell in the terminal grid.
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub content: char,
    pub style: ContentStyle,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            content: ' ',
            style: ContentStyle::default(),
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

    pub fn set_cell(&mut self, x: u16, y: u16, content: char, color: Option<Color>) {
        if x < self.width && y < self.height {
            let idx = (y as usize) * (self.width as usize) + (x as usize);
            self.cells[idx].content = content;
            if let Some(c) = color {
                self.cells[idx].style.foreground_color = Some(c);
            }
        }
    }

    pub fn draw_text(&mut self, x: u16, y: u16, text: &str, color: Option<Color>) {
        for (i, c) in text.chars().enumerate() {
            self.set_cell(x + i as u16, y, c, color);
        }
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
}

impl Renderer {
    pub fn new() -> io::Result<Self> {
        let (width, height) = terminal::size()?;
        let mut stdout = io::stdout();
        terminal::enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
        Ok(Self {
            stdout,
            current_canvas: Canvas::new(width, height),
            last_canvas: Canvas::new(width, height),
            sync_output: true,
        })
    }

    pub fn shutdown(&mut self) -> io::Result<()> {
        terminal::disable_raw_mode()?;
        execute!(self.stdout, LeaveAlternateScreen, cursor::Show)?;
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
        self.begin_sync()?;
        for y in 0..self.current_canvas.height {
            for x in 0..self.current_canvas.width {
                let idx = (y as usize) * (self.current_canvas.width as usize) + (x as usize);
                if self.current_canvas.cells[idx] != self.last_canvas.cells[idx] {
                    let cell = &self.current_canvas.cells[idx];
                    self.stdout.queue(cursor::MoveTo(x, y))?;
                    self.stdout.queue(style::SetForegroundColor(
                        cell.style.foreground_color.unwrap_or(Color::Reset),
                    ))?;
                    self.stdout.queue(style::Print(cell.content))?;
                }
            }
        }
        self.end_sync()?;
        self.stdout.flush()?;
        self.last_canvas.cells.clone_from(&self.current_canvas.cells);
        Ok(())
    }

    /// Optimized run loop that avoids complex async closure captures by separating event polling.
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
