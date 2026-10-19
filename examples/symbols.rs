//! Symbols example - every built-in symbol preset.
//!
//! Controls: ←/→ or h/l change value, q/Esc quit.

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    style::Color,
};
use tui_rating::{Rating, RatingState, RatingSymbols};

fn main() -> Result<()> {
    let presets: [(&str, RatingSymbols, Color); 11] = [
        (" stars ", RatingSymbols::stars(), Color::Yellow),
        (" hearts ", RatingSymbols::hearts(), Color::Red),
        (" circles ", RatingSymbols::circles(), Color::Green),
        (" squares ", RatingSymbols::squares(), Color::Cyan),
        (" diamonds ", RatingSymbols::diamonds(), Color::Magenta),
        (" sparkles ", RatingSymbols::sparkles(), Color::LightYellow),
        (" hexagons ", RatingSymbols::hexagons(), Color::Blue),
        (" bars ", RatingSymbols::bars(), Color::LightGreen),
        (" dots ", RatingSymbols::dots(), Color::LightCyan),
        (" blocks ", RatingSymbols::blocks(), Color::LightRed),
        (" ascii ", RatingSymbols::ascii(), Color::White),
    ];
    let mut state = RatingState::new(3.5, 5).with_half_steps(true);
    let mut terminal = ratatui::init();
    let result = (|| -> Result<()> {
        loop {
            terminal.draw(|f| {
                let areas =
                    Layout::vertical(vec![Constraint::Length(1); presets.len()]).split(f.area());
                for (i, (title, sym, color)) in presets.iter().enumerate() {
                    f.render_widget(
                        Rating::from_state(&state)
                            .symbols(sym.clone())
                            .filled_color(*color)
                            .show_value(true)
                            .label(title.trim()),
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
