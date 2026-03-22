//! Counter Demo: MVU, Styling, and Human-TUI interaction.

use machtui::core::Renderer;
use machtui::talon::{Model, Program};
use machtui::oracle::SemanticNode;
use crossterm::event::{Event, KeyCode, KeyEvent};
use std::io;

#[derive(Debug)]
struct App {
    count: i32,
    running: bool,
}

#[derive(Debug)]
enum Msg {
    Inc,
    Dec,
    Exit,
}

impl Model for App {
    type Message = Msg;

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Inc => self.count += 1,
            Msg::Dec => self.count -= 1,
            Msg::Exit => self.running = false,
        }
    }

    fn view(&self) -> String {
        format!("Counter: {} (Press '+' or '-' to change, 'q' to quit)", self.count)
    }

    fn semantic_view(&self) -> SemanticNode {
        SemanticNode::new("app")
            .with_content(&self.view())
    }
}

fn main() -> io::Result<()> {
    let app = App { count: 0, running: true };
    let mut prog = Program::new(app);
    let mut renderer = Renderer::new()?;

    renderer.run(|canvas, event| {
        if let Some(Event::Key(KeyEvent { code, .. })) = event {
            match code {
                KeyCode::Char('+') => prog.dispatch(Msg::Inc),
                KeyCode::Char('-') => prog.dispatch(Msg::Dec),
                KeyCode::Char('q') | KeyCode::Esc => prog.dispatch(Msg::Exit),
                _ => {}
            }
        }

        canvas.draw_text(2, 2, &prog.model().view(), None);
        Ok(prog.model().running)
    })
}
