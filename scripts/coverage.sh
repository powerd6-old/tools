#!/usr/bin/env zsh
if [[ $(rustup component list | grep -qL "llvm") ]]; then
    echo "Installing llvm-tools"
    rustup component add llvm-tools-preview
fi
cargo install grcov
cargo clean
CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test --workspace
echo "Generating LCOV file"
grcov . -s . --binary-path ./target/debug/ -t lcov --branch --ignore-not-existing -o ./target/debug/coverage.lcov
echo "Generating HTML coverage report"
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
rm -rf ./**/*.profraw
open ./target/debug/coverage/index.html