# xlaude - Xuanwo's Claude Code

A CLI tool for managing Claude instances with git worktree for parallel development workflows.

## Features

- **Create isolated workspaces**: Each Claude instance runs in its own git worktree
- **Seamless switching**: Open and switch between multiple development contexts
- **Smart cleanup**: Safely delete worktrees with uncommitted change detection
- **Session tracking**: View Claude conversation history across instances
- **Random naming**: Generate memorable names using BIP39 word list

## Installation

```bash
cargo install xlaude
```

Or build from source:

```bash
git clone https://github.com/xuanwo/xlaude
cd xlaude
cargo build --release
```

## Usage

### Create a new workspace

```bash
# Create with custom name
xlaude create feature-auth

# Create with random name (e.g., "dolphin", "rabbit")
xlaude create
```

This creates a new git worktree at `../<repo>-<name>` and a corresponding branch.

### Open an existing workspace

```bash
# Switch to specific workspace
xlaude open feature-auth

# Interactive selection
xlaude open

# Open with VS Code
xlaude open feature-auth --with code

# Open with vim
xlaude open feature-auth --with vim
```

This switches to the worktree directory. Use `--with` to launch a specific program.

### Add existing worktree

```bash
# Add current worktree with branch name
cd ../myproject-bugfix
xlaude add

# Add with custom name
xlaude add hotfix
```

### List all workspaces

```bash
xlaude list
```

Shows all managed worktrees with:
- Name, repository, and path
- Creation time
- Recent Claude sessions (up to 3)
- Last user message from each session

### Delete a workspace

```bash
# Delete current workspace
xlaude delete

# Delete specific workspace
xlaude delete feature-auth

# Force delete (skip directory existence check)
xlaude delete feature-auth --force
```

Performs safety checks for:
- Uncommitted changes
- Unpushed commits
- Confirms before deletion

Use `--force` when:
- Worktree directory was manually deleted
- Need to clean up orphaned entries from xlaude state

## Typical Workflow

1. **Start a new feature**:
   ```bash
   xlaude create auth-system
   xlaude open auth-system  # Just switch to directory
   # Or open with your preferred editor:
   xlaude open auth-system --with code
   ```

2. **Work on the feature** in your preferred environment

3. **Switch contexts**:
   ```bash
   xlaude open  # Select another workspace
   ```

4. **Clean up** when done:
   ```bash
   xlaude delete auth-system
   ```

## Configuration

State is persisted to `~/.config/xlaude/state.json`.

## Requirements

- Git with worktree support
- Claude CLI installed
- Rust (for building from source)

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.