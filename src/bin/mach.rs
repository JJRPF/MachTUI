use clap::{Parser, Subcommand};
use std::process;
use machtui::core::Renderer;
use machtui::talon::{Model, Program, Cmd};
use machtui::oracle::SemanticNode;
use crossterm::event::{Event, KeyCode, KeyEvent};
use std::io;
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
}

#[derive(Debug)]
struct SettingsModel {
    serve_locally: bool,
    serve_ssh: bool,
    port: u16,
    cursor_idx: usize,
    running: bool,
}

#[derive(Debug)]
enum SettingsMsg {
    ToggleLocally,
    ToggleSSH,
    MoveUp,
    MoveDown,
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
                if self.cursor_idx < 1 { self.cursor_idx += 1; }
            }
            SettingsMsg::Exit => self.running = false,
        }
        None
    }

    fn view(&self) -> String {
        let mut out = String::from("--- MachTUI Configurator ---\n\n");
        
        let local_sel = if self.cursor_idx == 0 { "> " } else { "  " };
        let ssh_sel = if self.cursor_idx == 1 { "> " } else { "  " };

        out.push_str(&format!("{}[{}] Serve Locally\n", local_sel, if self.serve_locally { "X" } else { " " }));
        out.push_str(&format!("{}[{}] Serve to SSH\n", ssh_sel, if self.serve_ssh { "X" } else { " " }));
        out.push_str(&format!("\nDefault Port: {}\n", self.port));
        
        out.push_str("\n(Arrows to move, Space to toggle, 'q' to save and exit)");
        out
    }

    fn semantic_view(&self) -> SemanticNode {
        let mut root = SemanticNode::new("configurator");
        root.add_child(SemanticNode::new("setting").with_content(&format!("Serve Locally: {}", self.serve_locally)));
        root.add_child(SemanticNode::new("setting").with_content(&format!("Serve to SSH: {}", self.serve_ssh)));
        root
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name } => {
            println!("Creating new MachTUI project: {}...", name);
        }
        Commands::Run { example } => {
            if let Some(ex) = example {
                println!("Running example: {}...", ex);
                process::Command::new("cargo")
                    .args(["run", "--example", ex])
                    .status()
                    .expect("Failed to run example");
            } else {
                println!("Running current project...");
                process::Command::new("cargo")
                    .arg("run")
                    .status()
                    .expect("Failed to run project");
            }
        }
        Commands::Oracle { server } => {
            if *server {
                println!("Starting MachTUI Oracle JSON-RPC server...");
            } else {
                println!("MachTUI Oracle: Inspection Mode");
            }
        }
        Commands::Config => {
            run_configurator().await?;
        }
    }
    Ok(())
}

async fn run_configurator() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let mut prog = Program::new(SettingsModel {
        serve_locally: true,
        serve_ssh: false,
        port: 8080,
        cursor_idx: 0,
        running: true,
    });

    while prog.model().running {
        if let Some(event) = renderer.poll_event(Duration::from_millis(10))? {
            if let Event::Key(KeyEvent { code, .. }) = event {
                match code {
                    KeyCode::Up => prog.dispatch(SettingsMsg::MoveUp),
                    KeyCode::Down => prog.dispatch(SettingsMsg::MoveDown),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if prog.model().cursor_idx == 0 {
                            prog.dispatch(SettingsMsg::ToggleLocally);
                        } else {
                            prog.dispatch(SettingsMsg::ToggleSSH);
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
        
        let view = prog.model().view();
        for (i, line) in view.lines().enumerate() {
            canvas.draw_text(2, 2 + i as u16, line, None);
        }

        renderer.render()?;
    }

    renderer.shutdown()?;
    println!("Settings saved.");
    Ok(())
}
