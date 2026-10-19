//! Styles example - gradients, value formats, label position, modifiers,
//! reversed fill and a highlighted cursor item.
//!
//! Controls: ←/→ or h/l change value, q/Esc quit.

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::Block,
};
use tui_rating::{LabelPosition, Rating, RatingState, RatingSymbols, ValueFormat};

fn main() -> Result<()> {
    let mut state = RatingState::new(3.5, 5).with_half_steps(true);
    let mut terminal = ratatui::init();
    let result = (|| -> Result<()> {
        loop {
            terminal.draw(|f| {
                let rows = Layout::vertical([Constraint::Length(3); 7]).split(f.area());
                let base = || Rating::from_state(&state);
                let cursor = (state.value().ceil() as usize).checked_sub(1);
                let widgets = [
                    (" traffic light ", base().traffic_light().show_value(true)),
                    (
                        " rgb gradient ",
                        base()
                            .symbols(RatingSymbols::hearts())
                            .gradient([Color::Rgb(255, 80, 120), Color::Rgb(120, 80, 255)])
                            .modifier(Modifier::BOLD),
                    ),
                    (
                        " percent ",
                        base()
                            .symbols(RatingSymbols::bars())
                            .filled_color(Color::LightGreen)
                            .value_format(ValueFormat::Percent)
                            .show_value(true),
                    ),
                    (
                        " label after ",
                        base()
                            .symbols(RatingSymbols::sparkles())
                            .filled_color(Color::Yellow)
                            .label("Score")
                            .label_position(LabelPosition::After)
                            .show_value(true),
                    ),
                    (
                        " reversed ",
                        base()
                            .symbols(RatingSymbols::hexagons())
                            .filled_color(Color::Cyan)
                            .reversed(true),
                    ),
                    (
                        " cursor highlight ",
                        base()
                            .symbols(RatingSymbols::circles())
                            .filled_color(Color::Green)
                            .highlight(cursor)
                            .highlight_style(
                                Style::default()
                                    .fg(Color::White)
                                    .add_modifier(Modifier::BOLD),
                            ),
                    ),
                    (
                        " dim empty ",
                        base()
                            .symbols(RatingSymbols::blocks())
                            .filled_color(Color::Magenta)
                            .empty_modifier(Modifier::DIM)
                            .gap(0),
                    ),
                ];
                for (area, (title, w)) in rows.iter().zip(widgets) {
                    f.render_widget(w.block(Block::bordered().title(title)), *area);
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
