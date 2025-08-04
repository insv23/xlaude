use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::Confirm;
use std::path::Path;

use crate::git::{execute_git, has_unpushed_commits, is_working_tree_clean};
use crate::state::{WorktreeInfo, XlaudeState};
use crate::utils::{find_current_worktree_name, get_worktree_info};

pub fn handle_delete(name: Option<String>, force: bool) -> Result<()> {
    let mut state = XlaudeState::load()?;
    let worktree_name = resolve_worktree_name(name, &state)?;
    let worktree_info = get_worktree_info(&state, &worktree_name)?;

    println!(
        "{} Checking worktree '{}'...",
        "🔍".yellow(),
        worktree_name.cyan()
    );

    if should_check_worktree_status(force, &worktree_info.path) {
        check_and_confirm_deletion(&worktree_info.path)?;
    } else if !force && !worktree_info.path.exists() {
        return handle_missing_directory(&worktree_info.path);
    }

    cleanup_git_references(force, worktree_info)?;
    update_state(&mut state, &worktree_name)?;
    
    println!(
        "{} Worktree '{}' deleted successfully",
        "✅".green(),
        worktree_name.cyan()
    );
    Ok(())
}

fn resolve_worktree_name(name: Option<String>, state: &XlaudeState) -> Result<String> {
    match name {
        Some(name) => Ok(name),
        None => find_current_worktree_name(state),
    }
}



fn should_check_worktree_status(force: bool, worktree_path: &Path) -> bool {
    !force && worktree_path.exists()
}

fn handle_missing_directory(worktree_path: &Path) -> Result<()> {
    println!(
        "{} Worktree directory not found: {}",
        "⚠️ ".red(),
        worktree_path.display()
    );
    println!("{} Use --force to remove from xlaude state anyway", "💡".yellow());
    Ok(())
}

fn check_and_confirm_deletion(worktree_path: &Path) -> Result<()> {
    let original_dir = std::env::current_dir()?;
    
    std::env::set_current_dir(worktree_path)
        .context("Failed to change to worktree directory")?;

    let has_uncommitted_changes = !is_working_tree_clean()?;
    let has_unpushed_commits_flag = has_unpushed_commits();

    std::env::set_current_dir(&original_dir)?;

    if has_uncommitted_changes || has_unpushed_commits_flag {
        display_warnings(has_uncommitted_changes, has_unpushed_commits_flag);
        confirm_deletion()?;
    }

    Ok(())
}

fn display_warnings(has_uncommitted_changes: bool, has_unpushed_commits_flag: bool) {
    println!();
    if has_uncommitted_changes {
        println!("{} You have uncommitted changes", "⚠️ ".red());
    }
    if has_unpushed_commits_flag {
        println!("{} You have unpushed commits", "⚠️ ".red());
    }
}

fn confirm_deletion() -> Result<()> {
    let confirmed = Confirm::new()
        .with_prompt("Are you sure you want to delete this worktree?")
        .default(false)
        .interact()?;

    if !confirmed {
        println!("{} Cancelled", "❌".red());
        anyhow::bail!("Operation cancelled");
    }

    Ok(())
}

fn cleanup_git_references(force: bool, worktree_info: &WorktreeInfo) -> Result<()> {
    if worktree_info.path.exists() {
        remove_existing_worktree(worktree_info)
    } else if force {
        cleanup_missing_worktree_references(&worktree_info.branch)
    } else {
        Ok(())
    }
}

fn remove_existing_worktree(worktree_info: &WorktreeInfo) -> Result<()> {
    println!("{} Removing worktree...", "🗑️ ".yellow());
    
    execute_git(&["worktree", "remove", worktree_info.path.to_str().unwrap()])
        .context("Failed to remove worktree")?;
    
    attempt_branch_deletion(&worktree_info.branch);
    Ok(())
}

fn cleanup_missing_worktree_references(branch_name: &str) -> Result<()> {
    println!("{} Cleaning up git worktree references...", "🧹".yellow());
    
    let _ = execute_git(&["worktree", "prune"]);
    attempt_branch_deletion(branch_name);
    
    Ok(())
}

fn attempt_branch_deletion(branch_name: &str) {
    let _ = execute_git(&["branch", "-d", branch_name]);
}

fn update_state(state: &mut XlaudeState, worktree_name: &str) -> Result<()> {
    state.worktrees.remove(worktree_name);
    state.save()
}
