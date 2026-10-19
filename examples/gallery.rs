//! Gallery - every design on auto-cycling pages (used for the README GIF).
//!
//! Controls: →/Tab next page, ← previous, space pause/resume auto-advance, q/Esc quit.

use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, Paragraph},
    Frame,
};
use tui_rating::{
    BadgeKind, BipolarRating, LabelPosition, Orientation, Rating, RatingBadge, RatingBar,
    RatingHistogram, RatingState, RatingStyle, RatingSymbols, ValueFormat,
};

const PAGES: [&str; 5] = [
    "Presets",
    "Symbols",
    "Smooth · bars · badges",
    "Reviews · vertical · bipolar",
    "Animation",
];
const PAGE_TICKS: u32 = 45; // 4.5 s at 100 ms

struct App {
    page: usize,
    ticks: u32,
    paused: bool,
    anim: [RatingState; 3],
    targets: [f64; 3],
}

impl App {
    fn new() -> Self {
        Self {
            page: 0,
            ticks: 0,
            paused: false,
            anim: [
                RatingState::new(0.0, 5),
                RatingState::new(0.0, 5).with_half_steps(true),
                RatingState::new(0.0, 10),
            ],
            targets: [4.0, 2.5, 10.0],
        }
    }

    fn goto(&mut self, page: usize) {
        self.page = page % PAGES.len();
        self.ticks = 0;
        for s in &mut self.anim {
            s.clear();
        }
    }

    fn tick(&mut self) {
        self.ticks += 1;
        if self.page == 4 {
            let t = self.ticks;
            for (i, s) in self.anim.iter_mut().enumerate() {
                let i32_ = i as u32;
                if t == 5 + i32_ * 6 {
                    s.animate_to(self.targets[i], 18);
                }
                if t == 32 + i32_ * 2 {
                    s.animate_to(if i == 2 { 3.0 } else { 1.0 }, 12);
                }
                s.tick();
            }
        }
        if !self.paused && self.ticks >= PAGE_TICKS {
            self.goto(self.page + 1);
        }
    }
}

fn main() -> Result<()> {
    let mut app = App::new();
    let mut terminal = ratatui::init();
    let result = (|| -> Result<()> {
        let tick = Duration::from_millis(100);
        let mut last = Instant::now();
        loop {
            terminal.draw(|f| draw(f, &app))?;
            let timeout = tick.saturating_sub(last.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(k) = event::read()? {
                    if k.kind != KeyEventKind::Press {
                        continue;
                    }
                    match k.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Right | KeyCode::Tab => app.goto(app.page + 1),
                        KeyCode::Left => app.goto(app.page + PAGES.len() - 1),
                        KeyCode::Char(' ') => app.paused = !app.paused,
                        _ => {}
                    }
                }
            }
            if last.elapsed() >= tick {
                last = Instant::now();
                app.tick();
            }
        }
    })();
    ratatui::restore();
    result
}

fn draw(f: &mut Frame, app: &App) {
    let [header, body, footer] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(1),
        Constraint::Length(1),
    ])
    .areas(f.area());
    let dots: String = (0..PAGES.len())
        .map(|i| if i == app.page { "●" } else { "○" })
        .collect::<Vec<_>>()
        .join(" ");
    f.render_widget(
        Paragraph::new(Line::from(format!(
            " tui-rating · {} · {dots}",
            PAGES[app.page]
        )))
        .style(Style::default().add_modifier(Modifier::BOLD)),
        header,
    );
    f.render_widget(
        Paragraph::new(" →/Tab next · ← prev · space pause · q quit")
            .style(Style::default().fg(Color::DarkGray)),
        footer,
    );
    let block = Block::bordered();
    let inner = block.inner(body);
    f.render_widget(block, body);
    match app.page {
        0 => presets(f, inner),
        1 => symbols(f, inner),
        2 => smooth_bars_badges(f, inner),
        3 => reviews(f, inner),
        _ => animation(f, inner, app),
    }
}

fn rows(area: Rect, n: usize) -> Vec<Rect> {
    Layout::vertical(vec![Constraint::Length(1); n])
        .split(area)
        .to_vec()
}

fn named(f: &mut Frame, area: Rect, name: &str, widget: Rating<'_>) {
    let [l, r] = Layout::horizontal([Constraint::Length(12), Constraint::Min(1)]).areas(area);
    f.render_widget(Paragraph::new(format!(" {name}")), l);
    f.render_widget(widget, r);
}

fn presets(f: &mut Frame, area: Rect) {
    let all = RatingStyle::all();
    for (row, (name, style)) in rows(area, all.len()).into_iter().zip(all) {
        named(
            f,
            row,
            name,
            Rating::new(3.5, 5)
                .half_steps(true)
                .preset(style)
                .show_value(true),
        );
    }
}

fn symbols(f: &mut Frame, area: Rect) {
    let sets = [
        ("stars", RatingSymbols::stars(), Color::Yellow),
        ("hearts", RatingSymbols::hearts(), Color::Red),
        ("circles", RatingSymbols::circles(), Color::Green),
        ("squares", RatingSymbols::squares(), Color::Cyan),
        ("diamonds", RatingSymbols::diamonds(), Color::Magenta),
        ("sparkles", RatingSymbols::sparkles(), Color::LightYellow),
        ("hexagons", RatingSymbols::hexagons(), Color::Blue),
        ("bars", RatingSymbols::bars(), Color::LightGreen),
        ("dots", RatingSymbols::dots(), Color::LightCyan),
        ("blocks", RatingSymbols::blocks(), Color::LightRed),
        ("ascii", RatingSymbols::ascii(), Color::White),
        ("fire", RatingSymbols::fire(), Color::Reset),
        ("pizza", RatingSymbols::pizza(), Color::Reset),
        ("moon", RatingSymbols::moon(), Color::Reset),
        ("star_emoji", RatingSymbols::star_emoji(), Color::Reset),
    ];
    for (row, (name, symbols, color)) in rows(area, sets.len()).into_iter().zip(sets) {
        let mut r = Rating::new(3.5, 5).half_steps(true).symbols(symbols);
        if color != Color::Reset {
            r = r.filled_color(color);
        }
        named(f, row, name, r.show_value(true));
    }
}

fn smooth_bars_badges(f: &mut Frame, area: Rect) {
    let r = rows(area, 17);
    f.render_widget(
        Paragraph::new(" Smooth fractional fill").add_modifier(Modifier::BOLD),
        r[0],
    );
    for (i, v) in [2.6, 5.3, 7.4, 9.8].into_iter().enumerate() {
        named(
            f,
            r[1 + i],
            &format!("{v:.1}"),
            Rating::new(v, 10)
                .symbols(RatingSymbols::smooth())
                .fractional(true)
                .gap(0)
                .traffic_light()
                .show_value(true),
        );
    }
    f.render_widget(Paragraph::new(" Bars").add_modifier(Modifier::BOLD), r[6]);
    f.render_widget(
        RatingBar::new(3.7, 5.0)
            .traffic_light()
            .label("gradient")
            .show_value(true),
        r[7],
    );
    f.render_widget(
        RatingBar::new(4.4, 5.0)
            .traffic_light()
            .color_by_value(true)
            .label("by value")
            .show_value(true)
            .value_format(ValueFormat::Percent),
        r[8],
    );
    f.render_widget(
        RatingBar::new(2.0, 5.0)
            .preset(RatingStyle::neon())
            .label("neon    ")
            .show_value(true),
        r[9],
    );
    f.render_widget(
        RatingBar::new(2.0, 5.0)
            .glyphs("▰", "▱")
            .partials(Vec::<String>::new())
            .filled_color(Color::Cyan)
            .label("segment ")
            .show_value(true),
        r[10],
    );
    f.render_widget(
        Paragraph::new(" Badges").add_modifier(Modifier::BOLD),
        r[12],
    );
    let [a, b, c] = Layout::horizontal([Constraint::Length(28); 3]).areas(r[13]);
    f.render_widget(RatingBadge::new(4.2, 5.0), a);
    f.render_widget(
        RatingBadge::new(4.2, 5.0)
            .kind(BadgeKind::Pill)
            .color(Color::Yellow),
        b,
    );
    f.render_widget(
        RatingBadge::new(8.1, 10.0)
            .kind(BadgeKind::Plain)
            .show_max(true)
            .traffic_light(),
        c,
    );
    let [a, b, c] = Layout::horizontal([Constraint::Length(28); 3]).areas(r[14]);
    f.render_widget(RatingBadge::new(4.8, 5.0).votes(1203), a);
    f.render_widget(
        RatingBadge::new(3.1, 5.0)
            .kind(BadgeKind::Pill)
            .votes(48_200)
            .compact_votes(true)
            .traffic_light(),
        b,
    );
    f.render_widget(
        RatingBadge::new(1.9, 5.0)
            .kind(BadgeKind::Plain)
            .symbol("♥")
            .votes(7)
            .color(Color::Red),
        c,
    );
}

fn reviews(f: &mut Frame, area: Rect) {
    let [left, right] =
        Layout::horizontal([Constraint::Percentage(55), Constraint::Percentage(45)]).areas(area);
    f.render_widget(
        RatingHistogram::new([42, 51, 120, 610, 1480])
            .summary(true)
            .traffic_light()
            .block(Block::bordered().title(" Customer reviews ")),
        left,
    );
    let [top, bottom] = Layout::vertical([Constraint::Length(8), Constraint::Min(1)]).areas(right);
    let [v1, v2, v3] = Layout::horizontal([Constraint::Length(9); 3]).areas(top);
    let v = |val, symbols: RatingSymbols, color| {
        Rating::new(val, 5)
            .orientation(Orientation::Vertical)
            .symbols(symbols)
            .filled_color(color)
            .gap(0)
            .alignment(Alignment::Center)
            .show_value(true)
    };
    f.render_widget(v(3.0, RatingSymbols::stars(), Color::Yellow), v1);
    f.render_widget(v(4.0, RatingSymbols::circles(), Color::Green), v2);
    f.render_widget(
        v(2.0, RatingSymbols::hexagons(), Color::Cyan).reversed(true),
        v3,
    );
    let b = rows(bottom, 6);
    f.render_widget(
        BipolarRating::new(-2.0, 2).label("mood").show_value(true),
        b[0],
    );
    f.render_widget(
        BipolarRating::new(1.0, 2).label("mood").show_value(true),
        b[1],
    );
    f.render_widget(BipolarRating::new(2.0, 2).thumbs().show_value(true), b[2]);
    f.render_widget(BipolarRating::new(-1.0, 3).arrows().show_value(true), b[3]);
    f.render_widget(
        Rating::new(2.0, 4)
            .focused(true)
            .label("focus")
            .label_position(LabelPosition::After),
        b[4],
    );
    f.render_widget(Rating::new(3.0, 4).disabled(true).label("off"), b[5]);
}

fn animation(f: &mut Frame, area: Rect, app: &App) {
    let r = rows(area, 7);
    let pulse = Style::default()
        .add_modifier(Modifier::BOLD | Modifier::REVERSED)
        .fg(Color::White);
    f.render_widget(
        Paragraph::new(" Eased fill-in with a pulse on the item being filled")
            .add_modifier(Modifier::BOLD),
        r[0],
    );
    named(
        f,
        r[2],
        "stars",
        Rating::from_state(&app.anim[0])
            .highlight_style(pulse)
            .show_value(true),
    );
    named(
        f,
        r[3],
        "hearts",
        Rating::from_state(&app.anim[1])
            .symbols(RatingSymbols::hearts())
            .filled_color(Color::Red)
            .highlight_style(pulse)
            .show_value(true),
    );
    f.render_widget(
        RatingBar::new(app.anim[2].visual_value(), 10.0)
            .traffic_light()
            .label("bar       ")
            .show_value(true),
        r[5],
    );
    f.render_widget(
        Paragraph::new(" RatingState::animate_to(target, ticks) + tick()")
            .style(Style::default().fg(Color::DarkGray)),
        r[6],
    );
}
