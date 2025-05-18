# Installation Guide

Athira can be installed through multiple package managers, via our install script, or using Nix.

## Installation Methods

### 1. Using Install Script (Recommended)

The easiest way to install Athira is through our install script:

```sh
curl -sSL https://raw.githubusercontent.com/victorprimex/athira/main/install.sh | bash
```

### 2. Using Package Managers

#### Cargo (Rust)
```sh
cargo install athira
```

#### NPM (Node.js)
```sh
npm install -g athira
```

#### PyPI (Python)
```sh
pip install athira
```

### 3. Using Nix

#### As a Flake (Recommended)
```sh
# Run directly
nix run github:victorprimex/athira

# Install into your profile
nix profile install github:victorprimex/athira

# Add to your NixOS configuration
{
  inputs.athira.url = "github:victorprimex/athira";
  
  # Add to your system packages
  environment.systemPackages = [ inputs.athira.packages.${system}.default ];
}
```

#### Development Shell
To enter a development environment with all dependencies:
```sh
# Using flakes
nix develop github:victorprimex/athira

# Or clone the repository and run
git clone https://github.com/victorprimex/athira.git
cd athira
nix develop
```

#### Building with Nix
You can build the package from source using Nix:
```sh
# Build the package
nix build github:victorprimex/athira

# Or after cloning the repository
git clone https://github.com/victorprimex/athira.git
cd athira
nix build

# The built binary will be available in ./result/bin/thira
```

### 4. Building from Source

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