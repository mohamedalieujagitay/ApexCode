#!/bin/bash
# ApexCode Advanced Installer
# This script installs ApexCode on Linux, macOS, and Windows (via WSL)

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\034[0m' # No Color
BOLD='\033[1m'

# Default values
DEFAULT_PREFIX="\${HOME}/.local"
DEFAULT_DATA_DIR="\${HOME}/.local/share/apexcode"
DEFAULT_CONFIG_DIR="\${HOME}/.config/apexcode"
DEFAULT_CACHE_DIR="\${HOME}/.cache/apexcode"

# Print functions with colors
print_header() {
    echo -e "${PURPLE}"
    cat << "EOF"
    /\
   /__\    ApexCode
  /    \   Humanizer for AI Agents & Commits
 /______\  github.com/mohamedalieujagitay/ApexCode
EOF
    echo -e "${NC}"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}!${NC} $1"
}

print_info() {
    echo -e "${BLUE}→${NC} $1"
}

print_header

# Default values
VERSION="latest"
PREFIX="\${DEFAULT_PREFIX}"
DATA_DIR="\${DEFAULT_DATA_DIR}"
CONFIG_DIR="\${DEFAULT_CONFIG_DIR}"
CACHE_DIR="\${DEFAULT_CACHE_DIR}"
FORCE=false
SKIP_DEPS=false
VERBOSE=false
PLATFORM=""
ARCHITECTURE=""

# Parse arguments
while [[ \$# -gt 0 ]]; do
    case \$1 in
        --version)
            VERSION="\$2"
            shift 2
            ;;
        --prefix)
            PREFIX="\$2"
            shift 2
            ;;
        --data-dir)
            DATA_DIR="\$2"
            shift 2
            ;;
        --config-dir)
            CONFIG_DIR="\$2"
            shift 2
            ;;
        --cache-dir)
            CACHE_DIR="\$2"
            shift 2
            ;;
        --force)
            FORCE=true
            shift
            ;;
        --skip-deps)
            SKIP_DEPS=true
            shift
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --help|-h)
            show_help
            exit 0
            ;;
        *)
            print_error "Unknown option: \$1"
            show_help
            exit 1
            ;;
    esac
done

# Detect platform
detect_platform() {
    case "\$(uname -s)" in
        Linux*)     PLATFORM="linux" ;;
        Darwin*)    PLATFORM="macos" ;;
        MINGW*|MSYS*|CYGWIN*)
            PLATFORM="windows"
            ;;
        *)
            PLATFORM="unknown"
            ;;
    esac
}

detect_architecture() {
    case "\$(uname -m)" in
        x86_64|amd64)  ARCHITECTURE="x86_64" ;;
        aarch64|arm64) ARCHITECTURE="aarch64" ;;
        armv7l)        ARCHITECTURE="armv7" ;;
        i386|i686)     ARCHITECTURE="i686" ;;
        *)
            ARCHITECTURE="unknown"
            ;;
    esac
}

# Show help
show_help() {
    cat << EOF
${CYAN}ApexCode Advanced Installer${NC}

${GREEN}Usage:${NC}
    ./install.sh [options]
    curl -sSL https://mohamedalieujagitay.github.io/ApexCode/install.sh | bash [options]

${GREEN}Options:${NC}
    --version <version>        Install specific version (default: latest)
    --prefix <path>           Install to custom prefix (default: ~/.local)
    --data-dir <path>        Data directory (default: ~/.local/share/apexcode)
    --config-dir <path>       Config directory (default: ~/.config/apexcode)
    --cache-dir <path>        Cache directory (default: ~/.cache/apexcode)
    --force                  Force reinstall even if already installed
    --skip-deps              Skip dependency installation
    --verbose                Show detailed output
    --help                   Show this help message

${GREEN}Examples:${NC}
    ./install.sh --version 0.1.0
    ./install.sh --prefix /usr/local
    curl -sSL https://mohamedalieujagitay.github.io/ApexCode/install.sh | bash -s -- --version 0.1.0

${YELLOW}For more information, visit: https://mohamedalieujagitay.github.io/ApexCode/${NC}
EOF
}

# Main installation function
main() {
    print_header
    detect_platform
    detect_architecture

    print_info "Detected platform: \${PLATFORM}-\${ARCHITECTURE}"

    # Check dependencies
    if [ "\${SKIP_DEPS}" = false ]; then
        check_dependencies
    fi

    # Check existing installation
    if [ "\${FORCE}" = false ]; then
        check_existing_installation
    fi

    # Get version
    get_version

    # Download and install
    download_and_install

    # Setup shell integration
    setup_shell_integration

    # Print post-install info
    print_post_install
}

# Run main
main