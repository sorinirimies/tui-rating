//! Rating state management.

use crate::animation::{Animation, Easing};

/// Mutable state for a [`Rating`](crate::Rating) widget.
///
/// The value is expressed in "items" (e.g. stars): `3.5` with a `max` of `5`
/// means three and a half stars. The value is always clamped to `0.0..=max`.
///
/// # Example
///
/// ```
/// use tui_rating::RatingState;
///
/// let mut state = RatingState::new(3.0, 5);
/// state.increment();
/// assert_eq!(state.value(), 4.0);
/// state.set_half_steps(true);
/// state.decrement();
/// assert_eq!(state.value(), 3.5);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RatingState {
    value: f64,
    max: usize,
    half_steps: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    hover: Option<f64>,
    #[cfg_attr(feature = "serde", serde(skip))]
    anim: Option<Animation>,
    #[cfg_attr(feature = "serde", serde(default))]
    read_only: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    click_to_clear: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    prev: Option<f64>,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub(crate) digit: Option<u8>,
}

/// A committed value change, as returned by `RatingState::handle_key_change`
/// (feature `crossterm`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Change {
    /// Value before the input.
    pub old: f64,
    /// Value after the input.
    pub new: f64,
}

impl RatingState {
    /// Creates a state with the given `value` and `max` number of items.
    ///
    /// `value` is clamped into `0.0..=max`; non-finite values become `0.0`.
    pub fn new(value: f64, max: usize) -> Self {
        let mut s = Self {
            value: 0.0,
            max,
            half_steps: false,
            hover: None,
            anim: None,
            read_only: false,
            click_to_clear: false,
            prev: None,
            digit: None,
        };
        s.set_value(value);
        s
    }

    /// Builder: enable or disable half-step increments.
    #[must_use]
    pub fn with_half_steps(mut self, enabled: bool) -> Self {
        self.set_half_steps(enabled);
        self
    }

    /// Builder: mark the state read-only (input handlers become no-ops).
    #[must_use]
    pub fn with_read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    /// Builder: clicking the item that is already the value clears the
    /// rating (the "click again to reset" behaviour of most web widgets).
    #[must_use]
    pub fn with_click_to_clear(mut self, enabled: bool) -> Self {
        self.click_to_clear = enabled;
        self
    }

    /// Whether clicking the current value clears it.
    pub fn click_to_clear(&self) -> bool {
        self.click_to_clear
    }

    /// Reverts to the value before the last change. Calling it again redoes
    /// the change (a single-level undo/redo toggle). Returns `true` if the
    /// value changed.
    pub fn undo(&mut self) -> bool {
        match self.prev {
            Some(p) if (p - self.value).abs() > f64::EPSILON => {
                self.prev = Some(self.value);
                self.anim = None;
                self.value = p;
                true
            }
            _ => false,
        }
    }

    /// `true` if [`undo`](Self::undo) would change the value.
    pub fn can_undo(&self) -> bool {
        self.prev
            .is_some_and(|p| (p - self.value).abs() > f64::EPSILON)
    }

    /// Screen-reader friendly text, e.g. `"3.5 out of 5"`.
    pub fn describe(&self) -> String {
        crate::util::describe(self.value, self.max)
    }

    /// Whether input handlers (`handle_key`, `handle_mouse`) are ignored.
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    /// Sets the read-only flag.
    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
    }

    /// Current value (`0.0..=max`).
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Maximum number of items.
    pub fn max(&self) -> usize {
        self.max
    }

    /// Whether half-step increments are enabled.
    pub fn half_steps(&self) -> bool {
        self.half_steps
    }

    /// Step size: `0.5` with half steps, otherwise `1.0`.
    pub fn step(&self) -> f64 {
        if self.half_steps {
            0.5
        } else {
            1.0
        }
    }

    /// Sets the value, clamping into range. Non-finite input resets to `0.0`.
    pub fn set_value(&mut self, value: f64) {
        self.anim = None;
        let new = if value.is_finite() {
            value.clamp(0.0, self.max as f64)
        } else {
            0.0
        };
        self.commit(new);
    }

    fn commit(&mut self, new: f64) {
        if (new - self.value).abs() > f64::EPSILON {
            self.prev = Some(self.value);
        }
        self.value = new;
    }

    /// Hover preview value (e.g. from the mouse), shown by
    /// [`Rating::from_state`](crate::Rating::from_state) without committing.
    pub fn hover(&self) -> Option<f64> {
        self.hover
    }

    /// Sets or clears the hover preview (clamped into range).
    pub fn set_hover(&mut self, hover: Option<f64>) {
        self.hover = hover
            .filter(|h| h.is_finite())
            .map(|h| h.clamp(0.0, self.max as f64));
    }

    /// Animates towards `target` over `ticks` ticks with [`Easing::EaseOut`].
    ///
    /// The committed [`value`](Self::value) jumps to `target` immediately;
    /// only [`visual_value`](Self::visual_value) eases. Call
    /// [`tick`](Self::tick) once per frame/tick.
    pub fn animate_to(&mut self, target: f64, ticks: u32) {
        self.animate_to_with(target, ticks, Easing::default());
    }

    /// Like [`animate_to`](Self::animate_to) with a chosen [`Easing`].
    pub fn animate_to_with(&mut self, target: f64, ticks: u32, easing: Easing) {
        let from = self.visual_value();
        self.set_value(target);
        self.anim = if ticks == 0 || from == self.value {
            None
        } else {
            Some(Animation::new(from, self.value, ticks, easing))
        };
    }

    /// Advances the animation one tick. Returns `true` while animating.
    pub fn tick(&mut self) -> bool {
        match self.anim.as_mut() {
            Some(a) => {
                if a.tick() {
                    true
                } else {
                    self.anim = None;
                    false
                }
            }
            None => false,
        }
    }

    /// `true` while an animation is running.
    pub fn is_animating(&self) -> bool {
        self.anim.is_some()
    }

    /// The value to draw: the animated value while animating, else
    /// [`value`](Self::value).
    pub fn visual_value(&self) -> f64 {
        self.anim.map_or(self.value, |a| a.value())
    }

    /// While animating, the 0-based index of the item currently being
    /// filled (useful for a pulse via
    /// [`Rating::highlight`](crate::Rating::highlight)).
    pub fn pulse_index(&self) -> Option<usize> {
        let v = self.anim?.value();
        (v > 0.0).then(|| v.ceil() as usize - 1)
    }

    /// Changes the maximum, re-clamping the value.
    pub fn set_max(&mut self, max: usize) {
        self.max = max;
        self.anim = None;
        self.set_value(self.value);
        self.set_hover(self.hover);
    }

    /// Enables/disables half steps. Disabling snaps the value to a whole number.
    pub fn set_half_steps(&mut self, enabled: bool) {
        self.half_steps = enabled;
        if !enabled {
            self.value = self.value.round();
        }
    }

    /// Increases the value by one step (saturating at `max`).
    pub fn increment(&mut self) {
        self.set_value(self.value + self.step());
    }

    /// Decreases the value by one step (saturating at `0`).
    pub fn decrement(&mut self) {
        self.set_value(self.value - self.step());
    }

    /// Sets the value to `0`.
    pub fn clear(&mut self) {
        self.anim = None;
        self.commit(0.0);
    }

    /// Sets the value to `max`.
    pub fn set_full(&mut self) {
        self.anim = None;
        self.commit(self.max as f64);
    }

    /// Sets the value to the n-th item (1-based), e.g. for number-key input.
    pub fn set_items(&mut self, n: usize) {
        self.set_value(n as f64);
    }

    /// Fraction filled, `0.0..=1.0` (`0.0` if `max == 0`).
    pub fn ratio(&self) -> f64 {
        if self.max == 0 {
            0.0
        } else {
            self.value / self.max as f64
        }
    }

    /// Percentage filled, `0.0..=100.0`.
    pub fn percentage(&self) -> f64 {
        self.ratio() * 100.0
    }

    /// `true` if value is `0`.
    pub fn is_empty(&self) -> bool {
        self.value <= 0.0
    }

    /// `true` if value equals `max`.
    pub fn is_full(&self) -> bool {
        self.value >= self.max as f64
    }
}

impl Default for RatingState {
    /// Zero of five items.
    fn default() -> Self {
        Self::new(0.0, 5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hover_clamps_and_clears() {
        let mut s = RatingState::new(1.0, 5);
        s.set_hover(Some(9.0));
        assert_eq!(s.hover(), Some(5.0));
        s.set_hover(Some(f64::NAN));
        assert_eq!(s.hover(), None);
        s.set_hover(Some(2.0));
        s.set_max(1);
        assert_eq!(s.hover(), Some(1.0));
        s.set_hover(None);
        assert_eq!(s.hover(), None);
    }

    #[test]
    fn read_only_flag() {
        let mut s = RatingState::default().with_read_only(true);
        assert!(s.is_read_only());
        s.set_read_only(false);
        assert!(!s.is_read_only());
    }

    #[test]
    fn animation_flow() {
        let mut s = RatingState::new(0.0, 5);
        s.animate_to_with(4.0, 4, Easing::Linear);
        assert_eq!(s.value(), 4.0);
        assert!(s.is_animating());
        assert_eq!(s.visual_value(), 0.0);
        assert_eq!(s.pulse_index(), None);
        assert!(s.tick());
        assert_eq!(s.visual_value(), 1.0);
        assert_eq!(s.pulse_index(), Some(0));
        s.tick();
        s.tick();
        assert!(!s.tick());
        assert!(!s.is_animating());
        assert_eq!(s.visual_value(), 4.0);
        assert!(!s.tick());
        assert_eq!(s.pulse_index(), None);
    }

    #[test]
    fn animation_skips_noop_and_cancels() {
        let mut s = RatingState::new(3.0, 5);
        s.animate_to(3.0, 5);
        assert!(!s.is_animating());
        s.animate_to(0.0, 0);
        assert!(!s.is_animating());
        assert_eq!(s.value(), 0.0);
        s.animate_to(5.0, 5);
        s.increment();
        assert!(!s.is_animating());
        s.animate_to(1.0, 3);
        s.clear();
        assert!(!s.is_animating());
        s.animate_to(1.0, 3);
        s.set_full();
        assert!(!s.is_animating());
        s.animate_to(1.0, 3);
        s.set_max(2);
        assert!(!s.is_animating());
    }

    #[test]
    fn clamps_on_new() {
        assert_eq!(RatingState::new(9.0, 5).value(), 5.0);
        assert_eq!(RatingState::new(-1.0, 5).value(), 0.0);
        assert_eq!(RatingState::new(f64::NAN, 5).value(), 0.0);
        assert_eq!(RatingState::new(f64::INFINITY, 5).value(), 0.0);
    }

    #[test]
    fn increment_decrement_saturate() {
        let mut s = RatingState::new(4.0, 5);
        s.increment();
        s.increment();
        assert_eq!(s.value(), 5.0);
        assert!(s.is_full());
        let mut s = RatingState::new(1.0, 5);
        s.decrement();
        s.decrement();
        assert_eq!(s.value(), 0.0);
        assert!(s.is_empty());
    }

    #[test]
    fn half_steps() {
        let mut s = RatingState::new(2.0, 5).with_half_steps(true);
        assert_eq!(s.step(), 0.5);
        s.increment();
        assert_eq!(s.value(), 2.5);
        s.decrement();
        s.decrement();
        assert_eq!(s.value(), 1.5);
    }

    #[test]
    fn disabling_half_steps_snaps() {
        let mut s = RatingState::new(2.5, 5).with_half_steps(true);
        s.set_half_steps(false);
        assert_eq!(s.value(), 3.0);
        assert!(!s.half_steps());
    }

    #[test]
    fn set_max_reclamps() {
        let mut s = RatingState::new(5.0, 5);
        s.set_max(3);
        assert_eq!(s.value(), 3.0);
        assert_eq!(s.max(), 3);
    }

    #[test]
    fn clear_full_items() {
        let mut s = RatingState::default();
        s.set_full();
        assert_eq!(s.value(), 5.0);
        s.clear();
        assert_eq!(s.value(), 0.0);
        s.set_items(3);
        assert_eq!(s.value(), 3.0);
        s.set_items(99);
        assert_eq!(s.value(), 5.0);
    }

    #[test]
    fn ratio_percentage() {
        let s = RatingState::new(2.5, 5).with_half_steps(true);
        assert!((s.ratio() - 0.5).abs() < 1e-9);
        assert!((s.percentage() - 50.0).abs() < 1e-9);
        let z = RatingState::new(0.0, 0);
        assert_eq!(z.ratio(), 0.0);
    }
}
