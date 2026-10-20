//! [`RatingRange`]: a two-handle range selector (e.g. a score band).

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Widget},
};

use crate::util::{number_text, render_lines};

/// Which handle of a [`RangeState`] is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Handle {
    /// The lower bound.
    #[default]
    Low,
    /// The upper bound.
    High,
}

/// `lo..=hi` inside `min..=max`, with an active handle.
///
/// ```
/// use tui_rating::RangeState;
///
/// let mut r = RangeState::new(0.0, 5.0, 1.0, 4.0);
/// r.move_active(1.0);            // low 1 → 2
/// r.toggle();
/// r.move_active(5.0);            // high clamps to max
/// assert_eq!((r.low(), r.high()), (2.0, 5.0));
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RangeState {
    min: f64,
    max: f64,
    lo: f64,
    hi: f64,
    step: f64,
    active: Handle,
}

impl RangeState {
    /// Creates a state; bounds and handles are sanitised (`min ≤ lo ≤ hi ≤ max`).
    pub fn new(min: f64, max: f64, lo: f64, hi: f64) -> Self {
        let fin = |v: f64, d: f64| if v.is_finite() { v } else { d };
        let (min, max) = (fin(min, 0.0), fin(max, 0.0));
        let (min, max) = if min <= max { (min, max) } else { (max, min) };
        let lo = fin(lo, min).clamp(min, max);
        let hi = fin(hi, max).clamp(min, max);
        let (lo, hi) = if lo <= hi { (lo, hi) } else { (hi, lo) };
        Self {
            min,
            max,
            lo,
            hi,
            step: 1.0,
            active: Handle::Low,
        }
    }

    /// Step of keyboard moves (default 1, must be positive).
    #[must_use]
    pub fn with_step(mut self, step: f64) -> Self {
        if step.is_finite() && step > 0.0 {
            self.step = step;
        }
        self
    }

    /// Lower bound of the selection.
    pub fn low(&self) -> f64 {
        self.lo
    }

    /// Upper bound of the selection.
    pub fn high(&self) -> f64 {
        self.hi
    }

    /// Scale minimum.
    pub fn min(&self) -> f64 {
        self.min
    }

    /// Scale maximum.
    pub fn max(&self) -> f64 {
        self.max
    }

    /// Step size.
    pub fn step(&self) -> f64 {
        self.step
    }

    /// The active handle.
    pub fn active(&self) -> Handle {
        self.active
    }

    /// Selects the active handle.
    pub fn set_active(&mut self, h: Handle) {
        self.active = h;
    }

    /// Switches the active handle.
    pub fn toggle(&mut self) {
        self.active = match self.active {
            Handle::Low => Handle::High,
            Handle::High => Handle::Low,
        };
    }

    /// Moves the active handle by `delta`; handles never cross.
    pub fn move_active(&mut self, delta: f64) {
        if !delta.is_finite() {
            return;
        }
        match self.active {
            Handle::Low => self.lo = (self.lo + delta).clamp(self.min, self.hi),
            Handle::High => self.hi = (self.hi + delta).clamp(self.lo, self.max),
        }
    }

    /// Applies a key: `←→` `h` `l` move, `Tab`/`Space` switch handle,
    /// `Home`/`End` jump to the scale ends. Returns `true` on change
    /// (feature `crossterm`).
    #[cfg(feature = "crossterm")]
    pub fn handle_key(&mut self, key: ratatui::crossterm::event::KeyEvent) -> bool {
        use ratatui::crossterm::event::{KeyCode, KeyEventKind};
        if key.kind == KeyEventKind::Release {
            return false;
        }
        let before = (self.lo, self.hi, self.active);
        match key.code {
            KeyCode::Left | KeyCode::Char('h') => self.move_active(-self.step),
            KeyCode::Right | KeyCode::Char('l') => self.move_active(self.step),
            KeyCode::Tab | KeyCode::Char(' ') => self.toggle(),
            KeyCode::Home => self.move_active(f64::MIN / 2.0),
            KeyCode::End => self.move_active(f64::MAX / 2.0),
            _ => return false,
        }
        (self.lo, self.hi, self.active) != before
    }
}

/// Renders a [`RangeState`] as `1 ├──●━━━━━●──┤ 4`.
///
/// ```
/// use tui_rating::{RangeState, RatingRange};
///
/// let r = RatingRange::from_state(&RangeState::new(0.0, 5.0, 1.0, 4.0));
/// assert_eq!(r.value_text(), "1 – 4");
/// ```
#[derive(Debug, Clone)]
pub struct RatingRange<'a> {
    min: f64,
    max: f64,
    lo: f64,
    hi: f64,
    active: Option<Handle>,
    color: Color,
    show_values: bool,
    alignment: Alignment,
    block: Option<Block<'a>>,
}

impl<'a> RatingRange<'a> {
    /// Creates a range widget (see [`RangeState::new`] for sanitising).
    pub fn new(min: f64, max: f64, lo: f64, hi: f64) -> Self {
        Self::from_state(&RangeState::new(min, max, lo, hi))
    }

    /// Creates a widget from a state, highlighting its active handle.
    pub fn from_state(s: &RangeState) -> Self {
        Self {
            min: s.min,
            max: s.max,
            lo: s.lo,
            hi: s.hi,
            active: Some(s.active),
            color: Color::Cyan,
            show_values: true,
            alignment: Alignment::Left,
            block: None,
        }
    }

    /// Colour of the selected span.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Shows the bounds around the track (default on).
    #[must_use]
    pub fn show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    /// Which handle looks active (`None` for neither).
    #[must_use]
    pub fn active(mut self, active: Option<Handle>) -> Self {
        self.active = active;
        self
    }

    /// Alignment (only visible when the area is wider than the content).
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

    /// `"1 – 4"`.
    pub fn value_text(&self) -> String {
        format!("{} – {}", number_text(self.lo), number_text(self.hi))
    }

    fn cell_of(&self, v: f64, track: usize) -> usize {
        if track <= 1 || self.max <= self.min {
            return 0;
        }
        (((v - self.min) / (self.max - self.min)) * (track - 1) as f64).round() as usize
    }

    /// Value at column `x`, for a track rendered in `area` (mouse support).
    /// Returns `None` outside the track.
    pub fn value_at(&self, area: Rect, x: u16, y: u16) -> Option<f64> {
        let inner = match &self.block {
            Some(b) => b.inner(area),
            None => area,
        };
        if inner.height == 0 || y != inner.y {
            return None;
        }
        let (prefix, suffix) = self.affixes();
        let track =
            (inner.width as usize).saturating_sub(prefix.chars().count() + suffix.chars().count());
        let start = inner.x as usize + prefix.chars().count();
        let x = x as usize;
        if track == 0 || x < start || x >= start + track {
            return None;
        }
        let t = if track <= 1 {
            0.0
        } else {
            (x - start) as f64 / (track - 1) as f64
        };
        Some(self.min + t * (self.max - self.min))
    }

    fn affixes(&self) -> (String, String) {
        if self.show_values {
            (
                format!("{} ", number_text(self.lo)),
                format!(" {}", number_text(self.hi)),
            )
        } else {
            (String::new(), String::new())
        }
    }

    /// The styled line for `width` cells.
    pub fn line(&self, width: usize) -> Line<'a> {
        let (prefix, suffix) = self.affixes();
        let track = width.saturating_sub(prefix.chars().count() + suffix.chars().count());
        let mut spans = vec![Span::styled(prefix, Style::default().fg(Color::DarkGray))];
        let (a, b) = (self.cell_of(self.lo, track), self.cell_of(self.hi, track));
        let on = Style::default().fg(self.color);
        let off = Style::default().fg(Color::DarkGray);
        for i in 0..track {
            let (glyph, style) = if i == a || i == b {
                let handle = if i == a && self.active == Some(Handle::Low) && a != b
                    || i == b && self.active == Some(Handle::High) && a != b
                    || a == b && self.active.is_some()
                {
                    "◉"
                } else {
                    "●"
                };
                (handle, on.add_modifier(Modifier::BOLD))
            } else if i > a && i < b {
                ("━", on)
            } else {
                ("─", off)
            };
            spans.push(Span::styled(glyph, style));
        }
        spans.push(Span::styled(suffix, Style::default().fg(Color::DarkGray)));
        Line::from(spans).alignment(self.alignment)
    }
}

impl Widget for RatingRange<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = match &self.block {
            Some(b) => b.inner(area).width,
            None => area.width,
        };
        render_lines(self.block.clone(), vec![self.line(w as usize)], area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(l: &Line<'_>) -> String {
        l.spans.iter().map(|s| s.content.as_ref()).collect()
    }

    #[test]
    fn state_sanitises_and_moves() {
        let s = RangeState::new(5.0, 0.0, 9.0, -3.0);
        assert_eq!((s.min(), s.max(), s.low(), s.high()), (0.0, 5.0, 0.0, 5.0));
        let s = RangeState::new(f64::NAN, f64::NAN, f64::NAN, f64::NAN);
        assert_eq!((s.low(), s.high()), (0.0, 0.0));
        let mut s = RangeState::new(0.0, 5.0, 2.0, 3.0)
            .with_step(0.5)
            .with_step(-1.0);
        assert_eq!(s.step(), 0.5);
        s.move_active(9.0);
        assert_eq!(s.low(), 3.0, "low stops at high");
        s.move_active(f64::NAN);
        s.set_active(Handle::High);
        assert_eq!(s.active(), Handle::High);
        s.move_active(-9.0);
        assert_eq!(s.high(), 3.0, "high stops at low");
    }

    #[test]
    fn line_and_text() {
        let r = RatingRange::new(0.0, 4.0, 1.0, 3.0).active(None);
        assert_eq!(r.value_text(), "1 – 3");
        assert_eq!(text(&r.line(15)), "1 ───●━━━━●── 3");
        let l = text(
            &RatingRange::new(0.0, 4.0, 1.0, 3.0)
                .show_values(false)
                .active(None)
                .line(5),
        );
        assert_eq!(l, "─●━●─");
        let l = text(
            &RatingRange::new(0.0, 4.0, 2.0, 2.0)
                .show_values(false)
                .line(5),
        );
        assert_eq!(l, "──◉──");
        assert_eq!(text(&RatingRange::new(0.0, 0.0, 0.0, 0.0).line(1)), "0  0");
    }

    #[test]
    fn value_at_and_render() {
        let r = RatingRange::new(0.0, 4.0, 1.0, 3.0).show_values(false);
        let area = Rect::new(0, 0, 5, 1);
        assert_eq!(r.value_at(area, 0, 0), Some(0.0));
        assert_eq!(r.value_at(area, 4, 0), Some(4.0));
        assert_eq!(r.value_at(area, 5, 0), None);
        assert_eq!(r.value_at(area, 1, 1), None);
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 3));
        RatingRange::from_state(&RangeState::new(0.0, 5.0, 1.0, 4.0))
            .color(Color::Red)
            .alignment(Alignment::Center)
            .block(Block::bordered())
            .render(Rect::new(0, 0, 20, 3), &mut buf);
    }

    #[cfg(feature = "crossterm")]
    #[test]
    fn keys() {
        use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
        let k = |c| KeyEvent::new(c, KeyModifiers::NONE);
        let mut s = RangeState::new(0.0, 5.0, 1.0, 4.0);
        assert!(s.handle_key(k(KeyCode::Right)));
        assert!(s.handle_key(k(KeyCode::Tab)));
        assert!(s.handle_key(k(KeyCode::End)));
        assert_eq!(s.high(), 5.0);
        assert!(!s.handle_key(k(KeyCode::Right)));
        assert!(s.handle_key(k(KeyCode::Char(' '))));
        assert!(s.handle_key(k(KeyCode::Home)));
        assert_eq!(s.low(), 0.0);
        assert!(!s.handle_key(k(KeyCode::Enter)));
    }
}
