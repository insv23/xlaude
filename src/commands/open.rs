use anyhow::{Context, Result};
use colored::Colorize;
use std::process::Command;

use crate::state::XlaudeState;
use crate::utils::{resolve_worktree_name, get_worktree_info};

pub fn handle_open(name: Option<String>, with: Option<String>) -> Result<()> {
    let state = XlaudeState::load()?;
    let worktree_name = resolve_worktree_name(name, &state)?;
    let worktree_info = get_worktree_info(&state, &worktree_name)?;

    open_worktree(&worktree_name, &worktree_info.path, with)
}

/// Opens a worktree at the given path with optional program
pub fn open_worktree(worktree_name: &str, worktree_path: &std::path::Path, with: Option<String>) -> Result<()> {
    // Change to worktree directory
    std::env::set_current_dir(worktree_path).context("Failed to change directory")?;

    match with {
        Some(program) => open_with_program(worktree_name, &program),
        None => {
            println!(
                "{} Switched to worktree '{}' at {}",
                "✓".green(),
                worktree_name.cyan(),
                worktree_path.display()
            );
            println!("{} You can now start working in this directory", "→".blue());
            Ok(())
        }
    }
}

/// Opens the worktree with a specific program
fn open_with_program(worktree_name: &str, program: &str) -> Result<()> {
    println!(
        "{} Opening worktree '{}' with {}...",
        "🚀".green(),
        worktree_name.cyan(),
        program.cyan()
    );

    let status = Command::new(program)
        .arg(".")
        .envs(std::env::vars()) // Inherit all environment variables
        .status()
        .context(format!("Failed to launch {}", program))?;

    if !status.success() {
        anyhow::bail!("{} exited with error", program);
    }

    Ok(())
}
