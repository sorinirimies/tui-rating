//! [`RatingSlider`]: a draggable continuous rating with exact numeric entry.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Widget},
};

use crate::util::{number_text, render_lines};

/// State of a [`RatingSlider`]: a value in `min..=max` moved in `step`s,
/// with a numeric entry mode for typing an exact value.
///
/// ```
/// use tui_rating::SliderState;
///
/// let mut s = SliderState::new(0.0, 5.0, 0.1);
/// s.set_value(3.44);
/// assert_eq!(s.value(), 3.4);        // snapped to the step
/// s.begin_entry();
/// for c in "4.25".chars() { s.push_entry(c); }
/// assert!(s.commit_entry());
/// assert_eq!(s.value(), 4.3);        // 4.25 snaps to the step too
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SliderState {
    min: f64,
    max: f64,
    step: f64,
    value: f64,
    #[cfg_attr(feature = "serde", serde(skip))]
    entry: Option<String>,
}

impl SliderState {
    /// Creates a slider at `min`. Invalid bounds/steps are sanitised
    /// (`min ≤ max`, `step > 0`).
    pub fn new(min: f64, max: f64, step: f64) -> Self {
        let fin = |v: f64| if v.is_finite() { v } else { 0.0 };
        let (min, max) = (fin(min), fin(max));
        let (min, max) = if min <= max { (min, max) } else { (max, min) };
        let step = if step.is_finite() && step > 0.0 {
            step
        } else {
            1.0
        };
        Self {
            min,
            max,
            step,
            value: min,
            entry: None,
        }
    }

    /// Builder: initial value.
    #[must_use]
    pub fn with_value(mut self, value: f64) -> Self {
        self.set_value(value);
        self
    }

    /// Current value.
    pub fn value(&self) -> f64 {
        self.value
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

    /// Fraction `0..=1` of the scale.
    pub fn ratio(&self) -> f64 {
        if self.max <= self.min {
            0.0
        } else {
            (self.value - self.min) / (self.max - self.min)
        }
    }

    /// Sets the value, snapped to the step grid and clamped.
    pub fn set_value(&mut self, value: f64) {
        if !value.is_finite() {
            return;
        }
        let snapped = self.min + ((value - self.min) / self.step).round() * self.step;
        // strip float noise such as 0.30000000000000004
        let snapped = (snapped * 1e9).round() / 1e9;
        self.value = snapped.clamp(self.min, self.max);
    }

    /// Moves by `steps` steps (may be negative).
    pub fn nudge(&mut self, steps: i32) {
        self.set_value(self.value + f64::from(steps) * self.step);
    }

    /// `true` while typing a value.
    pub fn is_entering(&self) -> bool {
        self.entry.is_some()
    }

    /// The text typed so far.
    pub fn entry(&self) -> Option<&str> {
        self.entry.as_deref()
    }

    /// Starts numeric entry with an empty buffer.
    pub fn begin_entry(&mut self) {
        self.entry = Some(String::new());
    }

    /// Appends a digit, `.` (once) or a leading `-`. Returns `true` if accepted.
    pub fn push_entry(&mut self, c: char) -> bool {
        let Some(buf) = self.entry.as_mut() else {
            return false;
        };
        let ok = match c {
            '0'..='9' => buf.len() < 12,
            '.' => !buf.contains('.') && buf.len() < 12,
            '-' => buf.is_empty() && self.min < 0.0,
            _ => false,
        };
        if ok {
            buf.push(c);
        }
        ok
    }

    /// Removes the last typed character.
    pub fn backspace_entry(&mut self) {
        if let Some(b) = self.entry.as_mut() {
            b.pop();
        }
    }

    /// Leaves entry mode and applies the typed number (snapped and clamped).
    /// Returns `false` (value untouched) if nothing parseable was typed.
    pub fn commit_entry(&mut self) -> bool {
        let Some(buf) = self.entry.take() else {
            return false;
        };
        match buf.parse::<f64>() {
            Ok(v) => {
                self.set_value(v);
                true
            }
            Err(_) => false,
        }
    }

    /// Leaves entry mode without applying.
    pub fn cancel_entry(&mut self) {
        self.entry = None;
    }

    /// Applies a key (feature `crossterm`). Returns `true` if the value or
    /// entry buffer changed.
    ///
    /// | Mode | Keys |
    /// |------|------|
    /// | normal | `←→` `h` `l` step · `PageUp`/`PageDown` ×10 · `Home`/`End` · `Enter`/`e` start typing |
    /// | typing | digits `.` `-` · `Backspace` · `Enter` commit · `Esc` cancel |
    #[cfg(feature = "crossterm")]
    pub fn handle_key(&mut self, key: ratatui::crossterm::event::KeyEvent) -> bool {
        use ratatui::crossterm::event::{KeyCode, KeyEventKind};
        if key.kind == KeyEventKind::Release {
            return false;
        }
        if self.is_entering() {
            return match key.code {
                KeyCode::Char(c) => self.push_entry(c),
                KeyCode::Backspace => {
                    self.backspace_entry();
                    true
                }
                KeyCode::Enter => {
                    let before = self.value;
                    self.commit_entry();
                    self.value != before
                }
                KeyCode::Esc => {
                    self.cancel_entry();
                    true
                }
                _ => false,
            };
        }
        let before = self.value;
        match key.code {
            KeyCode::Left | KeyCode::Down | KeyCode::Char('h') => self.nudge(-1),
            KeyCode::Right | KeyCode::Up | KeyCode::Char('l') => self.nudge(1),
            KeyCode::PageDown => self.nudge(-10),
            KeyCode::PageUp => self.nudge(10),
            KeyCode::Home => self.value = self.min,
            KeyCode::End => self.value = self.max,
            KeyCode::Enter | KeyCode::Char('e') => {
                self.begin_entry();
                return true;
            }
            _ => return false,
        }
        self.value != before
    }

    /// Applies a mouse click/drag on a `slider` rendered in `area`
    /// (feature `crossterm`). Returns `true` if the value changed.
    #[cfg(feature = "crossterm")]
    pub fn handle_mouse(
        &mut self,
        event: ratatui::crossterm::event::MouseEvent,
        slider: &RatingSlider<'_>,
        area: Rect,
    ) -> bool {
        use ratatui::crossterm::event::{MouseButton, MouseEventKind};
        if !matches!(
            event.kind,
            MouseEventKind::Down(MouseButton::Left) | MouseEventKind::Drag(MouseButton::Left)
        ) {
            return false;
        }
        match slider.value_at(area, event.column, event.row) {
            Some(v) => {
                let before = self.value;
                self.set_value(v);
                self.value != before
            }
            None => false,
        }
    }
}

/// Renders a continuous slider: `━━━━●─────  3.4/5`.
///
/// ```
/// use tui_rating::{RatingSlider, SliderState};
///
/// let s = SliderState::new(0.0, 5.0, 0.5).with_value(2.5);
/// assert_eq!(RatingSlider::from_state(&s).value_text(), "2.5/5");
/// ```
#[derive(Debug, Clone)]
pub struct RatingSlider<'a> {
    min: f64,
    max: f64,
    value: f64,
    entry: Option<String>,
    color: Color,
    show_value: bool,
    block: Option<Block<'a>>,
}

impl<'a> RatingSlider<'a> {
    /// Creates a slider for a value in `min..=max`.
    pub fn new(min: f64, max: f64, value: f64) -> Self {
        let st = SliderState::new(min, max, f64::MIN_POSITIVE.max(1e-9)).with_value(value);
        Self {
            min: st.min,
            max: st.max,
            value: st.value,
            entry: None,
            color: Color::Yellow,
            show_value: true,
            block: None,
        }
    }

    /// Creates a slider from a state (shows the entry buffer while typing).
    pub fn from_state(s: &SliderState) -> Self {
        Self {
            min: s.min,
            max: s.max,
            value: s.value,
            entry: s.entry.clone(),
            color: Color::Yellow,
            show_value: true,
            block: None,
        }
    }

    /// Track colour of the filled part and thumb.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Shows the value text after the track (default on).
    #[must_use]
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Wraps the widget in a block.
    #[must_use]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// `"3.4/5"`, or `"[4.2_]"` while typing.
    pub fn value_text(&self) -> String {
        match &self.entry {
            Some(e) => format!("[{e}_]"),
            None => format!("{}/{}", number_text_prec(self.value), number_text(self.max)),
        }
    }

    fn suffix(&self) -> String {
        if self.show_value {
            format!("  {}", self.value_text())
        } else {
            String::new()
        }
    }

    fn track_width(&self, inner_width: u16) -> usize {
        (inner_width as usize).saturating_sub(self.suffix().chars().count())
    }

    /// Value at column `x` (mouse support); `None` outside the track.
    pub fn value_at(&self, area: Rect, x: u16, y: u16) -> Option<f64> {
        let inner = match &self.block {
            Some(b) => b.inner(area),
            None => area,
        };
        let track = self.track_width(inner.width);
        if inner.height == 0 || y != inner.y || track == 0 || x < inner.x {
            return None;
        }
        let rel = (x - inner.x) as usize;
        if rel >= track {
            return None;
        }
        let t = if track <= 1 {
            0.0
        } else {
            rel as f64 / (track - 1) as f64
        };
        Some(self.min + t * (self.max - self.min))
    }

    /// The styled line for `width` cells.
    pub fn line(&self, width: usize) -> Line<'a> {
        let suffix = self.suffix();
        let track = width.saturating_sub(suffix.chars().count());
        let ratio = if self.max <= self.min {
            0.0
        } else {
            (self.value - self.min) / (self.max - self.min)
        };
        let thumb = if track <= 1 {
            0
        } else {
            (ratio * (track - 1) as f64).round() as usize
        };
        let on = Style::default().fg(self.color);
        let mut spans = Vec::with_capacity(track + 1);
        for i in 0..track {
            spans.push(match i.cmp(&thumb) {
                std::cmp::Ordering::Less => Span::styled("━", on),
                std::cmp::Ordering::Equal => Span::styled("●", on.add_modifier(Modifier::BOLD)),
                std::cmp::Ordering::Greater => {
                    Span::styled("─", Style::default().fg(Color::DarkGray))
                }
            });
        }
        let suffix_style = if self.entry.is_some() {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        spans.push(Span::styled(suffix, suffix_style));
        Line::from(spans)
    }
}

/// Like `number_text` but keeps up to two decimals (`3.45`).
fn number_text_prec(v: f64) -> String {
    let s = format!("{v:.2}");
    let s = s.trim_end_matches('0').trim_end_matches('.');
    if s.is_empty() || s == "-" || s == "-0" {
        "0".into()
    } else {
        s.to_string()
    }
}

impl Widget for RatingSlider<'_> {
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
    fn state_snaps_and_clamps() {
        let mut s = SliderState::new(5.0, 0.0, -1.0);
        assert_eq!((s.min(), s.max(), s.step()), (0.0, 5.0, 1.0));
        s.set_value(f64::NAN);
        assert_eq!(s.value(), 0.0);
        s.set_value(99.0);
        assert_eq!(s.value(), 5.0);
        assert_eq!(s.ratio(), 1.0);
        let mut s = SliderState::new(0.0, 1.0, 0.1);
        s.nudge(3);
        assert_eq!(s.value(), 0.3);
        s.nudge(-99);
        assert_eq!(s.value(), 0.0);
        assert_eq!(SliderState::new(2.0, 2.0, 1.0).ratio(), 0.0);
        assert_eq!(SliderState::new(0.0, 5.0, 1.0).with_value(3.0).value(), 3.0);
    }

    #[test]
    fn entry_mode() {
        let mut s = SliderState::new(0.0, 5.0, 0.5);
        assert!(!s.push_entry('1'), "not entering");
        assert!(!s.commit_entry());
        s.begin_entry();
        assert!(s.is_entering());
        assert!(!s.push_entry('x'));
        assert!(!s.push_entry('-'), "min >= 0");
        for c in "3.7".chars() {
            assert!(s.push_entry(c));
        }
        assert!(!s.push_entry('.'));
        assert_eq!(s.entry(), Some("3.7"));
        s.backspace_entry();
        s.backspace_entry();
        assert_eq!(s.entry(), Some("3"));
        assert!(s.commit_entry());
        assert_eq!(s.value(), 3.0);
        s.begin_entry();
        assert!(!s.commit_entry(), "empty");
        s.begin_entry();
        s.push_entry('1');
        s.cancel_entry();
        assert!(!s.is_entering());
        let mut n = SliderState::new(-5.0, 5.0, 1.0);
        n.begin_entry();
        assert!(n.push_entry('-'));
        n.push_entry('2');
        n.commit_entry();
        assert_eq!(n.value(), -2.0);
        let mut long = SliderState::new(0.0, 5.0, 1.0);
        long.begin_entry();
        for _ in 0..20 {
            long.push_entry('1');
        }
        assert_eq!(long.entry().unwrap().len(), 12);
        long.backspace_entry();
        let mut none = SliderState::new(0.0, 5.0, 1.0);
        none.backspace_entry();
    }

    #[test]
    fn line_and_hit() {
        let s = SliderState::new(0.0, 4.0, 1.0).with_value(2.0);
        let w = RatingSlider::from_state(&s);
        assert_eq!(w.value_text(), "2/4");
        assert_eq!(text(&w.clone().show_value(false).line(5)), "━━●──");
        assert_eq!(text(&w.line(10)), "━━●──  2/4");
        let w = RatingSlider::new(0.0, 4.0, 4.0).show_value(false);
        let area = Rect::new(2, 0, 5, 1);
        assert_eq!(w.value_at(area, 2, 0), Some(0.0));
        assert_eq!(w.value_at(area, 6, 0), Some(4.0));
        assert_eq!(w.value_at(area, 7, 0), None);
        assert_eq!(w.value_at(area, 1, 0), None);
        assert_eq!(w.value_at(area, 3, 1), None);
        let mut e = SliderState::new(0.0, 5.0, 1.0);
        e.begin_entry();
        e.push_entry('4');
        assert_eq!(RatingSlider::from_state(&e).value_text(), "[4_]");
        assert_eq!(number_text_prec(3.456), "3.46");
        assert_eq!(number_text_prec(-0.001), "0");
        assert_eq!(number_text_prec(0.0), "0");
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 3));
        RatingSlider::from_state(&e)
            .color(Color::Red)
            .block(Block::bordered())
            .render(Rect::new(0, 0, 20, 3), &mut buf);
    }

    #[cfg(feature = "crossterm")]
    #[test]
    fn keys_and_mouse() {
        use ratatui::crossterm::event::{
            KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
        };
        let k = |c| KeyEvent::new(c, KeyModifiers::NONE);
        let mut s = SliderState::new(0.0, 20.0, 1.0);
        assert!(s.handle_key(k(KeyCode::Right)));
        assert!(s.handle_key(k(KeyCode::PageUp)));
        assert_eq!(s.value(), 11.0);
        assert!(s.handle_key(k(KeyCode::Home)));
        assert!(s.handle_key(k(KeyCode::End)));
        assert!(!s.handle_key(k(KeyCode::Char('l'))));
        assert!(!s.handle_key(k(KeyCode::Tab)));
        assert!(s.handle_key(k(KeyCode::Enter)));
        assert!(s.handle_key(k(KeyCode::Char('7'))));
        assert!(!s.handle_key(k(KeyCode::Char('x'))));
        assert!(s.handle_key(k(KeyCode::Backspace)));
        assert!(s.handle_key(k(KeyCode::Char('9'))));
        assert!(s.handle_key(k(KeyCode::Enter)));
        assert_eq!(s.value(), 9.0);
        assert!(s.handle_key(k(KeyCode::Char('e'))));
        assert!(!s.handle_key(k(KeyCode::Tab)));
        assert!(s.handle_key(k(KeyCode::Esc)));
        let slider = RatingSlider::new(0.0, 20.0, 0.0).show_value(false);
        let area = Rect::new(0, 0, 21, 1);
        let ev = |kind, column| MouseEvent {
            kind,
            column,
            row: 0,
            modifiers: KeyModifiers::NONE,
        };
        assert!(s.handle_mouse(
            ev(MouseEventKind::Down(MouseButton::Left), 3),
            &slider,
            area
        ));
        assert_eq!(s.value(), 3.0);
        assert!(!s.handle_mouse(ev(MouseEventKind::Moved, 5), &slider, area));
        assert!(!s.handle_mouse(
            ev(MouseEventKind::Down(MouseButton::Left), 40),
            &slider,
            area
        ));
    }
}
