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

for_reals="false"
skip_clean="false"

while test $# -gt 0; do
  case "$1" in
    --for_reals) for_reals="true" && shift ;;
    --skip_clean) skip_clean="true" && shift ;;
    *) shift ;;
  esac
done

if [[ "${for_reals}" == "true" ]] && [[ "${skip_clean}" == "true" ]]
then
    echo -e "${YELLOW}Can not skip clean step when actually publishing${NC}\n"

    skip_clean="false"
fi

if [[ "${skip_clean}" == "true" ]]
then
  echo -e "${YELLOW}running clean"
    cargo clean
    confirm-success "clean"
fi

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

if [[ "${for_reals}" == "true" ]]
then
  echo -e "${YELLOW}running publish"
  cargo publish --locked --all-features
  confirm-success "publish"
else
  echo -e "${YELLOW}running publish dry run"
  cargo publish --locked --all-features --dry-run
  confirm-success "publish dry run"
fi