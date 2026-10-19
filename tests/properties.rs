//! Property tests: invariants that must hold for any input.

use proptest::prelude::*;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use tui_rating::{
    BipolarRating, ItemFill, Orientation, Rating, RatingBadge, RatingBar, RatingHistogram,
    RatingState, RatingSymbols, ValueFormat,
};

fn any_f64() -> impl Strategy<Value = f64> {
    prop_oneof![
        -10.0..20.0f64,
        Just(f64::NAN),
        Just(f64::INFINITY),
        Just(f64::NEG_INFINITY),
        Just(0.0),
    ]
}

proptest! {
    #[test]
    fn state_value_always_in_range(v in any_f64(), max in 0usize..50, ops in proptest::collection::vec(0u8..6, 0..30)) {
        let mut s = RatingState::new(v, max).with_half_steps(max % 2 == 0);
        for op in ops {
            match op {
                0 => s.increment(),
                1 => s.decrement(),
                2 => s.clear(),
                3 => s.set_full(),
                4 => s.set_items(max / 2),
                _ => s.set_max(max.saturating_sub(1)),
            }
            prop_assert!(s.value() >= 0.0 && s.value() <= s.max() as f64);
            prop_assert!((0.0..=1.0).contains(&s.ratio()));
        }
    }

    #[test]
    fn fill_is_monotonic(v in any_f64(), max in 0usize..30, half in any::<bool>(), smooth in any::<bool>()) {
        let symbols = if smooth { RatingSymbols::smooth() } else { RatingSymbols::circles() };
        let r = Rating::new(v, max).half_steps(half).symbols(symbols).fractional(smooth);
        let rank = |f: ItemFill| match f {
            ItemFill::Full => 3,
            ItemFill::Half => 2,
            ItemFill::Partial(p) => { assert!((1..=99).contains(&p)); 2 }
            ItemFill::Empty => 0,
        };
        let mut prev = 3;
        for i in 0..max {
            let now = rank(r.fill_at(i));
            prop_assert!(now <= prev, "fill must not increase with index");
            prev = now;
        }
    }

    #[test]
    fn value_text_never_panics(v in any_f64(), max in 0usize..1000) {
        for f in [ValueFormat::Fraction, ValueFormat::Number, ValueFormat::Percent] {
            let t = Rating::new(v, max).value_format(f).value_text();
            prop_assert!(!t.is_empty());
        }
    }

    #[test]
    fn rating_renders_inside_area(
        v in any_f64(), max in 0usize..20, w in 0u16..40, h in 0u16..8,
        gap in 0u16..4, vertical in any::<bool>(), reversed in any::<bool>(),
    ) {
        let area = Rect::new(0, 0, w, h);
        let mut buf = Buffer::empty(area);
        let r = Rating::new(v, max)
            .gap(gap)
            .reversed(reversed)
            .orientation(if vertical { Orientation::Vertical } else { Orientation::Horizontal })
            .label("L")
            .show_value(true);
        r.clone().render(area, &mut buf);
        // hit testing is total
        for x in 0..w.min(10) { for y in 0..h.min(4) { let _ = r.value_at(area, x, y); } }
    }

    #[test]
    fn hit_test_roundtrip(max in 1usize..12, gap in 0u16..3, i in 0usize..12) {
        let i = i % max;
        let r = Rating::new(0.0, max).gap(gap);
        let area = Rect::new(3, 2, 80, 1);
        let x = 3 + (i as u16) * (1 + gap);
        prop_assert_eq!(r.item_at(area, x, 2), Some(i));
        prop_assert_eq!(r.value_at(area, x, 2), Some(i as f64 + 1.0));
    }

    #[test]
    fn other_widgets_never_panic(
        v in any_f64(), m in any_f64(), w in 0u16..40, h in 0u16..6,
        counts in proptest::collection::vec(0u64..1_000_000, 0..8), range in 0usize..6,
    ) {
        let area = Rect::new(0, 0, w, h);
        let mut buf = Buffer::empty(area);
        RatingBar::new(v, m).show_value(true).traffic_light().render(area, &mut buf);
        RatingBadge::new(v, m).votes(1234).render(area, &mut buf);
        RatingHistogram::new(counts).summary(true).traffic_light().render(area, &mut buf);
        BipolarRating::new(v, range).show_value(true).render(area, &mut buf);
    }

    #[test]
    fn histogram_percentages_are_sane(counts in proptest::collection::vec(0u64..10_000, 1..8)) {
        let h = RatingHistogram::new(counts.clone());
        let sum: u32 = (1..=counts.len()).map(|l| u32::from(h.percent(l))).sum();
        if h.total() > 0 {
            // rounding can drift by at most one per level
            prop_assert!(sum.abs_diff(100) <= counts.len() as u32);
            let a = h.average().unwrap();
            prop_assert!(a >= 1.0 && a <= counts.len() as f64);
        } else {
            prop_assert_eq!(sum, 0);
        }
    }

    #[test]
    fn animation_always_ends_on_target(from in 0.0..10.0f64, to in 0.0..10.0f64, ticks in 0u32..40) {
        let mut s = RatingState::new(from, 10);
        s.animate_to(to, ticks);
        let mut n = 0;
        while s.tick() { n += 1; prop_assert!(n <= ticks); }
        prop_assert!(!s.is_animating());
        prop_assert_eq!(s.visual_value(), s.value());
    }
}
