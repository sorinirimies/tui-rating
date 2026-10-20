//! [`ThumbsRating`]: up/down votes with a net score, Reddit/YouTube style.

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Widget},
};

use crate::util::{compact, render_lines, thousands};

/// A voter's choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Vote {
    /// 👍
    Up,
    /// 👎
    Down,
}

/// Vote counts plus the current user's vote. Voting the same way again
/// withdraws the vote; switching sides moves it, like most vote UIs.
///
/// ```
/// use tui_rating::{ThumbsState, Vote};
///
/// let mut s = ThumbsState::new(10, 2);
/// s.vote(Vote::Up);
/// assert_eq!((s.up(), s.net()), (11, 9));
/// s.vote(Vote::Down); // switch
/// assert_eq!((s.up(), s.down()), (10, 3));
/// s.vote(Vote::Down); // withdraw
/// assert_eq!(s.down(), 2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThumbsState {
    up: u64,
    down: u64,
    vote: Option<Vote>,
}

impl ThumbsState {
    /// State with the other voters' counts and no vote of our own.
    pub fn new(up: u64, down: u64) -> Self {
        Self {
            up,
            down,
            vote: None,
        }
    }

    /// Up votes (including ours).
    pub fn up(&self) -> u64 {
        self.up
    }

    /// Down votes (including ours).
    pub fn down(&self) -> u64 {
        self.down
    }

    /// Our vote.
    pub fn current(&self) -> Option<Vote> {
        self.vote
    }

    /// `up − down`.
    pub fn net(&self) -> i64 {
        self.up as i64 - self.down as i64
    }

    /// Casts, switches or withdraws our vote.
    pub fn vote(&mut self, vote: Vote) {
        match self.vote {
            Some(v) if v == vote => {
                self.vote = None;
                match vote {
                    Vote::Up => self.up = self.up.saturating_sub(1),
                    Vote::Down => self.down = self.down.saturating_sub(1),
                }
            }
            prev => {
                match prev {
                    Some(Vote::Up) => self.up = self.up.saturating_sub(1),
                    Some(Vote::Down) => self.down = self.down.saturating_sub(1),
                    None => {}
                }
                self.vote = Some(vote);
                match vote {
                    Vote::Up => self.up = self.up.saturating_add(1),
                    Vote::Down => self.down = self.down.saturating_add(1),
                }
            }
        }
    }

    /// Applies a key: `↑` `k` `+` vote up, `↓` `j` `-` vote down.
    /// Returns `true` if the counts changed (feature `crossterm`).
    #[cfg(feature = "crossterm")]
    pub fn handle_key(&mut self, key: ratatui::crossterm::event::KeyEvent) -> bool {
        use ratatui::crossterm::event::{KeyCode, KeyEventKind};
        if key.kind == KeyEventKind::Release {
            return false;
        }
        match key.code {
            KeyCode::Up | KeyCode::Char('k' | '+') => self.vote(Vote::Up),
            KeyCode::Down | KeyCode::Char('j' | '-') => self.vote(Vote::Down),
            _ => return false,
        }
        true
    }
}

/// Renders `👍 1.2k  👎 48  +1.2k  96%`; our vote is highlighted.
///
/// ```
/// use tui_rating::{ThumbsRating, Vote};
///
/// let t = ThumbsRating::new(1203, 48).show_net(true).show_ratio(true);
/// assert_eq!(t.text(), "👍 1,203  👎 48  +1,155  96%");
/// ```
#[derive(Debug, Clone)]
pub struct ThumbsRating<'a> {
    up: u64,
    down: u64,
    vote: Option<Vote>,
    symbols: (String, String),
    compact: bool,
    show_net: bool,
    show_ratio: bool,
    up_color: Color,
    down_color: Color,
    alignment: Alignment,
    block: Option<Block<'a>>,
}

impl<'a> ThumbsRating<'a> {
    /// Creates a widget for the given counts.
    pub fn new(up: u64, down: u64) -> Self {
        Self {
            up,
            down,
            vote: None,
            symbols: ("👍".into(), "👎".into()),
            compact: false,
            show_net: false,
            show_ratio: false,
            up_color: Color::Rgb(70, 190, 90),
            down_color: Color::Rgb(220, 60, 60),
            alignment: Alignment::Left,
            block: None,
        }
    }

    /// Creates a widget from a [`ThumbsState`] (including our vote).
    pub fn from_state(state: &ThumbsState) -> Self {
        let mut t = Self::new(state.up(), state.down());
        t.vote = state.current();
        t
    }

    /// Highlights our vote.
    #[must_use]
    pub fn vote(mut self, vote: Option<Vote>) -> Self {
        self.vote = vote;
        self
    }

    /// Custom glyphs, e.g. `("▲", "▼")` or `("+", "-")`.
    #[must_use]
    pub fn symbols(mut self, up: impl Into<String>, down: impl Into<String>) -> Self {
        self.symbols = (up.into(), down.into());
        self
    }

    /// Abbreviates counts (`1.2k`).
    #[must_use]
    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    /// Shows the net score (`up − down`).
    #[must_use]
    pub fn show_net(mut self, show: bool) -> Self {
        self.show_net = show;
        self
    }

    /// Shows the share of up votes (`96%`).
    #[must_use]
    pub fn show_ratio(mut self, show: bool) -> Self {
        self.show_ratio = show;
        self
    }

    /// Colours of the up and down side.
    #[must_use]
    pub fn colors(mut self, up: Color, down: Color) -> Self {
        self.up_color = up;
        self.down_color = down;
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

    fn count(&self, n: u64) -> String {
        if self.compact {
            compact(n)
        } else {
            thousands(n)
        }
    }

    /// `up − down`.
    pub fn net(&self) -> i64 {
        self.up as i64 - self.down as i64
    }

    /// Share of up votes in `0..=1` (`None` without votes).
    pub fn ratio(&self) -> Option<f64> {
        let t = self.up.saturating_add(self.down);
        (t > 0).then(|| self.up as f64 / t as f64)
    }

    fn net_text(&self) -> String {
        let n = self.net();
        let abs = self.count(n.unsigned_abs());
        if n < 0 {
            format!("-{abs}")
        } else {
            format!("+{abs}")
        }
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
        let side = |glyph: &str, n: u64, color: Color, active: bool, other_active: bool| {
            let mut st = Style::default().fg(color);
            if active {
                st = st.add_modifier(Modifier::BOLD | Modifier::UNDERLINED);
            } else if other_active {
                st = st.add_modifier(Modifier::DIM);
            }
            Span::styled(format!("{glyph} {}", self.count(n)), st)
        };
        let up_on = self.vote == Some(Vote::Up);
        let down_on = self.vote == Some(Vote::Down);
        let mut spans = vec![
            side(&self.symbols.0, self.up, self.up_color, up_on, down_on),
            Span::raw("  "),
            side(&self.symbols.1, self.down, self.down_color, down_on, up_on),
        ];
        if self.show_net {
            let c = match self.net() {
                n if n > 0 => self.up_color,
                n if n < 0 => self.down_color,
                _ => Color::Gray,
            };
            spans.push(Span::styled(
                format!("  {}", self.net_text()),
                Style::default().fg(c).add_modifier(Modifier::BOLD),
            ));
        }
        if self.show_ratio {
            if let Some(r) = self.ratio() {
                spans.push(Span::styled(
                    format!("  {}%", (r * 100.0).round() as i64),
                    Style::default().fg(Color::DarkGray),
                ));
            }
        }
        Line::from(spans).alignment(self.alignment)
    }
}

impl Widget for ThumbsRating<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        render_lines(self.block.clone(), vec![self.line()], area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn voting_rules() {
        let mut s = ThumbsState::new(0, 0);
        s.vote(Vote::Down);
        assert_eq!((s.up(), s.down(), s.net()), (0, 1, -1));
        s.vote(Vote::Up);
        assert_eq!((s.up(), s.down()), (1, 0));
        s.vote(Vote::Up);
        assert_eq!((s.up(), s.current()), (0, None));
        assert_eq!(ThumbsState::default().net(), 0);
    }

    #[test]
    fn text_variants() {
        let t = ThumbsRating::new(1203, 48).show_net(true).show_ratio(true);
        assert_eq!(t.text(), "👍 1,203  👎 48  +1,155  96%");
        let t = ThumbsRating::new(5, 9).symbols("▲", "▼").show_net(true);
        assert_eq!(t.text(), "▲ 5  ▼ 9  -4");
        assert_eq!(ThumbsRating::new(2500, 0).compact(true).net(), 2500);
        assert!(ThumbsRating::new(2500, 0)
            .compact(true)
            .text()
            .contains("2.5k"));
        assert_eq!(ThumbsRating::new(0, 0).ratio(), None);
        let t = ThumbsRating::new(0, 0).show_ratio(true).show_net(true);
        assert!(t.text().ends_with("+0"));
    }

    #[test]
    fn highlights_vote_and_renders() {
        let st = ThumbsState {
            vote: Some(Vote::Up),
            ..ThumbsState::new(1, 1)
        };
        let t = ThumbsRating::from_state(&st)
            .colors(Color::Green, Color::Red)
            .alignment(Alignment::Right)
            .block(Block::bordered());
        let l = t.line();
        assert!(l.spans[0].style.add_modifier.contains(Modifier::BOLD));
        assert!(l.spans[2].style.add_modifier.contains(Modifier::DIM));
        let area = Rect::new(0, 0, 30, 3);
        let mut buf = Buffer::empty(area);
        t.vote(Some(Vote::Down)).render(area, &mut buf);
    }

    #[cfg(feature = "crossterm")]
    #[test]
    fn keys() {
        use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
        let mut s = ThumbsState::new(0, 0);
        assert!(s.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE)));
        assert!(s.handle_key(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE)));
        assert!(!s.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)));
        assert_eq!(s.current(), Some(Vote::Down));
    }
}
