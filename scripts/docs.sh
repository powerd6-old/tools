#!/usr/bin/env zsh
cargo doc --workspace --no-deps --document-private-items
open ./target/doc/powerd6_cli/index.html