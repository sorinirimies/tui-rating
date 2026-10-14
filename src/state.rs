//! Rating state management.

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
pub struct RatingState {
    value: f64,
    max: usize,
    half_steps: bool,
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
        self.value = if value.is_finite() {
            value.clamp(0.0, self.max as f64)
        } else {
            0.0
        };
    }

    /// Changes the maximum, re-clamping the value.
    pub fn set_max(&mut self, max: usize) {
        self.max = max;
        self.set_value(self.value);
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
        self.value = 0.0;
    }

    /// Sets the value to `max`.
    pub fn set_full(&mut self) {
        self.value = self.max as f64;
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
