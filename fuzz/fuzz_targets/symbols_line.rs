#![no_main]
#[path = "common.rs"]
mod common;
use arbitrary::Arbitrary;
use common::*;
use libfuzzer_sys::fuzz_target;
use tui_rating::{Rating, RatingState, RatingSymbols};

#[derive(Arbitrary, Debug)]
struct Input {
    value: f64,
    max: u32,
    half: bool,
    preset: u8,
    show_value: bool,
    align: u8,
    index: usize,
}

fuzz_target!(|i: Input| {
    let symbols = match i.preset % 6 {
        0 => RatingSymbols::stars(),
        1 => RatingSymbols::hearts(),
        2 => RatingSymbols::circles(),
        3 => RatingSymbols::squares(),
        4 => RatingSymbols::diamonds(),
        _ => RatingSymbols::ascii(),
    };
    let state = RatingState::new(i.value, cap(i.max)).with_half_steps(i.half);
    let r = Rating::from_state(&state)
        .symbols(symbols)
        .show_value(i.show_value)
        .alignment(align(i.align));
    let _ = r.fill_at(i.index);
    let _ = (r.display_value(), r.value_text(), r.line(), r.width());
});
