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
USE_DOCKER=false

compose_cmd=()

detect_compose() {
    if command -v docker &> /dev/null; then
        if docker compose version &> /dev/null; then
            compose_cmd=(docker compose)
            return 0
        fi
    fi

    if command -v docker-compose &> /dev/null; then
        compose_cmd=(docker-compose)
        return 0
    fi

    return 1
}

run_cargo() {
    local cargo_args=("$@")

    if command -v cargo &> /dev/null; then
        if command -v rtk &> /dev/null; then
            rtk cargo "${cargo_args[@]}"
        else
            cargo "${cargo_args[@]}"
        fi
        return $?
    fi

    if detect_compose; then
        USE_DOCKER=true
        "${compose_cmd[@]}" run --rm ndid-toolchain cargo "${cargo_args[@]}"
        return $?
    fi

    echo -e "${RED}Error: Cargo/Rust is not installed and Docker Compose is unavailable.${NC}"
    echo -e "${YELLOW}Hint: install Rust with 'curl https://sh.rustup.rs -sSf | sh' or install Docker to use the container workflow.${NC}"
    exit 1
}

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

# Action execution
if [ "$RUN_TESTS" = true ]; then
    echo -e "${YELLOW}Running tests...${NC}"
    run_cargo test
elif [ "$CHECK_ONLY" = true ]; then
    echo -e "${YELLOW}Running code check & lints...${NC}"
    run_cargo check && run_cargo clippy -- -D warnings && run_cargo fmt --all -- --check
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}All checks passed!${NC}"
    else
        echo -e "${RED}Checks failed!${NC}"
        exit 1
    fi
else
    # Compile and run gateway
    if ! command -v cargo &> /dev/null; then
        if detect_compose; then
            echo -e "${YELLOW}Cargo/Rust is not installed locally; using Docker Compose for the gateway.${NC}"
            echo -e "${CYAN}Building and starting services...${NC}"
            "${compose_cmd[@]}" up --build ndid-gateway ndid-blockchain ndid-redis
            exit $?
        fi
        echo -e "${RED}Error: Cargo/Rust is not installed and Docker Compose is unavailable.${NC}"
        echo -e "${YELLOW}Hint: install Rust with 'curl https://sh.rustup.rs -sSf | sh' or install Docker to use the container workflow.${NC}"
        exit 1
    fi

    if [ "$RELEASE_MODE" = true ]; then
        echo -e "${GREEN}Starting NDID Gateway in RELEASE mode...${NC}"
        echo -e "${CYAN}Configuration:${NC} $CONFIG_FILE"
        run_cargo run --release --bin ndid-gateway -- --config "$CONFIG_FILE"
    else
        echo -e "${GREEN}Starting NDID Gateway in DEBUG/DEV mode...${NC}"
        echo -e "${CYAN}Configuration:${NC} $CONFIG_FILE"
        run_cargo run --bin ndid-gateway -- --config "$CONFIG_FILE"
    fi
fi
