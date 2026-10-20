//! [`RatingDelta`] (`▲ 0.3`) and [`RatingTrend`] (sparkline of a rating over time).

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Widget},
};

use crate::util::{lerp_gradient, render_lines, traffic_light_colors};

/// A change indicator: `▲ 0.3` (green), `▼ 0.2` (red) or `● 0.0`.
///
/// ```
/// use tui_rating::RatingDelta;
///
/// assert_eq!(RatingDelta::between(4.0, 4.3).text(), "▲ 0.3");
/// assert_eq!(RatingDelta::between(4.0, 3.5).text(), "▼ 0.5");
/// assert_eq!(RatingDelta::between(4.0, 5.0).percent(true).text(), "▲ 25%");
/// ```
#[derive(Debug, Clone, Copy)]
pub struct RatingDelta {
    old: f64,
    new: f64,
    decimals: usize,
    percent: bool,
    up_color: Color,
    down_color: Color,
}

impl RatingDelta {
    /// Change from `old` to `new` (non-finite inputs count as `0`).
    pub fn between(old: f64, new: f64) -> Self {
        let f = |v: f64| if v.is_finite() { v } else { 0.0 };
        Self {
            old: f(old),
            new: f(new),
            decimals: 1,
            percent: false,
            up_color: Color::Rgb(70, 190, 90),
            down_color: Color::Rgb(220, 60, 60),
        }
    }

    /// Decimal places (default 1, max 6).
    #[must_use]
    pub fn decimals(mut self, decimals: usize) -> Self {
        self.decimals = decimals.min(6);
        self
    }

    /// Shows the relative change in percent instead of the difference.
    #[must_use]
    pub fn percent(mut self, percent: bool) -> Self {
        self.percent = percent;
        self
    }

    /// Colours for improvement and decline.
    #[must_use]
    pub fn colors(mut self, up: Color, down: Color) -> Self {
        self.up_color = up;
        self.down_color = down;
        self
    }

    /// `new − old`.
    pub fn diff(&self) -> f64 {
        self.new - self.old
    }

    /// Relative change in percent (`None` if `old` is `0`).
    pub fn relative(&self) -> Option<f64> {
        (self.old != 0.0).then(|| self.diff() / self.old.abs() * 100.0)
    }

    fn magnitude_text(&self) -> String {
        if self.percent {
            match self.relative() {
                Some(r) => format!("{}%", r.abs().round() as i64),
                None => "n/a".into(),
            }
        } else {
            format!("{:.*}", self.decimals, self.diff().abs())
        }
    }

    fn is_flat(&self) -> bool {
        let d = self.diff();
        let eps = 0.5 * 10f64.powi(-(self.decimals as i32));
        d.abs() < eps
    }

    /// Plain text (`▲ 0.3`).
    pub fn text(&self) -> String {
        let arrow = if self.is_flat() {
            "●"
        } else if self.diff() > 0.0 {
            "▲"
        } else {
            "▼"
        };
        format!("{arrow} {}", self.magnitude_text())
    }

    /// The styled span.
    pub fn span(&self) -> Span<'static> {
        let color = if self.is_flat() {
            Color::DarkGray
        } else if self.diff() > 0.0 {
            self.up_color
        } else {
            self.down_color
        };
        Span::styled(
            self.text(),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        )
    }
}

impl Widget for RatingDelta {
    fn render(self, area: Rect, buf: &mut Buffer) {
        render_lines(None, vec![Line::from(self.span())], area, buf);
    }
}

const BLOCKS: [&str; 8] = ["▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];

/// A sparkline of a rating over time, optionally followed by the latest
/// value and its [`RatingDelta`]: `▂▃▅▄▆▇ 4.2 ▲ 0.3`.
///
/// ```
/// use tui_rating::RatingTrend;
///
/// let t = RatingTrend::new([3.0, 3.5, 4.0, 4.4]).range(0.0, 5.0).show_latest(true);
/// assert_eq!(t.text(), "▅▆▇▇ 4.4 ▲ 0.4");
/// ```
#[derive(Debug, Clone)]
pub struct RatingTrend<'a> {
    values: Vec<f64>,
    min: Option<f64>,
    max: Option<f64>,
    gradient: Vec<Color>,
    color: Option<Color>,
    show_latest: bool,
    alignment: Alignment,
    block: Option<Block<'a>>,
}

impl<'a> RatingTrend<'a> {
    /// Creates a trend from oldest to newest value; non-finite values are dropped.
    pub fn new(values: impl IntoIterator<Item = f64>) -> Self {
        Self {
            values: values.into_iter().filter(|v| v.is_finite()).collect(),
            min: None,
            max: None,
            gradient: traffic_light_colors(),
            color: None,
            show_latest: false,
            alignment: Alignment::Left,
            block: None,
        }
    }

    /// Fixed value range (default: min/max of the data). Use `0..max` to
    /// compare trends on the same scale.
    #[must_use]
    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }

    /// One colour for all bars (default: gradient by height).
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Gradient (low → high) used to colour bars by height.
    #[must_use]
    pub fn gradient(mut self, colors: impl Into<Vec<Color>>) -> Self {
        self.gradient = colors.into();
        self
    }

    /// Appends the latest value and its change since the previous one.
    #[must_use]
    pub fn show_latest(mut self, show: bool) -> Self {
        self.show_latest = show;
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

    fn bounds(&self) -> (f64, f64) {
        let lo = self
            .min
            .unwrap_or_else(|| self.values.iter().copied().fold(f64::INFINITY, f64::min));
        let hi = self.max.unwrap_or_else(|| {
            self.values
                .iter()
                .copied()
                .fold(f64::NEG_INFINITY, f64::max)
        });
        (lo, hi)
    }

    /// Bar height `0..=7` of every value.
    pub fn levels(&self) -> Vec<usize> {
        let (lo, hi) = self.bounds();
        self.values
            .iter()
            .map(|v| {
                if hi.is_nan() || lo.is_nan() || hi <= lo {
                    return 3;
                }
                (((v - lo) / (hi - lo)).clamp(0.0, 1.0) * 7.0).round() as usize
            })
            .collect()
    }

    /// Plain text of the line.
    pub fn text(&self) -> String {
        self.line()
            .spans
            .iter()
            .map(|s| s.content.as_ref())
            .collect()
    }

    /// The styled line.
    pub fn line(&self) -> Line<'a> {
        let levels = self.levels();
        let mut spans: Vec<Span<'a>> = levels
            .iter()
            .map(|&l| {
                let color = self
                    .color
                    .or_else(|| lerp_gradient(&self.gradient, l as f64 / 7.0))
                    .unwrap_or(Color::Yellow);
                Span::styled(BLOCKS[l.min(7)], Style::default().fg(color))
            })
            .collect();
        if self.show_latest {
            if let Some(last) = self.values.last() {
                spans.push(Span::styled(
                    format!(" {last:.1}"),
                    Style::default().add_modifier(Modifier::BOLD),
                ));
                if self.values.len() >= 2 {
                    let prev = self.values[self.values.len() - 2];
                    spans.push(Span::raw(" "));
                    spans.push(RatingDelta::between(prev, *last).span());
                }
            }
        }
        Line::from(spans).alignment(self.alignment)
    }
}

impl Widget for RatingTrend<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        render_lines(self.block.clone(), vec![self.line()], area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delta_text() {
        assert_eq!(RatingDelta::between(4.0, 4.0).text(), "● 0.0");
        assert_eq!(RatingDelta::between(4.0, 4.04).text(), "● 0.0");
        assert_eq!(RatingDelta::between(4.0, 4.04).decimals(2).text(), "▲ 0.04");
        assert_eq!(RatingDelta::between(0.0, 4.0).percent(true).text(), "▲ n/a");
        assert_eq!(RatingDelta::between(5.0, 4.0).percent(true).text(), "▼ 20%");
        assert_eq!(RatingDelta::between(f64::NAN, 1.0).diff(), 1.0);
        assert_eq!(RatingDelta::between(2.0, 3.0).relative(), Some(50.0));
        assert_eq!(RatingDelta::between(1.0, 2.0).decimals(99).decimals, 6);
    }

    #[test]
    fn delta_colors_and_render() {
        let d = RatingDelta::between(1.0, 2.0).colors(Color::Blue, Color::Red);
        assert_eq!(d.span().style.fg, Some(Color::Blue));
        assert_eq!(
            RatingDelta::between(2.0, 1.0).span().style.fg,
            Some(Color::Rgb(220, 60, 60))
        );
        assert_eq!(
            RatingDelta::between(2.0, 2.0).span().style.fg,
            Some(Color::DarkGray)
        );
        let mut buf = Buffer::empty(Rect::new(0, 0, 8, 1));
        d.render(Rect::new(0, 0, 8, 1), &mut buf);
    }

    #[test]
    fn trend() {
        let t = RatingTrend::new([1.0, 2.0, 3.0, f64::NAN]);
        assert_eq!(t.levels(), vec![0, 4, 7]); // 3.5 rounds up
        assert_eq!(t.text(), "▁▅█");
        assert_eq!(RatingTrend::new([2.0, 2.0]).levels(), vec![3, 3]);
        assert_eq!(RatingTrend::new(Vec::new()).text(), "");
        let t = RatingTrend::new([4.0]).show_latest(true).color(Color::Cyan);
        assert_eq!(t.text(), "▄ 4.0");
        let t = RatingTrend::new([3.0, 2.0])
            .range(0.0, 5.0)
            .show_latest(true);
        assert_eq!(t.text(), "▅▄ 2.0 ▼ 1.0");
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 3));
        t.gradient([Color::Red, Color::Green])
            .alignment(Alignment::Right)
            .block(Block::bordered())
            .render(Rect::new(0, 0, 20, 3), &mut buf);
    }
}
