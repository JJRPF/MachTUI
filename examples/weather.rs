//! Weather Station Demo: Real-time API integration.

use machtui::core::Renderer;
use machtui::core::components::{Component, BoxComponent};
use machtui::core::http::HttpClient;
use machtui::talon::{Model, Program, Cmd};
use machtui::vision::icons::Icons;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use serde::Deserialize;
use std::io;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct WeatherData {
    current: Current,
}

#[derive(Debug, Deserialize)]
struct Current {
    temp_c: f32,
    condition: Condition,
}

#[derive(Debug, Deserialize)]
struct Condition {
    text: String,
}

#[derive(Debug)]
struct App {
    temp: String,
    condition: String,
    status: String,
    running: bool,
}

#[derive(Debug)]
enum Msg {
    FetchWeather,
    WeatherResult(Result<WeatherData, String>),
    Exit,
}

impl Model for App {
    type Message = Msg;

    fn update(&mut self, msg: Self::Message) -> Option<Cmd<Self::Message>> {
        match msg {
            Msg::FetchWeather => {
                self.status = "Refreshing...".into();
                return Some(HttpClient::fetch_json(
                    "https://api.weatherapi.com/v1/current.json?key=7f878b3&q=London",
                    Msg::WeatherResult
                ));
            }
            Msg::WeatherResult(res) => {
                match res {
                    Ok(data) => {
                        self.temp = format!("{:.1}°C", data.current.temp_c);
                        self.condition = data.current.condition.text;
                        self.status = "Success".into();
                    }
                    Err(e) => self.status = format!("Error: {}", e),
                }
            }
            Msg::Exit => self.running = false,
        }
        None
    }

    fn view(&self) -> String { self.temp.clone() }
    fn semantic_view(&self) -> machtui::oracle::SemanticNode { machtui::oracle::SemanticNode::new("weather_app") }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let mut prog = Program::new(App {
        temp: "--".into(),
        condition: "Unknown".into(),
        status: "Idle".into(),
        running: true,
    });

    prog.dispatch(Msg::FetchWeather);

    while prog.model().running {
        if let Some(event) = renderer.poll_event(Duration::from_millis(16))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('r') => prog.dispatch(Msg::FetchWeather),
                    _ => {}
                }
            }
        }

        prog.update().await;

        let canvas = renderer.canvas_mut();
        canvas.clear();

        let b = BoxComponent::new(&format!("{} WEATHER STATION ", Icons::GEAR));
        b.render(canvas, 4, 3, 40, 10);

        canvas.draw_text(6, 5, &format!("Temp: {}", prog.model().temp), Some(Color::Cyan));
        canvas.draw_text(6, 7, &format!("Cond: {}", prog.model().condition), Some(Color::Yellow));
        canvas.draw_text(6, 9, &format!("Stat: {}", prog.model().status), Some(Color::Grey));

        canvas.draw_text(4, 14, "Press 'r' to Refresh | 'q' to Exit", Some(Color::DarkGrey));

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
