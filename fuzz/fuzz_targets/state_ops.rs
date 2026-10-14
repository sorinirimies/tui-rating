#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use tui_rating::RatingState;

#[derive(Arbitrary, Debug)]
enum Op {
    SetValue(f64),
    SetMax(usize),
    Half(bool),
    Inc,
    Dec,
    Clear,
    Full,
    Items(usize),
}

#[derive(Arbitrary, Debug)]
struct Input {
    value: f64,
    max: usize,
    half: bool,
    ops: Vec<Op>,
}

fuzz_target!(|i: Input| {
    let mut s = RatingState::new(i.value, i.max).with_half_steps(i.half);
    for op in i.ops {
        match op {
            Op::SetValue(v) => s.set_value(v),
            Op::SetMax(m) => s.set_max(m),
            Op::Half(h) => s.set_half_steps(h),
            Op::Inc => s.increment(),
            Op::Dec => s.decrement(),
            Op::Clear => s.clear(),
            Op::Full => s.set_full(),
            Op::Items(n) => s.set_items(n),
        }
        let v = s.value();
        assert!(v.is_finite() && v >= 0.0 && v <= s.max() as f64);
        let _ = (s.ratio(), s.percentage(), s.is_empty(), s.is_full(), s.step());
    }
});
