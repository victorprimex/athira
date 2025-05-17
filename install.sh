#!/usr/bin/env bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
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

    # Create install directory if it doesn't exist
    mkdir -p "$install_dir"

    # Extract and install binary
    cd "$temp_dir"
    if [ "$OS" = "windows" ]; then
        unzip -q "${asset_name}" || error "Failed to extract binary"
        mv athira.exe "$install_dir/" || error "Failed to install binary"
    else
        tar xzf "${asset_name}" || error "Failed to extract binary"
        mv athira "$install_dir/" || error "Failed to install binary"
        chmod +x "$install_dir/athira" || error "Failed to set executable permissions"
    fi

    # Clean up
    rm -rf "$temp_dir"

    info "Installation successful!"
    info "Installed to: $install_dir/athira"

    # Check if install directory is in PATH
    if [[ ":$PATH:" != *":$install_dir:"* ]]; then
        warn "$install_dir is not in your PATH"
        echo "Add the following line to your shell's config file (.bashrc, .zshrc, etc.):"
        echo "  export PATH=\"\$PATH:$install_dir\""
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

    info "You can now run 'athira' to use the CLI"
}

main
