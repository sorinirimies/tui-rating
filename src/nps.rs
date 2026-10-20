//! [`NpsRating`] (0–10 selector) and [`NpsSummary`] (Net Promoter Score).

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Widget},
};

use crate::util::{render_lines, thousands};

/// NPS bucket of a 0–10 answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpsCategory {
    /// 0–6.
    Detractor,
    /// 7–8.
    Passive,
    /// 9–10.
    Promoter,
}

impl NpsCategory {
    /// Buckets a score (values above 10 count as 10).
    pub fn from_score(score: u8) -> Self {
        match score {
            0..=6 => Self::Detractor,
            7..=8 => Self::Passive,
            _ => Self::Promoter,
        }
    }

    /// `"Detractor"`, `"Passive"` or `"Promoter"`.
    pub fn label(self) -> &'static str {
        match self {
            Self::Detractor => "Detractor",
            Self::Passive => "Passive",
            Self::Promoter => "Promoter",
        }
    }

    /// Red / yellow / green.
    pub fn color(self) -> Color {
        match self {
            Self::Detractor => Color::Rgb(220, 60, 60),
            Self::Passive => Color::Rgb(240, 200, 50),
            Self::Promoter => Color::Rgb(70, 190, 90),
        }
    }
}

/// The classic 0–10 "how likely are you to recommend us?" selector, coloured
/// by detractor / passive / promoter.
///
/// ```text
///  0 1 2 3 4 5 6 7 8 9 10  Promoter   (9 highlighted)
/// ```
///
/// ```
/// use tui_rating::{NpsCategory, NpsRating};
///
/// let r = NpsRating::new(Some(9)).show_category(true);
/// assert_eq!(r.text(), " 0 1 2 3 4 5 6 7 8 9 10  Promoter");
/// assert_eq!(NpsCategory::from_score(6), NpsCategory::Detractor);
/// ```
#[derive(Debug, Clone)]
pub struct NpsRating<'a> {
    score: Option<u8>,
    show_category: bool,
    alignment: Alignment,
    block: Option<Block<'a>>,
}

impl<'a> NpsRating<'a> {
    /// Creates a selector; scores above 10 are clamped.
    pub fn new(score: Option<u8>) -> Self {
        Self {
            score: score.map(|s| s.min(10)),
            show_category: false,
            alignment: Alignment::Left,
            block: None,
        }
    }

    /// Appends the category name of the selected score.
    #[must_use]
    pub fn show_category(mut self, show: bool) -> Self {
        self.show_category = show;
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

    /// Category of the selected score.
    pub fn category(&self) -> Option<NpsCategory> {
        self.score.map(NpsCategory::from_score)
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
        let mut spans = Vec::new();
        for n in 0..=10u8 {
            let color = NpsCategory::from_score(n).color();
            let selected = self.score == Some(n);
            let style = if selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(color)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(color).add_modifier(Modifier::DIM)
            };
            spans.push(Span::raw(" "));
            spans.push(Span::styled(n.to_string(), style));
        }
        if self.show_category {
            if let Some(c) = self.category() {
                spans.push(Span::styled(
                    format!("  {}", c.label()),
                    Style::default().fg(c.color()).add_modifier(Modifier::BOLD),
                ));
            }
        }
        Line::from(spans).alignment(self.alignment)
    }
}

impl Widget for NpsRating<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        render_lines(self.block.clone(), vec![self.line()], area, buf);
    }
}

/// Aggregated Net Promoter Score: `%promoters − %detractors`, in `−100..=100`.
///
/// ```
/// use tui_rating::NpsSummary;
///
/// let s = NpsSummary::from_scores([10, 9, 9, 8, 3]);
/// assert_eq!(s.score(), Some(40.0)); // 60 % promoters − 20 % detractors
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NpsSummary {
    /// Answers 0–6.
    pub detractors: u64,
    /// Answers 7–8.
    pub passives: u64,
    /// Answers 9–10.
    pub promoters: u64,
}

impl NpsSummary {
    /// From bucket counts.
    pub fn new(detractors: u64, passives: u64, promoters: u64) -> Self {
        Self {
            detractors,
            passives,
            promoters,
        }
    }

    /// Buckets raw 0–10 answers.
    pub fn from_scores(scores: impl IntoIterator<Item = u8>) -> Self {
        let mut s = Self::default();
        for score in scores {
            match NpsCategory::from_score(score) {
                NpsCategory::Detractor => s.detractors += 1,
                NpsCategory::Passive => s.passives += 1,
                NpsCategory::Promoter => s.promoters += 1,
            }
        }
        s
    }

    /// Total answers.
    pub fn total(&self) -> u64 {
        self.detractors
            .saturating_add(self.passives)
            .saturating_add(self.promoters)
    }

    /// The NPS (`None` without answers).
    pub fn score(&self) -> Option<f64> {
        let t = self.total();
        (t > 0).then(|| (self.promoters as f64 - self.detractors as f64) / t as f64 * 100.0)
    }

    /// Share of each bucket `[detractors, passives, promoters]` in `0..=1`.
    pub fn shares(&self) -> [f64; 3] {
        let t = self.total();
        if t == 0 {
            return [0.0; 3];
        }
        [self.detractors, self.passives, self.promoters].map(|n| n as f64 / t as f64)
    }

    /// A widget drawing `NPS +42 ████▒▒▒ (1,203)`.
    pub fn widget(&self) -> NpsSummaryWidget<'static> {
        NpsSummaryWidget {
            summary: *self,
            block: None,
        }
    }
}

/// Renders an [`NpsSummary`]; create it with [`NpsSummary::widget`].
#[derive(Debug, Clone)]
pub struct NpsSummaryWidget<'a> {
    summary: NpsSummary,
    block: Option<Block<'a>>,
}

impl<'a> NpsSummaryWidget<'a> {
    /// Wraps the widget in a block.
    #[must_use]
    pub fn block<'b>(self, block: Block<'b>) -> NpsSummaryWidget<'b> {
        NpsSummaryWidget {
            summary: self.summary,
            block: Some(block),
        }
    }

    /// `"NPS +42"`, or `"NPS –"` without answers.
    pub fn score_text(&self) -> String {
        match self.summary.score() {
            Some(s) => format!("NPS {:+}", s.round() as i64),
            None => "NPS –".into(),
        }
    }

    fn line(&self, width: usize) -> Line<'a> {
        let head = format!("{} ", self.score_text());
        let tail = format!(" ({})", thousands(self.summary.total()));
        let bar_w = width.saturating_sub(head.chars().count() + tail.chars().count());
        let score_color = match self.summary.score() {
            Some(s) if s >= 50.0 => NpsCategory::Promoter.color(),
            Some(s) if s >= 0.0 => NpsCategory::Passive.color(),
            Some(_) => NpsCategory::Detractor.color(),
            None => Color::DarkGray,
        };
        let mut spans = vec![Span::styled(
            head,
            Style::default()
                .fg(score_color)
                .add_modifier(Modifier::BOLD),
        )];
        let shares = self.summary.shares();
        let cats = [
            NpsCategory::Detractor,
            NpsCategory::Passive,
            NpsCategory::Promoter,
        ];
        let mut used = 0usize;
        let mut acc = 0.0;
        for (share, cat) in shares.iter().zip(cats) {
            acc += share;
            let end = ((acc * bar_w as f64).round() as usize).min(bar_w);
            if end > used {
                spans.push(Span::styled(
                    "█".repeat(end - used),
                    Style::default().fg(cat.color()),
                ));
                used = end;
            }
        }
        if used < bar_w {
            spans.push(Span::styled(
                "░".repeat(bar_w - used),
                Style::default().fg(Color::DarkGray),
            ));
        }
        spans.push(Span::styled(tail, Style::default().fg(Color::DarkGray)));
        Line::from(spans)
    }
}

impl Widget for NpsSummaryWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner_w = match &self.block {
            Some(b) => b.inner(area).width,
            None => area.width,
        };
        render_lines(
            self.block.clone(),
            vec![self.line(inner_w as usize)],
            area,
            buf,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn categories() {
        assert_eq!(NpsCategory::from_score(0), NpsCategory::Detractor);
        assert_eq!(NpsCategory::from_score(7), NpsCategory::Passive);
        assert_eq!(NpsCategory::from_score(8), NpsCategory::Passive);
        assert_eq!(NpsCategory::from_score(10), NpsCategory::Promoter);
        assert_eq!(NpsCategory::from_score(200), NpsCategory::Promoter);
        assert_eq!(NpsCategory::Passive.label(), "Passive");
    }

    #[test]
    fn selector_text_and_clamp() {
        let r = NpsRating::new(Some(99));
        assert_eq!(r.category(), Some(NpsCategory::Promoter));
        assert!(NpsRating::new(None).category().is_none());
        assert!(NpsRating::new(Some(3))
            .show_category(true)
            .text()
            .ends_with("Detractor"));
        let area = Rect::new(0, 0, 40, 1);
        let mut buf = Buffer::empty(area);
        NpsRating::new(Some(10))
            .alignment(Alignment::Center)
            .block(Block::new())
            .render(area, &mut buf);
    }

    #[test]
    fn summary_math() {
        assert_eq!(NpsSummary::default().score(), None);
        assert_eq!(NpsSummary::default().shares(), [0.0; 3]);
        let s = NpsSummary::new(1, 1, 3);
        assert_eq!(s.total(), 5);
        assert_eq!(s.score(), Some(40.0));
        assert_eq!(NpsSummary::from_scores([0, 7, 9]).total(), 3);
        assert_eq!(NpsSummary::new(5, 0, 0).score(), Some(-100.0));
    }

    #[test]
    fn summary_renders() {
        let w = NpsSummary::new(10, 20, 70).widget();
        assert_eq!(w.score_text(), "NPS +60");
        assert_eq!(NpsSummary::default().widget().score_text(), "NPS –");
        let area = Rect::new(0, 0, 40, 3);
        let mut buf = Buffer::empty(area);
        w.block(Block::bordered()).render(area, &mut buf);
        let mut buf = Buffer::empty(Rect::new(0, 0, 0, 0));
        NpsSummary::new(1, 1, 1)
            .widget()
            .render(Rect::new(0, 0, 0, 0), &mut buf);
        let mut buf = Buffer::empty(Rect::new(0, 0, 12, 1));
        NpsSummary::new(0, 0, 0)
            .widget()
            .render(Rect::new(0, 0, 12, 1), &mut buf);
    }
}
