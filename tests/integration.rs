use ratatui::{
    backend::TestBackend,
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Color,
    widgets::{Block, Widget},
    Terminal,
};
use tui_rating::{ItemFill, Rating, RatingState, RatingSymbols};

fn row(buf: &Buffer, y: u16, w: u16) -> String {
    (0..w).map(|x| buf[(x, y)].symbol().to_string()).collect()
}

#[test]
fn state_drives_widget() {
    let mut state = RatingState::new(0.0, 5);
    for _ in 0..3 {
        state.increment();
    }
    let area = Rect::new(0, 0, 9, 1);
    let mut buf = Buffer::empty(area);
    Rating::from_state(&state).render(area, &mut buf);
    assert_eq!(row(&buf, 0, 9), "★ ★ ★ ☆ ☆");
}

#[test]
fn half_step_flow() {
    let mut state = RatingState::new(0.0, 3).with_half_steps(true);
    state.increment();
    state.increment();
    state.increment();
    assert_eq!(state.value(), 1.5);
    let area = Rect::new(0, 0, 5, 1);
    let mut buf = Buffer::empty(area);
    Rating::from_state(&state)
        .symbols(RatingSymbols::circles())
        .render(area, &mut buf);
    assert_eq!(row(&buf, 0, 5), "● ◐ ○");
}

#[test]
fn all_presets_render() {
    for s in [
        RatingSymbols::stars(),
        RatingSymbols::hearts(),
        RatingSymbols::circles(),
        RatingSymbols::squares(),
        RatingSymbols::diamonds(),
        RatingSymbols::ascii(),
    ] {
        let area = Rect::new(0, 0, 9, 1);
        let mut buf = Buffer::empty(area);
        Rating::new(2.5, 5)
            .half_steps(true)
            .symbols(s.clone())
            .render(area, &mut buf);
        assert!(row(&buf, 0, 9).contains(&s.filled));
    }
}

#[test]
fn styles_applied() {
    let area = Rect::new(0, 0, 3, 1);
    let mut buf = Buffer::empty(area);
    Rating::new(1.0, 2)
        .gap(0)
        .filled_color(Color::Red)
        .empty_color(Color::Blue)
        .render(area, &mut buf);
    assert_eq!(buf[(0, 0)].fg, Color::Red);
    assert_eq!(buf[(1, 0)].fg, Color::Blue);
}

#[test]
fn renders_in_terminal_with_block_and_alignment() {
    let mut term = Terminal::new(TestBackend::new(20, 3)).unwrap();
    term.draw(|f| {
        let r = Rating::new(2.0, 3)
            .gap(0)
            .alignment(Alignment::Center)
            .block(Block::bordered());
        f.render_widget(r, f.area());
    })
    .unwrap();
    let buf = term.backend().buffer();
    assert_eq!(buf[(8, 1)].symbol(), "★");
    assert_eq!(buf[(9, 1)].symbol(), "★");
    assert_eq!(buf[(10, 1)].symbol(), "☆");
}

#[test]
fn reference_render_does_not_consume() {
    let r = Rating::new(1.0, 2);
    let area = Rect::new(0, 0, 3, 1);
    let mut a = Buffer::empty(area);
    let mut b = Buffer::empty(area);
    (&r).render(area, &mut a);
    (&r).render(area, &mut b);
    assert_eq!(a, b);
}

#[test]
fn fill_at_reports_items() {
    let r = Rating::new(1.5, 3).half_steps(true);
    assert_eq!(
        (0..3).map(|i| r.fill_at(i)).collect::<Vec<_>>(),
        vec![ItemFill::Full, ItemFill::Half, ItemFill::Empty]
    );
}

#[test]
fn out_of_range_values_are_safe() {
    let mut s = RatingState::new(100.0, 5);
    assert!(s.is_full());
    s.set_value(-5.0);
    assert!(s.is_empty());
    let area = Rect::new(0, 0, 9, 1);
    let mut buf = Buffer::empty(area);
    Rating::new(f64::NAN, 5).render(area, &mut buf);
}
