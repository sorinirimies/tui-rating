# Examples

Run any example with `cargo run --example <name>` or `just example-<name>`.
Demo GIFs are generated with [VHS](https://github.com/charmbracelet/vhs) via `just vhs-all`.

## gallery
Every design on auto-cycling pages: presets, symbols, smooth fractional fill, bars, badges, review histogram, vertical, bipolar, disabled/focused and animation. `→`/`Tab` next, `←` previous, `space` pause.
```bash
cargo run --release --example gallery
```
![gallery](examples/vhs/output/gallery.gif)

## mouse
Click or drag to rate, hover to preview, `Tab`/`↑↓` move focus, `←/→` `0-9` keys, `r` toggles read-only. Clicks animate the fill. (The GIF shows the keyboard path; VHS cannot record the mouse.)
```bash
cargo run --features crossterm --example mouse
```
![mouse](examples/vhs/output/mouse.gif)

## basic
Static 4/5 rating with label, value text and border.
```rust
Rating::from_state(&RatingState::new(4.0, 5))
    .label("Rating").show_value(true)
    .block(Block::bordered());
```
![basic](examples/vhs/output/basic.gif)

## interactive
Five editable ratings (stars, hearts, circles with half steps, diamonds, 10 squares).
Controls: `↑/↓` select, `←/→` change, `0-9` set, `c` clear, `q` quit.

![interactive](examples/vhs/output/interactive.gif)

## half_stars
Half-step ratings in three styles. `t` toggles half steps.
```rust
let mut state = RatingState::new(2.5, 5).with_half_steps(true);
```
![half_stars](examples/vhs/output/half_stars.gif)

## symbols
All presets: `stars`, `hearts`, `circles`, `squares`, `diamonds`, `ascii`.

![symbols](examples/vhs/output/symbols.gif)

## styles
Gradients, percent value, label after, reversed fill, cursor highlight and dim empties.
```rust
Rating::new(4.0, 5).traffic_light().show_value(true);
```
![styles](examples/vhs/output/styles.gif)

## custom
Custom glyphs (`RatingSymbols::new("▰", "▱")`), RGB colors, gaps and a 10-item scale.

![custom](examples/vhs/output/custom.gif)

## alignment
`Alignment::Left`, `Center`, `Right`.

![alignment](examples/vhs/output/alignment.gif)
