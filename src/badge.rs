//! [`RatingBadge`]: compact numeric rating such as `[ 4.2 ★ ]`.

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Widget},
};

use crate::util::{compact, lerp_gradient, thousands};

/// Visual form of a [`RatingBadge`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeKind {
    /// `[ 4.2 ★ ]`
    #[default]
    Brackets,
    /// ` 4.2 ★ ` as a filled pill (dark text on a coloured background).
    Pill,
    /// `★ 4.2`
    Plain,
}

/// A compact numeric rating badge, with optional max and vote count:
/// `[ 4.2 ★ ]`, `★ 4.2/5 (1,203)`, a filled pill like IMDb…
///
/// # Example
///
/// ```
/// use tui_rating::{BadgeKind, RatingBadge};
///
/// let b = RatingBadge::new(4.2, 5.0).kind(BadgeKind::Plain).votes(1203);
/// assert_eq!(b.text(), "★ 4.2 (1,203)");
/// ```
#[derive(Debug, Clone)]
pub struct RatingBadge<'a> {
    value: f64,
    max: f64,
    symbol: String,
    kind: BadgeKind,
    decimals: usize,
    show_max: bool,
    votes: Option<u64>,
    compact_votes: bool,
    color: Option<Color>,
    gradient: Vec<Color>,
    modifier: Modifier,
    votes_style: Style,
    alignment: Alignment,
    block: Option<Block<'a>>,
}

impl<'a> RatingBadge<'a> {
    /// Creates a badge for `value` out of `max`.
    pub fn new(value: f64, max: f64) -> Self {
        let clean = |v: f64| if v.is_finite() && v > 0.0 { v } else { 0.0 };
        Self {
            value: clean(value),
            max: clean(max),
            symbol: "★".into(),
            kind: BadgeKind::Brackets,
            decimals: 1,
            show_max: false,
            votes: None,
            compact_votes: false,
            color: None,
            gradient: Vec::new(),
            modifier: Modifier::BOLD,
            votes_style: Style::default().fg(Color::DarkGray),
            alignment: Alignment::Left,
            block: None,
        }
    }

    /// The glyph shown next to the number (default `★`).
    #[must_use]
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = symbol.into();
        self
    }

    /// Visual form.
    #[must_use]
    pub fn kind(mut self, kind: BadgeKind) -> Self {
        self.kind = kind;
        self
    }

    /// Decimal places of the number (default 1).
    #[must_use]
    pub fn decimals(mut self, decimals: usize) -> Self {
        self.decimals = decimals.min(6);
        self
    }

    /// Appends `/max` to the number.
    #[must_use]
    pub fn show_max(mut self, show: bool) -> Self {
        self.show_max = show;
        self
    }

    /// Appends a vote count, e.g. `(1,203)`.
    #[must_use]
    pub fn votes(mut self, votes: u64) -> Self {
        self.votes = Some(votes);
        self
    }

    /// Abbreviates the vote count (`1.2k`).
    #[must_use]
    pub fn compact_votes(mut self, compact: bool) -> Self {
        self.compact_votes = compact;
        self
    }

    /// Fixed colour (default: yellow, or the gradient sampled at the rating).
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Picks the colour from a gradient by rating (low → first colour).
    #[must_use]
    pub fn gradient(mut self, colors: impl Into<Vec<Color>>) -> Self {
        self.gradient = colors.into();
        self
    }

    /// Red → yellow → green by rating.
    #[must_use]
    pub fn traffic_light(self) -> Self {
        self.gradient(crate::util::traffic_light_colors())
    }

    /// Text modifiers of the badge (default bold).
    #[must_use]
    pub fn modifier(mut self, modifier: Modifier) -> Self {
        self.modifier = modifier;
        self
    }

    /// Style of the vote count.
    #[must_use]
    pub fn votes_style(mut self, style: Style) -> Self {
        self.votes_style = style;
        self
    }

    /// Alignment inside the area.
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

    fn ratio(&self) -> f64 {
        if self.max <= 0.0 {
            0.0
        } else {
            (self.value / self.max).clamp(0.0, 1.0)
        }
    }

    fn resolved_color(&self) -> Color {
        self.color
            .or_else(|| lerp_gradient(&self.gradient, self.ratio()))
            .unwrap_or(Color::Yellow)
    }

    fn number(&self) -> String {
        let n = format!("{:.*}", self.decimals, self.value);
        if self.show_max {
            format!("{n}/{}", crate::util::number_text(self.max))
        } else {
            n
        }
    }

    fn votes_text(&self) -> Option<String> {
        self.votes.map(|v| {
            let n = if self.compact_votes {
                compact(v)
            } else {
                thousands(v)
            };
            format!(" ({n})")
        })
    }

    fn core_text(&self) -> String {
        let n = self.number();
        match self.kind {
            BadgeKind::Brackets => format!("[ {n} {} ]", self.symbol),
            BadgeKind::Pill => format!(" {n} {} ", self.symbol),
            BadgeKind::Plain => format!("{} {n}", self.symbol),
        }
    }

    /// The full plain text of the badge, e.g. `[ 4.2 ★ ] (1,203)`.
    pub fn text(&self) -> String {
        format!(
            "{}{}",
            self.core_text(),
            self.votes_text().unwrap_or_default()
        )
    }

    /// The styled line.
    pub fn line(&self) -> Line<'a> {
        let color = self.resolved_color();
        let core_style = match self.kind {
            BadgeKind::Pill => Style::default()
                .fg(Color::Black)
                .bg(color)
                .add_modifier(self.modifier),
            _ => Style::default().fg(color).add_modifier(self.modifier),
        };
        let mut spans = vec![Span::styled(self.core_text(), core_style)];
        if let Some(v) = self.votes_text() {
            spans.push(Span::styled(v, self.votes_style));
        }
        Line::from(spans).alignment(self.alignment)
    }

    /// Width in terminal cells.
    pub fn width(&self) -> usize {
        self.line().width()
    }
}

impl Widget for RatingBadge<'_> {
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

impl Widget for &RatingBadge<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.clone().render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texts() {
        let b = RatingBadge::new(4.2, 5.0);
        assert_eq!(b.text(), "[ 4.2 ★ ]");
        assert_eq!(b.clone().kind(BadgeKind::Pill).text(), " 4.2 ★ ");
        assert_eq!(
            b.clone().kind(BadgeKind::Plain).show_max(true).text(),
            "★ 4.2/5"
        );
        assert_eq!(b.clone().votes(1203).text(), "[ 4.2 ★ ] (1,203)");
        assert_eq!(
            b.clone().votes(1203).compact_votes(true).text(),
            "[ 4.2 ★ ] (1.2k)"
        );
        assert_eq!(b.clone().decimals(0).symbol("*").text(), "[ 4 * ]");
        assert_eq!(b.clone().decimals(99).decimals(2).text(), "[ 4.20 ★ ]");
        assert_eq!(b.width(), 9);
    }

    #[test]
    fn colors() {
        let b = RatingBadge::new(5.0, 5.0);
        assert_eq!(b.line().spans[0].style.fg, Some(Color::Yellow));
        let b2 = RatingBadge::new(5.0, 5.0).traffic_light();
        assert_eq!(b2.line().spans[0].style.fg, Some(Color::Rgb(70, 190, 90)));
        let b3 = RatingBadge::new(0.0, 0.0)
            .traffic_light()
            .color(Color::Cyan);
        assert_eq!(b3.line().spans[0].style.fg, Some(Color::Cyan));
        let p = RatingBadge::new(1.0, 5.0)
            .kind(BadgeKind::Pill)
            .color(Color::Red);
        let st = p.line().spans[0].style;
        assert_eq!((st.fg, st.bg), (Some(Color::Black), Some(Color::Red)));
        assert_eq!(RatingBadge::new(f64::NAN, 5.0).ratio(), 0.0);
    }

    #[test]
    fn renders_aligned_in_block() {
        let b = RatingBadge::new(4.0, 5.0)
            .votes(5)
            .votes_style(Style::default())
            .modifier(Modifier::empty())
            .alignment(Alignment::Right)
            .block(Block::bordered());
        let area = Rect::new(0, 0, 20, 3);
        let mut buf = Buffer::empty(area);
        (&b).render(area, &mut buf);
        let row: String = (0..20).map(|x| buf[(x, 1)].symbol().to_string()).collect();
        assert_eq!(row, "│     [ 4.0 ★ ] (5)│");
        let mut buf = Buffer::empty(Rect::new(0, 0, 0, 0));
        b.render(Rect::new(0, 0, 0, 0), &mut buf);
    }
}
