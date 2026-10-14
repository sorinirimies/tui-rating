//! The [`Rating`] widget.

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Widget},
};

#[cfg(feature = "theme")]
use tui_nuances::{Theme, Themed};

use crate::{RatingState, RatingSymbols};

/// How a single rating item is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemFill {
    /// Fully filled.
    Full,
    /// Half filled.
    Half,
    /// Empty.
    Empty,
}

/// A star-style rating widget (stars, hearts, circles, …).
///
/// # Example
///
/// ```
/// use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};
/// use tui_rating::{Rating, RatingState, RatingSymbols};
///
/// let state = RatingState::new(3.0, 5);
/// let rating = Rating::from_state(&state)
///     .symbols(RatingSymbols::hearts())
///     .filled_color(Color::Red)
///     .label("Quality")
///     .show_value(true);
///
/// let area = Rect::new(0, 0, 40, 1);
/// let mut buf = Buffer::empty(area);
/// rating.render(area, &mut buf);
/// ```
#[derive(Debug, Clone)]
pub struct Rating<'a> {
    value: f64,
    max: usize,
    half_steps: bool,
    symbols: RatingSymbols,
    filled_style: Option<Style>,
    half_style: Option<Style>,
    empty_style: Option<Style>,
    label: Option<String>,
    label_style: Option<Style>,
    value_style: Option<Style>,
    #[cfg(feature = "theme")]
    themed: Option<ThemeStyles>,
    show_value: bool,
    gap: u16,
    alignment: Alignment,
    block: Option<Block<'a>>,
}

/// Styles derived from a theme.
#[cfg_attr(not(feature = "theme"), allow(dead_code))]
#[derive(Debug, Clone, Copy)]
struct ThemeStyles {
    filled: Style,
    empty: Style,
    label: Style,
    value: Style,
}

#[cfg(feature = "theme")]
impl Themed for Rating<'_> {
    /// Maps `accent` → filled items, `disabled` → empty items,
    /// `foreground` → label, `muted` → value text.
    fn apply_theme(mut self, theme: &Theme) -> Self {
        self.themed = Some(ThemeStyles {
            filled: Style::default().fg(theme.accent),
            empty: Style::default().fg(theme.disabled),
            label: Style::default().fg(theme.foreground),
            value: Style::default().fg(theme.muted),
        });
        self
    }
}

impl<'a> Rating<'a> {
    /// Creates a rating with `value` out of `max` items. Value is clamped.
    pub fn new(value: f64, max: usize) -> Self {
        let value = if value.is_finite() {
            value.clamp(0.0, max as f64)
        } else {
            0.0
        };
        Self {
            value,
            max,
            half_steps: false,
            symbols: RatingSymbols::default(),
            filled_style: None,
            half_style: None,
            empty_style: None,
            label: None,
            label_style: None,
            value_style: None,
            #[cfg(feature = "theme")]
            themed: None,
            show_value: false,
            gap: 1,
            alignment: Alignment::Left,
            block: None,
        }
    }

    /// Creates a rating from a [`RatingState`] (value, max and half-step mode).
    pub fn from_state(state: &RatingState) -> Self {
        let mut r = Self::new(state.value(), state.max());
        r.half_steps = state.half_steps();
        r
    }

    /// Enables half-item rendering (values snap to the nearest `0.5`).
    #[must_use]
    pub fn half_steps(mut self, enabled: bool) -> Self {
        self.half_steps = enabled;
        self
    }

    /// Sets the symbol set.
    #[must_use]
    pub fn symbols(mut self, symbols: RatingSymbols) -> Self {
        self.symbols = symbols;
        self
    }

    /// Sets the style of filled items.
    #[must_use]
    pub fn filled_style(mut self, style: Style) -> Self {
        self.filled_style = Some(style);
        self
    }

    /// Shortcut: sets the foreground colour of filled items.
    #[must_use]
    pub fn filled_color(mut self, color: Color) -> Self {
        self.filled_style = Some(self.filled_style.unwrap_or_default().fg(color));
        self
    }

    /// Sets the style of half items (defaults to the filled style).
    #[must_use]
    pub fn half_style(mut self, style: Style) -> Self {
        self.half_style = Some(style);
        self
    }

    /// Sets the style of empty items.
    #[must_use]
    pub fn empty_style(mut self, style: Style) -> Self {
        self.empty_style = Some(style);
        self
    }

    /// Shortcut: sets the foreground colour of empty items.
    #[must_use]
    pub fn empty_color(mut self, color: Color) -> Self {
        self.empty_style = Some(self.empty_style.unwrap_or_default().fg(color));
        self
    }

    /// Sets a text label rendered before the items.
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the label style.
    #[must_use]
    pub fn label_style(mut self, style: Style) -> Self {
        self.label_style = Some(style);
        self
    }

    /// Shows a `value/max` text after the items.
    #[must_use]
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Sets the style of the value text.
    #[must_use]
    pub fn value_style(mut self, style: Style) -> Self {
        self.value_style = Some(style);
        self
    }

    /// Sets the number of spaces between items (default `1`).
    #[must_use]
    pub fn gap(mut self, gap: u16) -> Self {
        self.gap = gap;
        self
    }

    /// Sets horizontal alignment of the whole line.
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

    /// Styles the widget from a shared [`Theme`] (see [`Themed`]).
    ///
    /// Accepts `&Theme` or `Theme`. Explicit styles/colors set through
    /// `filled_style`, `filled_color`, `half_style`, `empty_style`,
    /// `empty_color`, `label_style` and `value_style` always win over the
    /// theme, regardless of call order. Without a theme the original default
    /// look is used.
    #[must_use]
    #[cfg(feature = "theme")]
    pub fn theme(self, theme: impl Into<Theme>) -> Self {
        self.apply_theme(&theme.into())
    }

    fn resolved(&self) -> (Style, Style, Style, Style) {
        #[cfg(feature = "theme")]
        let t = self.themed.as_ref();
        #[cfg(not(feature = "theme"))]
        let t: Option<&ThemeStyles> = None;
        let filled = self
            .filled_style
            .or(t.map(|t| t.filled))
            .unwrap_or_else(|| Style::default().fg(Color::Yellow));
        let empty = self
            .empty_style
            .or(t.map(|t| t.empty))
            .unwrap_or_else(|| Style::default().fg(Color::DarkGray));
        let label = self.label_style.or(t.map(|t| t.label)).unwrap_or_default();
        let value = self.value_style.or(t.map(|t| t.value)).unwrap_or_default();
        (filled, empty, label, value)
    }

    /// The value after snapping to the display granularity (`1.0` or `0.5`).
    pub fn display_value(&self) -> f64 {
        if self.half_steps {
            (self.value * 2.0).round() / 2.0
        } else {
            self.value.round()
        }
    }

    /// The `"3.5/5"`-style value text.
    pub fn value_text(&self) -> String {
        let v = self.display_value();
        if v.fract() == 0.0 {
            format!("{}/{}", v as i64, self.max)
        } else {
            format!("{:.1}/{}", v, self.max)
        }
    }

    /// Fill of the `index`-th item (0-based).
    pub fn fill_at(&self, index: usize) -> ItemFill {
        let remaining = self.display_value() - index as f64;
        if remaining >= 1.0 {
            ItemFill::Full
        } else if self.half_steps && remaining >= 0.5 {
            ItemFill::Half
        } else {
            ItemFill::Empty
        }
    }

    /// Builds the rendered line (label, items, value text) without a block.
    pub fn line(&self) -> Line<'a> {
        let (filled_style, empty_style, label_style, value_style) = self.resolved();
        let mut spans: Vec<Span<'a>> = Vec::new();
        if let Some(label) = &self.label {
            spans.push(Span::styled(format!("{label} "), label_style));
        }
        let gap = " ".repeat(self.gap as usize);
        for i in 0..self.max {
            if i > 0 && self.gap > 0 {
                spans.push(Span::raw(gap.clone()));
            }
            let (glyph, style) = match self.fill_at(i) {
                ItemFill::Full => (&self.symbols.filled, filled_style),
                ItemFill::Half => match &self.symbols.half {
                    Some(h) => (h, self.half_style.unwrap_or(filled_style)),
                    None => (&self.symbols.empty, self.half_style.unwrap_or(filled_style)),
                },
                ItemFill::Empty => (&self.symbols.empty, empty_style),
            };
            spans.push(Span::styled(glyph.clone(), style));
        }
        if self.show_value {
            spans.push(Span::styled(format!(" {}", self.value_text()), value_style));
        }
        Line::from(spans).alignment(self.alignment)
    }

    /// Width in terminal cells of the rendered line.
    pub fn width(&self) -> usize {
        self.line().width()
    }
}

impl Widget for Rating<'_> {
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
        let row = Rect::new(inner.x, inner.y, inner.width, 1);
        self.line().render(row, buf);
    }
}

impl Widget for &Rating<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.clone().render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(r: Rating<'_>, w: u16) -> String {
        let area = Rect::new(0, 0, w, 1);
        let mut buf = Buffer::empty(area);
        r.render(area, &mut buf);
        (0..w)
            .map(|x| buf[(x, 0)].symbol().to_string())
            .collect::<String>()
    }

    #[test]
    fn renders_filled_and_empty() {
        assert_eq!(text(Rating::new(3.0, 5), 9), "★ ★ ★ ☆ ☆");
    }

    #[test]
    fn zero_gap() {
        assert_eq!(text(Rating::new(2.0, 4).gap(0), 4), "★★☆☆");
    }

    #[test]
    fn half_with_glyph() {
        let r = Rating::new(2.5, 5)
            .half_steps(true)
            .symbols(RatingSymbols::circles())
            .gap(0);
        assert_eq!(text(r, 5), "●●◐○○");
    }

    #[test]
    fn half_without_glyph_uses_empty_with_filled_style() {
        let r = Rating::new(1.5, 3).half_steps(true).gap(0);
        let area = Rect::new(0, 0, 3, 1);
        let mut buf = Buffer::empty(area);
        r.render(area, &mut buf);
        assert_eq!(buf[(1, 0)].symbol(), "☆");
        assert_eq!(buf[(1, 0)].fg, Color::Yellow);
        assert_eq!(buf[(2, 0)].fg, Color::DarkGray);
    }

    #[test]
    fn no_half_rounds_to_whole() {
        assert_eq!(Rating::new(2.5, 5).display_value(), 3.0);
        assert_eq!(Rating::new(2.4, 5).display_value(), 2.0);
        assert_eq!(text(Rating::new(2.4, 3).gap(0), 3), "★★☆");
    }

    #[test]
    fn half_snaps_to_half() {
        assert_eq!(Rating::new(2.3, 5).half_steps(true).display_value(), 2.5);
        assert_eq!(Rating::new(2.2, 5).half_steps(true).display_value(), 2.0);
    }

    #[test]
    fn clamps_value() {
        assert_eq!(Rating::new(10.0, 3).display_value(), 3.0);
        assert_eq!(Rating::new(-3.0, 3).display_value(), 0.0);
        assert_eq!(Rating::new(f64::NAN, 3).display_value(), 0.0);
    }

    #[test]
    fn value_text_formats() {
        assert_eq!(Rating::new(3.0, 5).value_text(), "3/5");
        assert_eq!(Rating::new(3.5, 5).half_steps(true).value_text(), "3.5/5");
    }

    #[test]
    fn label_and_value() {
        let r = Rating::new(1.0, 2).label("Hi").show_value(true);
        assert_eq!(text(r, 16), "Hi ★ ☆ 1/2      ");
    }

    #[test]
    fn alignment_right_and_center() {
        let r = Rating::new(1.0, 2).gap(0).alignment(Alignment::Right);
        assert_eq!(text(r, 6), "    ★☆");
        let r = Rating::new(1.0, 2).gap(0).alignment(Alignment::Center);
        assert_eq!(text(r, 6), "  ★☆  ");
    }

    #[test]
    fn block_insets_content() {
        let r = Rating::new(1.0, 2).gap(0).block(Block::bordered());
        let area = Rect::new(0, 0, 6, 3);
        let mut buf = Buffer::empty(area);
        r.render(area, &mut buf);
        assert_eq!(buf[(1, 1)].symbol(), "★");
        assert_eq!(buf[(2, 1)].symbol(), "☆");
    }

    #[test]
    fn zero_area_does_not_panic() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 5, 1));
        Rating::new(1.0, 3).render(Rect::new(0, 0, 0, 0), &mut buf);
    }

    #[test]
    fn truncates_in_narrow_area() {
        assert_eq!(text(Rating::new(5.0, 5), 3), "★ ★");
    }

    #[test]
    fn max_zero_renders_nothing() {
        assert_eq!(text(Rating::new(0.0, 0), 3), "   ");
    }

    #[test]
    fn width_counts_cells() {
        assert_eq!(Rating::new(0.0, 5).width(), 9);
        assert_eq!(Rating::new(0.0, 5).gap(2).width(), 13);
    }

    #[test]
    fn from_state_copies_mode() {
        let s = RatingState::new(2.5, 5).with_half_steps(true);
        let r = Rating::from_state(&s);
        assert_eq!(r.fill_at(2), ItemFill::Half);
        assert_eq!(r.fill_at(1), ItemFill::Full);
        assert_eq!(r.fill_at(3), ItemFill::Empty);
    }
}

#[cfg(feature = "theme")]
#[cfg(test)]
mod theme_tests {
    use super::*;

    fn fg(r: Rating<'_>, x: u16) -> Option<Color> {
        let area = Rect::new(0, 0, 20, 1);
        let mut buf = Buffer::empty(area);
        r.render(area, &mut buf);
        buf[(x, 0)].style().fg
    }

    #[test]
    fn maps_roles() {
        let t = Theme::nord();
        let r = Rating::new(1.0, 2).theme(&t);
        assert_eq!(fg(r.clone(), 0), Some(t.accent));
        assert_eq!(fg(r, 2), Some(t.disabled));
    }

    #[test]
    fn explicit_wins_any_order() {
        let t = Theme::nord();
        let a = Rating::new(1.0, 2).filled_color(Color::Red).theme(&t);
        let b = Rating::new(1.0, 2).theme(&t).filled_color(Color::Red);
        assert_eq!(fg(a, 0), Some(Color::Red));
        assert_eq!(fg(b, 0), Some(Color::Red));
    }

    #[test]
    fn all_themes_render() {
        for t in Theme::all() {
            let r = Rating::new(2.5, 5)
                .half_steps(true)
                .label("x")
                .show_value(true);
            let _ = fg(r.theme(t), 0);
        }
    }
}
