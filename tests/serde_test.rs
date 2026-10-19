#![cfg(feature = "serde")]

use tui_rating::{Easing, RatingState, ValueFormat};

#[test]
fn state_roundtrip_skips_transient_fields() {
    let mut s = RatingState::new(3.5, 5)
        .with_half_steps(true)
        .with_read_only(true);
    s.set_hover(Some(2.0));
    s.animate_to(1.0, 5);
    let json = serde_json::to_string(&s).unwrap();
    assert!(!json.contains("hover") && !json.contains("anim"), "{json}");
    let back: RatingState = serde_json::from_str(&json).unwrap();
    assert_eq!(back.max(), 5);
    assert_eq!(back.value(), 1.0);
    assert!(back.half_steps() && back.is_read_only());
    assert_eq!(back.hover(), None);
    assert!(!back.is_animating());
}

#[test]
fn enums_roundtrip() {
    for f in [
        ValueFormat::Fraction,
        ValueFormat::Number,
        ValueFormat::Percent,
    ] {
        let j = serde_json::to_string(&f).unwrap();
        assert_eq!(serde_json::from_str::<ValueFormat>(&j).unwrap(), f);
    }
    let j = serde_json::to_string(&Easing::EaseInOut).unwrap();
    assert_eq!(
        serde_json::from_str::<Easing>(&j).unwrap(),
        Easing::EaseInOut
    );
}
