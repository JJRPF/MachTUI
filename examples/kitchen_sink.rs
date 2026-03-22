//! Kitchen Sink Demo: Gradients, ASCII Art, Mouse support, and MVU.

use machtui::core::Renderer;
use machtui::talon::{Model, Program, Cmd};
use machtui::vision::utils::get_ascii_art;
use machtui::oracle::SemanticNode;
use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent, MouseEventKind};
use crossterm::style::Color;
use std::io;
use std::time::Duration;

#[derive(Debug)]
struct KitchenSink {
    mouse_pos: (u16, u16),
    click_count: u32,
    running: bool,
}

#[derive(Debug)]
enum Msg {
    MouseMove(u16, u16),
    Click,
    Exit,
}

impl Model for KitchenSink {
    type Message = Msg;

    fn update(&mut self, msg: Self::Message) -> Option<Cmd<Self::Message>> {
        match msg {
            Msg::MouseMove(x, y) => self.mouse_pos = (x, y),
            Msg::Click => self.click_count += 1,
            Msg::Exit => self.running = false,
        }
        None
    }

    fn view(&self) -> String {
        format!("Clicks: {} | Mouse: {:?}", self.click_count, self.mouse_pos)
    }

    fn semantic_view(&self) -> SemanticNode {
        SemanticNode::new("kitchen_sink").with_content(&self.view())
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let mut prog = Program::new(KitchenSink {
        mouse_pos: (0, 0),
        click_count: 0,
        running: true,
    });

    let ascii_name = get_ascii_art("MACH TUI");

    while prog.model().running {
        if let Some(event) = renderer.poll_event(Duration::from_millis(10))? {
            match event {
                Event::Key(KeyEvent { code, .. }) => {
                    if code == KeyCode::Char('q') { prog.dispatch(Msg::Exit); }
                }
                Event::Mouse(MouseEvent { kind, column, row, .. }) => {
                    prog.dispatch(Msg::MouseMove(column, row));
                    if let MouseEventKind::Down(_) = kind {
                        prog.dispatch(Msg::Click);
                    }
                }
                _ => {}
            }
        }

        prog.update().await;

        let canvas = renderer.canvas_mut();
        canvas.clear();

        // Draw big ASCII header with gradient
        for (i, line) in ascii_name.iter().enumerate() {
            canvas.draw_gradient_text(2, 1 + i as u16, line, (255, 0, 255), (0, 255, 255));
        }

        canvas.draw_text(2, 8, "--- HIGH-END TUI EXPERIENCE ---", Some(Color::Yellow));
        canvas.draw_text(2, 10, &prog.model().view(), Some(Color::Green));
        canvas.draw_text(2, 12, "Use your mouse to interact!", Some(Color::Cyan));
        
        // Draw a mouse cursor indicator
        canvas.set_cell(prog.model().mouse_pos.0, prog.model().mouse_pos.1, 'X', Some(Color::Red));

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
