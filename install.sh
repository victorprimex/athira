#!/usr/bin/env bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# GitHub repository information
REPO_OWNER="victorprimex"
REPO_NAME="athira"
GITHUB_API="https://api.github.com"

# Print error message and exit
error() {
    echo -e "${RED}Error: $1${NC}" >&2
    exit 1
}

# Print info message
info() {
    echo -e "${GREEN}$1${NC}"
}

# Print warning message
warn() {
    echo -e "${YELLOW}Warning: $1${NC}"
}

# Detect user's actual shell
detect_shell() {
    local shell_path=""

    # Try multiple methods to detect the actual shell
    if [ -n "$FISH_VERSION" ]; then
        # Directly detected Fish shell
        shell_path=$(which fish)
    elif [ -n "$ZSH_VERSION" ]; then
        # Directly detected Zsh
        shell_path=$(which zsh)
    else
        # Check common methods to detect the login shell
        if command -v dscl >/dev/null 2>&1 && [ "$(uname)" = "Darwin" ]; then
            # macOS - use dscl to get user shell
            shell_path=$(dscl . -read ~/ UserShell | sed 's/UserShell: //')
        elif [ -r /etc/passwd ]; then
            # Unix-like systems - check passwd file
            shell_path=$(getent passwd $USER | cut -d: -f7)
        fi

        # Fallback to $SHELL if other methods fail
        if [ -z "$shell_path" ]; then
            shell_path=$SHELL
        fi
    fi

    # Extract just the shell name
    local shell_name
    shell_name=$(basename "$shell_path")

    # Determine config file path
    local config_file
    case "$shell_name" in
        fish)
            if [ -d "$HOME/.config/fish" ]; then
                config_file="~/.config/fish/config.fish"
            else
                config_file="~/.config/fish/config.fish (will need to create directory)"
            fi
            ;;
        zsh)    config_file="~/.zshrc";;
        bash)
            if [[ "$OS" == "darwin" ]]; then
                config_file="~/.bash_profile"
            else
                config_file="~/.bashrc"
            fi
            ;;
        *)      config_file="your shell's config file";;
    esac

    echo "$shell_name:$config_file"
}

# Print shell configuration help
print_shell_config() {
    local install_dir="$1"
    local shell_name="$2"
    local config_file="$3"

    echo -e "\n${BLUE}Installation Directory: ${install_dir}${NC}"
    echo -e "${YELLOW}Note: This directory needs to be added to your PATH${NC}"
    echo -e "\nChoose one of these methods to add it:\n"

    # Method 1: Direct command for immediate use
    echo -e "${BLUE}1. Immediate use (current session):${NC}"
    case "$shell_name" in
        fish)
            echo -e "${GREEN}    fish_add_path $install_dir${NC}"
            ;;
        *)
            echo -e "${GREEN}    export PATH=\"\$PATH:$install_dir\"${NC}"
            ;;
    esac

    # Method 2: Shell configuration file
    echo -e "\n${BLUE}2. Permanent addition:${NC}"
    case "$shell_name" in
        fish)
            echo -e "Add to $config_file:"
            echo -e "${GREEN}    if test -d $install_dir
    fish_add_path $install_dir
end${NC}"
            ;;
        *)
            echo -e "Add to your shell's config file (e.g., $config_file):"
            echo -e "${GREEN}    export PATH=\"\$PATH:$install_dir\"${NC}"
            ;;
    esac

    # Method 3: Nix Home Manager
    echo -e "\n${BLUE}3. Using Nix Home Manager:${NC}"
    echo -e "Add to your Home Manager configuration:"
    echo -e "${GREEN}    home.sessionPath = [
      \"$install_dir\"
    ];${NC}"

    echo -e "\n${YELLOW}After adding the PATH:${NC}"
    echo "• Start a new terminal session, or"
    echo "• Reload your shell configuration"

    echo -e "\n${BLUE}To verify the installation:${NC}"
    echo "• Run: athira --help"
    echo -e "• If it doesn't work, make sure the PATH is properly set: echo \$PATH\n"
}


# Detect OS and architecture
detect_platform() {
    # Detect OS
    case "$(uname -s)" in
        Linux*)     OS=linux;;
        Darwin*)    OS=darwin;;
        MINGW64*)   OS=windows;;
        MSYS*)      OS=windows;;
        *)          error "Unsupported operating system: $(uname -s)";;
    esac

    # Detect architecture
    local arch
    arch=$(uname -m)
    case "$arch" in
        x86_64|amd64) ARCH=x86_64;;
        aarch64|arm64) ARCH=arm64;;
        *) error "Unsupported architecture: $arch";;
    esac

    # Set asset extension
    if [ "$OS" = "windows" ]; then
        EXT=".zip"
    else
        EXT=".tar.gz"
    fi
}

# Get the latest release version
get_latest_version() {
    info "Fetching latest release version..."
    LATEST_VERSION=$(curl -sL ${GITHUB_API}/repos/${REPO_OWNER}/${REPO_NAME}/releases/latest | grep '"tag_name":' | cut -d'"' -f4)
    [ -z "$LATEST_VERSION" ] && error "Failed to fetch latest version"
    info "Latest version: $LATEST_VERSION"
}

# Download and install the binary
install_binary() {
    local asset_name="athira-${OS}-${ARCH}${EXT}"
    local download_url="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/${LATEST_VERSION}/${asset_name}"
    local temp_dir
    temp_dir=$(mktemp -d)
    local install_dir="$HOME/.local/bin"

    info "Downloading $asset_name..."
    curl -sL -o "${temp_dir}/${asset_name}" "$download_url" || error "Failed to download binary"

    # Create install directory and its parent directories
    info "Creating install directory: $install_dir"
    mkdir -p "$install_dir" || error "Failed to create install directory"

    # Extract and install binary
    cd "$temp_dir"
    info "Extracting binary..."
    if [ "$OS" = "windows" ]; then
        unzip -q "${asset_name}" || error "Failed to extract binary"
        mv athira.exe "$install_dir/" || error "Failed to install binary"
    else
        tar xzf "${asset_name}" || error "Failed to extract binary"
        # List contents of temp directory for debugging
        info "Extracted contents:"
        ls -la

        # Try to find the binary
        if [ -f "athira-${OS}-${ARCH}" ]; then
            mv "athira-${OS}-${ARCH}" "$install_dir/athira" || error "Failed to install binary"
        elif [ -f "athira" ]; then
            mv "athira" "$install_dir/athira" || error "Failed to install binary"
        else
            error "Could not find binary after extraction. Contents: $(ls -la)"
        fi

        chmod +x "$install_dir/athira" || error "Failed to set executable permissions"
    fi

    # Clean up
    rm -rf "$temp_dir"

    info "Installation successful!"
    info "Installed to: $install_dir/athira"

    # Check if install directory is in PATH and provide shell-specific instructions
    if [[ ":$PATH:" != *":$install_dir:"* ]]; then
        local shell_info
        shell_info=$(detect_shell)
        local shell_name="${shell_info%%:*}"
        local config_file="${shell_info#*:}"

        print_shell_config "$install_dir" "$shell_name" "$config_file"
    fi
}

main() {
    # Check for required commands
    command -v curl >/dev/null 2>&1 || error "curl is required but not installed"
    command -v tar >/dev/null 2>&1 || error "tar is required but not installed"

    info "Installing athira..."
    detect_platform
    get_latest_version
    install_binary

    info "You can now run 'athira' to use the CLI (after adding it to your PATH if needed)"
}

main
