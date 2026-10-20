//! Survey - a keyboard-driven feedback form combining the new widgets.
//!
//! `Tab`/`BackTab` switch section, each section handles its own keys
//! (see the hint line), `q`/`Esc` quit.

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Paragraph},
    Frame,
};
use tui_rating::{
    LikertLabels, LikertScale, LikertState, MultiCriteria, NpsRating, RangeState, RatingForm,
    RatingRange, RatingSlider, RatingState, SentimentFaces, SliderState, ThumbsRating, ThumbsState,
};

const SECTIONS: [(&str, &str); 7] = [
    (
        "Criteria",
        "↑↓ row · ←→ rate · 1-5 set · u undo · Backspace clear",
    ),
    ("NPS", "←→ · 0-9 (1 0 = 10) · Home/End"),
    ("Agreement", "←→ · 1-5 · Backspace clear"),
    ("Mood", "←→ · 1-5 · Backspace clear"),
    (
        "Score band",
        "←→ move handle · Space/Tab switch handle · Home/End",
    ),
    (
        "Exact score",
        "←→ step · PgUp/PgDn ×10 · Enter/e type a value",
    ),
    ("Helpful?", "↑ vote up · ↓ vote down (again withdraws)"),
];

struct App {
    focus: usize,
    form: RatingForm,
    nps: RatingState,
    likert: LikertState,
    faces: LikertState,
    range: RangeState,
    slider: SliderState,
    thumbs: ThumbsState,
}

impl App {
    fn new() -> Self {
        Self {
            focus: 0,
            form: RatingForm::new(["Service", "Quality", "Value"], 5).with_half_steps(true),
            nps: RatingState::new(0.0, 10),
            likert: LikertState::new(5),
            faces: LikertState::new(5),
            range: RangeState::new(0.0, 5.0, 1.0, 4.0).with_step(0.5),
            slider: SliderState::new(0.0, 5.0, 0.1).with_value(3.0),
            thumbs: ThumbsState::new(1203, 48),
        }
    }

    fn handle(&mut self, key: event::KeyEvent) {
        match self.focus {
            0 => {
                self.form.handle_key(key);
            }
            1 => {
                self.nps.handle_key(key);
            }
            2 => {
                self.likert.handle_key(key);
            }
            3 => {
                self.faces.handle_key(key);
            }
            4 => {
                self.range.handle_key(key);
            }
            5 => {
                self.slider.handle_key(key);
            }
            _ => {
                self.thumbs.handle_key(key);
            }
        }
    }
}

fn main() -> Result<()> {
    let mut app = App::new();
    let mut terminal = ratatui::init();
    let result = (|| -> Result<()> {
        loop {
            terminal.draw(|f| draw(f, &app))?;
            if let Event::Key(k) = event::read()? {
                if k.kind != KeyEventKind::Press {
                    continue;
                }
                // typing a slider value must not be interrupted by shortcuts
                let typing = app.focus == 5 && app.slider.is_entering();
                match k.code {
                    KeyCode::Char('q') | KeyCode::Esc if !typing => return Ok(()),
                    // section switching (Tab is used inside sections)
                    KeyCode::Char(']') if !typing => app.focus = (app.focus + 1) % SECTIONS.len(),
                    KeyCode::Char('[') if !typing => {
                        app.focus = (app.focus + SECTIONS.len() - 1) % SECTIONS.len();
                    }
                    _ => app.handle(k),
                }
            }
        }
    })();
    ratatui::restore();
    result
}

fn section(f: &mut Frame, area: Rect, app: &App, index: usize) -> Rect {
    let focused = app.focus == index;
    let style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let block = Block::bordered()
        .border_style(style)
        .title(format!(" {} ", SECTIONS[index].0))
        .title_style(style.add_modifier(Modifier::BOLD));
    let inner = block.inner(area);
    f.render_widget(block, area);
    inner
}

fn draw(f: &mut Frame, app: &App) {
    let [body, footer] =
        Layout::vertical([Constraint::Min(1), Constraint::Length(2)]).areas(f.area());
    let heights = [6, 3, 3, 3, 3, 3, 3];
    let areas = Layout::vertical(heights.map(Constraint::Length)).split(body);

    let r = section(f, areas[0], app, 0);
    f.render_widget(MultiCriteria::from_form(&app.form).overall(true), r);

    let r = section(f, areas[1], app, 1);
    f.render_widget(
        NpsRating::new((!app.nps.is_empty()).then(|| app.nps.value() as u8)).show_category(true),
        r,
    );

    let r = section(f, areas[2], app, 2);
    f.render_widget(
        LikertScale::agreement(app.likert.selected()).labels(LikertLabels::Ends),
        r,
    );

    let r = section(f, areas[3], app, 3);
    f.render_widget(
        SentimentFaces::new(app.faces.selected()).show_label(true),
        r,
    );

    let r = section(f, areas[4], app, 4);
    f.render_widget(RatingRange::from_state(&app.range), r);

    let r = section(f, areas[5], app, 5);
    f.render_widget(RatingSlider::from_state(&app.slider), r);

    let r = section(f, areas[6], app, 6);
    f.render_widget(
        ThumbsRating::from_state(&app.thumbs)
            .compact(true)
            .show_net(true)
            .show_ratio(true),
        r,
    );

    f.render_widget(
        Paragraph::new(vec![
            Line::from(format!(" {}", SECTIONS[app.focus].1)),
            Line::from(" [ ] switch section · q quit").style(Style::default().fg(Color::DarkGray)),
        ]),
        footer,
    );
}
