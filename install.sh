#!/usr/bin/env bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Package information
PACKAGE_NAME="athira"
REPO_OWNER="victorprimex"
REPO_NAME="athira"
GITHUB_API="https://api.github.com"
CARGO_PACKAGE="athira"
NPM_PACKAGE="athira"
PYPI_PACKAGE="athira"

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

# Check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Print installation methods
print_installation_methods() {
    echo -e "\n${BLUE}Available Installation Methods:${NC}"

    # Cargo (Rust)
    echo -e "\n${BLUE}Using Cargo (Rust):${NC}"
    echo -e "${GREEN}    cargo install $CARGO_PACKAGE${NC}"

    # NPM (Node.js)
    echo -e "\n${BLUE}Using NPM (Node.js):${NC}"
    echo -e "${GREEN}    npm install -g $NPM_PACKAGE${NC}"

    # PyPI (Python)
    echo -e "\n${BLUE}Using PyPI (Python):${NC}"
    echo -e "${GREEN}    pip install $PYPI_PACKAGE${NC}"

    # Binary Installation
    echo -e "\n${BLUE}Using this install script (Binary):${NC}"
    echo -e "${GREEN}    curl -sSL https://raw.githubusercontent.com/$REPO_OWNER/$REPO_NAME/main/install.sh | bash${NC}"

    # Manual Download
    echo -e "\n${BLUE}Manual Download:${NC}"
    echo -e "Visit: ${GREEN}https://github.com/$REPO_OWNER/$REPO_NAME/releases/
