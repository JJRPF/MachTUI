//! Installer Pro Demo: Wizard-based UI with Modals and Progress.

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use machtui::core::components::{BoxComponent, Component, ProgressBar};
use machtui::core::modals::Modal;
use machtui::core::Renderer;
use machtui::vision::icons::Icons;
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let mut step = 0;
    let mut progress = 0.0;
    let mut modal = Modal::new(
        " INSTALLATION COMPLETE ",
        "MachTUI has been successfully installed!",
    );

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(16))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Esc => modal.visible = false,
                    KeyCode::Enter => {
                        if step < 2 {
                            step += 1;
                        } else if step == 2 {
                            modal.visible = true;
                        }
                    }
                    _ => {}
                }
            }
        }

        if step == 1 && progress < 1.0 {
            progress += 0.01;
        }

        let canvas = renderer.canvas_mut();
        canvas.clear();

        // --- HEADER ---
        canvas.draw_gradient_text(
            2,
            1,
            &format!("{} MACH INSTALLER PRO", Icons::ROCKET),
            (0, 255, 255),
            (255, 0, 255),
        );

        // --- MAIN WIZARD AREA ---
        let main_box = BoxComponent::new(&format!(" STEP {} / 3 ", step + 1));
        main_box.render(canvas, 4, 4, 60, 15);

        match step {
            0 => {
                canvas.draw_text(
                    6,
                    6,
                    "Welcome to the MachTUI Setup Wizard.",
                    Some(Color::White),
                );
                canvas.draw_text(
                    6,
                    8,
                    "This will deploy the engine to your system.",
                    Some(Color::Grey),
                );
                canvas.draw_text(6, 12, "Press [ENTER] to continue...", Some(Color::Green));
            }
            1 => {
                canvas.draw_text(
                    6,
                    6,
                    "Extracting high-performance assets...",
                    Some(Color::White),
                );
                let pb = ProgressBar::new("Deployment", progress);
                pb.render(canvas, 6, 8, 50, 2);
                if progress >= 1.0 {
                    canvas.draw_text(
                        6,
                        12,
                        "Ready! Press [ENTER] to finalize.",
                        Some(Color::Green),
                    );
                }
            }
            2 => {
                canvas.draw_text(6, 6, "Finalizing system environment...", Some(Color::White));
                canvas.draw_text(6, 8, "Adding 'mach' to your PATH.", Some(Color::Cyan));
                canvas.draw_text(6, 12, "Press [ENTER] to finish.", Some(Color::Green));
            }
            _ => {}
        }

        // --- RENDER MODAL OVERLAY ---
        modal.render_on_top(canvas);

        canvas.draw_text(
            2,
            22,
            "Arrows: Nav | Enter: Action | Q: Quit",
            Some(Color::DarkGrey),
        );

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}
