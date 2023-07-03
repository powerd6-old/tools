#!/usr/bin/env zsh
cargo doc --workspace --no-deps
open ./target/doc/powerd6_cli/index.html