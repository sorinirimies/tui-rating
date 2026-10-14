# tui-rating ⭐

[![Crates.io](https://img.shields.io/crates/v/tui-rating.svg)](https://crates.io/crates/tui-rating)
[![Documentation](https://docs.rs/tui-rating/badge.svg)](https://docs.rs/tui-rating)
[![CI](https://github.com/sorinirimies/tui-rating/actions/workflows/ci.yml/badge.svg)](https://github.com/sorinirimies/tui-rating/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A customizable **rating widget** (stars, hearts, circles, …) for [ratatui](https://ratatui.rs) applications, with half-step support, labels, value text, borders and alignment.

![tui-rating demo](examples/vhs/output/tui-rating.gif)

## ✨ Features

- ⭐ Any number of items (5 stars, 10 squares, …)
- ½ Half-step ratings (`3.5 / 5`)
- 🎨 Six symbol presets + fully custom glyphs (emoji supported)
- 🖌️ Independent styles for filled / half / empty items, label and value
- 📐 Left / center / right alignment, configurable gap
- 🔲 Optional `Block` border
- 🧠 `RatingState` with clamping, increment/decrement, ratios
- 🛡️ Safe with out-of-range / `NaN` values and zero-size areas

## 📦 Installation

```bash
cargo add tui-rating
```

or in `Cargo.toml`:

```toml
[dependencies]
tui-rating = "0.1"
ratatui = "0.30"
```

## 🚀 Quick Start

```rust
use ratatui::widgets::Block;
use tui_rating::{Rating, RatingState, RatingSymbols};

let mut state = RatingState::new(3.0, 5).with_half_steps(true);
state.increment(); // 3.5

let rating = Rating::from_state(&state)
    .symbols(RatingSymbols::hearts())
    .label("Quality")
    .show_value(true)
    .block(Block::bordered());

// inside your draw closure:
// frame.render_widget(rating, area);
```

## 🧩 API Overview

| Type | Purpose |
|------|---------|
| `Rating` | The widget (builder style, implements `Widget`) |
| `RatingState` | Value / max / half-step state, input helpers |
| `RatingSymbols` | Glyph sets: `stars`, `hearts`, `circles`, `squares`, `diamonds`, `ascii`, or `new(filled, empty).half(..)` |
| `ItemFill` | `Full` / `Half` / `Empty`, returned by `Rating::fill_at` |

### `Rating` builder

`new(value, max)` · `from_state(&state)` · `half_steps(bool)` · `symbols(..)` · `filled_style/filled_color` · `half_style` · `empty_style/empty_color` · `label` · `label_style` · `show_value` · `value_style` · `gap` · `alignment` · `block`

### `RatingState`

`new(value, max)` · `with_half_steps` · `value` · `max` · `step` · `set_value` · `set_max` · `increment` · `decrement` · `clear` · `set_full` · `set_items` · `ratio` · `percentage` · `is_empty` · `is_full`

Half items use the symbol set's `half` glyph when present (circles, squares, ascii). Otherwise the empty glyph is drawn in the filled style (outlined star/heart).

## 🎬 Examples

Run with `cargo run --example <name>` (or `just example-<name>`).

| Example | Description |
|---------|-------------|
| `basic` | Static rating in a bordered block |
| `interactive` | Multiple editable ratings with different symbols |
| `half_stars` | Half-step ratings, toggle with `t` |
| `symbols` | All symbol presets |
| `custom` | Emoji, RGB colors, gaps, 10-item scale |
| `alignment` | Left / center / right |

Common controls: `←/→` (or `h/l`) change value, `↑/↓` select, `0-9` set, `c` clear, `q`/`Esc` quit. See [EXAMPLES.md](EXAMPLES.md).

## 🛠️ Development

Requires [`just`](https://github.com/casey/just) and [`nu`](https://www.nushell.sh).

```bash
just check-all     # fmt + clippy + tests + script tests
just test          # cargo test
just test-scripts  # nushell script tests
just vhs-all       # regenerate demo GIFs (needs vhs)
just bump 0.2.0    # bump version, changelog, tag
```

All automation lives in [`scripts/`](scripts/) and is written in Nushell. CI runs on both GitHub Actions (`.github/workflows`) and Gitea Actions (`.gitea/workflows`). Changelogs are generated with [git-cliff](https://git-cliff.org).

See [CONTRIBUTING.md](CONTRIBUTING.md).

## 📄 License

MIT © Sorin Albu-Irimies
