#!/bin/bash

IFS='/' read -ra script_parts <<< "$0"

NC='\033[0m'
RED='\033[0;31m'
YELLOW='\033[1;33m'

if [[ "$0" == "./${script_parts[-1]}" ]]
then
  echo -e "\n${YELLOW}Scripts need to be run from the project root${NC}, i.e. ./scripts-ps/${script_parts[-1]}\n"
  exit 1
fi

function confirm-success() {
    exit_code=$?
    if [ $exit_code -ne 0 ]
    then
        echo -e "\n$1 ${RED}failed${NC} with exitcode ${exit_code}\n"
        exit $exit_code
    fi
}

clear
echo -e "${YELLOW}running clean"
cargo clean
confirm-success "clean"

echo -e "${YELLOW}running test all unoptimized"
cargo test --features="all" -- --nocapture --test-threads=1
confirm-success "test all optimized"

echo -e "${YELLOW}running test all optimized"
cargo test --features="all" --release -- --nocapture --test-threads=1
confirm-success "test all optimized"

echo -e "${YELLOW}running test all-debug unoptimized"
cargo test --features="all-debug" -- --nocapture --test-threads=1
confirm-success "test all-debug optimized"

echo -e "${YELLOW}running test all-debug optimized"
cargo test --features="all-debug" --release -- --nocapture --test-threads=1
confirm-success "test all-debug optimized"