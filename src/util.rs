//! Internal helpers shared by the widgets.

use ratatui::{layout::Alignment, layout::Rect, style::Color, text::Span};

use crate::ValueFormat;

/// Block-eighth partial glyphs, thinnest first.
pub(crate) fn eighths() -> Vec<String> {
    ["▏", "▎", "▍", "▌", "▋", "▊", "▉"]
        .iter()
        .map(|s| (*s).to_string())
        .collect()
}

/// Cell width of a string.
pub(crate) fn cell_width(s: &str) -> usize {
    Span::raw(s).width()
}

/// Pads `s` with trailing spaces up to `width` cells.
pub(crate) fn pad_to(s: &str, width: usize) -> String {
    let w = cell_width(s);
    let mut out = s.to_string();
    for _ in w..width {
        out.push(' ');
    }
    out
}

/// Samples a color gradient at `t` in `0.0..=1.0`.
///
/// RGB neighbours blend smoothly, other colors step at the midpoint.
pub(crate) fn lerp_gradient(colors: &[Color], t: f64) -> Option<Color> {
    let n = colors.len();
    if n == 0 {
        return None;
    }
    if n == 1 {
        return Some(colors[0]);
    }
    let t = if t.is_finite() {
        t.clamp(0.0, 1.0)
    } else {
        0.0
    };
    let pos = t * (n - 1) as f64;
    let lo = (pos.floor() as usize).min(n - 1);
    let hi = (lo + 1).min(n - 1);
    let f = pos - lo as f64;
    Some(match (colors[lo], colors[hi]) {
        (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
            let m = |a: u8, b: u8| (a as f64 + (b as f64 - a as f64) * f).round() as u8;
            Color::Rgb(m(r1, r2), m(g1, g2), m(b1, b2))
        }
        (a, b) => {
            if f < 0.5 {
                a
            } else {
                b
            }
        }
    })
}

/// Red → yellow → green.
pub(crate) fn traffic_light_colors() -> Vec<Color> {
    vec![
        Color::Rgb(220, 60, 60),
        Color::Rgb(240, 200, 50),
        Color::Rgb(70, 190, 90),
    ]
}

/// X coordinate where a line of `width` cells starts inside `inner`.
pub(crate) fn align_x(inner: Rect, width: u16, alignment: Alignment) -> u16 {
    match alignment {
        Alignment::Left => inner.x,
        Alignment::Center => inner.x + inner.width.saturating_sub(width) / 2,
        Alignment::Right => inner.x + inner.width.saturating_sub(width),
    }
}

/// `3`, `3.5` (one decimal only when fractional).
pub(crate) fn number_text(v: f64) -> String {
    if (v - v.round()).abs() < 1e-9 {
        format!("{}", v.round() as i64)
    } else {
        format!("{v:.1}")
    }
}

/// Formats `v` out of `max` per [`ValueFormat`].
pub(crate) fn format_value(v: f64, max: usize, format: ValueFormat) -> String {
    let num = number_text(v);
    match format {
        ValueFormat::Fraction => format!("{num}/{max}"),
        ValueFormat::Number => num,
        ValueFormat::Percent => {
            let pct = if max == 0 {
                0.0
            } else {
                v / max as f64 * 100.0
            };
            format!("{}%", pct.round() as i64)
        }
    }
}

/// `1203` → `"1,203"`.
pub(crate) fn thousands(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}

/// `1203` → `"1.2k"`, `2_500_000` → `"2.5M"`.
pub(crate) fn compact(n: u64) -> String {
    let (div, suffix) = match n {
        0..=999 => return n.to_string(),
        1_000..=999_999 => (1_000.0, "k"),
        1_000_000..=999_999_999 => (1_000_000.0, "M"),
        _ => (1_000_000_000.0, "B"),
    };
    let v = n as f64 / div;
    let s = format!("{v:.1}");
    format!("{}{suffix}", s.strip_suffix(".0").unwrap_or(&s))
}

/// One bar cell: glyph, whether it is (partly) filled, position `0..=1`.
pub(crate) struct BarCell {
    pub glyph: String,
    pub filled: bool,
    pub t: f64,
}

/// Lays out a continuous bar of `width` cells for `ratio`.
pub(crate) fn bar_cells(
    ratio: f64,
    width: usize,
    filled: &str,
    empty: &str,
    partials: &[String],
) -> Vec<BarCell> {
    let ratio = if ratio.is_finite() {
        ratio.clamp(0.0, 1.0)
    } else {
        0.0
    };
    let exact = ratio * width as f64;
    let full = (exact.floor() as usize).min(width);
    let frac = exact - full as f64;
    let n = partials.len();
    (0..width)
        .map(|i| {
            let t = if width <= 1 {
                0.0
            } else {
                i as f64 / (width - 1) as f64
            };
            let (glyph, is_filled) = if i < full {
                (filled, true)
            } else if i == full && frac > 0.0 && n > 0 {
                let k = (frac * (n + 1) as f64).round() as usize;
                if k == 0 {
                    (empty, false)
                } else if k > n {
                    (filled, true)
                } else {
                    (partials[k - 1].as_str(), true)
                }
            } else {
                (empty, false)
            };
            BarCell {
                glyph: glyph.to_string(),
                filled: is_filled,
                t,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thousands_and_compact() {
        assert_eq!(thousands(0), "0");
        assert_eq!(thousands(999), "999");
        assert_eq!(thousands(1203), "1,203");
        assert_eq!(thousands(1_234_567), "1,234,567");
        assert_eq!(compact(999), "999");
        assert_eq!(compact(1203), "1.2k");
        assert_eq!(compact(2000), "2k");
        assert_eq!(compact(2_500_000), "2.5M");
        assert_eq!(compact(3_000_000_000), "3B");
    }

    #[test]
    fn bar_partial() {
        let cells = bar_cells(0.5, 4, "#", "-", &eighths());
        let s: String = cells.iter().map(|c| c.glyph.as_str()).collect();
        assert_eq!(s, "##--");
        let cells = bar_cells(0.375, 4, "#", "-", &eighths());
        let s: String = cells.iter().map(|c| c.glyph.as_str()).collect();
        assert_eq!(s, "#▌--");
        assert!(cells[1].filled && !cells[2].filled);
        let cells = bar_cells(f64::NAN, 3, "#", "-", &[]);
        assert!(cells.iter().all(|c| !c.filled));
        assert!(bar_cells(1.0, 0, "#", "-", &[]).is_empty());
    }

    #[test]
    fn gradient_edges() {
        assert_eq!(lerp_gradient(&[], 0.5), None);
        assert_eq!(lerp_gradient(&[Color::Red], 0.9), Some(Color::Red));
        assert_eq!(
            lerp_gradient(&[Color::Red, Color::Blue], f64::NAN),
            Some(Color::Red)
        );
    }

    #[test]
    fn align_and_pad() {
        let r = Rect::new(2, 0, 10, 1);
        assert_eq!(align_x(r, 4, Alignment::Left), 2);
        assert_eq!(align_x(r, 4, Alignment::Center), 5);
        assert_eq!(align_x(r, 4, Alignment::Right), 8);
        assert_eq!(align_x(r, 40, Alignment::Right), 2);
        assert_eq!(pad_to("a", 3), "a  ");
        assert_eq!(cell_width("🔥"), 2);
    }
}
