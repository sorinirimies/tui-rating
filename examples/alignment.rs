//! Alignment example - left / center / right placement.
//!
//! Controls: ←/→ or h/l change value, q/Esc quit.

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    widgets::Block,
};
use tui_rating::{Rating, RatingState};

fn main() -> Result<()> {
    let mut state = RatingState::new(3.0, 5);
    let mut terminal = ratatui::init();
    let result = (|| -> Result<()> {
        loop {
            terminal.draw(|f| {
                let areas = Layout::vertical([Constraint::Length(3); 3]).split(f.area());
                for (i, (al, t)) in [
                    (Alignment::Left, " left "),
                    (Alignment::Center, " center "),
                    (Alignment::Right, " right "),
                ]
                .into_iter()
                .enumerate()
                {
                    f.render_widget(
                        Rating::from_state(&state)
                            .alignment(al)
                            .show_value(true)
                            .block(Block::bordered().title(t)),
                        areas[i],
                    );
                }
            })?;
            if let Event::Key(k) = event::read()? {
                if k.kind != KeyEventKind::Press {
                    continue;
                }
                match k.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Right | KeyCode::Char('l') => state.increment(),
                    KeyCode::Left | KeyCode::Char('h') => state.decrement(),
                    _ => {}
                }
            }
        }
    })();
    ratatui::restore();
    result
}
