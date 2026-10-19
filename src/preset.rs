//! Bundled looks: [`RatingStyle`].

use ratatui::style::{Color, Modifier, Style};

use crate::RatingSymbols;

/// A bundle of symbols, colours and modifiers applied in one call with
/// [`Rating::preset`](crate::Rating::preset) or
/// [`RatingBar::preset`](crate::RatingBar::preset).
///
/// All fields are public, so presets can be tweaked:
///
/// ```
/// use ratatui::style::Color;
/// use tui_rating::{Rating, RatingStyle};
///
/// let mut style = RatingStyle::neon();
/// style.gap = 2;
/// let _ = Rating::new(4.0, 5).preset(style);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct RatingStyle {
    /// Glyph set.
    pub symbols: RatingSymbols,
    /// Filled items.
    pub filled: Style,
    /// Half / partial items (falls back to `filled`).
    pub half: Option<Style>,
    /// Empty items.
    pub empty: Style,
    /// Label text.
    pub label: Style,
    /// Value text.
    pub value: Style,
    /// Optional per-position colour gradient for filled items.
    pub gradient: Vec<Color>,
    /// Extra modifiers for filled items.
    pub modifier: Modifier,
    /// Extra modifiers for empty items.
    pub empty_modifier: Modifier,
    /// Spaces between items.
    pub gap: u16,
}

fn fg(r: u8, g: u8, b: u8) -> Style {
    Style::default().fg(Color::Rgb(r, g, b))
}

impl RatingStyle {
    fn base(symbols: RatingSymbols, filled: Style, empty: Style) -> Self {
        Self {
            symbols,
            filled,
            half: None,
            empty,
            label: Style::default(),
            value: Style::default(),
            gradient: Vec::new(),
            modifier: Modifier::empty(),
            empty_modifier: Modifier::empty(),
            gap: 1,
        }
    }

    /// Yellow stars on dark gray (the crate default look).
    pub fn classic() -> Self {
        Self::base(
            RatingSymbols::stars(),
            Style::default().fg(Color::Yellow),
            Style::default().fg(Color::DarkGray),
        )
    }

    /// Bold Netflix-red squares.
    pub fn netflix() -> Self {
        let mut s = Self::base(RatingSymbols::squares(), fg(229, 9, 20), fg(70, 70, 70));
        s.modifier = Modifier::BOLD;
        s.label = Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD);
        s.value = fg(180, 180, 180);
        s
    }

    /// GitHub-star gold on a muted outline.
    pub fn github() -> Self {
        let mut s = Self::base(RatingSymbols::stars(), fg(227, 179, 65), fg(110, 118, 129));
        s.label = fg(201, 209, 217);
        s.value = fg(139, 148, 158);
        s
    }

    /// IMDb yellow, bold stars.
    pub fn imdb() -> Self {
        let mut s = Self::base(RatingSymbols::stars(), fg(245, 197, 24), fg(60, 60, 60));
        s.modifier = Modifier::BOLD;
        s.label = Style::default().fg(Color::White);
        s
    }

    /// Magenta → cyan sparkles, bold.
    pub fn neon() -> Self {
        let mut s = Self::base(RatingSymbols::sparkles(), fg(255, 0, 200), fg(70, 40, 100));
        s.gradient = vec![Color::Rgb(255, 0, 200), Color::Rgb(0, 255, 255)];
        s.modifier = Modifier::BOLD;
        s.label = fg(0, 255, 255);
        s.value = fg(255, 0, 200);
        s
    }

    /// Green phosphor terminal: ASCII `#` and `.`.
    pub fn retro() -> Self {
        let mut s = Self::base(
            RatingSymbols::new("#", ".").half("+"),
            fg(51, 255, 51),
            fg(20, 100, 20),
        );
        s.modifier = Modifier::BOLD;
        s.label = fg(51, 255, 51);
        s.value = fg(20, 160, 20);
        s
    }

    /// Cool blue → cyan hexagons.
    pub fn ocean() -> Self {
        let mut s = Self::base(RatingSymbols::hexagons(), fg(0, 150, 255), fg(30, 50, 80));
        s.gradient = vec![Color::Rgb(30, 80, 255), Color::Rgb(0, 230, 230)];
        s
    }

    /// Warm orange → pink hearts.
    pub fn sunset() -> Self {
        let mut s = Self::base(RatingSymbols::hearts(), fg(255, 140, 60), fg(90, 50, 60));
        s.gradient = vec![Color::Rgb(255, 170, 40), Color::Rgb(255, 60, 130)];
        s
    }

    /// Every bundled preset with its name.
    pub fn all() -> Vec<(&'static str, RatingStyle)> {
        vec![
            ("classic", Self::classic()),
            ("netflix", Self::netflix()),
            ("github", Self::github()),
            ("imdb", Self::imdb()),
            ("neon", Self::neon()),
            ("retro", Self::retro()),
            ("ocean", Self::ocean()),
            ("sunset", Self::sunset()),
        ]
    }
}

impl Default for RatingStyle {
    fn default() -> Self {
        Self::classic()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presets_are_distinct_and_sane() {
        let all = RatingStyle::all();
        assert_eq!(all.len(), 8);
        for (name, s) in &all {
            assert!(!name.is_empty());
            assert!(!s.symbols.filled.is_empty());
        }
        for i in 0..all.len() {
            for j in i + 1..all.len() {
                assert_ne!(all[i].1, all[j].1, "{} == {}", all[i].0, all[j].0);
            }
        }
        assert_eq!(RatingStyle::default(), RatingStyle::classic());
    }
}
