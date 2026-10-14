# Changelog
All notable changes to this project will be documented in this file.

## [0.2.0] - 2026-10-14

### Bug Fixes
- Install nushell binary on gitea runner (Node 18 lacks File)
- Install nushell binary in all gitea workflows

### Documentation
- Re-record demos showing results only; reference tui-nuances version

### Features
- Shared theming via tui-nuances
- Make theming an optional "theme" feature

### Miscellaneous
- Depend on tui-nuances 0.1.5
- Depend on tui-nuances 0.1.6
- Depend on tui-nuances 0.1.6 from crates.io

### Ci
- Prebuilt Nushell, cargo-deny + cargo-audit security workflow; fuzzing; tui-nuances 0.1.4

## [0.1.0] - 2026-10-14

### Documentation
- Add README, contributing guide and CI workflows

### Features
- Add Rating widget, RatingState and RatingSymbols with tests
- Add examples and VHS demo tapes

### Miscellaneous
- Add nushell scripts, justfile and git-cliff config
- Prepare v0.1.0

