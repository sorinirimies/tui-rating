//! Symbol sets used to draw rating items.

use crate::util::{cell_width, eighths};

/// The glyphs used for filled, half-filled and empty rating items.
///
/// If `half` is `None`, half items are drawn with the `empty` glyph using the
/// filled style, which reads as an "outlined" item in the filled colour.
///
/// # Example
///
/// ```
/// use tui_rating::RatingSymbols;
///
/// let s = RatingSymbols::hearts();
/// assert_eq!(s.filled, "♥");
/// let custom = RatingSymbols::new("X", "o");
/// assert_eq!(custom.empty, "o");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RatingSymbols {
    /// Glyph for a fully filled item.
    pub filled: String,
    /// Optional glyph for a half-filled item.
    pub half: Option<String>,
    /// Glyph for an empty item.
    pub empty: String,
    /// Partial-fill glyphs, thinnest first, used by
    /// [`Rating::fractional`](crate::Rating::fractional).
    pub partials: Vec<String>,
}

impl RatingSymbols {
    /// Creates a custom symbol set without a dedicated half glyph.
    pub fn new(filled: impl Into<String>, empty: impl Into<String>) -> Self {
        Self {
            filled: filled.into(),
            half: None,
            empty: empty.into(),
            partials: Vec::new(),
        }
    }

    /// Builder: set partial-fill glyphs (thinnest first) for smooth
    /// fractional ratings.
    #[must_use]
    pub fn partials<I, S>(mut self, partials: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.partials = partials.into_iter().map(Into::into).collect();
        self
    }

    /// Width in terminal cells of the widest glyph; items are padded to it.
    pub fn cell_width(&self) -> usize {
        let mut w = cell_width(&self.filled).max(cell_width(&self.empty));
        if let Some(h) = &self.half {
            w = w.max(cell_width(h));
        }
        for p in &self.partials {
            w = w.max(cell_width(p));
        }
        w
    }

    /// Builder: set the half glyph.
    #[must_use]
    pub fn half(mut self, half: impl Into<String>) -> Self {
        self.half = Some(half.into());
        self
    }

    /// `★` / `☆`
    pub fn stars() -> Self {
        Self::new("★", "☆")
    }

    /// `♥` / `♡`
    pub fn hearts() -> Self {
        Self::new("♥", "♡")
    }

    /// `●` / `◐` / `○`
    pub fn circles() -> Self {
        Self::new("●", "○").half("◐")
    }

    /// `■` / `◧` / `□`
    pub fn squares() -> Self {
        Self::new("■", "□").half("◧")
    }

    /// `◆` / `◇`
    pub fn diamonds() -> Self {
        Self::new("◆", "◇")
    }

    /// `✦` / `✧`
    pub fn sparkles() -> Self {
        Self::new("✦", "✧")
    }

    /// `⬢` / `⬡`
    pub fn hexagons() -> Self {
        Self::new("⬢", "⬡")
    }

    /// `▰` / `▱` (segmented bar)
    pub fn bars() -> Self {
        Self::new("▰", "▱")
    }

    /// `•` / `·`
    pub fn dots() -> Self {
        Self::new("•", "·")
    }

    /// `█` / `▒` / `░` (block shades, half = `▓`)
    pub fn blocks() -> Self {
        Self::new("█", "░").half("▓")
    }

    /// `█` with block-eighth partials over `░`: smooth, continuous fill.
    ///
    /// Use with [`Rating::fractional`](crate::Rating::fractional).
    pub fn smooth() -> Self {
        Self::new("█", "░").partials(eighths())
    }

    /// `🔥` / `・` (wide emoji)
    pub fn fire() -> Self {
        Self::new("🔥", "・")
    }

    /// `🍕` / `・` (wide emoji)
    pub fn pizza() -> Self {
        Self::new("🍕", "・")
    }

    /// `🌙` / `・` (wide emoji)
    pub fn moon() -> Self {
        Self::new("🌙", "・")
    }

    /// `⭐` / `・` (wide emoji)
    pub fn star_emoji() -> Self {
        Self::new("⭐", "・")
    }

    /// `*` / `-` / `+` (plain ASCII)
    pub fn ascii() -> Self {
        Self::new("*", "-").half("+")
    }
}

impl Default for RatingSymbols {
    fn default() -> Self {
        Self::stars()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_stars() {
        assert_eq!(RatingSymbols::default(), RatingSymbols::stars());
    }

    #[test]
    fn presets_non_empty() {
        for s in [
            RatingSymbols::stars(),
            RatingSymbols::hearts(),
            RatingSymbols::circles(),
            RatingSymbols::squares(),
            RatingSymbols::diamonds(),
            RatingSymbols::sparkles(),
            RatingSymbols::hexagons(),
            RatingSymbols::bars(),
            RatingSymbols::dots(),
            RatingSymbols::blocks(),
            RatingSymbols::smooth(),
            RatingSymbols::fire(),
            RatingSymbols::pizza(),
            RatingSymbols::moon(),
            RatingSymbols::star_emoji(),
            RatingSymbols::ascii(),
        ] {
            assert!(!s.filled.is_empty());
            assert!(!s.empty.is_empty());
            assert_ne!(s.filled, s.empty);
        }
    }

    #[test]
    fn cell_width_is_widest() {
        assert_eq!(RatingSymbols::stars().cell_width(), 1);
        assert_eq!(RatingSymbols::fire().cell_width(), 2);
        assert_eq!(RatingSymbols::new("a", "b").half("🔥").cell_width(), 2);
        assert_eq!(
            RatingSymbols::new("a", "b").partials(["🔥"]).cell_width(),
            2
        );
        assert_eq!(RatingSymbols::smooth().partials.len(), 7);
    }

    #[test]
    fn half_builder() {
        assert_eq!(
            RatingSymbols::new("a", "b").half("c").half.as_deref(),
            Some("c")
        );
        assert!(RatingSymbols::stars().half.is_none());
        assert_eq!(RatingSymbols::circles().half.as_deref(), Some("◐"));
    }
}
