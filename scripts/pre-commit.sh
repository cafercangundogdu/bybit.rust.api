#!/bin/sh
set -e

RESET="\033[0m"
RED="\033[31m"
GREEN="\033[32m"
YELLOW="\033[33m"

echo "${YELLOW}[Pre-commit] Starting...${RESET}"

echo "${YELLOW}[1/6] Checking code formatting (cargo fmt)...${RESET}"
cargo fmt -- --check
if [ $? -ne 0 ]; then
    echo "${RED}[ERROR] Code formatting issue. Run 'cargo fmt'.${RESET}"
    exit 1
fi

echo "${YELLOW}[2/6] Linting (cargo clippy -- -D warnings)...${RESET}"
cargo clippy --all-targets -- -D warnings
if [ $? -ne 0 ]; then
    echo "${RED}[ERROR] Clippy found issues. Fix warnings before committing.${RESET}"
    exit 1
fi

echo "${YELLOW}[3/6] Compiling library (cargo check)...${RESET}"
cargo check
if [ $? -ne 0 ]; then
    echo "${RED}[ERROR] Library compilation failed.${RESET}"
    exit 1
fi

echo "${YELLOW}[4/6] Checking examples (cargo check --examples)...${RESET}"
cargo check --examples
if [ $? -ne 0 ]; then
    echo "${RED}[ERROR] Examples failed to compile.${RESET}"
    exit 1
fi

echo "${YELLOW}[5/6] Checking tests (cargo check --tests)...${RESET}"
cargo check --tests
if [ $? -ne 0 ]; then
    echo "${RED}[ERROR] Test compilation failed.${RESET}"
    exit 1
fi

echo "${YELLOW}[6/6] Running tests (cargo test)...${RESET}"
cargo test
if [ $? -ne 0 ]; then
    echo "${RED}[ERROR] Tests failed.${RESET}"
    exit 1
fi

echo "${GREEN}[SUCCESS] All checks passed. Committing.${RESET}"
exit 0
