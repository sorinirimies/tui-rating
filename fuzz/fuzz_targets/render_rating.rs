#![no_main]
#[path = "common.rs"]
mod common;
use arbitrary::Arbitrary;
use common::*;
use libfuzzer_sys::fuzz_target;
use ratatui::{buffer::Buffer, widgets::{Block, Widget}};
use tui_rating::{Rating, RatingSymbols};

#[derive(Arbitrary, Debug)]
struct Input {
    value: f64,
    max: u32,
    half: bool,
    gap: u16,
    align: u8,
    show_value: bool,
    block: bool,
    label: Option<String>,
    filled: String,
    empty: String,
    half_sym: Option<String>,
    area: Area,
}

fuzz_target!(|i: Input| {
    let mut syms = RatingSymbols::new(i.filled, i.empty);
    if let Some(h) = i.half_sym {
        syms = syms.half(h);
    }
    let mut r = Rating::new(i.value, cap(i.max))
        .half_steps(i.half)
        .symbols(syms)
        .gap(i.gap % 64)
        .alignment(align(i.align))
        .show_value(i.show_value);
    if let Some(l) = i.label {
        r = r.label(l);
    }
    if i.block {
        r = r.block(Block::bordered());
    }
    let area = i.area.rect();
    let mut buf = Buffer::empty(area);
    r.render(area, &mut buf);
});
