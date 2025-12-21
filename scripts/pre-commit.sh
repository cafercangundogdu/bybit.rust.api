#!/bin/sh
set -e

RESET="\033[0m"
RED="\033[31m"
GREEN="\033[32m"
YELLOW="\033[33m"

echo "${YELLOW}[Pre-commit] Starting...${RESET}"

echo "${YELLOW}[1/5] Checking code formatting (cargo fmt)...${RESET}"
cargo fmt -- --check
if [ $? -ne 0 ]; then
    echo "${RED}[ERROR] Code formatting issue. Please run 'cargo fmt'.${RESET}"
    exit 1
fi

echo "${YELLOW}[2/5] Compiling library (cargo check)...${RESET}"
cargo check
if [ $? -ne 0 ]; then
    echo "${RED}[ERROR] Library compilation failed.${RESET}"
    exit 1
fi

echo "${YELLOW}[3/5] Checking examples (cargo check --examples)...${RESET}"
cargo check --examples
if [ $? -ne 0 ]; then
    echo "${RED}[ERROR] Examples failed to compile. Changes might have broken examples.${RESET}"
    exit 1
fi

echo "${YELLOW}[4/5] Checking tests (cargo check --tests)...${RESET}"
cargo check --tests
if [ $? -ne 0 ]; then
    echo "${RED}[ERROR] Test compilation failed.${RESET}"
    exit 1
fi

echo "${YELLOW}[5/5] Running tests (cargo test)...${RESET}"
cargo test
if [ $? -ne 0 ]; then
    echo "${RED}[ERROR] Tests failed.${RESET}"
    exit 1
fi

echo "${GREEN}[SUCCESS] All checks passed. Committing.${RESET}"
exit 0
