//! Symbol sets used to draw rating items.

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
}

impl RatingSymbols {
    /// Creates a custom symbol set without a dedicated half glyph.
    pub fn new(filled: impl Into<String>, empty: impl Into<String>) -> Self {
        Self {
            filled: filled.into(),
            half: None,
            empty: empty.into(),
        }
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
            RatingSymbols::ascii(),
        ] {
            assert!(!s.filled.is_empty());
            assert!(!s.empty.is_empty());
            assert_ne!(s.filled, s.empty);
        }
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
