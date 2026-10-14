//! Interactive example - several ratings you can edit.
//!
//! Controls: ↑/↓ or j/k select, ←/→ or h/l change, 0-9 set, c clear, t/T theme, q/Esc quit.

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Paragraph},
    Frame,
};
use tui_rating::{Rating, RatingState, RatingSymbols, ThemeSwitcher};

struct Item {
    label: &'static str,
    state: RatingState,
    symbols: RatingSymbols,
    color: Color,
}

struct App {
    items: Vec<Item>,
    selected: usize,
    themes: ThemeSwitcher,
}

impl App {
    fn new() -> Self {
        let item = |label, state, symbols, color| Item {
            label,
            state,
            symbols,
            color,
        };
        Self {
            items: vec![
                item(
                    "Quality ",
                    RatingState::new(3.0, 5),
                    RatingSymbols::stars(),
                    Color::Yellow,
                ),
                item(
                    "Love    ",
                    RatingState::new(4.0, 5),
                    RatingSymbols::hearts(),
                    Color::Red,
                ),
                item(
                    "Taste   ",
                    RatingState::new(2.5, 5).with_half_steps(true),
                    RatingSymbols::circles(),
                    Color::Green,
                ),
                item(
                    "Spice   ",
                    RatingState::new(1.0, 3),
                    RatingSymbols::diamonds(),
                    Color::Magenta,
                ),
                item(
                    "Overall ",
                    RatingState::new(7.0, 10),
                    RatingSymbols::squares(),
                    Color::Cyan,
                ),
            ],
            selected: 0,
            themes: ThemeSwitcher::new(),
        }
    }

    fn handle(&mut self, code: KeyCode) -> bool {
        let n = self.items.len();
        let cur = &mut self.items[self.selected].state;
        match code {
            KeyCode::Char('q') | KeyCode::Esc => return true,
            KeyCode::Up | KeyCode::Char('k') => self.selected = (self.selected + n - 1) % n,
            KeyCode::Down | KeyCode::Char('j') => self.selected = (self.selected + 1) % n,
            KeyCode::Right | KeyCode::Char('l') => cur.increment(),
            KeyCode::Left | KeyCode::Char('h') => cur.decrement(),
            KeyCode::Char('c') => cur.clear(),
            KeyCode::Char('t') => {
                self.themes.next();
            }
            KeyCode::Char('T') => {
                self.themes.prev();
            }
            KeyCode::Char(d @ '0'..='9') => cur.set_items(d as usize - '0' as usize),
            _ => {}
        }
        false
    }

    fn ui(&self, frame: &mut Frame) {
        let mut cons = vec![Constraint::Length(3); self.items.len()];
        cons.push(Constraint::Min(1));
        let areas = Layout::vertical(cons).split(frame.area());
        for (i, item) in self.items.iter().enumerate() {
            let sel = i == self.selected;
            let border = if sel { Color::White } else { Color::DarkGray };
            let label_style = if sel {
                Style::default().add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            frame.render_widget(
                Rating::from_state(&item.state)
                    .symbols(item.symbols.clone())
                    .theme(self.themes.current())
                    .filled_color(item.color)
                    .label(item.label)
                    .label_style(label_style)
                    .show_value(true)
                    .block(Block::bordered().border_style(Style::default().fg(border))),
                areas[i],
            );
        }
        frame.render_widget(
            Paragraph::new(format!(
                "↑/↓ select  ←/→ change  0-9 set  c clear  t/T theme ({})  q quit",
                self.themes.current().name
            )),
            areas[self.items.len()],
        );
    }
}

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();
    let result = (|| -> Result<()> {
        loop {
            terminal.draw(|f| app.ui(f))?;
            if let Event::Key(k) = event::read()? {
                if k.kind == KeyEventKind::Press && app.handle(k.code) {
                    return Ok(());
                }
            }
        }
    })();
    ratatui::restore();
    result
}
