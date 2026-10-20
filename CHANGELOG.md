# Changelog
All notable changes to this project will be documented in this file.

## [Unreleased]

### Features
- New widgets: `NpsRating` / `NpsSummary`, `ThumbsRating` / `ThumbsState`, `LikertScale` / `LikertState`, `SentimentFaces`, `RatingRange` / `RangeState`, `MultiCriteria` / `RatingForm`, `RatingTrend` / `RatingDelta`, `RatingSlider` / `SliderState` (numeric entry), `RatingCompare`
- `RatingState`: single-level `undo`, `describe()`, click-to-clear, multi-digit key entry (`1` `0` → 10), `Backspace`/`Delete` clear, `handle_key_change` returning `Change { old, new }`, scroll-wheel in `handle_mouse`
- `Rating::monochrome` (no-colour / high-contrast mode) and `Rating::description` (screen-reader text)
- `RatingStyle::custom` builder (`filled_color`, `empty_color`, `gradient`, `gap`, …)
- `survey` example, two new gallery pages, faster animated gallery GIF

## [0.3.0] - 2026-10-19

### Features
- Smooth fill, emoji, bars, badges, histogram, bipolar, presets, mouse/keys, animation

## [0.2.1] - 2026-10-19

### Features
- Add gradients, value formats, label position, highlight, reversed fill and 5 symbol presets

### Miscellaneous
- Skip unconfigured remotes in *-all recipes
- Push/pull current branch instead of hardcoded main
- Align git/release/vhs recipes with pi-tokenburn
- Bump version to 0.2.1

### Ci
- Install Rust toolchain via action (rustup missing on runner)

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
- Bump version to 0.2.0

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

