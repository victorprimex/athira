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

# Print shell configuration help
print_shell_config() {
    local install_dir="$1"
    local shell_name="$2"
    local config_file="$3"

    echo -e "${BLUE}Detected shell: ${shell_name}${NC}"
    echo -e "${YELLOW}$install_dir is not in your PATH${NC}"
    echo -e "To add it, use one of these methods:\n"

    case "$shell_name" in
        fish)
            echo "1. Run this command:"
            echo -e "${GREEN}    fish_add_path $install_dir${NC}"
            echo
            echo "2. Or add this to ~/.config/fish/config.fish:"
            echo -e "${GREEN}    fish_add_path $install_dir${NC}"
            echo
            echo "3. If using home-manager, add to your configuration:"
            echo -e "${GREEN}    programs.fish = {
      enable = true;
      interactiveShellInit = ''
        fish_add_path $install_dir
      '';
    };${NC}"
            ;;
        zsh)
            echo "1. Add this line to $config_file:"
            echo -e "${GREEN}    export PATH=\"\$PATH:$install_dir\"${NC}"
            echo
            echo "2. If using home-manager, add to your configuration:"
            echo -e "${GREEN}    programs.zsh = {
      enable = true;
      initExtra = ''
        export PATH=\"\$PATH:$install_dir\"
      '';
    };${NC}"
            ;;
        bash)
            echo "1. Add this line to $config_file:"
            echo -e "${GREEN}    export PATH=\"\$PATH:$install_dir\"${NC}"
            echo
            echo "2. If using home-manager, add to your configuration:"
            echo -e "${GREEN}    programs.bash = {
      enable = true;
      initExtra = ''
        export PATH=\"\$PATH:$install_dir\"
      '';
    };${NC}"
            ;;
        *)
            echo "Add this line to your shell's configuration file:"
            echo -e "${GREEN}    export PATH=\"\$PATH:$install_dir\"${NC}"
            ;;
    esac

    echo -e "\nAfter adding, either:\n- Start a new terminal session, or\n- Reload your shell configuration"
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

# Detect user's shell
detect_shell() {
    # First try getting the user's default shell from /etc/passwd
    local shell_path
    if [ -r /etc/passwd ]; then
        shell_path=$(grep "^${USER}:" /etc/passwd | cut -d: -f7)
    fi

    # If that didn't work, try $SHELL
    if [ -z "$shell_path" ]; then
        shell_path="$SHELL"
    fi

    # Extract just the shell name
    local shell_name
    shell_name=$(basename "$shell_path")

    # Determine config file path
    local config_file
    case "$shell_name" in
        fish)   config_file="~/.config/fish/config.fish";;
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
