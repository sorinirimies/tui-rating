//! [`RatingBar`]: a continuous gauge-style rating.

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Widget},
};

use crate::{
    preset::RatingStyle,
    util::{bar_cells, cell_width, eighths, format_value, lerp_gradient, traffic_light_colors},
    LabelPosition, ValueFormat,
};

/// A continuous bar rating such as `████▌░░░░ 3.7/5`, with smooth block-eighth
/// edges, optional gradient and value text.
///
/// # Example
///
/// ```
/// use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
/// use tui_rating::{RatingBar, ValueFormat};
///
/// let bar = RatingBar::new(3.5, 5.0)
///     .width(10)
///     .show_value(true)
///     .value_format(ValueFormat::Percent);
/// let area = Rect::new(0, 0, 20, 1);
/// let mut buf = Buffer::empty(area);
/// bar.render(area, &mut buf);
/// ```
#[derive(Debug, Clone)]
pub struct RatingBar<'a> {
    value: f64,
    max: f64,
    width: Option<u16>,
    filled: String,
    empty: String,
    partials: Vec<String>,
    filled_style: Style,
    empty_style: Style,
    gradient: Vec<Color>,
    color_by_value: bool,
    modifier: Modifier,
    label: Option<String>,
    label_style: Style,
    label_position: LabelPosition,
    show_value: bool,
    value_format: ValueFormat,
    value_style: Style,
    alignment: Alignment,
    block: Option<Block<'a>>,
}

impl<'a> RatingBar<'a> {
    /// Creates a bar showing `value` out of `max` (both may be fractional).
    /// Non-finite or negative input is treated as `0`.
    pub fn new(value: f64, max: f64) -> Self {
        let clean = |v: f64| if v.is_finite() && v > 0.0 { v } else { 0.0 };
        let max = clean(max);
        Self {
            value: clean(value).min(max),
            max,
            width: None,
            filled: "█".into(),
            empty: "░".into(),
            partials: eighths(),
            filled_style: Style::default().fg(Color::Yellow),
            empty_style: Style::default().fg(Color::DarkGray),
            gradient: Vec::new(),
            color_by_value: false,
            modifier: Modifier::empty(),
            label: None,
            label_style: Style::default(),
            label_position: LabelPosition::Before,
            show_value: false,
            value_format: ValueFormat::Fraction,
            value_style: Style::default(),
            alignment: Alignment::Left,
            block: None,
        }
    }

    /// Fixed bar width in cells. By default the bar fills the free width.
    #[must_use]
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Glyphs for filled and empty cells.
    #[must_use]
    pub fn glyphs(mut self, filled: impl Into<String>, empty: impl Into<String>) -> Self {
        self.filled = filled.into();
        self.empty = empty.into();
        self
    }

    /// Partial-edge glyphs, thinnest first (default: block eighths). Pass an
    /// empty list for a hard edge.
    #[must_use]
    pub fn partials<I, S>(mut self, partials: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.partials = partials.into_iter().map(Into::into).collect();
        self
    }

    /// Style of filled cells.
    #[must_use]
    pub fn filled_style(mut self, style: Style) -> Self {
        self.filled_style = style;
        self
    }

    /// Shortcut: foreground colour of filled cells.
    #[must_use]
    pub fn filled_color(mut self, color: Color) -> Self {
        self.filled_style = self.filled_style.fg(color);
        self
    }

    /// Style of empty cells.
    #[must_use]
    pub fn empty_style(mut self, style: Style) -> Self {
        self.empty_style = style;
        self
    }

    /// Colours filled cells along the bar (left = first colour).
    #[must_use]
    pub fn gradient(mut self, colors: impl Into<Vec<Color>>) -> Self {
        self.gradient = colors.into();
        self
    }

    /// Red → yellow → green gradient along the bar.
    #[must_use]
    pub fn traffic_light(self) -> Self {
        self.gradient(traffic_light_colors())
    }

    /// Colours the *whole* fill by the rating itself (low = first colour,
    /// high = last) instead of by cell position. Needs a
    /// [`gradient`](Self::gradient).
    #[must_use]
    pub fn color_by_value(mut self, enabled: bool) -> Self {
        self.color_by_value = enabled;
        self
    }

    /// Extra modifiers for filled cells.
    #[must_use]
    pub fn modifier(mut self, modifier: Modifier) -> Self {
        self.modifier = modifier;
        self
    }

    /// Applies colours, gradient, modifier, label and value styles from a
    /// [`RatingStyle`] (its symbols are ignored).
    #[must_use]
    pub fn preset(mut self, preset: RatingStyle) -> Self {
        self.filled_style = preset.filled;
        self.empty_style = preset.empty;
        self.gradient = preset.gradient;
        self.modifier = preset.modifier;
        self.label_style = preset.label;
        self.value_style = preset.value;
        self
    }

    /// Text label.
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

    /// Label before (default) or after the bar.
    #[must_use]
    pub fn label_position(mut self, position: LabelPosition) -> Self {
        self.label_position = position;
        self
    }

    /// Shows the value text after the bar.
    #[must_use]
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Value text format.
    #[must_use]
    pub fn value_format(mut self, format: ValueFormat) -> Self {
        self.value_format = format;
        self
    }

    /// Value text style.
    #[must_use]
    pub fn value_style(mut self, style: Style) -> Self {
        self.value_style = style;
        self
    }

    /// Alignment of the line when the bar has a fixed width.
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

    /// Fraction filled, `0.0..=1.0`.
    pub fn ratio(&self) -> f64 {
        if self.max <= 0.0 {
            0.0
        } else {
            (self.value / self.max).clamp(0.0, 1.0)
        }
    }

    /// The value text.
    pub fn value_text(&self) -> String {
        // Fraction / Number show `max` rounded; scale-free percent is exact.
        let max = self.max.round() as usize;
        format_value(self.value, max, self.value_format)
    }

    fn texts(&self) -> (String, String, String) {
        let (mut before, mut after) = (String::new(), String::new());
        let value = if self.show_value {
            format!(" {}", self.value_text())
        } else {
            String::new()
        };
        if let Some(l) = &self.label {
            match self.label_position {
                LabelPosition::Before => before = format!("{l} "),
                LabelPosition::After => after = format!(" {l}"),
            }
        }
        (before, value, after)
    }

    /// Builds the line for a bar `bar_width` cells wide.
    pub fn line(&self, bar_width: usize) -> Line<'a> {
        let (before, value, after) = self.texts();
        let mut spans = Vec::new();
        if !before.is_empty() {
            spans.push(Span::styled(before, self.label_style));
        }
        let ratio = self.ratio();
        let by_value = if self.color_by_value {
            lerp_gradient(&self.gradient, ratio)
        } else {
            None
        };
        for cell in bar_cells(ratio, bar_width, &self.filled, &self.empty, &self.partials) {
            let style = if cell.filled {
                let mut st = self.filled_style.add_modifier(self.modifier);
                if let Some(c) = by_value.or_else(|| lerp_gradient(&self.gradient, cell.t)) {
                    st = st.fg(c);
                }
                st
            } else {
                self.empty_style
            };
            spans.push(Span::styled(cell.glyph, style));
        }
        if !value.is_empty() {
            spans.push(Span::styled(value, self.value_style));
        }
        if !after.is_empty() {
            spans.push(Span::styled(after, self.label_style));
        }
        Line::from(spans).alignment(self.alignment)
    }
}

impl Widget for RatingBar<'_> {
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
        let (before, value, after) = self.texts();
        let fixed = cell_width(&before) + cell_width(&value) + cell_width(&after);
        let free = (inner.width as usize).saturating_sub(fixed);
        let bar_w = self.width.map_or(free, |w| (w as usize).min(free)).max(1);
        let row = Rect::new(inner.x, inner.y, inner.width, 1);
        self.line(bar_w).render(row, buf);
    }
}

impl Widget for &RatingBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.clone().render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(b: RatingBar<'_>, w: u16) -> String {
        let area = Rect::new(0, 0, w, 1);
        let mut buf = Buffer::empty(area);
        b.render(area, &mut buf);
        (0..w).map(|x| buf[(x, 0)].symbol().to_string()).collect()
    }

    #[test]
    fn fixed_width_bar() {
        assert_eq!(text(RatingBar::new(2.0, 4.0).width(4), 6), "██░░  ");
    }

    #[test]
    fn smooth_edge_and_value() {
        let b = RatingBar::new(3.75, 5.0)
            .width(5)
            .show_value(true)
            .value_format(ValueFormat::Number);
        assert_eq!(text(b, 9), "███▊░ 3.8");
    }

    #[test]
    fn fills_free_width_and_label() {
        let b = RatingBar::new(1.0, 2.0)
            .label("Q")
            .label_position(LabelPosition::After)
            .show_value(true)
            .value_format(ValueFormat::Percent);
        // fixed: " 50%" (4) + " Q" (2) → 10 - 6 = 4 bar cells
        assert_eq!(text(b, 10), "██░░ 50% Q");
    }

    #[test]
    fn hard_edge_without_partials() {
        let b = RatingBar::new(1.5, 4.0)
            .width(4)
            .partials(Vec::<String>::new())
            .glyphs("#", "-");
        assert_eq!(text(b, 4), "#---");
    }

    #[test]
    fn gradient_by_position_and_value() {
        let area = Rect::new(0, 0, 4, 1);
        let mut buf = Buffer::empty(area);
        RatingBar::new(4.0, 4.0)
            .width(4)
            .gradient([Color::Rgb(0, 0, 0), Color::Rgb(30, 0, 0)])
            .render(area, &mut buf);
        assert_eq!(buf[(0, 0)].fg, Color::Rgb(0, 0, 0));
        assert_eq!(buf[(3, 0)].fg, Color::Rgb(30, 0, 0));
        let mut buf = Buffer::empty(area);
        RatingBar::new(2.0, 4.0)
            .width(4)
            .gradient([Color::Rgb(0, 0, 0), Color::Rgb(100, 0, 0)])
            .color_by_value(true)
            .render(area, &mut buf);
        assert_eq!(buf[(0, 0)].fg, Color::Rgb(50, 0, 0));
        assert_eq!(buf[(1, 0)].fg, Color::Rgb(50, 0, 0));
        assert_eq!(buf[(3, 0)].fg, Color::DarkGray);
    }

    #[test]
    fn preset_block_alignment_and_degenerate() {
        let b = RatingBar::new(1.0, 1.0)
            .preset(RatingStyle::neon())
            .traffic_light()
            .filled_color(Color::Red)
            .filled_style(Style::default())
            .empty_style(Style::default())
            .modifier(Modifier::BOLD)
            .label_style(Style::default())
            .value_style(Style::default())
            .alignment(Alignment::Right)
            .width(2)
            .block(Block::bordered());
        let area = Rect::new(0, 0, 6, 3);
        let mut buf = Buffer::empty(area);
        (&b).render(area, &mut buf);
        assert_eq!(buf[(3, 1)].symbol(), "█");
        let mut buf = Buffer::empty(Rect::new(0, 0, 0, 0));
        b.clone().render(Rect::new(0, 0, 0, 0), &mut buf);
        assert_eq!(RatingBar::new(f64::NAN, -1.0).ratio(), 0.0);
        assert_eq!(RatingBar::new(9.0, 3.0).ratio(), 1.0);
        assert_eq!(RatingBar::new(9.0, 3.0).value_text(), "3/3");
    }
}
