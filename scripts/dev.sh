#!/bin/bash
# ============================================================================
# DEVELOPMENT WORKFLOW SCRIPT
# ============================================================================
#
# Quick commands for development workflow:
# ./scripts/dev.sh fmt     - Format code
# ./scripts/dev.sh check   - Check compilation
# ./scripts/dev.sh test    - Run tests
# ./scripts/dev.sh lint    - Run clippy
# ./scripts/dev.sh ci      - Run all CI checks
# ./scripts/dev.sh build   - Build release binary
# ============================================================================

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

# ============================================================================
# COMMANDS
# ============================================================================

cmd_fmt() {
    echo -e "${BLUE}📝 Formatting code...${NC}"
    cargo fmt --all
    echo -e "${GREEN}✅ Code formatted${NC}"
}

cmd_check() {
    echo -e "${BLUE}🔨 Checking compilation...${NC}"
    cargo check --all-targets
    echo -e "${GREEN}✅ Compilation check passed${NC}"
}

cmd_test() {
    echo -e "${BLUE}🧪 Running tests...${NC}"

    if [ -n "$DATABASE_URL" ] || [ -f .env ]; then
        cargo test
    else
        echo -e "${YELLOW}⚠️  No DATABASE_URL found, running unit tests only${NC}"
        cargo test --lib
    fi

    echo -e "${GREEN}✅ Tests passed${NC}"
}

cmd_lint() {
    echo -e "${BLUE}🔍 Running Clippy...${NC}"
    cargo clippy --all-targets --all-features -- -D warnings
    echo -e "${GREEN}✅ No lint warnings${NC}"
}

cmd_ci() {
    echo -e "${BLUE}🚀 Running full CI check...${NC}"

    echo ""
    cmd_fmt
    echo ""

    echo ""
    cmd_lint
    echo ""

    echo ""
    cmd_check
    echo ""

    echo ""
    cmd_test
    echo ""

    echo -e "${GREEN}✨ All CI checks passed!${NC}"
}

cmd_build() {
    echo -e "${BLUE}📦 Building release binary...${NC}"
    cargo build --release
    echo -e "${GREEN}✅ Release binary built: target/release/judiw${NC}"

    # Show binary size
    if [[ "$OSTYPE" == "darwin"* ]]; then
        SIZE=$(du -h target/release/judiw | cut -f1)
        echo -e "${BLUE}📊 Binary size: ${SIZE}${NC}"
    else
        SIZE=$(du -h target/release/judiw | cut -f1)
        echo -e "${BLUE}📊 Binary size: ${SIZE}${NC}"
    fi
}

cmd_install() {
    echo -e "${BLUE}📦 Installing binary...${NC}"
    cargo install --path .
    echo -e "${GREEN}✅ Binary installed: ~/.cargo/bin/judiw${NC}"
}

cmd_clean() {
    echo -e "${BLUE}🧹 Cleaning build artifacts...${NC}"
    cargo clean
    echo -e "${GREEN}✅ Build artifacts cleaned${NC}"
}

cmd_stats() {
    echo -e "${BLUE}📊 Project statistics:${NC}"
    echo ""

    # Count lines of code
    if command -v tokei &> /dev/null; then
        tokei
    else
        echo "Rust source files:"
        find src -name "*.rs" -exec wc -l {} + | tail -1
    fi

    echo ""
    echo "Tests:"
    cargo test --no-run --quiet 2>&1 | grep "running" || echo "  (run 'cargo test' to see test count)"

    echo ""
    echo "Dependencies:"
    cargo tree --depth 1 | grep -c "^[a-z]" || echo "  (run 'cargo tree' to see dependencies)"
}

cmd_help() {
    echo "Judiw Terminal Development Commands"
    echo ""
    echo "Usage: ./scripts/dev.sh <command>"
    echo ""
    echo "Commands:"
    echo "  fmt     Format code with rustfmt"
    echo "  check   Check compilation (faster than build)"
    echo "  test    Run all tests"
    echo "  lint    Run Clippy lints"
    echo "  ci      Run full CI check (fmt + lint + check + test)"
    echo "  build   Build release binary"
    echo "  install Install binary to ~/.cargo/bin"
    echo "  clean   Clean build artifacts"
    echo "  stats   Show project statistics"
    echo "  help    Show this help message"
    echo ""
    echo "Examples:"
    echo "  ./scripts/dev.sh fmt       # Format code"
    echo "  ./scripts/dev.sh ci        # Run all checks"
    echo "  ./scripts/dev.sh build     # Build release"
    echo ""
}

# ============================================================================
# MAIN
# ============================================================================

case "${1:-help}" in
    fmt) cmd_fmt ;;
    check) cmd_check ;;
    test) cmd_test ;;
    lint) cmd_lint ;;
    ci) cmd_ci ;;
    build) cmd_build ;;
    install) cmd_install ;;
    clean) cmd_clean ;;
    stats) cmd_stats ;;
    help|--help|-h) cmd_help ;;
    *)
        echo -e "${RED}Unknown command: $1${NC}"
        echo ""
        cmd_help
        exit 1
        ;;
esac
