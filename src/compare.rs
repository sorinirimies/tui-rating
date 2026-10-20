//! [`RatingCompare`]: two ratings side by side with the difference.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Widget},
};

use crate::{
    util::{cell_width, pad_to},
    LabelPosition, RatingBar, RatingDelta,
};

/// Compares two items (versions, products, before/after) as two bars plus a
/// delta row; the winner's label is bold.
///
/// ```text
/// v1.0 ████████████░░░░░░ 3.1/5
/// v2.0 ████████████████░░ 4.2/5
///      ▲ 1.1 (+35%)
/// ```
///
/// ```
/// use tui_rating::RatingCompare;
///
/// let c = RatingCompare::new(("v1", 3.1), ("v2", 4.2), 5.0);
/// assert_eq!(c.winner(), Some(1));
/// assert_eq!(c.delta_text(), "▲ 1.1 (+35%)");
/// ```
#[derive(Debug, Clone)]
pub struct RatingCompare<'a> {
    a: (String, f64),
    b: (String, f64),
    max: f64,
    colors: (Color, Color),
    show_delta: bool,
    block: Option<Block<'a>>,
}

impl<'a> RatingCompare<'a> {
    /// Compares `a` and `b`, both out of `max`.
    pub fn new(a: (impl Into<String>, f64), b: (impl Into<String>, f64), max: f64) -> Self {
        let clean = |v: f64| if v.is_finite() { v.max(0.0) } else { 0.0 };
        Self {
            a: (a.0.into(), clean(a.1)),
            b: (b.0.into(), clean(b.1)),
            max: if max.is_finite() && max > 0.0 {
                max
            } else {
                1.0
            },
            colors: (Color::Rgb(0, 190, 230), Color::Rgb(230, 80, 190)),
            show_delta: true,
            block: None,
        }
    }

    /// Bar colours of `a` and `b`.
    #[must_use]
    pub fn colors(mut self, a: Color, b: Color) -> Self {
        self.colors = (a, b);
        self
    }

    /// Shows the delta row (default on; needs a 3rd row).
    #[must_use]
    pub fn show_delta(mut self, show: bool) -> Self {
        self.show_delta = show;
        self
    }

    /// Wraps the widget in a block.
    #[must_use]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Index (`0` = a, `1` = b) of the higher value; `None` on a tie.
    pub fn winner(&self) -> Option<usize> {
        match self.a.1.partial_cmp(&self.b.1) {
            Some(std::cmp::Ordering::Greater) => Some(0),
            Some(std::cmp::Ordering::Less) => Some(1),
            _ => None,
        }
    }

    /// `"▲ 1.1 (+35%)"` (change from `a` to `b`).
    pub fn delta_text(&self) -> String {
        let d = RatingDelta::between(self.a.1, self.b.1);
        match d.relative() {
            Some(r) => format!("{} ({:+}%)", d.text(), r.round() as i64),
            None => d.text(),
        }
    }

    fn inner(&self, area: Rect) -> Rect {
        match &self.block {
            Some(b) => b.inner(area),
            None => area,
        }
    }
}

impl Widget for RatingCompare<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner = self.inner(area);
        if let Some(b) = self.block.clone() {
            b.render(area, buf);
        }
        if inner.width == 0 || inner.height == 0 {
            return;
        }
        let label_w = cell_width(&self.a.0).max(cell_width(&self.b.0));
        let winner = self.winner();
        for (i, (item, color)) in [(&self.a, self.colors.0), (&self.b, self.colors.1)]
            .into_iter()
            .enumerate()
        {
            if i as u16 >= inner.height {
                return;
            }
            let label_style = if winner == Some(i) {
                Style::default().fg(color).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(color)
            };
            RatingBar::new(item.1, self.max)
                .filled_color(color)
                .label(format!("{} ", pad_to(&item.0, label_w)))
                .label_style(label_style)
                .label_position(LabelPosition::Before)
                .show_value(true)
                .render(Rect::new(inner.x, inner.y + i as u16, inner.width, 1), buf);
        }
        if self.show_delta && inner.height >= 3 {
            let d = RatingDelta::between(self.a.1, self.b.1);
            let line = Line::from(vec![
                Span::raw(" ".repeat(label_w + 1)),
                Span::styled(self.delta_text(), d.span().style),
            ]);
            line.render(Rect::new(inner.x, inner.y + 2, inner.width, 1), buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winner_and_delta() {
        let c = RatingCompare::new(("a", 4.0), ("b", 3.0), 5.0);
        assert_eq!(c.winner(), Some(0));
        assert_eq!(c.delta_text(), "▼ 1.0 (-25%)");
        assert_eq!(
            RatingCompare::new(("a", 2.0), ("b", 2.0), 5.0).winner(),
            None
        );
        let z = RatingCompare::new(("a", 0.0), ("b", f64::NAN), 0.0);
        assert_eq!(z.winner(), None);
        assert_eq!(z.delta_text(), "● 0.0");
        assert_eq!(
            RatingCompare::new(("a", 0.0), ("b", 2.0), 5.0).delta_text(),
            "▲ 2.0"
        );
    }

    #[test]
    fn renders_in_various_sizes() {
        let c =
            RatingCompare::new(("v1.0", 3.1), ("v2.0", 4.2), 5.0).colors(Color::Red, Color::Blue);
        let area = Rect::new(0, 0, 30, 3);
        let mut buf = Buffer::empty(area);
        c.clone().render(area, &mut buf);
        let row = |y| -> String { (0..30).map(|x| buf[(x, y)].symbol().to_string()).collect() };
        assert!(row(0).starts_with("v1.0 "));
        assert!(row(1).starts_with("v2.0 "));
        assert!(row(2).contains("▲ 1.1 (+35%)"));
        for h in 0..3 {
            let a = Rect::new(0, 0, 30, h);
            let mut b = Buffer::empty(Rect::new(0, 0, 30, 3));
            c.clone().show_delta(false).render(a, &mut b);
        }
        let mut b = Buffer::empty(Rect::new(0, 0, 30, 5));
        c.block(Block::bordered())
            .render(Rect::new(0, 0, 30, 5), &mut b);
        let mut z = Buffer::empty(Rect::new(0, 0, 0, 0));
        RatingCompare::new(("a", 1.0), ("b", 1.0), 5.0).render(Rect::new(0, 0, 0, 0), &mut z);
    }
}
