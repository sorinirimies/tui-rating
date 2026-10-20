//! [`RatingForm`] state and the [`MultiCriteria`] widget: several labelled
//! ratings (service, quality, value …) with an overall average.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Widget},
};

use crate::{
    util::{cell_width, pad_to, render_lines},
    Rating, RatingState, RatingStyle,
};

/// Several labelled [`RatingState`]s with a focused row: the state of a
/// multi-criteria review form.
///
/// ```
/// use tui_rating::RatingForm;
///
/// let mut form = RatingForm::new(["Service", "Quality", "Value"], 5);
/// form.focused_mut().set_value(4.0);
/// form.focus_next();
/// form.focused_mut().set_value(5.0);
/// assert_eq!(form.average(), Some(3.0)); // over all rows (unrated count as 0)
/// assert_eq!(form.rated_average(), Some(4.5)); // only the rated ones
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RatingForm {
    rows: Vec<(String, RatingState)>,
    focus: usize,
}

impl RatingForm {
    /// One empty rating (`0` of `max`) per label.
    pub fn new<I, S>(labels: I, max: usize) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            rows: labels
                .into_iter()
                .map(|l| (l.into(), RatingState::new(0.0, max)))
                .collect(),
            focus: 0,
        }
    }

    /// Enables half steps on every row.
    #[must_use]
    pub fn with_half_steps(mut self, enabled: bool) -> Self {
        for (_, s) in &mut self.rows {
            s.set_half_steps(enabled);
        }
        self
    }

    /// Number of rows.
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// `true` without rows.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Rows as `(label, state)`.
    pub fn rows(&self) -> &[(String, RatingState)] {
        &self.rows
    }

    /// Index of the focused row.
    pub fn focus(&self) -> usize {
        self.focus
    }

    /// Focuses row `i` (clamped).
    pub fn set_focus(&mut self, i: usize) {
        self.focus = i.min(self.rows.len().saturating_sub(1));
    }

    /// Focuses the next row (wraps).
    pub fn focus_next(&mut self) {
        if !self.rows.is_empty() {
            self.focus = (self.focus + 1) % self.rows.len();
        }
    }

    /// Focuses the previous row (wraps).
    pub fn focus_prev(&mut self) {
        if !self.rows.is_empty() {
            self.focus = (self.focus + self.rows.len() - 1) % self.rows.len();
        }
    }

    /// The focused row's state.
    ///
    /// # Panics
    /// Panics if the form has no rows.
    pub fn focused_mut(&mut self) -> &mut RatingState {
        &mut self.rows[self.focus].1
    }

    /// Mean over all rows (unrated rows count as `0`); `None` if empty.
    pub fn average(&self) -> Option<f64> {
        (!self.rows.is_empty())
            .then(|| self.rows.iter().map(|(_, s)| s.value()).sum::<f64>() / self.rows.len() as f64)
    }

    /// Mean over rated rows only; `None` if none is rated.
    pub fn rated_average(&self) -> Option<f64> {
        let rated: Vec<f64> = self
            .rows
            .iter()
            .map(|(_, s)| s.value())
            .filter(|v| *v > 0.0)
            .collect();
        (!rated.is_empty()).then(|| rated.iter().sum::<f64>() / rated.len() as f64)
    }

    /// `true` once every row has a value.
    pub fn is_complete(&self) -> bool {
        !self.rows.is_empty() && self.rows.iter().all(|(_, s)| !s.is_empty())
    }

    /// Applies a key: `↑↓` `j` `k` `Tab`/`BackTab` move focus, everything
    /// else goes to the focused [`RatingState::handle_key`]. Returns `true`
    /// if anything changed (feature `crossterm`).
    #[cfg(feature = "crossterm")]
    pub fn handle_key(&mut self, key: ratatui::crossterm::event::KeyEvent) -> bool {
        use ratatui::crossterm::event::{KeyCode, KeyEventKind};
        if self.rows.is_empty() || key.kind == KeyEventKind::Release {
            return false;
        }
        let before = self.focus;
        match key.code {
            KeyCode::Down | KeyCode::Tab | KeyCode::Char('j') => self.focus_next(),
            KeyCode::Up | KeyCode::BackTab | KeyCode::Char('k') => self.focus_prev(),
            _ => return self.focused_mut().handle_key(key),
        }
        self.focus != before
    }
}

/// Renders labelled ratings plus an overall row.
///
/// ```
/// use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
/// use tui_rating::MultiCriteria;
///
/// let w = MultiCriteria::new([("Service", 4.0), ("Value", 3.0)], 5).overall(true);
/// assert_eq!(w.overall_value(), Some(3.5));
/// let area = Rect::new(0, 0, 30, 4);
/// let mut buf = Buffer::empty(area);
/// w.render(area, &mut buf);
/// ```
#[derive(Debug, Clone)]
pub struct MultiCriteria<'a> {
    rows: Vec<(String, f64)>,
    max: usize,
    focus: Option<usize>,
    half_steps: bool,
    overall: bool,
    overall_label: String,
    style: RatingStyle,
    block: Option<Block<'a>>,
}

impl<'a> MultiCriteria<'a> {
    /// Creates a widget from `(label, value)` rows, each out of `max`.
    pub fn new<I, S>(rows: I, max: usize) -> Self
    where
        I: IntoIterator<Item = (S, f64)>,
        S: Into<String>,
    {
        Self {
            rows: rows.into_iter().map(|(l, v)| (l.into(), v)).collect(),
            max,
            focus: None,
            half_steps: true,
            overall: false,
            overall_label: "Overall".into(),
            style: RatingStyle::classic(),
            block: None,
        }
    }

    /// Creates a widget from a [`RatingForm`], including its focus.
    pub fn from_form(form: &RatingForm) -> Self {
        let max = form.rows().first().map_or(5, |(_, s)| s.max());
        let mut w = Self::new(
            form.rows()
                .iter()
                .map(|(l, s)| (l.clone(), s.visual_value())),
            max,
        );
        w.focus = Some(form.focus());
        w.half_steps = form.rows().iter().any(|(_, s)| s.half_steps());
        w
    }

    /// Adds a smooth "Overall" row with the mean.
    #[must_use]
    pub fn overall(mut self, show: bool) -> Self {
        self.overall = show;
        self
    }

    /// Text of the overall row (default `Overall`).
    #[must_use]
    pub fn overall_label(mut self, label: impl Into<String>) -> Self {
        self.overall_label = label.into();
        self
    }

    /// Focuses a row.
    #[must_use]
    pub fn focus(mut self, focus: Option<usize>) -> Self {
        self.focus = focus;
        self
    }

    /// Half-step display of the rows (default on).
    #[must_use]
    pub fn half_steps(mut self, enabled: bool) -> Self {
        self.half_steps = enabled;
        self
    }

    /// Applies a [`RatingStyle`] to every row.
    #[must_use]
    pub fn preset(mut self, style: RatingStyle) -> Self {
        self.style = style;
        self
    }

    /// Wraps the widget in a block.
    #[must_use]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// The mean of all rows (`None` if empty).
    pub fn overall_value(&self) -> Option<f64> {
        (!self.rows.is_empty())
            .then(|| self.rows.iter().map(|(_, v)| *v).sum::<f64>() / self.rows.len() as f64)
    }

    /// One line per row (plus the overall row).
    pub fn lines(&self) -> Vec<Line<'a>> {
        let label_w = self
            .rows
            .iter()
            .map(|(l, _)| cell_width(l))
            .chain(self.overall.then(|| cell_width(&self.overall_label)))
            .max()
            .unwrap_or(0);
        let mut lines = Vec::new();
        for (i, (label, v)) in self.rows.iter().enumerate() {
            let r = Rating::new(*v, self.max)
                .half_steps(self.half_steps)
                .preset(self.style.clone())
                .label(format!("{} ", pad_to(label, label_w)))
                .show_value(true)
                .focused(self.focus == Some(i));
            lines.push(r.line());
        }
        if self.overall {
            if let Some(avg) = self.overall_value() {
                let line = Rating::new(avg, self.max)
                    .preset(self.style.clone())
                    .fractional(!self.style.symbols.partials.is_empty())
                    .half_steps(self.style.symbols.partials.is_empty())
                    .label(format!("{} ", pad_to(&self.overall_label, label_w)))
                    .label_style(Style::default().add_modifier(Modifier::BOLD))
                    .show_value(true)
                    .line();
                lines.push(line);
            }
        }
        lines
    }
}

impl Widget for MultiCriteria<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        render_lines(self.block.clone(), self.lines(), area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn form() -> RatingForm {
        RatingForm::new(["A", "Bee"], 5)
    }

    #[test]
    fn form_navigation_and_stats() {
        let mut f = form().with_half_steps(true);
        assert_eq!((f.len(), f.focus()), (2, 0));
        assert!(!f.is_empty());
        assert_eq!(f.average(), Some(0.0));
        assert_eq!(f.rated_average(), None);
        assert!(!f.is_complete());
        f.focus_prev();
        assert_eq!(f.focus(), 1);
        f.focus_next();
        f.focus_next();
        assert_eq!(f.focus(), 1);
        f.set_focus(99);
        assert_eq!(f.focus(), 1);
        f.focused_mut().set_value(2.5);
        f.set_focus(0);
        f.focused_mut().set_value(4.0);
        assert!(f.is_complete());
        assert_eq!(f.average(), Some(3.25));
        let e = RatingForm::new(Vec::<String>::new(), 5);
        assert!(e.is_empty() && e.average().is_none() && !e.is_complete());
        let mut e = e;
        e.focus_next();
        e.focus_prev();
        e.set_focus(3);
        assert_eq!(e.focus(), 0);
    }

    #[test]
    fn widget_lines() {
        let w = MultiCriteria::from_form(&form())
            .overall(true)
            .overall_label("All");
        let lines = w.lines();
        assert_eq!(lines.len(), 3);
        let t: String = lines[0].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(t.starts_with("[A   "), "{t:?}");
        let w = MultiCriteria::new([("x", 5.0)], 5)
            .overall(true)
            .focus(Some(0))
            .half_steps(false)
            .preset(RatingStyle::neon());
        assert_eq!(w.lines().len(), 2);
        assert!(MultiCriteria::new(Vec::<(String, f64)>::new(), 5)
            .overall(true)
            .lines()
            .is_empty());
        let area = Rect::new(0, 0, 30, 5);
        let mut buf = Buffer::empty(area);
        MultiCriteria::new([("Service", 4.0)], 5)
            .block(Block::bordered())
            .render(area, &mut buf);
        assert_eq!(
            MultiCriteria::new([("a", 1.0), ("b", 2.0)], 5).overall_value(),
            Some(1.5)
        );
    }

    #[cfg(feature = "crossterm")]
    #[test]
    fn keys() {
        use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
        let k = |c| KeyEvent::new(c, KeyModifiers::NONE);
        let mut f = form();
        assert!(f.handle_key(k(KeyCode::Down)));
        assert_eq!(f.focus(), 1);
        assert!(f.handle_key(k(KeyCode::Right)));
        assert_eq!(f.rows()[1].1.value(), 1.0);
        assert!(f.handle_key(k(KeyCode::Up)));
        assert!(!f.handle_key(k(KeyCode::Enter)));
        let mut e = RatingForm::new(Vec::<String>::new(), 5);
        assert!(!e.handle_key(k(KeyCode::Down)));
    }
}
