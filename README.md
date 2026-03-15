# Bivio (bvo)

A fast, lightweight CLI tool to manage and navigate between your projects.
> this is something i built on top of my workflow, it might not be perfect but it works for me

This tool adheres to the [smol contract](https://bower.sh/smol-contract) 

## Features

- **Project Registry** - Track all your projects in one centralized location
- **Tagging System** - Organize projects with arbitrary tags for easy filtering
- **Interactive Picker** - Quickly select and jump to any project with fuzzy search
- **Git Bare Repository Support** - First-class support for bare repos and worktrees
- **Shell Integration** - Generate shell hooks for seamless navigation
- **Cross-Platform** - Works on macOS, Linux, and Windows

## Building from Source

### Prerequisites

- Rust 1.70 or later
- Cargo

### Build

```bash
git clone https://github.com/rawnly/bivio.git
cd bivio
cargo build --release
```

The binary will be available at `target/release/bvo`.

### Install locally

```bash
cargo install --path .
```

## Usage

### Command Reference

- `bvo add [path]` - Register a project (defaults to current directory)
- `bvo list` - List all projects (use `--json` for machine output)
- `bvo pick [query]` - Interactive picker with optional search
- `bvo remove [name]` - Remove a project (alias: `rm`)
- `bvo tag [project] [tags...]` - Add/remove tags (use `-r` to remove)
- `bvo init [shell]` - Print shell integration hooks
- `bvo config <get|set|view>` - Read or update settings
- `bvo check-update` - Check for new releases

### Adding Projects

```bash
# Add current directory as a project
bvo add

# Add a specific path
bvo add /path/to/project

# Add with custom name and tags
bvo add -n "my-awesome-project" -t rust,cli,work
```

### Listing Projects

```bash
# List all projects
bvo list

# Filter by tags
bvo list -t rust
bvo list -t rust,cli
```

### Navigating to Projects

```bash
# Interactive project picker
bvo pick

# Search for a specific project
bvo pick my-proj

# Filter picker by tags
bvo pick -t work

# Combine search and tag filter
bvo pick my-proj -t rust
```

For bare repositories, the picker will let you select a specific worktree.

### Managing Tags

```bash
# Add tags to a project
bvo tag my-project rust cli

# Remove tags from a project
bvo tag my-project -r old-tag

# Interactive tag management
bvo tag
```

### Removing Projects

```bash
# Remove by name
bvo remove my-project

# Interactive removal
bvo rm

# Remove all projects with specific tags
bvo rm -t deprecated

# Remove all projects
bvo rm --all
```

### Configuration

```bash
# Get config value
bvo config get default_shell

# Set default shell
bvo config set default_shell zsh
```

## Shell Integration

Generate a shell hook to quickly navigate between projects using `bvo`.

### Zsh

Add to your `~/.zshrc`:

```bash
eval "$(bvo init)"
```

### Bash

Add to your `~/.bashrc`:

```bash
eval "$(bvo init)"
```

### Fish

Add to your `~/.config/fish/config.fish`:

```fish
bvo init | source
```

### Using the `bvo` command

Once configured, use `bvo` to quickly navigate:

```bash
# Open interactive picker and cd to selected project
bvo

# Search for a project and cd to it
bvo my-proj

# Filter by tags
bvo -t rust

# Combine search and tag filter
bvo my-proj -t work
```

## Data Storage

Project data and configuration are stored in the XDG config directory:

- Linux/macOS: `~/.config/bvo/`
- Windows: `%APPDATA%\bvo\`

Files:
- `projects.json` - Project registry
- `config.json` - Application settings

## Contributing

Contributions are welcome! Feel free to:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is open source and available under the [MIT License](LICENSE).
