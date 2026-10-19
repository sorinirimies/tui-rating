#![no_main]
#[path = "common.rs"]
mod common;
use arbitrary::Arbitrary;
use common::*;
use libfuzzer_sys::fuzz_target;
use ratatui::{buffer::Buffer, widgets::Widget};
use tui_rating::{
    BipolarRating, Orientation, Rating, RatingBadge, RatingBar, RatingHistogram, RatingSymbols,
};

#[derive(Arbitrary, Debug)]
struct Input {
    value: f64,
    max: u32,
    half: bool,
    smooth: bool,
    vertical: bool,
    reversed: bool,
    focused: bool,
    disabled: bool,
    gap: u16,
    align: u8,
    label: Option<String>,
    hover: Option<f64>,
    counts: Vec<u32>,
    range: u8,
    votes: u64,
    click: (u16, u16),
    area: Area,
}

fuzz_target!(|i: Input| {
    let area = i.area.rect();
    let mut buf = Buffer::empty(area);
    let mut r = Rating::new(i.value, cap(i.max))
        .half_steps(i.half)
        .fractional(i.smooth)
        .reversed(i.reversed)
        .focused(i.focused)
        .disabled(i.disabled)
        .hover(i.hover)
        .gap(i.gap % 64)
        .alignment(align(i.align))
        .show_value(true);
    if i.smooth {
        r = r.symbols(RatingSymbols::smooth());
    }
    if i.vertical {
        r = r.orientation(Orientation::Vertical);
    }
    if let Some(l) = i.label {
        r = r.label(l);
    }
    let _ = r.value_at(area, i.click.0, i.click.1);
    let _ = r.item_at(area, i.click.0, i.click.1);
    r.render(area, &mut buf);
    RatingBar::new(i.value, f64::from(i.max)).traffic_light().show_value(true).render(area, &mut buf);
    RatingBadge::new(i.value, f64::from(i.max)).votes(i.votes).traffic_light().render(area, &mut buf);
    let counts: Vec<u64> = i.counts.iter().take(64).map(|&c| u64::from(c)).collect();
    RatingHistogram::new(counts).summary(true).traffic_light().render(area, &mut buf);
    BipolarRating::new(i.value, (i.range % 32) as usize).show_value(true).render(area, &mut buf);
});
