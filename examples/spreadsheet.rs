//! Spreadsheet Demo: Complex Table Layout and Grid interactions.

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use machtui::core::components::{BoxComponent, Component};
use machtui::core::Renderer;
use std::io;
use std::time::Duration;

#[derive(Clone)]
struct CellData {
    value: String,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let mut grid = vec![
        vec![
            CellData {
                value: "0".to_string()
            };
            5
        ];
        10
    ];
    let mut sel_x = 0;
    let mut sel_y = 0;

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(16))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => {
                        if sel_y > 0 {
                            sel_y -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if sel_y < 9 {
                            sel_y += 1;
                        }
                    }
                    KeyCode::Left => {
                        if sel_x > 0 {
                            sel_x -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if sel_x < 4 {
                            sel_x += 1;
                        }
                    }
                    KeyCode::Char(c) => {
                        grid[sel_y][sel_x].value.push(c);
                    }
                    KeyCode::Backspace => {
                        grid[sel_y][sel_x].value.pop();
                    }
                    _ => {}
                }
            }
        }

        let canvas = renderer.canvas_mut();
        canvas.clear();

        let col_w = 12;
        let row_h = 2;

        // --- DRAW GRID ---
        for y in 0..10 {
            for x in 0..5 {
                let px = 2 + (x as u16 * col_w);
                let py = 2 + (y as u16 * row_h);

                let is_sel = x == sel_x && y == sel_y;
                let color = if is_sel {
                    Color::Yellow
                } else {
                    Color::DarkGrey
                };

                let cell_box = BoxComponent {
                    title: "".into(),
                    border_color: color,
                };
                cell_box.render(canvas, px, py, col_w, 3);

                canvas.draw_text(px + 1, py + 1, &grid[y][x].value, Some(Color::White));
            }
        }

        canvas.draw_gradient_text(2, 0, "MACHTUI SPREADSHEET PRO", (0, 255, 0), (0, 255, 255));
        canvas.draw_text(
            2,
            24,
            "Arrows: Navigate | Type: Edit | Q: Quit",
            Some(Color::Grey),
        );

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
