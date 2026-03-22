//! MachBrowser Demo: HTML-to-MachTUI Rendering.

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use machtui::core::components::{BoxComponent, Component};
use machtui::core::Renderer;
use machtui::plume::converter::HtmlConverter;
use machtui::vision::icons::Icons;
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;

    let html_content = r#"
        <body>
            <h1 id="header" class="title">MACHTUI WEB</h1>
            <p class="description">Rendering HTML directly in your terminal with high-fidelity Plume conversion.</p>
            <div id="content">
                <button class="btn">Click Me</button>
                <button class="btn">Explore Docs</button>
            </div>
        </body>
    "#;

    let root_node = HtmlConverter::convert(html_content).expect("HTML Conversion failed");

    loop {
        if let Some(event) = renderer.poll_event(Duration::from_millis(16))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                if code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        let canvas = renderer.canvas_mut();
        canvas.clear();

        // --- HEADER ---
        canvas.draw_gradient_text(
            2,
            1,
            &format!("{} MACH BROWSER", Icons::ROCKET),
            (0, 255, 255),
            (255, 0, 255),
        );

        // --- RENDER CONVERTED HTML ---
        let b = BoxComponent::new(" HTML VIEWPORT ");
        b.render(canvas, 2, 3, canvas.width - 4, canvas.height - 6);

        // Simple recursive render of the converted node
        render_layout_node(&root_node, canvas, 4, 5);

        canvas.draw_text(
            2,
            canvas.height - 1,
            "Press 'q' to exit | HTML -> Plume -> MachTUI",
            Some(Color::DarkGrey),
        );

        renderer.render()?;
    }

    renderer.shutdown()?;
    Ok(())
}

fn render_layout_node(
    node: &machtui::plume::LayoutNode,
    canvas: &mut machtui::core::Canvas,
    x: u16,
    y: u16,
) -> u16 {
    let mut offset_y = 0;

    match node.tag.as_str() {
        "h1" => {
            if let Some(ref id) = node.id {
                canvas.draw_text(x, y, id, Some(Color::Yellow));
                offset_y += 2;
            }
        }
        "p" => {
            if let Some(ref id) = node.id {
                canvas.draw_text(x, y, id, Some(Color::White));
                offset_y += 2;
            }
        }
        "button" => {
            if let Some(ref id) = node.id {
                let btn = BoxComponent::new(id);
                btn.render(canvas, x, y, (id.len() + 4) as u16, 3);
                offset_y += 4;
            }
        }
        _ => {}
    }

    for child in &node.children {
        offset_y += render_layout_node(child, canvas, x + 2, y + offset_y);
    }

    offset_y
}
