mod cmd;
mod config;
mod constants;
mod logger;
mod models;

use clap::{Parser, Subcommand};

use crate::cmd::Cmd;

#[derive(Parser)]
#[command(name = "claude-afk", about = "Push notifications for Claude Code")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set up device pairing by scanning a QR code
    Setup,
    /// Send a notification (accepts JSON as argument or reads from stdin)
    Notify {
        /// JSON input (if not provided, reads from stdin)
        json: Option<String>,
    },
    /// Show current configuration status
    Status,
    /// Enable notifications
    Activate,
    /// Disable notifications
    Deactivate,
    /// Clear device pairing
    Clear,
    /// Clear all debug logs (debug builds only)
    #[cfg(debug_assertions)]
    ClearLogs,
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Commands::Setup => Cmd::setup(),
        Commands::Notify { json } => Cmd::notify(json),
        Commands::Status => Cmd::status(),
        Commands::Activate => Cmd::activate(),
        Commands::Deactivate => Cmd::deactivate(),
        Commands::Clear => Cmd::clear(),
        #[cfg(debug_assertions)]
        Commands::ClearLogs => Cmd::clear_logs(),
    }
}
