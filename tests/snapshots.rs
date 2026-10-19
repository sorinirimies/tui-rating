//! Whole-screen snapshots rendered through a `TestBackend`.

use ratatui::{
    backend::TestBackend,
    layout::{Alignment, Constraint, Layout},
    style::Color,
    widgets::Block,
    Terminal,
};
use tui_rating::{
    BadgeKind, BipolarRating, LabelPosition, Orientation, Rating, RatingBadge, RatingBar,
    RatingHistogram, RatingState, RatingStyle, RatingSymbols, ValueFormat,
};

fn lines(terminal: &Terminal<TestBackend>) -> Vec<String> {
    let buf = terminal.backend().buffer();
    let w = buf.area.width;
    (0..buf.area.height)
        .map(|y| {
            let mut out = String::new();
            let mut skip = 0;
            for x in 0..w {
                if skip > 0 {
                    skip -= 1;
                    continue;
                }
                let sym = buf[(x, y)].symbol();
                skip = ratatui::text::Span::raw(sym).width().saturating_sub(1);
                out.push_str(sym);
            }
            out.trim_end().to_string()
        })
        .collect()
}

#[test]
fn rating_gallery_snapshot() {
    let mut t = Terminal::new(TestBackend::new(34, 10)).unwrap();
    t.draw(|f| {
        let rows = Layout::vertical([Constraint::Length(1); 10]).split(f.area());
        f.render_widget(Rating::new(3.0, 5).label("stars").show_value(true), rows[0]);
        f.render_widget(
            Rating::new(3.5, 5)
                .half_steps(true)
                .symbols(RatingSymbols::circles())
                .gap(0),
            rows[1],
        );
        f.render_widget(
            Rating::new(3.75, 5)
                .symbols(RatingSymbols::smooth())
                .fractional(true)
                .gap(0)
                .value_format(ValueFormat::Percent)
                .show_value(true),
            rows[2],
        );
        f.render_widget(
            Rating::new(4.0, 5)
                .symbols(RatingSymbols::fire())
                .label("heat")
                .label_position(LabelPosition::After),
            rows[3],
        );
        f.render_widget(
            Rating::new(2.0, 4)
                .reversed(true)
                .alignment(Alignment::Right),
            rows[4],
        );
        f.render_widget(Rating::new(2.0, 3).focused(true), rows[5]);
        f.render_widget(RatingBar::new(3.7, 5.0).width(10).show_value(true), rows[6]);
        f.render_widget(
            RatingBadge::new(4.2, 5.0)
                .votes(1203)
                .kind(BadgeKind::Plain),
            rows[7],
        );
        f.render_widget(BipolarRating::new(-1.0, 2).show_value(true), rows[8]);
        f.render_widget(
            Rating::new(1.0, 2).preset(RatingStyle::retro()).gap(0),
            rows[9],
        );
    })
    .unwrap();
    assert_eq!(
        lines(&t),
        [
            "stars ★ ★ ★ ☆ ☆ 3/5",
            "●●●◐○",
            "███▊░ 75%",
            "🔥 🔥 🔥 🔥 ・ heat",
            "                           ☆ ☆ ★ ★",
            "[★ ★ ☆]",
            "███████▍░░ 3.7/5",
            "★ 4.2 (1,203)",
            "○ ● ┃ ○ ○ -1",
            "#.",
        ]
    );
}

#[test]
fn review_summary_snapshot() {
    let mut t = Terminal::new(TestBackend::new(34, 8)).unwrap();
    t.draw(|f| {
        f.render_widget(
            RatingHistogram::new([20, 30, 80, 400, 1200])
                .summary(true)
                .block(Block::bordered().title(" Reviews ")),
            f.area(),
        );
    })
    .unwrap();
    let got = lines(&t);
    assert_eq!(got[0], "┌ Reviews ───────────────────────┐");
    assert_eq!(got[1], "│4.6 ★  ·  1,730 ratings         │");
    assert!(got[2].starts_with("│5★ ") && got[2].contains("69%") && got[2].ends_with("1,200│"));
    assert!(got[7 - 1].starts_with("│1★ ") && got[6].contains("1%"));
}

#[test]
fn vertical_thermometer_snapshot() {
    let mut t = Terminal::new(TestBackend::new(6, 6)).unwrap();
    let state = RatingState::new(2.0, 4);
    t.draw(|f| {
        f.render_widget(
            Rating::from_state(&state)
                .orientation(Orientation::Vertical)
                .gap(0)
                .alignment(Alignment::Center)
                .filled_color(Color::Green)
                .label("T")
                .show_value(true),
            f.area(),
        );
    })
    .unwrap();
    assert_eq!(
        lines(&t),
        ["T", "☆", "☆", "★", "★", "2/4"]
            .iter()
            .map(|s| format!(
                "{}{}",
                " ".repeat((6 - ratatui::text::Line::from(*s).width()) / 2),
                s
            ))
            .collect::<Vec<_>>()
    );
}
