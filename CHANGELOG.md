# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2023-06-24

### Added

- CLI now has a `render` command
- CLI now has a `--version` option

### Changed

- Contents and Types in modules are now sorted alphabetically

### Fixed

- When releasing, rebuild before opening a PR, to update `Cargo.lock`
- `get_first_child_named` now returns the first child, alphabetically


## [0.1.3] - 2023-06-21

### Fixed

- Releasing now updates the packages versions accordingly

## [0.1.2] - 2023-06-20

### Fixed

- CLI now logs operations appropriately
- Fixed `expect` messages to explain what failed instead of what should have happened

## [0.1.1] - 2023-06-20

### Changed

- Renamed binary from `tools` to `powerd6_cli`

### Fixed

- Identifiers generated for files now use their relative paths to create the correct values

## [0.1.0] - 2023-06-19

### Added

- Rust structs for Modules, Types and Contents
- Mechanism to map sparse file structures into a virtual representation
- Mechanism to convert virtual representation of files into a Module
- Add support for JSON files
- Add support for YAML files
- Add support for plaintext (markdown, hjs, txt) files
- Add CLI to build modules from directories
