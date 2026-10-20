//! [`SentimentFaces`]: 😡 🙁 😐 🙂 😍 emoji scale.

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Widget},
};

use crate::util::{align_x, cell_width, lerp_gradient, pad_to, render_lines, traffic_light_colors};

/// A row of faces from unhappy to delighted; the selected one is drawn on a
/// coloured background.
///
/// ```
/// use tui_rating::SentimentFaces;
///
/// let f = SentimentFaces::new(Some(3));
/// assert_eq!(f.selected_label(), Some("Good"));
/// assert_eq!(SentimentFaces::from_ratio(1.0).selected(), Some(4));
/// ```
#[derive(Debug, Clone)]
pub struct SentimentFaces<'a> {
    faces: Vec<String>,
    names: Vec<String>,
    selected: Option<usize>,
    colors: Vec<Color>,
    show_label: bool,
    gap: u16,
    alignment: Alignment,
    block: Option<Block<'a>>,
}

impl<'a> SentimentFaces<'a> {
    /// Five emoji faces; `selected` is 0-based (`0` = worst).
    pub fn new(selected: Option<usize>) -> Self {
        Self::with_faces(
            ["😡", "🙁", "😐", "🙂", "😍"],
            ["Awful", "Bad", "Okay", "Good", "Great"],
            selected,
        )
    }

    /// Five ASCII emoticons (`>:(` `:(` `:|` `:)` `:D`), for terminals
    /// without emoji support.
    pub fn ascii(selected: Option<usize>) -> Self {
        Self::with_faces(
            [">:(", ":(", ":|", ":)", ":D"],
            ["Awful", "Bad", "Okay", "Good", "Great"],
            selected,
        )
    }

    /// Maps a ratio `0.0..=1.0` to the nearest face.
    pub fn from_ratio(ratio: f64) -> Self {
        let r = if ratio.is_finite() {
            ratio.clamp(0.0, 1.0)
        } else {
            0.0
        };
        Self::new(Some((r * 4.0).round() as usize))
    }

    /// Custom faces and names (lowest first).
    pub fn with_faces<F, N, S, T>(faces: F, names: N, selected: Option<usize>) -> Self
    where
        F: IntoIterator<Item = S>,
        N: IntoIterator<Item = T>,
        S: Into<String>,
        T: Into<String>,
    {
        let faces: Vec<String> = faces.into_iter().map(Into::into).collect();
        let selected = selected.filter(|&i| i < faces.len());
        Self {
            faces,
            names: names.into_iter().map(Into::into).collect(),
            selected,
            colors: traffic_light_colors(),
            show_label: false,
            gap: 1,
            alignment: Alignment::Left,
            block: None,
        }
    }

    /// Appends the name of the selected face.
    #[must_use]
    pub fn show_label(mut self, show: bool) -> Self {
        self.show_label = show;
        self
    }

    /// Selection background gradient (low → high).
    #[must_use]
    pub fn gradient(mut self, colors: impl Into<Vec<Color>>) -> Self {
        self.colors = colors.into();
        self
    }

    /// Spaces between faces (default 1).
    #[must_use]
    pub fn gap(mut self, gap: u16) -> Self {
        self.gap = gap;
        self
    }

    /// Alignment inside the area.
    #[must_use]
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Wraps the widget in a block.
    #[must_use]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Selected index.
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    /// Name of the selected face.
    pub fn selected_label(&self) -> Option<&str> {
        self.selected
            .and_then(|i| self.names.get(i))
            .map(String::as_str)
    }

    fn cell_w(&self) -> usize {
        self.faces.iter().map(|f| cell_width(f)).max().unwrap_or(0)
    }

    fn color_at(&self, i: usize) -> Color {
        let t = if self.faces.len() <= 1 {
            0.0
        } else {
            i as f64 / (self.faces.len() - 1) as f64
        };
        lerp_gradient(&self.colors, t).unwrap_or(Color::Yellow)
    }

    /// Face index at a terminal position (hit testing).
    pub fn index_at(&self, area: Rect, x: u16, y: u16) -> Option<usize> {
        let inner = match &self.block {
            Some(b) => b.inner(area),
            None => area,
        };
        if inner.height == 0 || y != inner.y || self.faces.is_empty() {
            return None;
        }
        let w = self.cell_w() as u16;
        let pitch = w + self.gap;
        let total = (pitch * self.faces.len() as u16).saturating_sub(self.gap);
        let start = align_x(inner, total, self.alignment);
        let rel = x.checked_sub(start)?;
        let i = (rel / pitch.max(1)) as usize;
        (rel % pitch.max(1) < w && i < self.faces.len()).then_some(i)
    }

    /// The styled line.
    pub fn line(&self) -> Line<'a> {
        let w = self.cell_w();
        let mut spans = Vec::new();
        for (i, face) in self.faces.iter().enumerate() {
            if i > 0 && self.gap > 0 {
                spans.push(Span::raw(" ".repeat(self.gap as usize)));
            }
            let style = if self.selected == Some(i) {
                Style::default()
                    .bg(self.color_at(i))
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().add_modifier(Modifier::DIM)
            };
            spans.push(Span::styled(pad_to(face, w), style));
        }
        if self.show_label {
            if let Some(n) = self.selected_label() {
                spans.push(Span::styled(
                    format!("  {n}"),
                    Style::default()
                        .fg(self.color_at(self.selected.unwrap_or(0)))
                        .add_modifier(Modifier::BOLD),
                ));
            }
        }
        Line::from(spans).alignment(self.alignment)
    }
}

impl Widget for SentimentFaces<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        render_lines(self.block.clone(), vec![self.line()], area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_and_ratio() {
        assert_eq!(SentimentFaces::new(Some(9)).selected(), None);
        assert_eq!(SentimentFaces::from_ratio(0.0).selected(), Some(0));
        assert_eq!(SentimentFaces::from_ratio(0.5).selected(), Some(2));
        assert_eq!(SentimentFaces::from_ratio(f64::NAN).selected(), Some(0));
        assert_eq!(SentimentFaces::from_ratio(9.0).selected(), Some(4));
        assert_eq!(SentimentFaces::new(Some(0)).selected_label(), Some("Awful"));
        assert_eq!(SentimentFaces::new(None).selected_label(), None);
    }

    #[test]
    fn line_shape() {
        let f = SentimentFaces::ascii(Some(1)).show_label(true).gap(2);
        let text: String = f.line().spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(text, ">:(  :(   :|   :)   :D   Bad");
        let sel = &f.line().spans[2];
        assert_eq!(sel.style.fg, Some(Color::Black));
        assert!(sel.style.bg.is_some());
        let e = SentimentFaces::with_faces(Vec::<String>::new(), Vec::<String>::new(), None);
        assert_eq!(e.color_at(0), Color::Rgb(220, 60, 60));
        assert_eq!(e.index_at(Rect::new(0, 0, 10, 1), 0, 0), None);
    }

    #[test]
    fn hit_testing_and_render() {
        let f = SentimentFaces::new(Some(2)).block(Block::bordered());
        let area = Rect::new(0, 0, 30, 3);
        // inner starts x=1; faces are 2 wide, pitch 3
        assert_eq!(f.index_at(area, 1, 1), Some(0));
        assert_eq!(f.index_at(area, 2, 1), Some(0));
        assert_eq!(f.index_at(area, 3, 1), None);
        assert_eq!(f.index_at(area, 4, 1), Some(1));
        assert_eq!(f.index_at(area, 0, 1), None);
        assert_eq!(f.index_at(area, 4, 0), None);
        let mut buf = Buffer::empty(area);
        f.alignment(Alignment::Center).render(area, &mut buf);
    }
}
