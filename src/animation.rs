//! Tick-based value animation for [`RatingState`](crate::RatingState).

/// Easing curve of an animation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Easing {
    /// Constant speed.
    Linear,
    /// Fast start, slow end (default).
    #[default]
    EaseOut,
    /// Slow start and end.
    EaseInOut,
}

impl Easing {
    /// Maps linear progress `t` (`0..=1`) to eased progress.
    pub fn apply(self, t: f64) -> f64 {
        let t = if t.is_finite() {
            t.clamp(0.0, 1.0)
        } else {
            1.0
        };
        match self {
            Easing::Linear => t,
            Easing::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            Easing::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
        }
    }
}

/// A running animation between two values.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Animation {
    from: f64,
    to: f64,
    tick: u32,
    duration: u32,
    easing: Easing,
}

impl Animation {
    /// Creates an animation lasting `duration` ticks.
    pub fn new(from: f64, to: f64, duration: u32, easing: Easing) -> Self {
        Self {
            from,
            to,
            tick: 0,
            duration,
            easing,
        }
    }

    /// Current interpolated value.
    pub fn value(&self) -> f64 {
        if self.is_done() {
            return self.to;
        }
        let t = self.tick as f64 / self.duration as f64;
        self.from + (self.to - self.from) * self.easing.apply(t)
    }

    /// Target value.
    pub fn target(&self) -> f64 {
        self.to
    }

    /// Advances one tick. Returns `true` while still running.
    pub fn tick(&mut self) -> bool {
        if !self.is_done() {
            self.tick += 1;
        }
        !self.is_done()
    }

    /// `true` once the duration has elapsed.
    pub fn is_done(&self) -> bool {
        self.tick >= self.duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn easing_endpoints() {
        for e in [Easing::Linear, Easing::EaseOut, Easing::EaseInOut] {
            assert_eq!(e.apply(0.0), 0.0);
            assert!((e.apply(1.0) - 1.0).abs() < 1e-9);
            assert_eq!(e.apply(f64::NAN), 1.0);
            assert!(e.apply(0.5) > 0.0 && e.apply(0.5) < 1.0 + 1e-9);
        }
        assert_eq!(Easing::Linear.apply(0.25), 0.25);
        assert!(Easing::EaseOut.apply(0.5) > 0.5);
        assert!(Easing::EaseInOut.apply(0.25) < 0.25);
        assert!(Easing::EaseInOut.apply(0.75) > 0.75);
    }

    #[test]
    fn runs_to_target() {
        let mut a = Animation::new(0.0, 4.0, 4, Easing::Linear);
        assert_eq!(a.value(), 0.0);
        assert!(a.tick());
        assert_eq!(a.value(), 1.0);
        a.tick();
        a.tick();
        assert!(!a.tick());
        assert!(a.is_done());
        assert_eq!(a.value(), 4.0);
        assert_eq!(a.target(), 4.0);
        assert!(!a.tick());
    }

    #[test]
    fn zero_duration_is_instant() {
        let a = Animation::new(1.0, 3.0, 0, Easing::EaseOut);
        assert!(a.is_done());
        assert_eq!(a.value(), 3.0);
    }
}
