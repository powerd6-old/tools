#!/usr/bin/env zsh
cargo doc --workspace --no-deps
open ./target/doc/tools/index.html