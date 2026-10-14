#![no_main]
use libfuzzer_sys::fuzz_target;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use tui_rating::{Rating, RatingSymbols};

// Raw bytes -> lossy UTF-8 used as label and glyphs (odd/wide/combining chars).
fuzz_target!(|data: &[u8]| {
    let s = String::from_utf8_lossy(data).into_owned();
    let third = s.chars().count() / 3;
    let a: String = s.chars().take(third).collect();
    let b: String = s.chars().skip(third).take(third).collect();
    let c: String = s.chars().skip(2 * third).collect();
    let r = Rating::new(2.5, 5)
        .half_steps(true)
        .symbols(RatingSymbols::new(a, b).half(c.clone()))
        .label(c)
        .show_value(true);
    for (w, h) in [(0, 0), (1, 1), (7, 1), (40, 3), (200, 100)] {
        let area = Rect::new(3, 2, w, h);
        let mut buf = Buffer::empty(area);
        r.clone().render(area, &mut buf);
    }
});
