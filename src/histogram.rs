//! [`RatingHistogram`]: an Amazon-style review summary.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Widget},
};

use crate::util::{
    bar_cells, cell_width, compact, eighths, lerp_gradient, thousands, traffic_light_colors,
};

/// A review-summary histogram: one row per star level, highest first.
///
/// ```text
/// 4.3 ★  ·  1,943 ratings
/// 5★ ██████████▌░░░░  62%  1,203
/// 4★ ████░░░░░░░░░░░  22%    430
/// ...
/// ```
///
/// `counts[0]` is the number of 1-star ratings, `counts[n-1]` the top level.
///
/// # Example
///
/// ```
/// use tui_rating::RatingHistogram;
///
/// let h = RatingHistogram::new([5, 10, 20, 40, 25]);
/// assert_eq!(h.total(), 100);
/// assert_eq!(h.average(), Some(3.7));
/// ```
#[derive(Debug, Clone)]
pub struct RatingHistogram<'a> {
    counts: Vec<u64>,
    symbol: String,
    filled: String,
    empty: String,
    partials: Vec<String>,
    gradient: Vec<Color>,
    filled_style: Style,
    empty_style: Style,
    label_style: Style,
    count_style: Style,
    show_percent: bool,
    show_count: bool,
    compact_counts: bool,
    summary: bool,
    bar_width: Option<u16>,
    block: Option<Block<'a>>,
}

impl<'a> RatingHistogram<'a> {
    /// Creates a histogram from per-level counts (index 0 = lowest level).
    pub fn new(counts: impl Into<Vec<u64>>) -> Self {
        Self {
            counts: counts.into(),
            symbol: "★".into(),
            filled: "█".into(),
            empty: "░".into(),
            partials: eighths(),
            gradient: Vec::new(),
            filled_style: Style::default().fg(Color::Yellow),
            empty_style: Style::default().fg(Color::DarkGray),
            label_style: Style::default(),
            count_style: Style::default().fg(Color::DarkGray),
            show_percent: true,
            show_count: true,
            compact_counts: false,
            summary: false,
            bar_width: None,
            block: None,
        }
    }

    /// Total number of ratings.
    pub fn total(&self) -> u64 {
        self.counts.iter().sum()
    }

    /// Weighted average level (1-based), rounded to one decimal, or `None`
    /// with no ratings.
    pub fn average(&self) -> Option<f64> {
        let total = self.total();
        if total == 0 {
            return None;
        }
        let sum: f64 = self
            .counts
            .iter()
            .enumerate()
            .map(|(i, &c)| (i + 1) as f64 * c as f64)
            .sum();
        Some((sum / total as f64 * 10.0).round() / 10.0)
    }

    /// Percentage (`0..=100`, rounded) of ratings at 1-based `level`.
    pub fn percent(&self, level: usize) -> u8 {
        let total = self.total();
        match level.checked_sub(1).and_then(|i| self.counts.get(i)) {
            Some(&c) if total > 0 => (c as f64 / total as f64 * 100.0).round() as u8,
            _ => 0,
        }
    }

    /// The level glyph (default `★`).
    #[must_use]
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = symbol.into();
        self
    }

    /// Bar glyphs for filled and empty cells.
    #[must_use]
    pub fn glyphs(mut self, filled: impl Into<String>, empty: impl Into<String>) -> Self {
        self.filled = filled.into();
        self.empty = empty.into();
        self
    }

    /// Colours bars by level (lowest level = first colour).
    #[must_use]
    pub fn gradient(mut self, colors: impl Into<Vec<Color>>) -> Self {
        self.gradient = colors.into();
        self
    }

    /// Red (1★) → yellow → green (top level).
    #[must_use]
    pub fn traffic_light(self) -> Self {
        self.gradient(traffic_light_colors())
    }

    /// Style of filled bar cells (when no gradient is set).
    #[must_use]
    pub fn filled_style(mut self, style: Style) -> Self {
        self.filled_style = style;
        self
    }

    /// Style of empty bar cells.
    #[must_use]
    pub fn empty_style(mut self, style: Style) -> Self {
        self.empty_style = style;
        self
    }

    /// Style of level labels and the summary.
    #[must_use]
    pub fn label_style(mut self, style: Style) -> Self {
        self.label_style = style;
        self
    }

    /// Style of the percent / count columns.
    #[must_use]
    pub fn count_style(mut self, style: Style) -> Self {
        self.count_style = style;
        self
    }

    /// Shows the percent column (default on).
    #[must_use]
    pub fn show_percent(mut self, show: bool) -> Self {
        self.show_percent = show;
        self
    }

    /// Shows the count column (default on).
    #[must_use]
    pub fn show_count(mut self, show: bool) -> Self {
        self.show_count = show;
        self
    }

    /// Abbreviates counts (`1.2k`).
    #[must_use]
    pub fn compact_counts(mut self, compact: bool) -> Self {
        self.compact_counts = compact;
        self
    }

    /// Adds a header row: average and total, e.g. `4.3 ★  ·  1,943 ratings`.
    #[must_use]
    pub fn summary(mut self, summary: bool) -> Self {
        self.summary = summary;
        self
    }

    /// Fixed bar width. By default bars fill the free width.
    #[must_use]
    pub fn bar_width(mut self, width: u16) -> Self {
        self.bar_width = Some(width);
        self
    }

    /// Wraps the widget in a block.
    #[must_use]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    fn count_text(&self, c: u64) -> String {
        if self.compact_counts {
            compact(c)
        } else {
            thousands(c)
        }
    }

    /// Number of rows needed (levels, plus the summary row).
    pub fn height(&self) -> usize {
        self.counts.len() + usize::from(self.summary)
    }

    /// Builds every row for bars of `bar_width` cells (`None` = default 10).
    pub fn lines(&self, bar_width: Option<usize>) -> Vec<Line<'a>> {
        let mut lines = Vec::new();
        let total = self.total();
        if self.summary {
            let avg = self
                .average()
                .map_or_else(|| "–".to_string(), |a| format!("{a:.1}"));
            lines.push(Line::from(Span::styled(
                format!(
                    "{avg} {}  ·  {} ratings",
                    self.symbol,
                    self.count_text(total)
                ),
                self.label_style,
            )));
        }
        let levels = self.counts.len();
        let cnt_w = if self.show_count {
            self.counts
                .iter()
                .map(|&c| cell_width(&self.count_text(c)))
                .max()
                .unwrap_or(0)
        } else {
            0
        };
        for level in (1..=levels).rev() {
            let c = self.counts[level - 1];
            let ratio = if total == 0 {
                0.0
            } else {
                c as f64 / total as f64
            };
            let t = if levels <= 1 {
                1.0
            } else {
                (level - 1) as f64 / (levels - 1) as f64
            };
            let color = lerp_gradient(&self.gradient, t);
            let mut spans = vec![Span::styled(
                format!(
                    "{:>w$}{} ",
                    level,
                    self.symbol,
                    w = levels.to_string().len()
                ),
                self.label_style,
            )];
            for cell in bar_cells(
                ratio,
                bar_width.unwrap_or(10),
                &self.filled,
                &self.empty,
                &self.partials,
            ) {
                let style = if cell.filled {
                    color.map_or(self.filled_style, |c| self.filled_style.fg(c))
                } else {
                    self.empty_style
                };
                spans.push(Span::styled(cell.glyph, style));
            }
            if self.show_percent {
                spans.push(Span::styled(
                    format!(" {:>3}%", self.percent(level)),
                    self.count_style,
                ));
            }
            if self.show_count {
                spans.push(Span::styled(
                    format!(" {:>cnt_w$}", self.count_text(c)),
                    self.count_style,
                ));
            }
            lines.push(Line::from(spans));
        }
        lines
    }

    /// Width (cells) of everything except the bar.
    fn fixed_width(&self) -> usize {
        let levels = self.counts.len();
        let label = levels.to_string().len() + cell_width(&self.symbol) + 1;
        let pct = if self.show_percent { 5 } else { 0 };
        let cnt = if self.show_count {
            1 + self
                .counts
                .iter()
                .map(|&c| cell_width(&self.count_text(c)))
                .max()
                .unwrap_or(0)
        } else {
            0
        };
        label + pct + cnt
    }
}

impl Widget for RatingHistogram<'_> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let inner = match self.block.take() {
            Some(block) => {
                let inner = block.inner(area);
                block.render(area, buf);
                inner
            }
            None => area,
        };
        if inner.width == 0 || inner.height == 0 || self.counts.is_empty() {
            return;
        }
        let free = (inner.width as usize).saturating_sub(self.fixed_width());
        let bar_w = self
            .bar_width
            .map_or(free, |w| (w as usize).min(free))
            .max(1);
        for (i, line) in self
            .lines(Some(bar_w))
            .into_iter()
            .enumerate()
            .take(inner.height as usize)
        {
            line.render(Rect::new(inner.x, inner.y + i as u16, inner.width, 1), buf);
        }
    }
}

impl Widget for &RatingHistogram<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.clone().render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rows(h: RatingHistogram<'_>, w: u16, hgt: u16) -> Vec<String> {
        let area = Rect::new(0, 0, w, hgt);
        let mut buf = Buffer::empty(area);
        h.render(area, &mut buf);
        (0..hgt)
            .map(|y| {
                (0..w)
                    .map(|x| buf[(x, y)].symbol().to_string())
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect()
    }

    #[test]
    fn stats() {
        let h = RatingHistogram::new([5, 10, 20, 40, 25]);
        assert_eq!(h.total(), 100);
        assert_eq!(h.average(), Some(3.7));
        assert_eq!(h.percent(4), 40);
        assert_eq!(h.percent(0), 0);
        assert_eq!(h.percent(9), 0);
        assert_eq!(h.height(), 5);
        assert_eq!(h.clone().summary(true).height(), 6);
        let empty = RatingHistogram::new(Vec::<u64>::new());
        assert_eq!(empty.average(), None);
        assert_eq!(RatingHistogram::new([0, 0]).percent(1), 0);
    }

    #[test]
    fn renders_rows_highest_first() {
        let h = RatingHistogram::new([0, 0, 0, 1, 3])
            .bar_width(4)
            .compact_counts(true);
        let r = rows(h, 30, 5);
        assert_eq!(r[0], "5★ ███░  75% 3");
        assert_eq!(r[1], "4★ █░░░  25% 1");
        assert_eq!(r[4], "1★ ░░░░   0% 0");
    }

    #[test]
    fn summary_and_hidden_columns() {
        let h = RatingHistogram::new([1, 1])
            .summary(true)
            .show_percent(false)
            .show_count(false)
            .bar_width(2);
        let r = rows(h, 30, 3);
        assert_eq!(r[0], "1.5 ★  ·  2 ratings");
        assert_eq!(r[1], "2★ █░");
        let r = rows(RatingHistogram::new([0]).summary(true), 30, 2);
        assert_eq!(r[0], "– ★  ·  0 ratings");
    }

    #[test]
    fn fills_free_width_and_gradient_and_edges() {
        let area = Rect::new(0, 0, 12, 2);
        let mut buf = Buffer::empty(area);
        RatingHistogram::new([1, 1])
            .traffic_light()
            .show_count(false)
            .block(Block::default())
            .render(area, &mut buf);
        // 2★ label(3) + pct(5) = 8 fixed → 4 bar cells; row 0 = level 2 = green
        assert_eq!(buf[(3, 0)].fg, Color::Rgb(70, 190, 90));
        assert_eq!(buf[(3, 1)].fg, Color::Rgb(220, 60, 60));
        let mut b = Buffer::empty(Rect::new(0, 0, 0, 0));
        RatingHistogram::new([1]).render(Rect::new(0, 0, 0, 0), &mut b);
        let mut b = Buffer::empty(area);
        (&RatingHistogram::new(Vec::<u64>::new())
            .symbol("*")
            .glyphs("#", "-")
            .filled_style(Style::default())
            .empty_style(Style::default())
            .label_style(Style::default())
            .count_style(Style::default())
            .gradient([Color::Red]))
            .render(area, &mut b);
    }
}
