//! Half-step example - fractional ratings with ←/→ in 0.5 steps.
//!
//! Controls: ←/→ or h/l change, t toggle half steps, q/Esc quit.

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    style::Color,
    widgets::{Block, Paragraph},
};
use tui_rating::{Rating, RatingState, RatingSymbols};

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut state = RatingState::new(2.5, 5).with_half_steps(true);
    let result = (|| -> Result<()> {
        loop {
            terminal.draw(|f| {
                let [a, b, c, d] = Layout::vertical([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(1),
                ])
                .areas(f.area());
                f.render_widget(
                    Rating::from_state(&state)
                        .show_value(true)
                        .block(Block::bordered().title(" Stars (outlined half) ")),
                    a,
                );
                f.render_widget(
                    Rating::from_state(&state)
                        .symbols(RatingSymbols::circles())
                        .filled_color(Color::Green)
                        .show_value(true)
                        .block(Block::bordered().title(" Circles (◐ half glyph) ")),
                    b,
                );
                f.render_widget(
                    Rating::from_state(&state)
                        .symbols(RatingSymbols::squares())
                        .filled_color(Color::Cyan)
                        .half_style(ratatui::style::Style::default().fg(Color::Blue))
                        .show_value(true)
                        .block(Block::bordered().title(" Squares (custom half style) ")),
                    c,
                );
                f.render_widget(
                    Paragraph::new(format!(
                        "half steps: {}  ←/→ change  t toggle  q quit",
                        state.half_steps()
                    )),
                    d,
                );
            })?;
            if let Event::Key(k) = event::read()? {
                if k.kind != KeyEventKind::Press {
                    continue;
                }
                match k.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Right | KeyCode::Char('l') => state.increment(),
                    KeyCode::Left | KeyCode::Char('h') => state.decrement(),
                    KeyCode::Char('t') => state.set_half_steps(!state.half_steps()),
                    _ => {}
                }
            }
        }
    })();
    ratatui::restore();
    result
}
