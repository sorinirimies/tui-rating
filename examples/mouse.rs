//! Mouse example - click, drag and hover to rate; keyboard works too.
//!
//! Controls: click/drag set, hover previews, ↑/↓ or Tab move focus,
//! ←/→ h/l change, 0-9 set, Home/End clear/full, r toggle read-only, q/Esc quit.

use std::{io::stdout, time::Duration};

use anyhow::Result;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseEvent,
    },
    execute,
};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Paragraph},
    Frame,
};
use tui_rating::{Rating, RatingState, RatingStyle, RatingSymbols, ValueFormat};

struct Field {
    title: &'static str,
    state: RatingState,
    rating: fn(&RatingState) -> Rating<'static>,
    area: Rect,
}

fn stars(s: &RatingState) -> Rating<'static> {
    Rating::from_state(s)
        .preset(RatingStyle::github())
        .show_value(true)
}

fn hearts(s: &RatingState) -> Rating<'static> {
    Rating::from_state(s)
        .symbols(RatingSymbols::hearts())
        .filled_color(Color::Red)
        .show_value(true)
        .value_format(ValueFormat::Percent)
}

fn fire(s: &RatingState) -> Rating<'static> {
    Rating::from_state(s)
        .symbols(RatingSymbols::fire())
        .show_value(true)
}

fn smooth(s: &RatingState) -> Rating<'static> {
    Rating::from_state(s)
        .symbols(RatingSymbols::smooth())
        .fractional(true)
        .gap(0)
        .traffic_light()
        .show_value(true)
}

struct App {
    fields: Vec<Field>,
    focus: usize,
}

fn main() -> Result<()> {
    let field = |title, state, rating| Field {
        title,
        state,
        rating,
        area: Rect::default(),
    };
    let mut app = App {
        fields: vec![
            field(
                " stars (half steps) ",
                RatingState::new(3.5, 5).with_half_steps(true),
                stars,
            ),
            field(" hearts ", RatingState::new(2.0, 5), hearts),
            field(" fire ", RatingState::new(4.0, 5), fire),
            field(
                " smooth · click = whole items ",
                RatingState::new(3.0, 5),
                smooth,
            ),
        ],
        focus: 0,
    };
    let mut terminal = ratatui::init();
    execute!(stdout(), EnableMouseCapture)?;
    let result = (|| -> Result<()> {
        loop {
            terminal.draw(|f| draw(f, &mut app))?;
            if !event::poll(Duration::from_millis(50))? {
                for fld in &mut app.fields {
                    fld.state.tick();
                }
                continue;
            }
            match event::read()? {
                Event::Key(k) if k.kind == KeyEventKind::Press => match k.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Tab => app.focus = (app.focus + 1) % app.fields.len(),
                    KeyCode::BackTab => {
                        app.focus = (app.focus + app.fields.len() - 1) % app.fields.len();
                    }
                    KeyCode::Char('r') => {
                        let s = &mut app.fields[app.focus].state;
                        s.set_read_only(!s.is_read_only());
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        app.focus = (app.focus + app.fields.len() - 1) % app.fields.len();
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        app.focus = (app.focus + 1) % app.fields.len();
                    }
                    _ => {
                        let s = &mut app.fields[app.focus].state;
                        let before = s.visual_value();
                        if s.handle_key(k) {
                            let to = s.value();
                            s.set_value(before);
                            s.animate_to(to, 6);
                        }
                    }
                },
                Event::Mouse(m) => handle_mouse(&mut app, m),
                _ => {}
            }
        }
    })();
    execute!(stdout(), DisableMouseCapture)?;
    ratatui::restore();
    result
}

fn handle_mouse(app: &mut App, m: MouseEvent) {
    for (i, fld) in app.fields.iter_mut().enumerate() {
        let rating = (fld.rating)(&fld.state);
        let inside = fld.area.contains((m.column, m.row).into());
        if !inside {
            fld.state.set_hover(None);
            continue;
        }
        let before = fld.state.visual_value();
        if fld.state.handle_mouse(m, &rating, fld.area) {
            let to = fld.state.value();
            fld.state.set_value(before);
            fld.state.animate_to(to, 6);
            app.focus = i;
        }
    }
}

fn draw(f: &mut Frame, app: &mut App) {
    let n = app.fields.len();
    let [header, body, footer] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3 * n as u16),
        Constraint::Length(2),
    ])
    .areas(f.area());
    f.render_widget(
        Paragraph::new(" click · drag · hover · Tab · ←/→ · 0-9 · r read-only · q quit")
            .style(Style::default().add_modifier(Modifier::BOLD)),
        header,
    );
    let rects = Layout::vertical(vec![Constraint::Length(3); n]).split(body);
    for (i, (fld, rect)) in app.fields.iter_mut().zip(rects.iter()).enumerate() {
        fld.area = *rect;
        let focused = i == app.focus;
        let ro = fld.state.is_read_only();
        let block = Block::bordered()
            .title(format!(
                "{}{}",
                fld.title,
                if ro { "· read-only " } else { "" }
            ))
            .border_style(if focused {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default().fg(Color::DarkGray)
            });
        let inner = block.inner(*rect);
        fld.area = inner;
        f.render_widget(block, *rect);
        let rating = (fld.rating)(&fld.state).focused(focused).disabled(ro);
        f.render_widget(rating, inner);
    }
    let fld = &app.fields[app.focus];
    f.render_widget(
        Paragraph::new(format!(
            " focus: {} · value {:.1}/{} · hover {:?}",
            fld.title.trim(),
            fld.state.value(),
            fld.state.max(),
            fld.state.hover().map(|h| (h * 2.0).round() / 2.0)
        ))
        .style(Style::default().fg(Color::DarkGray)),
        footer,
    );
}
