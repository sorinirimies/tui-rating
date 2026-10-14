#![cfg(feature = "theme")]
use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};
use tui_rating::{Rating, Theme, ThemeProvider, Themed};

fn first_fg(r: Rating<'_>) -> Option<Color> {
    let area = Rect::new(0, 0, 20, 1);
    let mut buf = Buffer::empty(area);
    r.render(area, &mut buf);
    buf[(0, 0)].style().fg
}

#[test]
fn default_unchanged() {
    assert_eq!(first_fg(Rating::new(1.0, 3)), Some(Color::Yellow));
}

#[test]
fn every_theme_renders_and_applies() {
    for t in Theme::all() {
        assert_eq!(first_fg(Rating::new(1.0, 3).theme(&t)), Some(t.accent));
    }
}

#[test]
fn precedence_and_provider() {
    struct App;
    impl ThemeProvider for App {
        fn nuance_theme(&self) -> Theme {
            Theme::dracula()
        }
    }
    assert_eq!(
        first_fg(Rating::new(1.0, 3).themed(&App)),
        Some(Theme::dracula().accent)
    );
    let r = Rating::new(1.0, 3).filled_color(Color::Blue).themed(&App);
    assert_eq!(first_fg(r), Some(Color::Blue));
}
