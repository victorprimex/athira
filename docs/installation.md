# Installation Guide

## Prerequisites

Before installing Athira, ensure you have the following installed:

1. [Rust and Cargo](https://rustup.rs/) (1.56.0 or later)
2. Git (2.28.0 or later)

## Installation Methods

### 1. Using Cargo (Recommended)

The easiest way to install Athira is through Cargo, Rust's package manager:

```sh
cargo install athira
```

This will download, compile, and install the latest stable version of Athira.

### 2. Building from Source

If you want to build from source:

```sh
# Clone the repository
git clone https://github.com/yourusername/athira.git
cd athira

# Build and install
cargo build --release
cargo install --path .
```

## Verifying Installation

After installation, verify that Athira is properly installed:

```sh
thira --version
```

You should see the version number of Athira displayed.

## Initial Setup

1. Initialize a new Git repository (if you haven't already):
   ```sh
   git init
   ```

2. Initialize Athira's configuration:
   ```sh
   thira hooks init
   ```
   This will create a `hooks.yaml` file in your project root.

3. Install the Git hooks:
   ```sh
   thira hooks install
   ```

## Configuration Location

Athira uses two main configuration locations:

- `hooks.yaml` - Project-specific hook configurations
- `.thira/` - Default directory for hook scripts (configurable)

## Troubleshooting

### Common Issues

1. **Command not found**
   - Ensure Cargo's bin directory is in your PATH
   - Try restarting your terminal

2. **Git hooks not running**
   - Check if hooks are installed: `thira hooks list`
   - Verify hooks path: `thira hooks show-path`
   - Ensure hook files are executable

3. **Permission denied**
   - Check file permissions in your `.thira` directory
   - Ensure you have write access to the Git hooks directory

### Getting Help

If you encounter any issues:

1. Run commands with more verbosity:
   ```sh
   RUST_LOG=debug thira <command>
   ```

2. Check your Git hooks path:
   ```sh
   thira hooks show-path
   ```

3. Reset to default configuration:
   ```sh
   thira hooks reset-path
   ```

For more help, visit our [GitHub repository](https://github.com/yourusername/athira/issues).