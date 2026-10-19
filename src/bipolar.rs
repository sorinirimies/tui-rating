//! [`BipolarRating`]: a scale with a negative and a positive side.

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Widget},
};

use crate::util::{cell_width, pad_to};

/// A bipolar scale such as `−2 … +2` or 👎 … 👍: items fill outwards from a
/// centre marker, in a negative colour on the left and a positive colour on
/// the right.
///
/// ```text
/// ○ ● ┃ ○ ○     value -1 of ±2
/// ○ ○ ┃ ● ●     value +2 of ±2
/// ```
///
/// # Example
///
/// ```
/// use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
/// use tui_rating::BipolarRating;
///
/// let r = BipolarRating::new(-1.0, 2).show_value(true);
/// assert_eq!(r.value_text(), "-1");
/// let area = Rect::new(0, 0, 20, 1);
/// let mut buf = Buffer::empty(area);
/// r.render(area, &mut buf);
/// ```
#[derive(Debug, Clone)]
pub struct BipolarRating<'a> {
    value: i64,
    range: usize,
    negative: (String, String),
    positive: (String, String),
    center: String,
    negative_style: Style,
    positive_style: Style,
    empty_style: Style,
    center_style: Style,
    value_style: Style,
    label: Option<String>,
    label_style: Style,
    show_value: bool,
    gap: u16,
    alignment: Alignment,
    block: Option<Block<'a>>,
}

impl<'a> BipolarRating<'a> {
    /// Creates a scale from `-range` to `+range`. The value is rounded and
    /// clamped; non-finite values become `0`.
    pub fn new(value: f64, range: usize) -> Self {
        let value = if value.is_finite() {
            (value.round() as i64).clamp(-(range as i64), range as i64)
        } else {
            0
        };
        Self {
            value,
            range,
            negative: ("●".into(), "○".into()),
            positive: ("●".into(), "○".into()),
            center: "┃".into(),
            negative_style: Style::default().fg(Color::Red),
            positive_style: Style::default().fg(Color::Green),
            empty_style: Style::default().fg(Color::DarkGray),
            center_style: Style::default().fg(Color::Gray),
            value_style: Style::default(),
            label: None,
            label_style: Style::default(),
            show_value: false,
            gap: 1,
            alignment: Alignment::Left,
            block: None,
        }
    }

    /// Glyphs `(filled, empty)` for the negative side.
    #[must_use]
    pub fn negative_symbols(mut self, filled: impl Into<String>, empty: impl Into<String>) -> Self {
        self.negative = (filled.into(), empty.into());
        self
    }

    /// Glyphs `(filled, empty)` for the positive side.
    #[must_use]
    pub fn positive_symbols(mut self, filled: impl Into<String>, empty: impl Into<String>) -> Self {
        self.positive = (filled.into(), empty.into());
        self
    }

    /// 👎 on the left, 👍 on the right (wide emoji, `・` for empty).
    #[must_use]
    pub fn thumbs(self) -> Self {
        self.negative_symbols("👎", "・")
            .positive_symbols("👍", "・")
    }

    /// `▼` on the left, `▲` on the right.
    #[must_use]
    pub fn arrows(self) -> Self {
        self.negative_symbols("▼", "▽").positive_symbols("▲", "△")
    }

    /// Centre marker glyph (default `┃`).
    #[must_use]
    pub fn center(mut self, center: impl Into<String>) -> Self {
        self.center = center.into();
        self
    }

    /// Style of filled negative items (default red).
    #[must_use]
    pub fn negative_style(mut self, style: Style) -> Self {
        self.negative_style = style;
        self
    }

    /// Style of filled positive items (default green).
    #[must_use]
    pub fn positive_style(mut self, style: Style) -> Self {
        self.positive_style = style;
        self
    }

    /// Style of empty items.
    #[must_use]
    pub fn empty_style(mut self, style: Style) -> Self {
        self.empty_style = style;
        self
    }

    /// Style of the centre marker.
    #[must_use]
    pub fn center_style(mut self, style: Style) -> Self {
        self.center_style = style;
        self
    }

    /// Label before the scale.
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Label style.
    #[must_use]
    pub fn label_style(mut self, style: Style) -> Self {
        self.label_style = style;
        self
    }

    /// Shows the signed value (`+2`, `-1`, `0`) after the scale.
    #[must_use]
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Style of the value text.
    #[must_use]
    pub fn value_style(mut self, style: Style) -> Self {
        self.value_style = style;
        self
    }

    /// Spaces between items (default 1).
    #[must_use]
    pub fn gap(mut self, gap: u16) -> Self {
        self.gap = gap;
        self
    }

    /// Alignment of the line.
    #[must_use]
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Wraps the widget in a block.
    #[must_use]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// The (clamped, rounded) value.
    pub fn value(&self) -> i64 {
        self.value
    }

    /// Signed value text: `+2`, `-1`, `0`.
    pub fn value_text(&self) -> String {
        match self.value {
            0 => "0".to_string(),
            v if v > 0 => format!("+{v}"),
            v => v.to_string(),
        }
    }

    /// Builds the styled line.
    pub fn line(&self) -> Line<'a> {
        let width = [&self.negative, &self.positive]
            .iter()
            .flat_map(|(a, b)| [cell_width(a), cell_width(b)])
            .max()
            .unwrap_or(1);
        let gap = " ".repeat(self.gap as usize);
        let mut spans: Vec<Span<'a>> = Vec::new();
        if let Some(l) = &self.label {
            spans.push(Span::styled(format!("{l} "), self.label_style));
        }
        let r = self.range as i64;
        let mut first = true;
        let mut sep = |spans: &mut Vec<Span<'a>>| {
            if !first && self.gap > 0 {
                spans.push(Span::raw(gap.clone()));
            }
            first = false;
        };
        // Negative side, outermost (-range) first.
        for level in (1..=r).rev() {
            sep(&mut spans);
            let filled = self.value < 0 && level <= -self.value;
            let (g, st) = if filled {
                (&self.negative.0, self.negative_style)
            } else {
                (&self.negative.1, self.empty_style)
            };
            spans.push(Span::styled(pad_to(g, width), st));
        }
        sep(&mut spans);
        spans.push(Span::styled(self.center.clone(), self.center_style));
        for level in 1..=r {
            sep(&mut spans);
            let filled = self.value > 0 && level <= self.value;
            let (g, st) = if filled {
                (&self.positive.0, self.positive_style)
            } else {
                (&self.positive.1, self.empty_style)
            };
            spans.push(Span::styled(pad_to(g, width), st));
        }
        if self.show_value {
            spans.push(Span::styled(
                format!(" {}", self.value_text()),
                self.value_style,
            ));
        }
        Line::from(spans).alignment(self.alignment)
    }
}

impl Widget for BipolarRating<'_> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let inner = match self.block.take() {
            Some(block) => {
                let inner = block.inner(area);
                block.render(area, buf);
                inner
            }
            None => area,
        };
        if inner.width == 0 || inner.height == 0 {
            return;
        }
        self.line()
            .render(Rect::new(inner.x, inner.y, inner.width, 1), buf);
    }
}

impl Widget for &BipolarRating<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.clone().render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(r: BipolarRating<'_>, w: u16) -> String {
        let area = Rect::new(0, 0, w, 1);
        let mut buf = Buffer::empty(area);
        r.render(area, &mut buf);
        (0..w).map(|x| buf[(x, 0)].symbol().to_string()).collect()
    }

    #[test]
    fn fills_outwards_from_center() {
        assert_eq!(text(BipolarRating::new(-1.0, 2), 9), "○ ● ┃ ○ ○");
        assert_eq!(text(BipolarRating::new(2.0, 2), 9), "○ ○ ┃ ● ●");
        assert_eq!(text(BipolarRating::new(-2.0, 2), 9), "● ● ┃ ○ ○");
        assert_eq!(text(BipolarRating::new(0.0, 1), 5), "○ ┃ ○");
        assert_eq!(text(BipolarRating::new(0.0, 0).gap(0), 1), "┃");
    }

    #[test]
    fn clamps_and_formats() {
        assert_eq!(BipolarRating::new(9.0, 2).value(), 2);
        assert_eq!(BipolarRating::new(-9.0, 2).value(), -2);
        assert_eq!(BipolarRating::new(f64::NAN, 2).value(), 0);
        assert_eq!(BipolarRating::new(1.4, 3).value_text(), "+1");
        assert_eq!(BipolarRating::new(-2.0, 3).value_text(), "-2");
        assert_eq!(BipolarRating::new(0.0, 3).value_text(), "0");
    }

    #[test]
    fn colors_and_presets() {
        let area = Rect::new(0, 0, 9, 1);
        let mut buf = Buffer::empty(area);
        BipolarRating::new(1.0, 2).render(area, &mut buf);
        assert_eq!(buf[(6, 0)].fg, Color::Green);
        assert_eq!(buf[(8, 0)].fg, Color::DarkGray);
        let mut buf = Buffer::empty(area);
        BipolarRating::new(-1.0, 2).render(area, &mut buf);
        assert_eq!(buf[(2, 0)].fg, Color::Red);
        let t = BipolarRating::new(1.0, 1).thumbs().gap(0);
        assert_eq!(t.line().width(), 5);
        let a = BipolarRating::new(1.0, 1).arrows().gap(0);
        assert_eq!(text(a, 3), "▽┃▲");
    }

    #[test]
    fn label_value_block() {
        let r = BipolarRating::new(1.0, 1)
            .label("Mood")
            .show_value(true)
            .center("0")
            .negative_style(Style::default())
            .positive_style(Style::default())
            .empty_style(Style::default())
            .center_style(Style::default())
            .label_style(Style::default())
            .value_style(Style::default())
            .alignment(Alignment::Center)
            .block(Block::bordered());
        let area = Rect::new(0, 0, 20, 3);
        let mut buf = Buffer::empty(area);
        (&r).render(area, &mut buf);
        let row: String = (0..20).map(|x| buf[(x, 1)].symbol().to_string()).collect();
        assert!(row.contains("Mood ○ 0 ● +1"), "{row}");
        let mut b = Buffer::empty(Rect::new(0, 0, 0, 0));
        r.render(Rect::new(0, 0, 0, 0), &mut b);
    }
}
