#!/usr/bin/env zsh
cargo install grcov
cargo clean
CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test --workspace
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
rm -rf ./**/*.profraw
open ./target/debug/coverage/index.html