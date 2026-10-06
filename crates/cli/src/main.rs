//! ApexCode CLI
//!
//! A conversational, easy-to-use CLI for ApexCode.

use anyhow::Result;
use clap::{Parser, Subcommand};

/// ApexCode - Make your AI code look human
#[derive(Parser)]
#[command(name = "apexcode")]
#[command(author = "Mohamed Alieu Jagitay")]
#[command(version)]
#[command(about = "Make your AI-generated code look human-written", long_about = None)]
#[command(after_help = "
Quick Start:
  apexcode              # Launch dashboard (default command)
  apexcode fix          # Auto-fix AI patterns in staged files
  apexcode scan         # See what's detected
  apexcode dashboard    # Launch visual dashboard

Learn more: https://mohamedalieujagitay.github.io/ApexCode/
")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Fix AI patterns in your code (recommended)
    Fix {
        /// Files to fix (default: staged files)
        #[arg(short, long)]
        files: Vec<String>,
        /// Show what would be changed without applying
        #[arg(short, long)]
        dry_run: bool,
    },
    /// Scan for AI-generated content
    Scan {
        /// Files to scan (default: staged files)
        #[arg(short, long)]
        files: Vec<String>,
    },
    /// Check your stealth score
    Score {
        /// Show detailed breakdown
        #[arg(short, long)]
        detailed: bool,
    },
    /// Launch visual dashboard
    Dashboard,
    /// Configure settings
    Config {
        /// Show current settings
        #[arg(short, long)]
        show: bool,
        /// Reset to defaults
        #[arg(short, long)]
        reset: bool,
    },
    /// Initialize in current directory
    Init {
        /// Force reinitialize
        #[arg(short, long)]
        force: bool,
    },
    /// Install IDE integration
    Install {
        /// IDE to install for (claude, cursor, windsurf, antigravity, all)
        #[arg(short = 'i', long)]
        ide: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // If no command, launch dashboard by default
    let command = cli.command.unwrap_or(Commands::Dashboard);

    match command {
        Commands::Fix { files, dry_run } => cmd_fix(files, dry_run),
        Commands::Scan { files } => cmd_scan(files),
        Commands::Score { detailed } => cmd_score(detailed),
        Commands::Dashboard => cmd_dashboard(),
        Commands::Config { show, reset } => cmd_config(show, reset),
        Commands::Init { force } => cmd_init(force),
        Commands::Install { ide } => cmd_install(ide),
    }
}

fn cmd_fix(_files: Vec<String>, _dry_run: bool) -> Result<()> {
    // TODO: fix isn't implemented yet; it returns without doing anything
    Ok(())
}

fn cmd_scan(_files: Vec<String>) -> Result<()> {
    // TODO: scan isn't implemented yet; it returns without doing anything
    Ok(())
}

fn cmd_score(_detailed: bool) -> Result<()> {
    // TODO: score isn't implemented yet; it returns without doing anything
    Ok(())
}

fn cmd_dashboard() -> Result<()> {
    let mut tui = apexcode_tui::ApexTui::new();
    tui.run()
}

fn cmd_config(_show: bool, _reset: bool) -> Result<()> {
    // TODO: config isn't implemented yet; it returns without doing anything
    Ok(())
}

fn cmd_init(_force: bool) -> Result<()> {
    // TODO: init isn't implemented yet; it returns without doing anything
    Ok(())
}

fn cmd_install(_ide: Option<String>) -> Result<()> {
    // TODO: install isn't implemented yet; it returns without doing anything
    Ok(())
}
