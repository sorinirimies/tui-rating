//! Custom example - custom glyphs, RGB colours, gaps and a 10-item scale.
//!
//! Controls: ←/→ or h/l change value, q/Esc quit.

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType},
};
use tui_rating::{Rating, RatingState, RatingSymbols};

fn main() -> Result<()> {
    let mut state = RatingState::new(6.0, 10);
    let mut terminal = ratatui::init();
    let result = (|| -> Result<()> {
        loop {
            terminal.draw(|f| {
                let [a, b, c] = Layout::vertical([Constraint::Length(3); 3]).areas(f.area());
                f.render_widget(
                    Rating::from_state(&state)
                        .symbols(RatingSymbols::new("🔥", "·"))
                        .gap(0)
                        .show_value(true)
                        .block(Block::bordered().title(" emoji, no gap ")),
                    a,
                );
                f.render_widget(
                    Rating::from_state(&state)
                        .symbols(RatingSymbols::new("▰", "▱"))
                        .filled_color(Color::Rgb(255, 140, 0))
                        .empty_color(Color::Rgb(80, 60, 40))
                        .gap(0)
                        .label("Power")
                        .label_style(Style::default().add_modifier(Modifier::BOLD))
                        .value_style(Style::default().fg(Color::Rgb(255, 140, 0)))
                        .show_value(true)
                        .block(
                            Block::bordered()
                                .border_type(BorderType::Rounded)
                                .title(" RGB bars "),
                        ),
                    b,
                );
                f.render_widget(
                    Rating::from_state(&state)
                        .symbols(RatingSymbols::new("◉", "○"))
                        .filled_style(
                            Style::default()
                                .fg(Color::LightBlue)
                                .add_modifier(Modifier::BOLD),
                        )
                        .gap(3)
                        .block(
                            Block::bordered()
                                .border_type(BorderType::Double)
                                .title(" wide gap "),
                        ),
                    c,
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
                    _ => {}
                }
            }
        }
    })();
    ratatui::restore();
    result
}
