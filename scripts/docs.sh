#!/usr/bin/env zsh
cargo clean
cargo doc --workspace --no-deps
open ./target/doc/tools/index.html