use clap::{Parser, Subcommand};
use std::process;

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
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name } => {
            println!("Creating new MachTUI project: {}...", name);
            // Scaffolding logic here
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
                // Server logic here
            } else {
                println!("MachTUI Oracle: Inspection Mode");
                // Inspection logic here
            }
        }
    }
}
