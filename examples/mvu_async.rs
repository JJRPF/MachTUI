//! MVU Async Demo: Showcasing Cmd and background tasks.

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use machtui::core::Renderer;
use machtui::oracle::SemanticNode;
use machtui::talon::{Cmd, Model, Program};
use std::io;
use std::time::Duration;

#[derive(Debug)]
struct AsyncApp {
    status: String,
    bg_color: (u8, u8, u8),
    running: bool,
}

#[derive(Debug)]
enum Msg {
    TriggerTask,
    TaskFinished(String, (u8, u8, u8)),
    Exit,
}

impl Model for AsyncApp {
    type Message = Msg;

    fn update(&mut self, msg: Self::Message) -> Option<Cmd<Self::Message>> {
        match msg {
            Msg::TriggerTask => {
                self.status = "Running async task...".to_string();
                // Return a Command (async side-effect)
                return Some(Box::pin(async {
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    let r = rand::random::<u8>();
                    let g = rand::random::<u8>();
                    let b = rand::random::<u8>();
                    Some(Msg::TaskFinished("Task Complete!".to_string(), (r, g, b)))
                }));
            }
            Msg::TaskFinished(s, color) => {
                self.status = s;
                self.bg_color = color;
            }
            Msg::Exit => self.running = false,
        }
        None
    }

    fn view(&self) -> String {
        self.status.clone()
    }

    fn semantic_view(&self) -> SemanticNode {
        SemanticNode::new("async_app").with_content(&self.view())
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let mut prog = Program::new(AsyncApp {
        status: "Idle".to_string(),
        bg_color: (20, 20, 20),
        running: true,
    });

    while prog.model().running {
        if let Some(event) = renderer.poll_event(Duration::from_millis(10))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                match code {
                    KeyCode::Char('t') => prog.dispatch(Msg::TriggerTask),
                    KeyCode::Char('q') => prog.dispatch(Msg::Exit),
                    _ => {}
                }
            }
        }

        prog.update().await;

        let canvas = renderer.canvas_mut();
        canvas.clear();

        let (r, g, b) = prog.model().bg_color;
        canvas.draw_text(2, 2, "--- MachTUI Async MVU ---", Some(Color::Yellow));
        canvas.draw_text(
            2,
            4,
            &format!("Status: {}", prog.model().status),
            Some(Color::Rgb { r, g, b }),
        );
        canvas.draw_text(
            2,
            6,
            "Press 't' to trigger a 2-second background task",
            Some(Color::Cyan),
        );
        canvas.draw_text(2, 8, "Press 'q' to exit", Some(Color::Grey));

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
