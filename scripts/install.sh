#!/usr/bin/env bash
#
# ApexCode Installer
# Adversarial Stylometry Framework for the AI-Augmented Developer
#
# Usage: curl -sSL https://mohamedalieujagitay.github.io/ApexCode/install.sh | bash
#        or: ./install.sh [options]
#
# Options:
#   --version <version>    Install specific version
#   --prefix <path>         Install to custom prefix (default: ~/.local)
#   --force                 Force reinstall even if already installed
#   --no-deps               Skip dependency installation
#   --help                  Show this help message

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
REPO="mohamedalieujagitay/ApexCode"
DEFAULT_VERSION="latest"
DEFAULT_PREFIX="${HOME}/.local"
INSTALL_DIR="${DEFAULT_PREFIX}/bin"
CONFIG_DIR="${HOME}/.config/apexcode"
DATA_DIR="${HOME}/.local/share/apexcode"

# Parse arguments
VERSION="${DEFAULT_VERSION}"
PREFIX="${DEFAULT_PREFIX}"
FORCE=false
SKIP_DEPS=false

show_help() {
    cat << EOF
${CYAN}ApexCode Installer${NC}

${GREEN}Usage:${NC}
    curl -sSL https://mohamedalieujagitay.github.io/ApexCode/install.sh | bash
    ./install.sh [options]

${GREEN}Options:${NC}
    --version <version>    Install specific version (default: latest)
    --prefix <path>        Install to custom prefix (default: ~/.local)
    --force                Force reinstall even if already installed
    --no-deps              Skip dependency installation
    --help                 Show this help message

${GREEN}Examples:${NC}
    ./install.sh --version 0.1.0
    ./install.sh --prefix /usr/local
    curl -sSL https://mohamedalieujagitay.github.io/ApexCode/install.sh | bash -s -- --version 0.1.0

${YELLOW}For more information, visit: https://mohamedalieujagitay.github.io/ApexCode/${NC}
EOF
}

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

detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"

    case "${OS}" in
        Linux*)  OS="linux" ;;
        Darwin*) OS="macos" ;;
        MINGW*|MSYS*|CYGWIN*)
            OS="windows"
            ;;
        *)
            print_error "Unsupported OS: ${OS}"
            exit 1
            ;;
    esac

    case "${ARCH}" in
        x86_64|amd64)  ARCH="x86_64" ;;
        aarch64|arm64) ARCH="aarch64" ;;
        armv7l)        ARCH="armv7" ;;
        i386|i686)     ARCH="i686" ;;
        *)
            print_error "Unsupported architecture: ${ARCH}"
            exit 1
            ;;
    esac

    print_info "Detected platform: ${OS}-${ARCH}"
}

check_dependencies() {
    if [ "${SKIP_DEPS}" = true ]; then
        print_warning "Skipping dependency check"
        return
    fi

    print_info "Checking dependencies..."

    local missing_deps=()

    # Check for curl
    if ! command -v curl &> /dev/null; then
        missing_deps+=("curl")
    fi

    # Check for tar
    if ! command -v tar &> /dev/null; then
        missing_deps+=("tar")
    fi

    if [ ${#missing_deps[@]} -gt 0 ]; then
        print_error "Missing dependencies: ${missing_deps[*]}"
        echo ""
        print_info "Install them using:"
        case "${OS}" in
            linux)
                if command -v apt-get &> /dev/null; then
                    echo "  sudo apt-get install ${missing_deps[*]}"
                elif command -v yum &> /dev/null; then
                    echo "  sudo yum install ${missing_deps[*]}"
                elif command -v pacman &> /dev/null; then
                    echo "  sudo pacman -S ${missing_deps[*]}"
                fi
                ;;
            macos)
                echo "  brew install ${missing_deps[*]}"
                ;;
        esac
        exit 1
    fi

    print_success "All dependencies found"
}

check_existing_installation() {
    if [ "${FORCE}" = true ]; then
        print_warning "Force reinstall enabled"
        return
    fi

    if [ -f "${INSTALL_DIR}/apexcode" ]; then
        local current_version
        current_version=$("${INSTALL_DIR}/apexcode" --version 2>/dev/null || echo "unknown")
        print_warning "ApexCode is already installed at ${INSTALL_DIR}"
        print_info "Current version: ${current_version}"
        echo ""
        print_info "To reinstall, use: ./install.sh --force"
        exit 0
    fi
}

get_latest_version() {
    if [ "${VERSION}" != "latest" ]; then
        echo "${VERSION}"
        return
    fi

    print_info "Fetching latest version..."
    VERSION=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

    if [ -z "${VERSION}" ]; then
        print_error "Failed to fetch latest version"
        exit 1
    fi

    print_success "Latest version: ${VERSION}"
}

download_binary() {
    local binary_name="apexcode-${OS}-${ARCH}"
    local download_url="https://github.com/${REPO}/releases/download/${VERSION}/${binary_name}"

    print_info "Downloading from ${download_url}..."

    local temp_dir
    temp_dir=$(mktemp -d)

    if ! curl -fsSL "${download_url}" -o "${temp_dir}/apexcode"; then
        print_error "Failed to download binary"
        rm -rf "${temp_dir}"
        exit 1
    fi

    chmod +x "${temp_dir}/apexcode"

    # Verify binary
    if ! "${temp_dir}/apexcode" --version &> /dev/null; then
        print_error "Downloaded binary is not valid"
        rm -rf "${temp_dir}"
        exit 1
    fi

    echo "${temp_dir}"
}

install_binary() {
    local temp_dir="$1"

    print_info "Installing to ${INSTALL_DIR}..."

    # Create install directory
    mkdir -p "${INSTALL_DIR}"

    # Copy binary
    cp "${temp_dir}/apexcode" "${INSTALL_DIR}/apexcode"
    chmod +x "${INSTALL_DIR}/apexcode"

    # Create config directory
    mkdir -p "${CONFIG_DIR}"

    # Create data directory
    mkdir -p "${DATA_DIR}"

    # Clean up
    rm -rf "${temp_dir}"

    print_success "Installed successfully"
}

setup_shell_integration() {
    local shellrc=""
    local shell_name=""

    # Detect shell
    if [ -n "${ZSH_VERSION}" ]; then
        shellrc="${HOME}/.zshrc"
        shell_name="zsh"
    elif [ -n "${BASH_VERSION}" ]; then
        shellrc="${HOME}/.bashrc"
        shell_name="bash"
    elif [ -f "${HOME}/.zshrc" ]; then
        shellrc="${HOME}/.zshrc"
        shell_name="zsh"
    elif [ -f "${HOME}/.bashrc" ]; then
        shellrc="${HOME}/.bashrc"
        shell_name="bash"
    else
        print_warning "Could not detect shell, skipping shell integration"
        return
    fi

    # Check if already in PATH
    if echo ":${PATH}:" | grep -q ":${INSTALL_DIR}:"; then
        print_success "${INSTALL_DIR} already in PATH"
        return
    fi

    print_info "Adding ${INSTALL_DIR} to PATH in ${shellrc}..."

    # Add to shellrc
    echo "" >> "${shellrc}"
    echo "# ApexCode" >> "${shellrc}"
    echo "export PATH=\"${INSTALL_DIR}:\${PATH}\"" >> "${shellrc}"

    print_success "Added to PATH. Restart your shell or run:"
    echo ""
    echo "  export PATH=\"${INSTALL_DIR}:\${PATH}\""
}

print_post_install() {
    echo ""
    echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}  Installation complete!${NC}"
    echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
    echo ""
    print_info "Binary location: ${INSTALL_DIR}/apexcode"
    print_info "Config directory: ${CONFIG_DIR}"
    print_info "Data directory: ${DATA_DIR}"
    echo ""
    print_info "Quick start:"
    echo ""
    echo "  apexcode              # Launch dashboard (default command)"
    echo "  apexcode scan         # Scan for AI patterns"
    echo "  apexcode humanize    # Humanize code"
    echo "  apexcode score        # Check stealth score"
    echo "  apexcode dashboard   # Launch TUI dashboard"
    echo ""
    print_info "For more information:"
    echo ""
    echo "  apexcode --help"
    echo "  https://github.com/${REPO}"
    echo ""
}

main() {
    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --version)
                VERSION="$2"
                shift 2
                ;;
            --prefix)
                PREFIX="$2"
                INSTALL_DIR="${PREFIX}/bin"
                shift 2
                ;;
            --force)
                FORCE=true
                shift
                ;;
            --no-deps)
                SKIP_DEPS=true
                shift
                ;;
            --help|-h)
                show_help
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done

    # Override with environment variables
    VERSION="${APEXCODE_VERSION:-${VERSION}}"
    PREFIX="${APEXCODE_PREFIX:-${PREFIX}}"
    INSTALL_DIR="${PREFIX}/bin"
    FORCE="${APEXCODE_FORCE:-${FORCE}}"

    # Print header
    print_header

    # Detect platform
    detect_platform

    # Check dependencies
    check_dependencies

    # Check existing installation
    check_existing_installation

    # Get version
    get_latest_version

    # Download binary
    local temp_dir
    temp_dir=$(download_binary)

    # Install binary
    install_binary "${temp_dir}"

    # Setup shell integration
    setup_shell_integration

    # Print post-install info
    print_post_install
}

# Run main function
main "$@"