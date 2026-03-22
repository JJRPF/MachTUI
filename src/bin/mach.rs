use clap::{Parser, Subcommand};
use std::process;
use machtui::core::Renderer;
use machtui::talon::{Model, Program, Cmd};
use machtui::oracle::SemanticNode;
use machtui::vision::utils::get_ascii_art;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use std::io;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "mach")]
#[command(about = "The MachTUI Engine CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new MachTUI project
    New {
        /// Project name
        name: String,
    },
    /// Run the current project in development mode
    Run {
        /// Example name to run
        #[arg(short, long)]
        example: Option<String>,
    },
    /// Inspect the current UI via the Oracle AI protocol
    Oracle {
        /// Start a headless JSON-RPC server
        #[arg(short, long)]
        server: bool,
    },
    /// Launch the MachTUI Configurator
    Config,
    /// Run visual snapshot tests
    Test,
}

#[derive(Debug)]
struct SettingsModel {
    serve_locally: bool,
    serve_ssh: bool,
    port: u16,
    examples: Vec<String>,
    cursor_idx: usize,
    running: bool,
    selected_example: Option<String>,
}

#[derive(Debug)]
enum SettingsMsg {
    ToggleLocally,
    ToggleSSH,
    MoveUp,
    MoveDown,
    LaunchExample(String),
    Exit,
}

impl Model for SettingsModel {
    type Message = SettingsMsg;

    fn update(&mut self, msg: Self::Message) -> Option<Cmd<Self::Message>> {
        match msg {
            SettingsMsg::ToggleLocally => self.serve_locally = !self.serve_locally,
            SettingsMsg::ToggleSSH => self.serve_ssh = !self.serve_ssh,
            SettingsMsg::MoveUp => {
                if self.cursor_idx > 0 { self.cursor_idx -= 1; }
            }
            SettingsMsg::MoveDown => {
                if self.cursor_idx < 2 + self.examples.len() - 1 { self.cursor_idx += 1; }
            }
            SettingsMsg::LaunchExample(name) => {
                self.selected_example = Some(name);
                self.running = false;
            }
            SettingsMsg::Exit => self.running = false,
        }
        None
    }

    fn view(&self) -> String {
        // View is handled manually in the render loop for high-end effects
        String::new()
    }

    fn semantic_view(&self) -> SemanticNode {
        SemanticNode::new("configurator")
    }
}

/// Finds the MachTUI root directory (either ~/MachTUI or fallback to current)
fn get_project_root() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let home_path = PathBuf::from(home).join("MachTUI");
    if home_path.exists() {
        home_path
    } else {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let root = get_project_root();

    match &cli.command {
        Commands::New { name } => {
            println!("Creating new MachTUI project: {}...", name);
        }
        Commands::Run { example } => {
            if let Some(ex) = example {
                run_example(&root, &ex);
            } else {
                println!("Running current project...");
                process::Command::new("cargo")
                    .current_dir(&root)
                    .arg("run")
                    .status()
                    .expect("Failed to run project");
            }
        }
        Commands::Oracle { server } => {
            if *server {
                println!("Starting MachTUI Oracle JSON-RPC server on port 9090...");
                
                #[derive(Debug)]
                struct MockApp;
                impl Model for MockApp {
                    type Message = ();
                    fn update(&mut self, _: ()) -> Option<Cmd<()>> { None }
                    fn view(&self) -> String { "Headless MachTUI".into() }
                    fn semantic_view(&self) -> SemanticNode { SemanticNode::new("headless_root") }
                }

                let prog = Program::new(MockApp);
                machtui::oracle::server::start_ai_server(&prog, 9090).await.expect("Server failed");
            } else {
                println!("MachTUI Oracle: Inspection Mode");
            }
        }
        Commands::Config => {
            if let Some(ex) = run_configurator().await? {
                run_example(&root, &ex);
            }
        }
        Commands::Test => {
            println!("Running MachTUI visual snapshot tests...");
            let root = get_project_root();
            let snapshot_dir = root.join("snapshots");
            if !snapshot_dir.exists() {
                std::fs::create_dir(&snapshot_dir).expect("Failed to create snapshot dir");
            }
            println!("Snapshots directory: {:?}", snapshot_dir);
            // In a real implementation, this would iterate over examples and compare outputs
            println!("0 tests failed, 0 tests passed (Snapshot verification foundation ready)");
        }
    }
    Ok(())
}

fn run_example(root: &std::path::Path, name: &str) {
    println!("Running example: {}...", name);
    process::Command::new("cargo")
        .current_dir(root)
        .args(["run", "--example", name])
        .status()
        .expect("Failed to run example");
}

async fn run_configurator() -> io::Result<Option<String>> {
    let mut renderer = Renderer::new()?;
    let mut prog = Program::new(SettingsModel {
        serve_locally: true,
        serve_ssh: false,
        port: 8080,
        examples: vec![
            "project_pro".to_string(),
            "diagram_pro".to_string(),
            "search_pro".to_string(),
            "video_pro".to_string(),
            "matrix".to_string(),
            "paint".to_string(),
            "db_manager".to_string(),
            "gallery".to_string(),
            "tab_manager".to_string(),
            "personal_site".to_string(),
            "weather".to_string(),
            "system_dashboard".to_string(),
            "finance".to_string(),
            "mach_ide".to_string(),
            "spreadsheet".to_string(),
            "portfolio".to_string(),
            "agenda".to_string(),
            "music_visualizer".to_string(),
            "sys_top".to_string(),
            "chat".to_string(),
            "game".to_string(),
            "explorer".to_string(),
            "dashboard".to_string(),
            "kitchen_sink".to_string(),
            "counter".to_string(),
            "vision_waves".to_string(),
            "mvu_async".to_string(),
            "oracle_headless".to_string(),
        ],
        cursor_idx: 0,
        running: true,
        selected_example: None,
    });

    let header_art = get_ascii_art("MACH");

    while prog.model().running {
        if let Some(event) = renderer.poll_event(Duration::from_millis(10))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                match code {
                    KeyCode::Up => prog.dispatch(SettingsMsg::MoveUp),
                    KeyCode::Down => prog.dispatch(SettingsMsg::MoveDown),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        let idx = prog.model().cursor_idx;
                        if idx == 0 {
                            prog.dispatch(SettingsMsg::ToggleLocally);
                        } else if idx == 1 {
                            prog.dispatch(SettingsMsg::ToggleSSH);
                        } else {
                            let ex_idx = idx - 2;
                            let ex_name = prog.model().examples[ex_idx].clone();
                            prog.dispatch(SettingsMsg::LaunchExample(ex_name));
                        }
                    }
                    KeyCode::Char('q') | KeyCode::Esc => prog.dispatch(SettingsMsg::Exit),
                    _ => {}
                }
            }
        }

        prog.update().await;

        let canvas = renderer.canvas_mut();
        canvas.clear();
        
        // --- PRETTY HEADER ---
        for (i, line) in header_art.iter().enumerate() {
            canvas.draw_gradient_text(4, 1 + i as u16, line, (255, 100, 0), (255, 255, 0));
        }
        canvas.draw_text(4, 7, "SYSTEM CONFIGURATOR v0.5", Some(Color::Grey));

        // --- SETTINGS ---
        canvas.draw_text(4, 9, "⚙ SETTINGS", Some(Color::Cyan));
        
        let local_sel = if prog.model().cursor_idx == 0 { "> " } else { "  " };
        let local_color = if prog.model().cursor_idx == 0 { Color::White } else { Color::Grey };
        canvas.draw_text(4, 10, &format!("{}[{}] Serve Locally", local_sel, if prog.model().serve_locally { "X" } else { " " }), Some(local_color));

        let ssh_sel = if prog.model().cursor_idx == 1 { "> " } else { "  " };
        let ssh_color = if prog.model().cursor_idx == 1 { Color::White } else { Color::Grey };
        canvas.draw_text(4, 11, &format!("{}[{}] Serve to SSH", ssh_sel, if prog.model().serve_ssh { "X" } else { " " }), Some(ssh_color));

        // --- EXAMPLES ---
        canvas.draw_text(4, 13, "🚀 LAUNCH EXAMPLES", Some(Color::Magenta));
        for (i, ex) in prog.model().examples.iter().enumerate() {
            let sel = if prog.model().cursor_idx == 2 + i { "> " } else { "  " };
            let color = if prog.model().cursor_idx == 2 + i { Color::Green } else { Color::Grey };
            canvas.draw_text(4, 14 + i as u16, &format!("{}{}", sel, ex), Some(color));
        }

        canvas.draw_text(4, 21, "Arrows: Move | Space: Toggle | Enter: Launch | Q: Exit", Some(Color::DarkGrey));

        renderer.render()?;
    }

    let selected = prog.model().selected_example.clone();
    renderer.shutdown()?;
    Ok(selected)
}
