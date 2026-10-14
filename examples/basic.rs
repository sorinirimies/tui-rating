//! Basic example - a static rating in a few lines.
//!
//! Press `q` or `Esc` to quit.

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, Paragraph},
    Frame,
};
use tui_rating::{Rating, RatingState};

fn ui(frame: &mut Frame) {
    let state = RatingState::new(4.0, 5);
    let [top, rest] =
        Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).areas(frame.area());
    frame.render_widget(
        Rating::from_state(&state)
            .label("Rating")
            .show_value(true)
            .block(Block::bordered().title(" tui-rating ")),
        top,
    );
    frame.render_widget(Paragraph::new("Press q or Esc to quit"), rest);
}

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let result = (|| -> Result<()> {
        loop {
            terminal.draw(ui)?;
            if let Event::Key(k) = event::read()? {
                if matches!(k.code, KeyCode::Char('q') | KeyCode::Esc) {
                    return Ok(());
                }
            }
        }
    })();
    ratatui::restore();
    result
}
