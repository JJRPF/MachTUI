//! Counter Demo: High-End Styling, Gradients, and Mouse.

use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent, MouseEventKind};
use crossterm::style::Color;
use machtui::core::Renderer;
use machtui::oracle::SemanticNode;
use machtui::talon::{Cmd, Model, Program};
use machtui::vision::utils::get_ascii_art;
use std::io;
use std::time::Duration;

#[derive(Debug)]
struct App {
    count: i32,
    running: bool,
    hovered: bool,
}

#[derive(Debug)]
enum Msg {
    Inc,
    Dec,
    SetHover(bool),
    Exit,
}

impl Model for App {
    type Message = Msg;

    fn update(&mut self, msg: Self::Message) -> Option<Cmd<Self::Message>> {
        match msg {
            Msg::Inc => self.count += 1,
            Msg::Dec => self.count -= 1,
            Msg::SetHover(h) => self.hovered = h,
            Msg::Exit => self.running = false,
        }
        None
    }

    fn view(&self) -> String {
        format!("Count: {}", self.count)
    }

    fn semantic_view(&self) -> SemanticNode {
        SemanticNode::new("counter_app").with_content(&self.view())
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let mut prog = Program::new(App {
        count: 0,
        running: true,
        hovered: false,
    });

    let header = get_ascii_art("COUNTER");

    while prog.model().running {
        if let Some(event) = renderer.poll_event(Duration::from_millis(10))? {
            match event {
                Event::Key(KeyEvent { code, .. }) => match code {
                    KeyCode::Char('+') => prog.dispatch(Msg::Inc),
                    KeyCode::Char('-') => prog.dispatch(Msg::Dec),
                    KeyCode::Char('q') => prog.dispatch(Msg::Exit),
                    _ => {}
                },
                Event::Mouse(MouseEvent {
                    kind, column, row, ..
                }) => {
                    // Simple hit detection for the counter area
                    let is_in_area = column >= 2 && column <= 20 && row >= 8 && row <= 10;
                    prog.dispatch(Msg::SetHover(is_in_area));

                    if is_in_area {
                        if let MouseEventKind::Down(_) = kind {
                            prog.dispatch(Msg::Inc);
                        }
                    }
                }
                _ => {}
            }
        }

        prog.update().await;

        let canvas = renderer.canvas_mut();
        canvas.clear();

        for (i, line) in header.iter().enumerate() {
            canvas.draw_gradient_text(2, 1 + i as u16, line, (0, 255, 0), (0, 255, 255));
        }

        let bg_color = if prog.model().hovered {
            Some(Color::DarkGrey)
        } else {
            None
        };
        let fg_color = if prog.model().hovered {
            Color::Yellow
        } else {
            Color::White
        };

        canvas.draw_text(2, 8, "-------------------", Some(fg_color));
        let count_text = format!("|  CURRENT: {:<5} |", prog.model().count);
        for (i, c) in count_text.chars().enumerate() {
            canvas.set_cell(2 + i as u16, 9, c, Some(fg_color));
            if let Some(bg) = bg_color {
                canvas.cells[(9 as usize) * (canvas.width as usize) + (2 + i as usize)]
                    .style
                    .background_color = Some(bg);
            }
        }
        canvas.draw_text(2, 10, "-------------------", Some(fg_color));

        canvas.draw_text(2, 12, "Press + / - or Click the box!", Some(Color::Grey));
        canvas.draw_text(2, 13, "Press 'q' to exit", Some(Color::DarkRed));

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
