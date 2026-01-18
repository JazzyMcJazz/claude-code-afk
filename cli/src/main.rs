mod cmd;
mod config;
mod constants;
mod logger;
mod models;

use std::io::IsTerminal;

use clap::{Parser, Subcommand};

use crate::cmd::Cmd;

#[derive(Parser)]
#[command(name = "claude-afk", about = "Push notifications for Claude Code")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Set up device pairing by scanning a QR code
    Pair,
    /// Send a notification (accepts JSON as argument or reads from stdin).
    /// The command used by Claude Code hooks
    Notify {
        /// JSON input (if not provided, reads from stdin)
        json: Option<String>,
    },
    /// Show current configuration status
    Status,
    /// Enable notifications
    Activate,
    /// Alias for Activate
    AFK,
    /// Disable notifications
    Deactivate,
    /// Alias for Deactivate
    Back,
    /// Clear device pairing
    Clear,
    /// Install Claude Code hooks for push notifications
    InstallHooks,
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
        // No subcommand: default to notify if stdin is piped, otherwise show help
        None => {
            if !std::io::stdin().is_terminal() {
                Cmd::notify(None)
            } else {
                // Re-parse with --help to show usage
                use clap::CommandFactory;
                Cli::command().print_help()?;
                println!();
                Ok(())
            }
        }
        Some(Commands::Pair) => Cmd::pair(),
        Some(Commands::Notify { json }) => Cmd::notify(json),
        Some(Commands::Status) => Cmd::status(),
        Some(Commands::Activate) | Some(Commands::AFK) => Cmd::activate(),
        Some(Commands::Deactivate) | Some(Commands::Back) => Cmd::deactivate(),
        Some(Commands::Clear) => Cmd::clear(),
        Some(Commands::InstallHooks) => Cmd::install_hooks(),
        #[cfg(debug_assertions)]
        Some(Commands::ClearLogs) => Cmd::clear_logs(),
    }
}
