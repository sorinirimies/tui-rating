use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use tui_rating::{Rating, RatingBar, RatingHistogram, RatingState, RatingStyle, RatingSymbols};

fn bench(c: &mut Criterion) {
    let area = Rect::new(0, 0, 80, 12);
    c.bench_function("rating_5_stars", |b| {
        b.iter(|| {
            let mut buf = Buffer::empty(area);
            Rating::new(black_box(3.5), 5)
                .half_steps(true)
                .show_value(true)
                .render(area, &mut buf);
        })
    });
    c.bench_function("rating_10_gradient_smooth", |b| {
        b.iter(|| {
            let mut buf = Buffer::empty(area);
            Rating::new(black_box(6.7), 10)
                .symbols(RatingSymbols::smooth())
                .fractional(true)
                .preset(RatingStyle::neon())
                .render(area, &mut buf);
        })
    });
    c.bench_function("rating_vertical", |b| {
        b.iter(|| {
            let mut buf = Buffer::empty(area);
            Rating::new(black_box(3.0), 10)
                .orientation(tui_rating::Orientation::Vertical)
                .render(area, &mut buf);
        })
    });
    c.bench_function("bar", |b| {
        b.iter(|| {
            let mut buf = Buffer::empty(area);
            RatingBar::new(black_box(3.7), 5.0)
                .traffic_light()
                .show_value(true)
                .render(area, &mut buf);
        })
    });
    c.bench_function("histogram", |b| {
        b.iter(|| {
            let mut buf = Buffer::empty(area);
            RatingHistogram::new(black_box([12u64, 30, 80, 400, 1200]))
                .summary(true)
                .traffic_light()
                .render(area, &mut buf);
        })
    });
    c.bench_function("value_at", |b| {
        let r = Rating::new(3.0, 5);
        b.iter(|| r.value_at(area, black_box(4), 0))
    });
    c.bench_function("state_animation", |b| {
        b.iter(|| {
            let mut s = RatingState::new(0.0, 5);
            s.animate_to(black_box(5.0), 30);
            while s.tick() {}
            s.visual_value()
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
