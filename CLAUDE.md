# xlaude - Claude 实例管理工具

xlaude 是一个用于管理 Claude 实例的命令行工具，通过 git worktree 实现多分支并行开发。

## 核心功能

### xlaude create [name]
创建新的 worktree 和分支：
- 必须在 main/master/develop 分支上执行
- 如果不提供 name，自动从 BIP39 词库随机选择一个词
- 创建新分支 `<name>`
- 创建 worktree 到 `../<repo-name>-<name>` 目录
- **不会自动启动 Claude**

### xlaude open [name] [--with <program>]
打开已存在的 worktree：
- 有参数：打开指定的 worktree
- 无参数：显示交互式选择列表
- 切换到 worktree 目录
- `--with` 或 `-w`：使用指定程序打开目录（如 `code`、`vim`、`emacs` 等）
- 不带 `--with`：仅切换到 worktree 目录，不启动任何程序
- 继承所有环境变量

### xlaude delete [name] [--force]
删除 worktree 并清理：
- 有参数：删除指定的 worktree
- 无参数：删除当前所在的 worktree
- 检查未提交的修改和未推送的 commit
- 需要时进行二次确认
- 自动删除 worktree 和本地分支（如果安全）
- `--force` 或 `-f`：强制删除，跳过目录存在性检查
  - 适用于手动删除了 worktree 目录的情况
  - 清理 xlaude 状态和 git worktree 引用

### xlaude add [name]
将当前 worktree 添加到 xlaude 管理：
- 必须在 git worktree 中执行
- 如果不提供 name，默认使用当前分支名
- 检查是否已被管理，避免重复添加
- 适用于手动创建的 worktree 或从其他地方克隆的项目

### xlaude list
列出所有活跃的 worktree，显示：
- 名称
- 仓库名
- 路径
- 创建时间
- Claude sessions（如果存在）
  - 显示最多 3 个最近的 session
  - 每个 session 显示：最后更新时间和最后的用户消息
  - 超过 3 个时显示剩余数量

## 技术实现

- 使用 Rust 开发
- 直接调用系统 git 命令
- 状态持久化到：
  - macOS: `~/Library/Application Support/com.xuanwo.xlaude/state.json`
  - Linux: `~/.config/xlaude/state.json`
  - Windows: `%APPDATA%\xuanwo\xlaude\config\state.json`
- 使用 clap 构建 CLI
- 使用 BIP39 词库生成随机名称
- 彩色输出和交互式确认

## 使用示例

```bash
# 在 opendal 项目中创建新的工作分支
cd opendal
xlaude create feature-x  # 创建 ../opendal-feature-x 目录

# 使用随机名称创建
xlaude create  # 可能创建 ../opendal-dolphin 目录

# 打开 worktree
xlaude open feature-x  # 仅切换到指定的 worktree 目录
xlaude open  # 交互式选择要打开的 worktree

# 使用特定程序打开
xlaude open feature-x --with code  # 使用 VS Code 打开
xlaude open feature-x --with vim   # 使用 vim 打开
xlaude open --with code           # 交互式选择后用 VS Code 打开

# 将已存在的 worktree 添加到管理
cd ../opendal-bugfix
xlaude add  # 使用当前分支名作为名称
xlaude add hotfix  # 或指定自定义名称

# 列出所有活跃的实例
xlaude list

# 删除当前 worktree
xlaude delete

# 删除指定 worktree
xlaude delete feature-x

# 强制删除（目录已被手动删除的情况）
xlaude delete feature-x --force

# 典型工作流
xlaude create my-feature  # 创建 worktree
xlaude open my-feature   # 切换到 worktree 目录
xlaude open my-feature --with code  # 或使用 VS Code 打开
# ... 工作完成后 ...
xlaude delete my-feature # 清理 worktree
```