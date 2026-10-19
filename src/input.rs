//! Keyboard and mouse handling for [`RatingState`] (feature `crossterm`).

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyEventKind, MouseButton, MouseEvent, MouseEventKind},
    layout::Rect,
};

use crate::{Rating, RatingState};

impl RatingState {
    /// Applies a key press. Returns `true` if the value changed.
    ///
    /// | Key | Action |
    /// |-----|--------|
    /// | `←` `↓` `h` `-` | decrement |
    /// | `→` `↑` `l` `+` `=` | increment |
    /// | `Home` | clear |
    /// | `End` | full |
    /// | `0`–`9` | set that many items (clamped to `max`) |
    ///
    /// Ignored when [read-only](RatingState::is_read_only) and for key
    /// releases.
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        if self.is_read_only() || key.kind == KeyEventKind::Release {
            return false;
        }
        let before = self.value();
        match key.code {
            KeyCode::Left | KeyCode::Down | KeyCode::Char('h' | '-') => self.decrement(),
            KeyCode::Right | KeyCode::Up | KeyCode::Char('l' | '+' | '=') => self.increment(),
            KeyCode::Home => self.clear(),
            KeyCode::End => self.set_full(),
            KeyCode::Char(c) if c.is_ascii_digit() => {
                self.set_items(c.to_digit(10).unwrap_or(0) as usize);
            }
            _ => return false,
        }
        self.value() != before
    }

    /// Applies a mouse event to a `rating` rendered in `area`.
    /// Returns `true` if the value changed.
    ///
    /// Moving updates the [hover preview](RatingState::hover); left click or
    /// drag over an item selects it (see [`Rating::value_at`]). Ignored when
    /// [read-only](RatingState::is_read_only).
    pub fn handle_mouse(&mut self, event: MouseEvent, rating: &Rating<'_>, area: Rect) -> bool {
        if self.is_read_only() {
            return false;
        }
        let at = rating.value_at(area, event.column, event.row);
        match event.kind {
            MouseEventKind::Moved => {
                self.set_hover(at);
                false
            }
            MouseEventKind::Down(MouseButton::Left) | MouseEventKind::Drag(MouseButton::Left) => {
                self.set_hover(at);
                match at {
                    Some(v) if (v - self.value()).abs() > f64::EPSILON => {
                        self.set_value(v);
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use ratatui::crossterm::event::{KeyEventState, KeyModifiers};

    use super::*;

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn mouse(kind: MouseEventKind, column: u16, row: u16) -> MouseEvent {
        MouseEvent {
            kind,
            column,
            row,
            modifiers: KeyModifiers::NONE,
        }
    }

    #[test]
    fn keys() {
        let mut s = RatingState::new(2.0, 5);
        assert!(s.handle_key(key(KeyCode::Right)));
        assert_eq!(s.value(), 3.0);
        assert!(s.handle_key(key(KeyCode::Char('h'))));
        assert!(s.handle_key(key(KeyCode::Down)));
        assert_eq!(s.value(), 1.0);
        assert!(s.handle_key(key(KeyCode::Char('+'))));
        assert!(s.handle_key(key(KeyCode::Up)));
        assert!(s.handle_key(key(KeyCode::Char('l'))));
        assert!(s.handle_key(key(KeyCode::Char('-'))));
        assert!(s.handle_key(key(KeyCode::End)));
        assert_eq!(s.value(), 5.0);
        assert!(!s.handle_key(key(KeyCode::Right)), "saturated: unchanged");
        assert!(s.handle_key(key(KeyCode::Home)));
        assert_eq!(s.value(), 0.0);
        assert!(s.handle_key(key(KeyCode::Char('4'))));
        assert_eq!(s.value(), 4.0);
        assert!(s.handle_key(key(KeyCode::Char('9'))));
        assert_eq!(s.value(), 5.0);
        assert!(s.handle_key(key(KeyCode::Char('0'))));
        assert_eq!(s.value(), 0.0);
        assert!(!s.handle_key(key(KeyCode::Char('x'))));
        assert!(!s.handle_key(key(KeyCode::Enter)));
    }

    #[test]
    fn key_release_and_read_only_ignored() {
        let mut s = RatingState::new(2.0, 5);
        let mut k = key(KeyCode::Right);
        k.kind = KeyEventKind::Release;
        k.state = KeyEventState::NONE;
        assert!(!s.handle_key(k));
        let mut ro = RatingState::new(2.0, 5).with_read_only(true);
        assert!(!ro.handle_key(key(KeyCode::Right)));
        assert_eq!(ro.value(), 2.0);
    }

    #[test]
    fn mouse_click_hover_drag() {
        let rating = Rating::new(0.0, 5);
        let area = Rect::new(0, 0, 20, 1);
        let mut s = RatingState::new(0.0, 5);
        // items at x = 0,2,4,6,8
        assert!(!s.handle_mouse(mouse(MouseEventKind::Moved, 4, 0), &rating, area));
        assert_eq!(s.hover(), Some(3.0));
        assert!(!s.handle_mouse(mouse(MouseEventKind::Moved, 5, 0), &rating, area));
        assert_eq!(s.hover(), None, "gap clears hover");
        assert!(s.handle_mouse(
            mouse(MouseEventKind::Down(MouseButton::Left), 6, 0),
            &rating,
            area
        ));
        assert_eq!(s.value(), 4.0);
        assert!(!s.handle_mouse(
            mouse(MouseEventKind::Down(MouseButton::Left), 6, 0),
            &rating,
            area
        ));
        assert!(s.handle_mouse(
            mouse(MouseEventKind::Drag(MouseButton::Left), 0, 0),
            &rating,
            area
        ));
        assert_eq!(s.value(), 1.0);
        assert!(!s.handle_mouse(
            mouse(MouseEventKind::Down(MouseButton::Left), 19, 0),
            &rating,
            area
        ));
        assert!(!s.handle_mouse(
            mouse(MouseEventKind::Down(MouseButton::Right), 4, 0),
            &rating,
            area
        ));
        let mut ro = RatingState::new(0.0, 5).with_read_only(true);
        assert!(!ro.handle_mouse(
            mouse(MouseEventKind::Down(MouseButton::Left), 0, 0),
            &rating,
            area
        ));
    }
}
