# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- CLI now logs operations appropriately

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