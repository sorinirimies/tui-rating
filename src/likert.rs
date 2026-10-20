//! [`LikertScale`]: labelled agreement / satisfaction scales.

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Widget},
};

use crate::util::{lerp_gradient, render_lines, traffic_light_colors};

/// Where [`LikertScale`] shows its labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LikertLabels {
    /// First label left, last label right of the dots.
    #[default]
    Ends,
    /// Only the label of the selected point, after the dots.
    Selected,
    /// No labels.
    None,
}

/// A Likert scale: a row of points with a label for each, e.g.
/// `Strongly disagree ○ ○ ● ○ ○ Strongly agree`.
///
/// ```
/// use tui_rating::LikertScale;
///
/// let s = LikertScale::agreement(Some(3));
/// assert_eq!(s.selected_label(), Some("Agree"));
/// assert_eq!(s.text(), "Strongly disagree ○ ○ ○ ● ○ Strongly agree");
/// ```
#[derive(Debug, Clone)]
pub struct LikertScale<'a> {
    labels: Vec<String>,
    selected: Option<usize>,
    mode: LikertLabels,
    colors: Vec<Color>,
    glyphs: (String, String),
    alignment: Alignment,
    block: Option<Block<'a>>,
}

impl<'a> LikertScale<'a> {
    /// Creates a scale from `labels` (lowest first). `selected` is 0-based.
    pub fn new<I, S>(labels: I, selected: Option<usize>) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let labels: Vec<String> = labels.into_iter().map(Into::into).collect();
        let selected = selected.filter(|&i| i < labels.len());
        Self {
            labels,
            selected,
            mode: LikertLabels::Ends,
            colors: traffic_light_colors(),
            glyphs: ("●".into(), "○".into()),
            alignment: Alignment::Left,
            block: None,
        }
    }

    /// Strongly disagree … Strongly agree (5 points).
    pub fn agreement(selected: Option<usize>) -> Self {
        Self::new(
            [
                "Strongly disagree",
                "Disagree",
                "Neutral",
                "Agree",
                "Strongly agree",
            ],
            selected,
        )
    }

    /// Seven-point agreement scale.
    pub fn agreement7(selected: Option<usize>) -> Self {
        Self::new(
            [
                "Strongly disagree",
                "Disagree",
                "Somewhat disagree",
                "Neutral",
                "Somewhat agree",
                "Agree",
                "Strongly agree",
            ],
            selected,
        )
    }

    /// Very dissatisfied … Very satisfied (5 points).
    pub fn satisfaction(selected: Option<usize>) -> Self {
        Self::new(
            [
                "Very dissatisfied",
                "Dissatisfied",
                "Neutral",
                "Satisfied",
                "Very satisfied",
            ],
            selected,
        )
    }

    /// Never … Always (5 points).
    pub fn frequency(selected: Option<usize>) -> Self {
        Self::new(
            ["Never", "Rarely", "Sometimes", "Often", "Always"],
            selected,
        )
    }

    /// Where labels are shown (default [`LikertLabels::Ends`]).
    #[must_use]
    pub fn labels(mut self, mode: LikertLabels) -> Self {
        self.mode = mode;
        self
    }

    /// Gradient (low → high) used to colour the selected point.
    #[must_use]
    pub fn gradient(mut self, colors: impl Into<Vec<Color>>) -> Self {
        self.colors = colors.into();
        self
    }

    /// Selected / unselected glyphs.
    #[must_use]
    pub fn glyphs(mut self, selected: impl Into<String>, other: impl Into<String>) -> Self {
        self.glyphs = (selected.into(), other.into());
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

    /// Number of points.
    pub fn len(&self) -> usize {
        self.labels.len()
    }

    /// `true` without points.
    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }

    /// Label of the selected point.
    pub fn selected_label(&self) -> Option<&str> {
        self.selected.map(|i| self.labels[i].as_str())
    }

    fn color_at(&self, i: usize) -> Color {
        let t = if self.labels.len() <= 1 {
            0.0
        } else {
            i as f64 / (self.labels.len() - 1) as f64
        };
        lerp_gradient(&self.colors, t).unwrap_or(Color::Yellow)
    }

    /// Plain text of the line.
    pub fn text(&self) -> String {
        self.line()
            .spans
            .iter()
            .map(|s| s.content.as_ref())
            .collect()
    }

    /// Point index at column `x` (hit testing).
    pub fn index_at(&self, area: Rect, x: u16, y: u16) -> Option<usize> {
        let inner = match &self.block {
            Some(b) => b.inner(area),
            None => area,
        };
        if y != inner.y || inner.height == 0 {
            return None;
        }
        let line = self.line();
        let start = crate::util::align_x(
            inner,
            line.width().min(u16::MAX as usize) as u16,
            self.alignment,
        );
        let mut col = i64::from(start);
        let mut dot = 0usize;
        let lead = match self.mode {
            LikertLabels::Ends => self
                .labels
                .first()
                .map_or(0, |l| crate::util::cell_width(l) + 1),
            _ => 0,
        };
        col += lead as i64;
        let gw = crate::util::cell_width(&self.glyphs.0)
            .max(crate::util::cell_width(&self.glyphs.1)) as i64;
        while dot < self.labels.len() {
            if i64::from(x) >= col && i64::from(x) < col + gw {
                return Some(dot);
            }
            col += gw + 1;
            dot += 1;
        }
        None
    }

    /// The styled line.
    pub fn line(&self) -> Line<'a> {
        let dim = Style::default().fg(Color::DarkGray);
        let mut spans = Vec::new();
        if self.mode == LikertLabels::Ends {
            if let Some(first) = self.labels.first() {
                spans.push(Span::styled(first.clone(), dim));
                spans.push(Span::raw(" "));
            }
        }
        for i in 0..self.labels.len() {
            if i > 0 {
                spans.push(Span::raw(" "));
            }
            if self.selected == Some(i) {
                spans.push(Span::styled(
                    self.glyphs.0.clone(),
                    Style::default()
                        .fg(self.color_at(i))
                        .add_modifier(Modifier::BOLD),
                ));
            } else {
                spans.push(Span::styled(self.glyphs.1.clone(), dim));
            }
        }
        match self.mode {
            LikertLabels::Ends if self.labels.len() > 1 => {
                spans.push(Span::raw(" "));
                spans.push(Span::styled(
                    self.labels[self.labels.len() - 1].clone(),
                    dim,
                ));
            }
            LikertLabels::Selected => {
                if let Some(i) = self.selected {
                    spans.push(Span::styled(
                        format!("  {}", self.labels[i]),
                        Style::default()
                            .fg(self.color_at(i))
                            .add_modifier(Modifier::BOLD),
                    ));
                }
            }
            _ => {}
        }
        Line::from(spans).alignment(self.alignment)
    }
}

impl Widget for LikertScale<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        render_lines(self.block.clone(), vec![self.line()], area, buf);
    }
}

/// Selection state for a [`LikertScale`].
///
/// ```
/// use tui_rating::LikertState;
///
/// let mut s = LikertState::new(5);
/// s.next();
/// s.next();
/// assert_eq!(s.selected(), Some(1));
/// s.prev();
/// s.prev();
/// assert_eq!(s.selected(), Some(0));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LikertState {
    selected: Option<usize>,
    len: usize,
}

impl LikertState {
    /// Nothing selected, `len` points.
    pub fn new(len: usize) -> Self {
        Self {
            selected: None,
            len,
        }
    }

    /// Selected index.
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    /// Selects `index` (ignored if out of range).
    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index.filter(|&i| i < self.len);
    }

    /// Moves right; selects the first point if none is selected.
    pub fn next(&mut self) {
        self.selected = match self.selected {
            None if self.len > 0 => Some(0),
            Some(i) => Some((i + 1).min(self.len.saturating_sub(1))),
            s => s,
        };
    }

    /// Moves left; selects the first point if none is selected.
    pub fn prev(&mut self) {
        self.selected = match self.selected {
            None if self.len > 0 => Some(0),
            Some(i) => Some(i.saturating_sub(1)),
            s => s,
        };
    }

    /// Applies a key: `←→` `h` `l`, `1`–`9` pick a point, `Backspace` clears.
    /// Returns `true` if the selection changed (feature `crossterm`).
    #[cfg(feature = "crossterm")]
    pub fn handle_key(&mut self, key: ratatui::crossterm::event::KeyEvent) -> bool {
        use ratatui::crossterm::event::{KeyCode, KeyEventKind};
        if key.kind == KeyEventKind::Release {
            return false;
        }
        let before = self.selected;
        match key.code {
            KeyCode::Left | KeyCode::Char('h') => self.prev(),
            KeyCode::Right | KeyCode::Char('l') => self.next(),
            KeyCode::Backspace | KeyCode::Delete => self.selected = None,
            KeyCode::Char(c @ '1'..='9') => self.select(Some(c as usize - '1' as usize)),
            _ => return false,
        }
        self.selected != before
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_modes() {
        let s = LikertScale::satisfaction(Some(4));
        assert_eq!(s.len(), 5);
        assert!(!s.is_empty());
        assert_eq!(s.selected_label(), Some("Very satisfied"));
        let sel = s.clone().labels(LikertLabels::Selected);
        assert_eq!(sel.text(), "○ ○ ○ ○ ●  Very satisfied");
        assert_eq!(s.clone().labels(LikertLabels::None).text(), "○ ○ ○ ○ ●");
        assert_eq!(LikertScale::agreement7(None).len(), 7);
        assert_eq!(LikertScale::frequency(Some(99)).selected_label(), None);
        let e = LikertScale::new(Vec::<String>::new(), None);
        assert!(e.is_empty());
        assert_eq!(e.text(), "");
    }

    #[test]
    fn hit_testing() {
        let s = LikertScale::new(["a", "b", "c"], Some(0)).glyphs("#", "-");
        // "a # - - c": dots at x = 2, 4, 6
        let area = Rect::new(0, 0, 20, 1);
        assert_eq!(s.index_at(area, 2, 0), Some(0));
        assert_eq!(s.index_at(area, 6, 0), Some(2));
        assert_eq!(s.index_at(area, 3, 0), None);
        assert_eq!(s.index_at(area, 2, 3), None);
        let none = s.clone().labels(LikertLabels::None);
        assert_eq!(none.index_at(area, 0, 0), Some(0));
    }

    #[test]
    fn renders() {
        let s = LikertScale::agreement(Some(2))
            .gradient([Color::Red, Color::Green])
            .alignment(Alignment::Center)
            .block(Block::bordered());
        let area = Rect::new(0, 0, 60, 3);
        let mut buf = Buffer::empty(area);
        s.render(area, &mut buf);
        let mut b = Buffer::empty(Rect::new(0, 0, 0, 0));
        LikertScale::agreement(None).render(Rect::new(0, 0, 0, 0), &mut b);
        let one = LikertScale::new(["only"], Some(0));
        assert_eq!(one.color_at(0), Color::Rgb(220, 60, 60));
    }

    #[test]
    fn state_navigation() {
        let mut s = LikertState::new(3);
        s.prev();
        assert_eq!(s.selected(), Some(0));
        s.next();
        s.next();
        s.next();
        assert_eq!(s.selected(), Some(2));
        s.select(Some(9));
        assert_eq!(s.selected(), None);
        let mut e = LikertState::new(0);
        e.next();
        e.prev();
        assert_eq!(e.selected(), None);
    }

    #[cfg(feature = "crossterm")]
    #[test]
    fn keys() {
        use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
        let k = |c| KeyEvent::new(c, KeyModifiers::NONE);
        let mut s = LikertState::new(5);
        assert!(s.handle_key(k(KeyCode::Right)));
        assert!(s.handle_key(k(KeyCode::Char('4'))));
        assert_eq!(s.selected(), Some(3));
        assert!(s.handle_key(k(KeyCode::Left)));
        assert!(s.handle_key(k(KeyCode::Backspace)));
        assert!(!s.handle_key(k(KeyCode::Enter)));
    }
}
