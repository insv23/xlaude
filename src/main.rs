use anyhow::Result;
use clap::{Parser, Subcommand};

mod claude;
mod commands;
mod git;
mod state;
mod utils;

use commands::{handle_add, handle_create, handle_delete, handle_list, handle_new, handle_open};

#[derive(Parser)]
#[command(name = "xlaude")]
#[command(about = "Manage Claude instances with git worktrees", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new git worktree
    Create {
        /// Name for the worktree (random BIP39 word if not provided)
        name: Option<String>,
    },
    /// Open an existing worktree
    Open {
        /// Name of the worktree to open (interactive selection if not provided)
        name: Option<String>,
        /// Command to open the worktree with (e.g., "code", "vim", "emacs")
        #[arg(short, long)]
        with: Option<String>,
    },
    /// Delete a worktree and clean up
    Delete {
        /// Name of the worktree to delete (current if not provided)
        name: Option<String>,
        /// Force delete even if worktree directory is missing
        #[arg(short, long)]
        force: bool,
    },
    /// Add current worktree to xlaude management
    Add {
        /// Name for the worktree (defaults to current branch name)
        name: Option<String>,
    },
    /// List all active Claude instances
    List,
    /// Create and open a new git worktree
    New {
        /// Name for the worktree (random BIP39 word if not provided)
        name: Option<String>,
        /// Command to open the worktree with (e.g., "code", "vim", "emacs")
        #[arg(short, long)]
        with: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { name } => handle_create(name),
        Commands::Open { name, with } => handle_open(name, with),
        Commands::Delete { name, force } => handle_delete(name, force),
        Commands::Add { name } => handle_add(name),
        Commands::List => handle_list(),
        Commands::New { name, with } => handle_new(name, with),
    }
}
