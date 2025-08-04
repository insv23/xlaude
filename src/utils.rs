use anyhow::{Context, Result};
use dialoguer::Select;
use rand::RngCore;
use rand::seq::SliceRandom;

use crate::state::{WorktreeInfo, XlaudeState};

pub fn generate_random_name() -> Result<String> {
    // Generate 128 bits of entropy for a 12-word mnemonic
    let mut entropy = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut entropy);

    let mnemonic = bip39::Mnemonic::from_entropy(&entropy)?;
    let words: Vec<&str> = mnemonic.words().collect();
    words
        .choose(&mut rand::thread_rng())
        .map(|&word| word.to_string())
        .context("Failed to generate random name")
}

/// Resolves a worktree name from an optional parameter
/// If name is provided, validates it exists in state
/// If name is None, presents interactive selection
pub fn resolve_worktree_name(name: Option<String>, state: &XlaudeState) -> Result<String> {
    if state.worktrees.is_empty() {
        anyhow::bail!("No worktrees found. Create one first with 'xlaude create'");
    }

    match name {
        Some(worktree_name) => {
            if !state.worktrees.contains_key(&worktree_name) {
                anyhow::bail!("Worktree '{}' not found", worktree_name);
            }
            Ok(worktree_name)
        }
        None => select_worktree_interactively(state),
    }
}

/// Presents an interactive selection of available worktrees
fn select_worktree_interactively(state: &XlaudeState) -> Result<String> {
    let names: Vec<&String> = state.worktrees.keys().collect();
    let selection = Select::new()
        .with_prompt("Select a worktree to open")
        .items(&names)
        .interact()?;
    Ok(names[selection].clone())
}

/// Gets worktree info by name, with proper error context
pub fn get_worktree_info<'a>(state: &'a XlaudeState, worktree_name: &str) -> Result<&'a WorktreeInfo> {
    state
        .worktrees
        .get(worktree_name)
        .context(format!("Worktree '{}' not found", worktree_name))
}

/// Finds the current worktree name based on the current directory
pub fn find_current_worktree_name(state: &XlaudeState) -> Result<String> {
    let current_dir = std::env::current_dir()?;
    let current_dir_name = current_dir
        .file_name()
        .and_then(|n| n.to_str())
        .context("Failed to get current directory name")?;

    state
        .worktrees
        .values()
        .find(|worktree| {
            worktree.path
                .file_name()
                .and_then(|n| n.to_str()) == Some(current_dir_name)
        })
        .map(|worktree| worktree.name.clone())
        .context("Current directory is not a managed worktree")
}

