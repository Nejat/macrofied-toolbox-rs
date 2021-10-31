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
echo -e "${YELLOW}running clippy option unoptimized"
cargo clippy --features="option"
confirm-success "clippy option unoptimized"

echo -e "${YELLOW}running clippy option optimized"
cargo clippy --release --features="option"
confirm-success "clippy option optimized"

echo -e "${YELLOW}running clippy option-debug unoptimized"
cargo clippy --features="option-debug"
confirm-success "clippy option-debug unoptimized"

echo -e "${YELLOW}running clippy option-debug optimized"
cargo clippy --release --features="option-debug"
confirm-success "clippy option-debug optimized"

echo -e "${YELLOW}running clippy result unoptimized"
cargo clippy --features="result"
confirm-success "clippy result unoptimized"

echo -e "${YELLOW}running clippy result optimized"
cargo clippy --release --features="result"
confirm-success "clippy result optimized"

echo -e "${YELLOW}running clippy result-debug unoptimized"
cargo clippy --features="result-debug"
confirm-success "clippy result-debug unoptimized"

echo -e "${YELLOW}running clippy result-debug optimized"
cargo clippy --release --features="result-debug"
confirm-success "clippy result-debug optimized"
