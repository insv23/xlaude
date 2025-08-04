use anyhow::Result;

use crate::commands::create::create_worktree;
use crate::commands::open::open_worktree;

pub fn handle_new(name: Option<String>, with: Option<String>) -> Result<()> {
    let (worktree_name, worktree_path) = create_worktree(name)?;
    open_worktree(&worktree_name, &worktree_path, with)
}