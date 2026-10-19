# Examples

Examples for the `tui-rating` library. Full docs with GIFs: [EXAMPLES.md](../EXAMPLES.md).

```bash
cargo run --example interactive
```

| Example | Description |
|---------|-------------|
| `gallery.rs` | Every design, auto-cycling pages |
| `mouse.rs` | Mouse + keyboard + animation (`--features crossterm`) |
| `basic.rs` | Static rating |
| `interactive.rs` | Editable ratings with different symbols |
| `half_stars.rs` | Half-step ratings |
| `symbols.rs` | All symbol presets |
| `styles.rs` | Gradients, value formats, label position, highlight |
| `custom.rs` | Custom glyphs, RGB colors, gaps |
| `alignment.rs` | Left / center / right |

## Controls

- **↑/↓** or **j/k** select, **←/→** or **h/l** change, **0-9** set, **c** clear, **q**/**Esc** quit

## Regenerating demos

```bash
just vhs-all                      # all tapes
nu scripts/generate_all_tapes.nu  # same, directly
```
