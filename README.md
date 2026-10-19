# tui-rating ⭐

[![Crates.io](https://img.shields.io/crates/v/tui-rating.svg)](https://crates.io/crates/tui-rating)
[![Documentation](https://docs.rs/tui-rating/badge.svg)](https://docs.rs/tui-rating)
[![CI](https://github.com/sorinirimies/tui-rating/actions/workflows/ci.yml/badge.svg)](https://github.com/sorinirimies/tui-rating/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A customizable **rating toolkit** for [ratatui](https://ratatui.rs): stars, hearts, emoji and smooth bars, gradients, review histograms, badges, bipolar scales, mouse + keyboard input and animation.

![tui-rating gallery](examples/vhs/output/gallery.gif)

## ✨ Features

**Designs**
- ⭐ `Rating`: any number of items, half steps, **smooth fractional fill** (`3.7` → `███▊░`), 16 symbol presets incl. wide emoji (🔥 🍕 🌙 ⭐), vertical layout
- 📊 `RatingBar`: continuous gauge with block-eighth edges, gradient or colour-by-value
- 🏷️ `RatingBadge`: `[ 4.2 ★ ]`, filled pill, `★ 4.2/5 (1,203)` with vote counts
- 🗳️ `RatingHistogram`: Amazon-style review summary (5★→1★ bars, %, counts, average)
- ↔️ `BipolarRating`: `−n … +n` scales with a centre marker (👎 … 👍, ▼ … ▲)
- 🎭 `RatingStyle` presets: `classic`, `netflix`, `github`, `imdb`, `neon`, `retro`, `ocean`, `sunset`

**Styling**
- 🌈 Gradients (`gradient`, `traffic_light`), per-item `color_fn`, bold/dim modifiers
- 🔢 Value text as fraction / number / percent, label before or after
- 🖌️ Independent styles for filled / half / empty / label / value, `Block`, alignment, gap
- 🚫 `disabled` look and a `focused` indicator
- 🎨 Optional shared theming via the `theme` feature (`tui-nuances`)

**Interaction**
- 🖱️ Mouse: `Rating::value_at(area, x, y)` hit-testing, hover preview, click/drag (feature `crossterm`)
- ⌨️ `RatingState::handle_key`: `←/→` `h/l` `+/-`, `0-9`, `Home`/`End`
- 🎞️ `animate_to(target, ticks)` with easing and a pulse on the item being filled
- 🧱 `StatefulWidget` support via `Rating::stateful()`, optional `serde` for `RatingState`
- 🛡️ Safe with out-of-range / `NaN` values and zero-size areas; ~99% test coverage, property tests

## 📦 Installation

```bash
cargo add tui-rating
```

or in `Cargo.toml`:

```toml
[dependencies]
tui-rating = "0.3"
ratatui = "0.30"
```

Optional features: `theme` (tui-nuances), `crossterm` (key/mouse helpers), `serde`.

```toml
tui-rating = { version = "0.3", features = ["crossterm", "serde"] }
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
| `Rating` | Item-style widget (builder, implements `Widget`; `.stateful()` for `StatefulWidget`) |
| `RatingBar` | Continuous gauge-style bar |
| `RatingBadge` / `BadgeKind` | Compact numeric badge (`Brackets`, `Pill`, `Plain`) with optional votes |
| `RatingHistogram` | Review-summary histogram |
| `BipolarRating` | Negative / positive scale |
| `RatingState` | Value / max / half-step state, hover, animation, key + mouse input |
| `RatingSymbols` | Glyph sets: `stars`, `hearts`, `circles`, `squares`, `diamonds`, `sparkles`, `hexagons`, `bars`, `dots`, `blocks`, `smooth`, `fire`, `pizza`, `moon`, `star_emoji`, `ascii`, or `new(filled, empty).half(..).partials(..)` |
| `RatingStyle` | Bundled look: symbols + colours + modifiers (`Rating::preset`, `RatingBar::preset`) |
| `Easing` / `Animation` | Tick-based value animation |
| `ValueFormat`, `LabelPosition`, `Orientation` | Value text, label placement, layout direction |
| `ItemFill` | `Full` / `Half` / `Partial(pct)` / `Empty`, from `Rating::fill_at` |

### `Rating` builder

`new(value, max)` · `from_state(&state)` · `half_steps` · `fractional` · `symbols` · `preset` · `filled_style/filled_color` · `half_style` · `empty_style/empty_color` · `label` · `label_style` · `label_position` · `show_value` · `value_format` · `value_style` · `gradient` · `traffic_light` · `color_fn` · `modifier` · `empty_modifier` · `highlight` · `highlight_style` · `hover` · `reversed` · `orientation` · `disabled` · `focused` · `focus_style` · `gap` · `alignment` · `block`

Queries: `fill_at` · `display_value` · `value_text` · `width` · `height` · `lines` · `item_at` · `value_at`

### Styling

```rust
use ratatui::style::{Color, Modifier};
use tui_rating::{LabelPosition, Rating, RatingSymbols, ValueFormat};

Rating::new(4.0, 5)
    .symbols(RatingSymbols::hearts())
    .gradient([Color::Rgb(255, 80, 120), Color::Rgb(120, 80, 255)])
    .modifier(Modifier::BOLD)
    .empty_modifier(Modifier::DIM)
    .value_format(ValueFormat::Percent)
    .show_value(true)
    .label("Score")
    .label_position(LabelPosition::After)
    .highlight(Some(3)); // cursor / hover preview
```

Precedence: `color_fn` > explicit `filled_color`/`filled_style` foreground > gradient. Builder calls after `.preset(..)` override the preset.

![styles](examples/vhs/output/styles.gif)

### Presets

```rust
use tui_rating::{Rating, RatingStyle};

let r = Rating::new(4.0, 5).preset(RatingStyle::neon()).show_value(true);
// RatingStyle::all() lists every preset by name; fields are public, tweak freely
```

### Smooth fractional fill, bars, badges, histogram, bipolar

```rust
use tui_rating::*;

// 3.7 of 5 → ███▊░ (needs a symbol set with partial glyphs)
Rating::new(3.7, 5).symbols(RatingSymbols::smooth()).fractional(true).gap(0);

// continuous gauge, 88% in green
RatingBar::new(4.4, 5.0).traffic_light().color_by_value(true)
    .show_value(true).value_format(ValueFormat::Percent);

// [ 4.2 ★ ] (1,203)   |   ★ 4.2/5 (1.2k)
RatingBadge::new(4.2, 5.0).votes(1203);
RatingBadge::new(4.2, 5.0).kind(BadgeKind::Plain).show_max(true).votes(1203).compact_votes(true);

// counts[0] = 1★ … counts[4] = 5★
RatingHistogram::new([42, 51, 120, 610, 1480]).summary(true).traffic_light();

// ○ ● ┃ ○ ○   (-1 of ±2)
BipolarRating::new(-1.0, 2).show_value(true);
```

### Vertical, disabled, focused

```rust
use tui_rating::{Orientation, Rating};

Rating::new(3.0, 5).orientation(Orientation::Vertical); // fills bottom-up
Rating::new(3.0, 5).focused(true);   // [★ ★ ★ ☆ ☆]
Rating::new(3.0, 5).disabled(true);  // dimmed, no gradient/highlight
```

### Mouse, keyboard and animation (`crossterm` feature)

```rust
use ratatui::layout::Rect;
use tui_rating::{Rating, RatingState};

let mut state = RatingState::new(3.0, 5);
let area = Rect::new(0, 0, 20, 1);

// in your event loop:
// Event::Key(k)   => { state.handle_key(k); }
// Event::Mouse(m) => { state.handle_mouse(m, &Rating::from_state(&state), area); }
//
// hover preview + animated fill come for free through from_state:
state.animate_to(5.0, 12);   // eased fill-in over 12 ticks
// each frame / tick: state.tick();
let widget = Rating::from_state(&state); // uses state.visual_value() + hover
```

Without the feature, use `Rating::value_at(area, x, y)` and `state.set_hover(..)` / `set_value(..)` yourself. Render through `StatefulWidget` with `frame.render_stateful_widget(rating.stateful(), area, &mut state)`.

### `RatingState`

`new(value, max)` · `with_half_steps` · `with_read_only` · `value` · `max` · `step` · `set_value` · `set_max` · `increment` · `decrement` · `clear` · `set_full` · `set_items` · `ratio` · `percentage` · `is_empty` · `is_full` · `hover/set_hover` · `animate_to(_with)` · `tick` · `is_animating` · `visual_value` · `pulse_index` · `handle_key` · `handle_mouse`

Half items use the symbol set's `half` glyph when present (circles, squares, ascii). Otherwise the empty glyph is drawn in the filled style (outlined star/heart).

## 🎨 Theming

Optional Cargo feature (off by default, no extra dependency). Enable it:

```toml
tui-rating = { version = "0.3", features = ["theme"] }
```

Backward compatible: without a theme the default look is unchanged.
Colors come from the shared [`tui-nuances`](https://github.com/sorinirimies/tui-nuances) `Theme`
(re-exported as `tui_rating::Theme`).

```rust
use tui_rating::{Rating, Theme, Themed};

let theme = Theme::by_name("nord").unwrap_or_default();
let rating = Rating::new(3.0, 5).theme(&theme).show_value(true);
// or: Rating::new(3.0, 5).apply_theme(&theme) / .themed(&app_theme)
```

Mapping: `accent` → filled/half items, `disabled` → empty items, `foreground` → label, `muted` → value text.
**Precedence:** explicit `filled_style/filled_color/half_style/empty_style/empty_color/label_style/value_style`
always win over the theme, whether called before or after `.theme()`. The `Block` is not themed (style it yourself, e.g. `theme.border_style(true)`).

To pass your app's theme, implement `tui_rating::ThemeProvider` for it (then `.themed(&app_theme)`),
or `impl From<&AppTheme> for Theme` and call `.theme(&app_theme)`.

## 🎬 Examples

Run with `cargo run --example <name>` (or `just example-<name>`).

| Example | Description |
|---------|-------------|
| `gallery` | **Every design** on auto-cycling pages (presets, symbols, smooth, bars, badges, histogram, vertical, bipolar, animation) |
| `mouse` | Click / drag / hover to rate, keyboard, read-only toggle, animation (needs `--features crossterm`) |
| `basic` | Static rating in a bordered block |
| `interactive` | Multiple editable ratings with different symbols; `t`/`T` switch theme (needs `--features theme`) |
| `half_stars` | Half-step ratings, toggle with `t` |
| `symbols` | All symbol presets |
| `styles` | Gradients, percent value, label position, reversed, highlight |
| `custom` | Emoji, RGB colors, gaps, 10-item scale |
| `alignment` | Left / center / right |

![mouse example](examples/vhs/output/mouse.gif)

Demo GIFs live in `examples/vhs/output/` (see [EXAMPLES.md](EXAMPLES.md)).

Common controls: `←/→` (or `h/l`) change value, `↑/↓` select, `0-9` set, `c` clear, `q`/`Esc` quit. See [EXAMPLES.md](EXAMPLES.md).

## 🛠️ Development

Requires [`just`](https://github.com/casey/just) and [`nu`](https://www.nushell.sh).

```bash
just check-all     # fmt + clippy + tests + script tests
just test          # cargo test --all-features
just test-coverage # llvm-cov HTML report
just bench         # criterion benchmarks
just test-scripts  # nushell script tests
just vhs-all       # regenerate demo GIFs (needs vhs)
just bump 0.3.0    # bump version, changelog, tag
```

All automation lives in [`scripts/`](scripts/) and is written in Nushell. CI runs on both GitHub Actions (`.github/workflows`) and Gitea Actions (`.gitea/workflows`). Changelogs are generated with [git-cliff](https://git-cliff.org).

See [CONTRIBUTING.md](CONTRIBUTING.md).

## 🧪 Fuzzing

Fuzz targets live in `fuzz/` (needs nightly + `cargo install cargo-fuzz`):

```sh
cargo +nightly fuzz list
cargo +nightly fuzz run render_rating
```

## 📄 License

MIT © Sorin Albu-Irimies
