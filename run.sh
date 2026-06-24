#!/bin/bash

# NDID High-Performance Blockchain Banking System Run Script
# This script helps compile, run, and test the banking gateway.

# Colors for premium console output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Print header
echo -e "${BLUE}=======================================================${NC}"
echo -e "${CYAN}   NDID High-Performance Blockchain Banking System     ${NC}"
echo -e "${BLUE}=======================================================${NC}"

# Default values
CONFIG_FILE="config/default.toml"
RELEASE_MODE=false
RUN_TESTS=false
CHECK_ONLY=false

# Helper menu
show_help() {
    echo "Usage: ./run.sh [options]"
    echo ""
    echo "Options:"
    echo "  -c, --config <path>    Specify configuration file path (default: config/default.toml)"
    echo "  -r, --release          Run or build in release mode"
    echo "  -t, --test             Run cargo test suite"
    echo "  -k, --check            Run cargo check, clippy, and format checks"
    echo "  -h, --help             Show this help message"
    echo ""
    echo "Examples:"
    echo "  ./run.sh               Run the gateway in debug mode"
    echo "  ./run.sh -r            Run the gateway in release mode"
    echo "  ./run.sh -t            Run all integration and unit tests"
    echo "  ./run.sh -k            Run syntax, linter, and format checks"
}

# Parse arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        -c|--config) CONFIG_FILE="$2"; shift ;;
        -r|--release) RELEASE_MODE=true ;;
        -t|--test) RUN_TESTS=true ;;
        -k|--check) CHECK_ONLY=true ;;
        -h|--help) show_help; exit 0 ;;
        *) echo -e "${RED}Unknown parameter: $1${NC}"; show_help; exit 1 ;;
    esac
    shift
done

# Check if Rust/Cargo is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Cargo/Rust is not installed. Please install Rust first.${NC}"
    exit 1
fi

# Action execution
if [ "$RUN_TESTS" = true ]; then
    echo -e "${YELLOW}Running tests...${NC}"
    if command -v rtk &> /dev/null; then
        rtk cargo test
    else
        cargo test
    fi
elif [ "$CHECK_ONLY" = true ]; then
    echo -e "${YELLOW}Running code check & lints...${NC}"
    if command -v rtk &> /dev/null; then
        rtk cargo check && rtk cargo clippy -- -D warnings && rtk cargo fmt --all -- --check
    else
        cargo check && cargo clippy -- -D warnings && cargo fmt --all -- --check
    fi
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}All checks passed!${NC}"
    else
        echo -e "${RED}Checks failed!${NC}"
        exit 1
    fi
else
    # Compile and run gateway
    if [ "$RELEASE_MODE" = true ]; then
        echo -e "${GREEN}Starting NDID Gateway in RELEASE mode...${NC}"
        echo -e "${CYAN}Configuration:${NC} $CONFIG_FILE"
        if command -v rtk &> /dev/null; then
            rtk cargo run --release --bin ndid-gateway -- --config "$CONFIG_FILE"
        else
            cargo run --release --bin ndid-gateway -- --config "$CONFIG_FILE"
        fi
    else
        echo -e "${GREEN}Starting NDID Gateway in DEBUG/DEV mode...${NC}"
        echo -e "${CYAN}Configuration:${NC} $CONFIG_FILE"
        if command -v rtk &> /dev/null; then
            rtk cargo run --bin ndid-gateway -- --config "$CONFIG_FILE"
        else
            cargo run --bin ndid-gateway -- --config "$CONFIG_FILE"
        fi
    fi
fi
