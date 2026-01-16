# Side Project Manager (spm)

A fast, lightweight CLI tool to manage and navigate between your side projects.
> this is something i built on top of my workflow, it might not be perfect but it works for me

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
git clone https://github.com/rawnly/side-project-manager.git
cd side-project-manager
cargo build --release
```

The binary will be available at `target/release/spm`.

### Install locally

```bash
cargo install --path .
```

## Usage

### Adding Projects

```bash
# Add current directory as a project
spm add

# Add a specific path
spm add /path/to/project

# Add with custom name and tags
spm add -n "my-awesome-project" -t rust,cli,work
```

### Listing Projects

```bash
# List all projects
spm list

# Filter by tags
spm list -t rust
spm list -t rust,cli
```

### Navigating to Projects

```bash
# Interactive project picker
spm pick

# Search for a specific project
spm pick my-proj

# Filter picker by tags
spm pick -t work

# Combine search and tag filter
spm pick my-proj -t rust
```

For bare repositories, the picker will let you select a specific worktree.

### Managing Tags

```bash
# Add tags to a project
spm tag my-project rust cli

# Remove tags from a project
spm tag my-project -r old-tag

# Interactive tag management
spm tag
```

### Removing Projects

```bash
# Remove by name
spm remove my-project

# Interactive removal
spm rm

# Remove all projects with specific tags
spm rm -t deprecated

# Remove all projects
spm rm --all
```

### Configuration

```bash
# Get config value
spm config get default_shell

# Set default shell
spm config set default_shell zsh
```

## Shell Integration

Generate a shell hook to quickly navigate between projects using the `sp` command.

### Zsh

Add to your `~/.zshrc`:

```bash
eval "$(spm init zsh)"
```

### Bash

Add to your `~/.bashrc`:

```bash
eval "$(spm init bash)"
```

### Fish

Add to your `~/.config/fish/config.fish`:

```fish
spm init fish | source
```

### Using the `sp` command

Once configured, use `sp` to quickly navigate:

```bash
# Open interactive picker and cd to selected project
sp

# Search for a project and cd to it
sp my-proj

# Filter by tags
sp -t rust

# Combine search and tag filter
sp my-proj -t work
```

## Data Storage

Project data and configuration are stored in the XDG config directory:

- Linux/macOS: `~/.config/spm/`
- Windows: `%APPDATA%\spm\`

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
