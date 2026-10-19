//! The [`Rating`] widget.

use std::{fmt, sync::Arc};

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, StatefulWidget, Widget},
};

#[cfg(feature = "theme")]
use tui_nuances::{Theme, Themed};

use crate::{
    preset::RatingStyle,
    util::{align_x, cell_width, format_value, lerp_gradient, pad_to, traffic_light_colors},
    RatingState, RatingSymbols,
};

/// How a single rating item is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemFill {
    /// Fully filled.
    Full,
    /// Half filled.
    Half,
    /// Partly filled, `1..=99` percent (only with [`Rating::fractional`]).
    Partial(u8),
    /// Empty.
    Empty,
}

/// How the value text is formatted when [`Rating::show_value`] is on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ValueFormat {
    /// `3/5` or `3.5/5` (default).
    #[default]
    Fraction,
    /// `3` or `3.5`.
    Number,
    /// `60%`.
    Percent,
}

/// Where the label is placed relative to the items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LabelPosition {
    /// Before the items (default).
    #[default]
    Before,
    /// After the items (and value text).
    After,
}

/// Direction items are laid out in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Orientation {
    /// One row, items left to right (default).
    #[default]
    Horizontal,
    /// One item per row, filling from the bottom up like a thermometer.
    Vertical,
}

/// Custom per-item colour callback, see [`Rating::color_fn`].
#[derive(Clone)]
struct ColorFn(Arc<dyn Fn(usize, ItemFill) -> Option<Color> + Send + Sync>);

impl fmt::Debug for ColorFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ColorFn(..)")
    }
}

/// A star-style rating widget (stars, hearts, circles, …).
///
/// # Example
///
/// ```
/// use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};
/// use tui_rating::{Rating, RatingState, RatingSymbols};
///
/// let state = RatingState::new(3.0, 5);
/// let rating = Rating::from_state(&state)
///     .symbols(RatingSymbols::hearts())
///     .filled_color(Color::Red)
///     .label("Quality")
///     .show_value(true);
///
/// let area = Rect::new(0, 0, 40, 1);
/// let mut buf = Buffer::empty(area);
/// rating.render(area, &mut buf);
/// ```
#[derive(Debug, Clone)]
pub struct Rating<'a> {
    value: f64,
    max: usize,
    half_steps: bool,
    fractional: bool,
    symbols: RatingSymbols,
    filled_style: Option<Style>,
    half_style: Option<Style>,
    empty_style: Option<Style>,
    label: Option<String>,
    label_style: Option<Style>,
    value_style: Option<Style>,
    #[cfg(feature = "theme")]
    themed: Option<ThemeStyles>,
    show_value: bool,
    value_format: ValueFormat,
    label_position: LabelPosition,
    gradient: Vec<Color>,
    color_fn: Option<ColorFn>,
    modifier: Modifier,
    empty_modifier: Modifier,
    highlight: Option<usize>,
    highlight_style: Style,
    hover: Option<f64>,
    reversed: bool,
    orientation: Orientation,
    disabled: bool,
    focused: bool,
    focus_style: Style,
    gap: u16,
    alignment: Alignment,
    block: Option<Block<'a>>,
}

/// Styles derived from a theme.
#[cfg_attr(not(feature = "theme"), allow(dead_code))]
#[derive(Debug, Clone, Copy)]
struct ThemeStyles {
    filled: Style,
    empty: Style,
    label: Style,
    value: Style,
}

#[cfg(feature = "theme")]
impl Themed for Rating<'_> {
    /// Maps `accent` → filled items, `disabled` → empty items,
    /// `foreground` → label, `muted` → value text.
    fn apply_theme(mut self, theme: &Theme) -> Self {
        self.themed = Some(ThemeStyles {
            filled: Style::default().fg(theme.accent),
            empty: Style::default().fg(theme.disabled),
            label: Style::default().fg(theme.foreground),
            value: Style::default().fg(theme.muted),
        });
        self
    }
}

/// Saturating `usize` → `u16`.
fn to_u16(n: usize) -> u16 {
    u16::try_from(n).unwrap_or(u16::MAX)
}

/// One laid-out row of the widget.
struct Row<'a> {
    line: Line<'a>,
    /// `(logical item index, x offset within the line, width)`.
    items: Vec<(usize, u16, u16)>,
}

impl<'a> Rating<'a> {
    /// Creates a rating with `value` out of `max` items. Value is clamped.
    pub fn new(value: f64, max: usize) -> Self {
        let value = if value.is_finite() {
            value.clamp(0.0, max as f64)
        } else {
            0.0
        };
        Self {
            value,
            max,
            half_steps: false,
            fractional: false,
            symbols: RatingSymbols::default(),
            filled_style: None,
            half_style: None,
            empty_style: None,
            label: None,
            label_style: None,
            value_style: None,
            #[cfg(feature = "theme")]
            themed: None,
            show_value: false,
            value_format: ValueFormat::Fraction,
            label_position: LabelPosition::Before,
            gradient: Vec::new(),
            color_fn: None,
            modifier: Modifier::empty(),
            empty_modifier: Modifier::empty(),
            highlight: None,
            highlight_style: Style::default().add_modifier(Modifier::REVERSED | Modifier::BOLD),
            hover: None,
            reversed: false,
            orientation: Orientation::Horizontal,
            disabled: false,
            focused: false,
            focus_style: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            gap: 1,
            alignment: Alignment::Left,
            block: None,
        }
    }

    /// Creates a rating from a [`RatingState`].
    ///
    /// Copies value (the animated value while animating), max, half-step
    /// mode and the hover preview. While an animation runs, the item being
    /// filled is marked via [`highlight`](Self::highlight).
    pub fn from_state(state: &RatingState) -> Self {
        let mut r = Self::new(state.visual_value(), state.max());
        r.sync_state(state);
        r
    }

    fn sync_state(&mut self, state: &RatingState) {
        self.value = state.visual_value();
        self.max = state.max();
        self.half_steps = state.half_steps();
        self.hover = state.hover();
        if state.is_animating() {
            self.highlight = state.pulse_index();
        }
    }

    /// Enables half-item rendering (values snap to the nearest `0.5`).
    #[must_use]
    pub fn half_steps(mut self, enabled: bool) -> Self {
        self.half_steps = enabled;
        self
    }

    /// Smooth fractional fill: the value is not snapped and the last,
    /// partly filled item uses one of the symbol set's
    /// [`partials`](RatingSymbols::partials) (e.g. `3.7` → `███▉░`).
    ///
    /// Has no effect on symbol sets without partial glyphs. Try
    /// [`RatingSymbols::smooth`].
    #[must_use]
    pub fn fractional(mut self, enabled: bool) -> Self {
        self.fractional = enabled;
        self
    }

    /// Sets the symbol set.
    #[must_use]
    pub fn symbols(mut self, symbols: RatingSymbols) -> Self {
        self.symbols = symbols;
        self
    }

    /// Applies a bundled [`RatingStyle`] (symbols, colours, modifiers, gap).
    ///
    /// Builder calls made afterwards override the preset.
    #[must_use]
    pub fn preset(mut self, preset: RatingStyle) -> Self {
        self.symbols = preset.symbols;
        // A gradient replaces the preset's fixed filled colour.
        let mut filled = preset.filled;
        if !preset.gradient.is_empty() {
            filled.fg = None;
        }
        self.filled_style = Some(filled);
        self.half_style = preset.half;
        self.empty_style = Some(preset.empty);
        self.label_style = Some(preset.label);
        self.value_style = Some(preset.value);
        self.gradient = preset.gradient;
        self.modifier = preset.modifier;
        self.empty_modifier = preset.empty_modifier;
        self.gap = preset.gap;
        self
    }

    /// Sets the style of filled items.
    #[must_use]
    pub fn filled_style(mut self, style: Style) -> Self {
        self.filled_style = Some(style);
        self
    }

    /// Shortcut: sets the foreground colour of filled items.
    #[must_use]
    pub fn filled_color(mut self, color: Color) -> Self {
        self.filled_style = Some(self.filled_style.unwrap_or_default().fg(color));
        self
    }

    /// Sets the style of half items (defaults to the filled style).
    #[must_use]
    pub fn half_style(mut self, style: Style) -> Self {
        self.half_style = Some(style);
        self
    }

    /// Sets the style of empty items.
    #[must_use]
    pub fn empty_style(mut self, style: Style) -> Self {
        self.empty_style = Some(style);
        self
    }

    /// Shortcut: sets the foreground colour of empty items.
    #[must_use]
    pub fn empty_color(mut self, color: Color) -> Self {
        self.empty_style = Some(self.empty_style.unwrap_or_default().fg(color));
        self
    }

    /// Sets a text label rendered before the items.
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the label style.
    #[must_use]
    pub fn label_style(mut self, style: Style) -> Self {
        self.label_style = Some(style);
        self
    }

    /// Shows a `value/max` text after the items.
    #[must_use]
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Sets the style of the value text.
    #[must_use]
    pub fn value_style(mut self, style: Style) -> Self {
        self.value_style = Some(style);
        self
    }

    /// Sets the value text format (default [`ValueFormat::Fraction`]).
    #[must_use]
    pub fn value_format(mut self, format: ValueFormat) -> Self {
        self.value_format = format;
        self
    }

    /// Places the label before (default) or after the items.
    #[must_use]
    pub fn label_position(mut self, position: LabelPosition) -> Self {
        self.label_position = position;
        self
    }

    /// Colors filled items by position, interpolating between the given
    /// colors (RGB colors blend smoothly, others step). Overrides the filled
    /// foreground. An empty list disables the gradient.
    #[must_use]
    pub fn gradient(mut self, colors: impl Into<Vec<Color>>) -> Self {
        self.gradient = colors.into();
        self
    }

    /// Classic traffic-light gradient: red → yellow → green.
    #[must_use]
    pub fn traffic_light(self) -> Self {
        self.gradient(traffic_light_colors())
    }

    /// Colors each item with a callback `(index, fill) -> Option<Color>`.
    ///
    /// Called for every item (empty ones too). `None` keeps the regular
    /// style. Takes precedence over [`gradient`](Self::gradient) and the
    /// explicit styles' foreground.
    #[must_use]
    pub fn color_fn<F>(mut self, f: F) -> Self
    where
        F: Fn(usize, ItemFill) -> Option<Color> + Send + Sync + 'static,
    {
        self.color_fn = Some(ColorFn(Arc::new(f)));
        self
    }

    /// Adds text modifiers (e.g. `Modifier::BOLD`) to filled and half items.
    #[must_use]
    pub fn modifier(mut self, modifier: Modifier) -> Self {
        self.modifier = modifier;
        self
    }

    /// Adds text modifiers (e.g. `Modifier::DIM`) to empty items.
    #[must_use]
    pub fn empty_modifier(mut self, modifier: Modifier) -> Self {
        self.empty_modifier = modifier;
        self
    }

    /// Highlights the item at `index` (0-based), e.g. a hover/cursor preview.
    #[must_use]
    pub fn highlight(mut self, index: Option<usize>) -> Self {
        self.highlight = index;
        self
    }

    /// Style patched onto the highlighted item (default reversed + bold).
    #[must_use]
    pub fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = style;
        self
    }

    /// Previews `value` (e.g. under the mouse) instead of the real value and
    /// highlights the item it reaches. `None` disables the preview.
    #[must_use]
    pub fn hover(mut self, value: Option<f64>) -> Self {
        self.hover = value.filter(|v| v.is_finite());
        self
    }

    /// Fills right-to-left (horizontal) or top-down (vertical).
    #[must_use]
    pub fn reversed(mut self, reversed: bool) -> Self {
        self.reversed = reversed;
        self
    }

    /// Horizontal (default) or vertical layout.
    #[must_use]
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Dims the whole widget and drops gradient, callback colours and
    /// highlight: a read-only / inactive look.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Draws a focus indicator: `[ ]` brackets around a horizontal rating,
    /// a `▌` marker beside each item of a vertical one.
    #[must_use]
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Style of the focus indicator (default bold cyan).
    #[must_use]
    pub fn focus_style(mut self, style: Style) -> Self {
        self.focus_style = style;
        self
    }

    /// Sets the number of spaces between items (default `1`).
    #[must_use]
    pub fn gap(mut self, gap: u16) -> Self {
        self.gap = gap;
        self
    }

    /// Sets horizontal alignment of the whole line.
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

    /// Styles the widget from a shared [`Theme`] (see [`Themed`]).
    ///
    /// Accepts `&Theme` or `Theme`. Explicit styles/colors set through
    /// `filled_style`, `filled_color`, `half_style`, `empty_style`,
    /// `empty_color`, `label_style` and `value_style` always win over the
    /// theme, regardless of call order. Without a theme the original default
    /// look is used.
    #[must_use]
    #[cfg(feature = "theme")]
    pub fn theme(self, theme: impl Into<Theme>) -> Self {
        self.apply_theme(&theme.into())
    }

    fn resolved(&self) -> (Style, Style, Style, Style) {
        #[cfg(feature = "theme")]
        let t = self.themed.as_ref();
        #[cfg(not(feature = "theme"))]
        let t: Option<&ThemeStyles> = None;
        let filled = self
            .filled_style
            .or(t.map(|t| t.filled))
            .unwrap_or_else(|| Style::default().fg(Color::Yellow));
        let empty = self
            .empty_style
            .or(t.map(|t| t.empty))
            .unwrap_or_else(|| Style::default().fg(Color::DarkGray));
        let label = self.label_style.or(t.map(|t| t.label)).unwrap_or_default();
        let value = self.value_style.or(t.map(|t| t.value)).unwrap_or_default();
        (filled, empty, label, value)
    }

    /// The value being drawn: the hover preview if set, else the value.
    fn effective_value(&self) -> f64 {
        match self.hover {
            Some(h) => h.clamp(0.0, self.max as f64),
            None => self.value,
        }
    }

    fn is_smooth(&self) -> bool {
        self.fractional && !self.symbols.partials.is_empty()
    }

    /// The value after snapping to the display granularity (`1.0`, `0.5`,
    /// or unsnapped when [`fractional`](Self::fractional) applies).
    pub fn display_value(&self) -> f64 {
        let v = self.effective_value();
        if self.is_smooth() {
            v
        } else if self.half_steps {
            (v * 2.0).round() / 2.0
        } else {
            v.round()
        }
    }

    /// The value text (`"3.5/5"`, `"3.5"` or `"70%"`, per [`ValueFormat`]).
    pub fn value_text(&self) -> String {
        format_value(self.display_value(), self.max, self.value_format)
    }

    /// Fill of the `index`-th item (0-based).
    pub fn fill_at(&self, index: usize) -> ItemFill {
        let remaining = self.display_value() - index as f64;
        if remaining >= 1.0 {
            ItemFill::Full
        } else if self.is_smooth() {
            if remaining <= 0.0 {
                ItemFill::Empty
            } else {
                ItemFill::Partial((remaining * 100.0).round().clamp(1.0, 99.0) as u8)
            }
        } else if self.half_steps && remaining >= 0.5 {
            ItemFill::Half
        } else {
            ItemFill::Empty
        }
    }

    /// The item highlighted by hover or [`highlight`](Self::highlight).
    fn highlighted(&self) -> Option<usize> {
        if self.disabled {
            return None;
        }
        self.highlight.or_else(|| {
            let h = self.hover?;
            (h > 0.0).then(|| h.ceil() as usize - 1)
        })
    }

    fn gradient_color(&self, index: usize) -> Option<Color> {
        let t = if self.max <= 1 {
            0.0
        } else {
            index as f64 / (self.max - 1) as f64
        };
        lerp_gradient(&self.gradient, t)
    }

    /// Glyph and style of the `index`-th item.
    fn item(&self, index: usize, styles: (Style, Style)) -> (String, Style) {
        let (filled_style, empty_style) = styles;
        let fill = self.fill_at(index);
        let s = &self.symbols;
        let half_style = self.half_style.unwrap_or(filled_style);
        let (glyph, mut style, drawn_filled) = match fill {
            ItemFill::Full => (&s.filled, filled_style, true),
            ItemFill::Half => match &s.half {
                Some(h) => (h, half_style, true),
                None => (&s.empty, half_style, true),
            },
            ItemFill::Partial(p) => {
                let n = s.partials.len();
                let k = (f64::from(p) / 100.0 * (n + 1) as f64).round() as usize;
                if k == 0 {
                    (&s.empty, empty_style, false)
                } else if k > n {
                    (&s.filled, filled_style, true)
                } else {
                    (&s.partials[k - 1], half_style, true)
                }
            }
            ItemFill::Empty => (&s.empty, empty_style, false),
        };
        if !self.disabled {
            if let Some(c) = self
                .color_fn
                .as_ref()
                .and_then(|f| (f.0)(index, fill))
                .or_else(|| {
                    if drawn_filled && self.filled_style.and_then(|s| s.fg).is_none() {
                        self.gradient_color(index)
                    } else {
                        None
                    }
                })
            {
                style = style.fg(c);
            }
        }
        style = style.add_modifier(if drawn_filled {
            self.modifier
        } else {
            self.empty_modifier
        });
        if self.disabled {
            style = style.add_modifier(Modifier::DIM);
        }
        if self.highlighted() == Some(index) {
            style = style.patch(self.highlight_style);
        }
        (pad_to(glyph, s.cell_width()), style)
    }

    /// Logical item index shown in slot `pos`.
    fn logical(&self, pos: usize) -> usize {
        let flip = match self.orientation {
            Orientation::Horizontal => self.reversed,
            Orientation::Vertical => !self.reversed,
        };
        if flip {
            self.max - 1 - pos
        } else {
            pos
        }
    }

    fn rows(&self) -> Vec<Row<'a>> {
        let (filled_style, empty_style, mut label_style, mut value_style) = self.resolved();
        if self.disabled {
            label_style = label_style.add_modifier(Modifier::DIM);
            value_style = value_style.add_modifier(Modifier::DIM);
        }
        let iw = to_u16(self.symbols.cell_width());
        let focus = self.focused;
        let label_before = match (&self.label, self.label_position) {
            (Some(l), LabelPosition::Before) => Some(format!("{l} ")),
            _ => None,
        };
        let label_after = match (&self.label, self.label_position) {
            (Some(l), LabelPosition::After) => Some(format!(" {l}")),
            _ => None,
        };
        match self.orientation {
            Orientation::Horizontal => {
                let mut spans: Vec<Span<'a>> = Vec::new();
                let mut items = Vec::new();
                let mut x: u16 = 0;
                if focus {
                    spans.push(Span::styled("[", self.focus_style));
                    x = x.saturating_add(1);
                }
                if let Some(l) = label_before {
                    x = x.saturating_add(to_u16(cell_width(&l)));
                    spans.push(Span::styled(l, label_style));
                }
                for pos in 0..self.max {
                    if pos > 0 && self.gap > 0 {
                        spans.push(Span::raw(" ".repeat(self.gap as usize)));
                        x = x.saturating_add(self.gap);
                    }
                    let i = self.logical(pos);
                    let (glyph, style) = self.item(i, (filled_style, empty_style));
                    items.push((i, x, iw));
                    x = x.saturating_add(iw);
                    spans.push(Span::styled(glyph, style));
                }
                if self.show_value {
                    spans.push(Span::styled(format!(" {}", self.value_text()), value_style));
                }
                if let Some(l) = label_after {
                    spans.push(Span::styled(l, label_style));
                }
                if focus {
                    spans.push(Span::styled("]", self.focus_style));
                }
                vec![Row {
                    line: Line::from(spans).alignment(self.alignment),
                    items,
                }]
            }
            Orientation::Vertical => {
                let mut rows = Vec::new();
                let plain = |s: String, st: Style, a: Alignment| Row {
                    line: Line::from(Span::styled(s, st)).alignment(a),
                    items: Vec::new(),
                };
                if let Some(l) = label_before {
                    rows.push(plain(l.trim_end().to_string(), label_style, self.alignment));
                }
                for pos in 0..self.max {
                    if pos > 0 {
                        for _ in 0..self.gap {
                            rows.push(plain(String::new(), Style::default(), self.alignment));
                        }
                    }
                    let i = self.logical(pos);
                    let (glyph, style) = self.item(i, (filled_style, empty_style));
                    let mut spans = Vec::new();
                    let mut off = 0;
                    if focus {
                        spans.push(Span::styled("▌", self.focus_style));
                        off = 1;
                    }
                    spans.push(Span::styled(glyph, style));
                    rows.push(Row {
                        line: Line::from(spans).alignment(self.alignment),
                        items: vec![(i, off, iw)],
                    });
                }
                if self.show_value {
                    rows.push(plain(self.value_text(), value_style, self.alignment));
                }
                if let Some(l) = label_after {
                    rows.push(plain(
                        l.trim_start().to_string(),
                        label_style,
                        self.alignment,
                    ));
                }
                rows
            }
        }
    }

    /// Builds the rendered horizontal line (label, items, value text)
    /// without a block. For vertical ratings this is the first row only; use
    /// [`lines`](Self::lines).
    pub fn line(&self) -> Line<'a> {
        self.rows().swap_remove(0).line
    }

    /// All rendered lines: one for horizontal, one per row for vertical.
    pub fn lines(&self) -> Vec<Line<'a>> {
        self.rows().into_iter().map(|r| r.line).collect()
    }

    /// Width in terminal cells of the widest rendered line.
    pub fn width(&self) -> usize {
        self.rows()
            .iter()
            .map(|r| r.line.width())
            .max()
            .unwrap_or(0)
    }

    /// Height in rows (excluding any block).
    pub fn height(&self) -> usize {
        self.rows().len()
    }

    /// Wraps the widget for use with
    /// [`Frame::render_stateful_widget`](ratatui::Frame::render_stateful_widget)
    /// and a [`RatingState`].
    pub fn stateful(self) -> StatefulRating<'a> {
        StatefulRating(self)
    }

    fn inner(&self, area: Rect) -> Rect {
        self.block.as_ref().map_or(area, |b| b.inner(area))
    }

    /// The 0-based item under terminal cell `(x, y)` when rendered in `area`.
    ///
    /// Accounts for block, alignment, label, gap, glyph width, orientation
    /// and [`reversed`](Self::reversed). Gaps and text return `None`.
    pub fn item_at(&self, area: Rect, x: u16, y: u16) -> Option<usize> {
        self.hit(area, x, y).map(|(i, _, _)| i)
    }

    /// `(item, cell offset within item, item width)` under `(x, y)`.
    fn hit(&self, area: Rect, x: u16, y: u16) -> Option<(usize, u16, u16)> {
        let inner = self.inner(area);
        if self.max == 0 || inner.width == 0 || inner.height == 0 {
            return None;
        }
        let rel = y.checked_sub(inner.y)?;
        if rel >= inner.height {
            return None;
        }
        let rows = self.rows();
        let row = rows.get(rel as usize)?;
        let start = align_x(inner, to_u16(row.line.width()), self.alignment);
        row.items.iter().find_map(|&(i, off, w)| {
            let a = start.checked_add(off)?;
            let b = a.checked_add(w)?;
            (x >= a && x < b && x < inner.x + inner.width).then(|| (i, x - a, w))
        })
    }

    /// The rating value a click at `(x, y)` would select, or `None` outside
    /// the items.
    ///
    /// Selects `item + 1`. With [`half_steps`](Self::half_steps) and glyphs at
    /// least two cells wide, the left half selects `item + 0.5`.
    pub fn value_at(&self, area: Rect, x: u16, y: u16) -> Option<f64> {
        let (i, off, w) = self.hit(area, x, y)?;
        let left_half = self.half_steps && w >= 2 && u32::from(off) * 2 < u32::from(w);
        Some(i as f64 + if left_half { 0.5 } else { 1.0 })
    }
}

impl Widget for Rating<'_> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let inner = match self.block.take() {
            Some(block) => {
                let inner = block.inner(area);
                block.render(area, buf);
                inner
            }
            None => area,
        };
        if inner.width == 0 || inner.height == 0 {
            return;
        }
        for (i, row) in self
            .rows()
            .into_iter()
            .enumerate()
            .take(inner.height as usize)
        {
            let r = Rect::new(inner.x, inner.y + i as u16, inner.width, 1);
            row.line.render(r, buf);
        }
    }
}

impl Widget for &Rating<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(self.clone(), area, buf);
    }
}

/// A [`Rating`] that renders from a [`RatingState`] as a
/// [`StatefulWidget`]; create it with [`Rating::stateful`].
///
/// Value, max, half-step mode, hover preview and animation come from the
/// state; all styling comes from the wrapped widget. This is a separate type
/// (not an impl on `Rating`) so that `rating.render(..)` stays unambiguous
/// when both `Widget` and `StatefulWidget` are in scope.
///
/// ```
/// use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};
/// use tui_rating::{Rating, RatingState};
///
/// let mut state = RatingState::new(3.0, 5);
/// let area = Rect::new(0, 0, 9, 1);
/// let mut buf = Buffer::empty(area);
/// Rating::new(0.0, 1).show_value(false).stateful().render(area, &mut buf, &mut state);
/// assert_eq!(buf[(4, 0)].symbol(), "★");
/// ```
#[derive(Debug, Clone)]
pub struct StatefulRating<'a>(Rating<'a>);

impl StatefulWidget for StatefulRating<'_> {
    type State = RatingState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut RatingState) {
        self.0.sync_state(state);
        self.0.render(area, buf);
    }
}

impl StatefulWidget for &StatefulRating<'_> {
    type State = RatingState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut RatingState) {
        StatefulWidget::render(self.clone(), area, buf, state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(r: Rating<'_>, w: u16) -> String {
        let area = Rect::new(0, 0, w, 1);
        let mut buf = Buffer::empty(area);
        r.render(area, &mut buf);
        (0..w)
            .map(|x| buf[(x, 0)].symbol().to_string())
            .collect::<String>()
    }

    #[test]
    fn renders_filled_and_empty() {
        assert_eq!(text(Rating::new(3.0, 5), 9), "★ ★ ★ ☆ ☆");
    }

    #[test]
    fn zero_gap() {
        assert_eq!(text(Rating::new(2.0, 4).gap(0), 4), "★★☆☆");
    }

    #[test]
    fn half_with_glyph() {
        let r = Rating::new(2.5, 5)
            .half_steps(true)
            .symbols(RatingSymbols::circles())
            .gap(0);
        assert_eq!(text(r, 5), "●●◐○○");
    }

    #[test]
    fn half_without_glyph_uses_empty_with_filled_style() {
        let r = Rating::new(1.5, 3).half_steps(true).gap(0);
        let area = Rect::new(0, 0, 3, 1);
        let mut buf = Buffer::empty(area);
        r.render(area, &mut buf);
        assert_eq!(buf[(1, 0)].symbol(), "☆");
        assert_eq!(buf[(1, 0)].fg, Color::Yellow);
        assert_eq!(buf[(2, 0)].fg, Color::DarkGray);
    }

    #[test]
    fn no_half_rounds_to_whole() {
        assert_eq!(Rating::new(2.5, 5).display_value(), 3.0);
        assert_eq!(Rating::new(2.4, 5).display_value(), 2.0);
        assert_eq!(text(Rating::new(2.4, 3).gap(0), 3), "★★☆");
    }

    #[test]
    fn half_snaps_to_half() {
        assert_eq!(Rating::new(2.3, 5).half_steps(true).display_value(), 2.5);
        assert_eq!(Rating::new(2.2, 5).half_steps(true).display_value(), 2.0);
    }

    #[test]
    fn clamps_value() {
        assert_eq!(Rating::new(10.0, 3).display_value(), 3.0);
        assert_eq!(Rating::new(-3.0, 3).display_value(), 0.0);
        assert_eq!(Rating::new(f64::NAN, 3).display_value(), 0.0);
    }

    #[test]
    fn value_text_formats() {
        assert_eq!(Rating::new(3.0, 5).value_text(), "3/5");
        assert_eq!(Rating::new(3.5, 5).half_steps(true).value_text(), "3.5/5");
    }

    #[test]
    fn label_and_value() {
        let r = Rating::new(1.0, 2).label("Hi").show_value(true);
        assert_eq!(text(r, 16), "Hi ★ ☆ 1/2      ");
    }

    #[test]
    fn alignment_right_and_center() {
        let r = Rating::new(1.0, 2).gap(0).alignment(Alignment::Right);
        assert_eq!(text(r, 6), "    ★☆");
        let r = Rating::new(1.0, 2).gap(0).alignment(Alignment::Center);
        assert_eq!(text(r, 6), "  ★☆  ");
    }

    #[test]
    fn block_insets_content() {
        let r = Rating::new(1.0, 2).gap(0).block(Block::bordered());
        let area = Rect::new(0, 0, 6, 3);
        let mut buf = Buffer::empty(area);
        r.render(area, &mut buf);
        assert_eq!(buf[(1, 1)].symbol(), "★");
        assert_eq!(buf[(2, 1)].symbol(), "☆");
    }

    #[test]
    fn zero_area_does_not_panic() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 5, 1));
        Rating::new(1.0, 3).render(Rect::new(0, 0, 0, 0), &mut buf);
    }

    #[test]
    fn truncates_in_narrow_area() {
        assert_eq!(text(Rating::new(5.0, 5), 3), "★ ★");
    }

    #[test]
    fn max_zero_renders_nothing() {
        assert_eq!(text(Rating::new(0.0, 0), 3), "   ");
    }

    #[test]
    fn width_counts_cells() {
        assert_eq!(Rating::new(0.0, 5).width(), 9);
        assert_eq!(Rating::new(0.0, 5).gap(2).width(), 13);
    }

    fn cell_fg(r: Rating<'_>, x: u16) -> Color {
        let area = Rect::new(0, 0, 30, 1);
        let mut buf = Buffer::empty(area);
        r.render(area, &mut buf);
        buf[(x, 0)].fg
    }

    #[test]
    fn value_formats() {
        let r = Rating::new(3.0, 5);
        assert_eq!(
            r.clone().value_format(ValueFormat::Number).value_text(),
            "3"
        );
        assert_eq!(
            r.clone().value_format(ValueFormat::Percent).value_text(),
            "60%"
        );
        let h = Rating::new(3.5, 5).half_steps(true);
        assert_eq!(
            h.clone().value_format(ValueFormat::Number).value_text(),
            "3.5"
        );
        assert_eq!(h.value_format(ValueFormat::Percent).value_text(), "70%");
        assert_eq!(
            Rating::new(0.0, 0)
                .value_format(ValueFormat::Percent)
                .value_text(),
            "0%"
        );
    }

    #[test]
    fn label_after() {
        let r = Rating::new(1.0, 2)
            .gap(0)
            .label("Hi")
            .label_position(LabelPosition::After);
        assert_eq!(text(r, 6), "★☆ Hi ");
    }

    #[test]
    fn label_after_with_value() {
        let r = Rating::new(1.0, 2)
            .gap(0)
            .show_value(true)
            .label("Hi")
            .label_position(LabelPosition::After);
        assert_eq!(text(r, 10), "★☆ 1/2 Hi ");
    }

    #[test]
    fn reversed_fills_from_right() {
        let r = Rating::new(2.0, 4).gap(0).reversed(true);
        assert_eq!(text(r, 4), "☆☆★★");
    }

    #[test]
    fn gradient_interpolates_rgb() {
        let r = Rating::new(3.0, 3)
            .gap(0)
            .gradient([Color::Rgb(0, 0, 0), Color::Rgb(100, 200, 50)]);
        assert_eq!(cell_fg(r.clone(), 0), Color::Rgb(0, 0, 0));
        assert_eq!(cell_fg(r.clone(), 1), Color::Rgb(50, 100, 25));
        assert_eq!(cell_fg(r, 2), Color::Rgb(100, 200, 50));
    }

    #[test]
    fn gradient_steps_named_and_single() {
        let r = Rating::new(2.0, 2)
            .gap(0)
            .gradient([Color::Red, Color::Green]);
        assert_eq!(cell_fg(r, 0), Color::Red);
        let r = Rating::new(2.0, 2).gap(0).gradient([Color::Blue]);
        assert_eq!(cell_fg(r, 1), Color::Blue);
        let r = Rating::new(1.0, 1).gap(0).traffic_light();
        assert_ne!(cell_fg(r, 0), Color::Reset);
    }

    #[test]
    fn gradient_skips_empty_and_explicit_fill_wins() {
        let r = Rating::new(1.0, 2).gap(0).gradient([Color::Red]);
        assert_eq!(cell_fg(r, 1), Color::DarkGray);
        let r = Rating::new(1.0, 2)
            .gap(0)
            .filled_color(Color::Cyan)
            .gradient([Color::Red]);
        assert_eq!(cell_fg(r, 0), Color::Cyan);
    }

    #[test]
    fn modifiers_and_highlight() {
        let area = Rect::new(0, 0, 4, 1);
        let mut buf = Buffer::empty(area);
        Rating::new(1.0, 2)
            .gap(0)
            .modifier(Modifier::BOLD)
            .empty_modifier(Modifier::DIM)
            .highlight(Some(1))
            .highlight_style(Style::default().add_modifier(Modifier::UNDERLINED))
            .render(area, &mut buf);
        assert!(buf[(0, 0)].modifier.contains(Modifier::BOLD));
        assert!(buf[(1, 0)].modifier.contains(Modifier::DIM));
        assert!(buf[(1, 0)].modifier.contains(Modifier::UNDERLINED));
        assert!(!buf[(0, 0)].modifier.contains(Modifier::UNDERLINED));
    }

    #[test]
    fn from_state_copies_mode() {
        let s = RatingState::new(2.5, 5).with_half_steps(true);
        let r = Rating::from_state(&s);
        assert_eq!(r.fill_at(2), ItemFill::Half);
        assert_eq!(r.fill_at(1), ItemFill::Full);
        assert_eq!(r.fill_at(3), ItemFill::Empty);
    }

    fn rect_text(r: Rating<'_>, w: u16, h: u16) -> Vec<String> {
        let area = Rect::new(0, 0, w, h);
        let mut buf = Buffer::empty(area);
        r.render(area, &mut buf);
        (0..h)
            .map(|y| (0..w).map(|x| buf[(x, y)].symbol().to_string()).collect())
            .collect()
    }

    #[test]
    fn huge_inputs_do_not_overflow() {
        // regression: fuzz found u16 overflow in the layout offsets
        let long = "x".repeat(70_000);
        let r = Rating::new(1.0, 40).label(long).gap(u16::MAX).focused(true);
        let area = Rect::new(0, 0, 10, 2);
        let mut buf = Buffer::empty(area);
        r.clone().render(area, &mut buf);
        assert_eq!(r.item_at(area, 3, 0), None);
        let v = Rating::new(1.0, 3)
            .orientation(Orientation::Vertical)
            .gap(u16::MAX);
        let mut buf = Buffer::empty(area);
        v.render(area, &mut buf);
    }

    #[test]
    fn fractional_smooth_fill() {
        let r = Rating::new(3.75, 5)
            .symbols(RatingSymbols::smooth())
            .fractional(true)
            .gap(0);
        assert_eq!(text(r.clone(), 5), "███▊░");
        assert_eq!(r.fill_at(3), ItemFill::Partial(75));
        assert_eq!(r.fill_at(4), ItemFill::Empty);
        assert_eq!(r.value_text(), "3.8/5");
        // tiny remainder rounds to the empty glyph, near-full to the full one
        let tiny = Rating::new(0.01, 2)
            .symbols(RatingSymbols::smooth())
            .fractional(true)
            .gap(0);
        assert_eq!(text(tiny, 2), "░░");
        let near = Rating::new(0.99, 1)
            .symbols(RatingSymbols::smooth())
            .fractional(true);
        assert_eq!(text(near, 1), "█");
        // without partials, fractional is a no-op
        let plain = Rating::new(2.4, 3).fractional(true).gap(0);
        assert_eq!(text(plain, 3), "★★☆");
    }

    #[test]
    fn wide_emoji_items_are_padded() {
        let r = Rating::new(1.0, 3).symbols(RatingSymbols::fire()).gap(1);
        assert_eq!(r.width(), 2 * 3 + 2);
        let r = Rating::new(1.0, 2)
            .symbols(RatingSymbols::new("🔥", "x"))
            .gap(0);
        assert_eq!(r.width(), 4, "narrow glyph padded to the wide one");
    }

    #[test]
    fn vertical_layout_fills_bottom_up() {
        let r = Rating::new(2.0, 3)
            .orientation(Orientation::Vertical)
            .label("L")
            .show_value(true)
            .gap(0);
        assert_eq!(r.height(), 5);
        assert_eq!(
            rect_text(r.clone(), 3, 5),
            ["L  ", "☆  ", "★  ", "★  ", "2/3"]
        );
        let top_down = r.clone().reversed(true);
        assert_eq!(rect_text(top_down, 3, 5)[1], "★  ");
        // gap adds blank rows; label after
        let g = Rating::new(1.0, 2)
            .orientation(Orientation::Vertical)
            .label("L")
            .label_position(LabelPosition::After)
            .gap(1);
        assert_eq!(rect_text(g.clone(), 2, 4), ["☆ ", "  ", "★ ", "L "]);
        assert_eq!(g.lines().len(), 4);
        assert_eq!(g.width(), 1);
        // height clips
        assert_eq!(rect_text(g, 2, 2), ["☆ ", "  "]);
    }

    #[test]
    fn focus_and_disabled() {
        let r = Rating::new(1.0, 2).focused(true).gap(0);
        assert_eq!(text(r.clone(), 4), "[★☆]");
        let v = Rating::new(1.0, 2)
            .orientation(Orientation::Vertical)
            .focused(true)
            .gap(0);
        assert_eq!(rect_text(v, 2, 2), ["▌☆", "▌★"]);
        let area = Rect::new(0, 0, 4, 1);
        let mut buf = Buffer::empty(area);
        Rating::new(1.0, 2)
            .gap(0)
            .disabled(true)
            .show_value(true)
            .traffic_light()
            .highlight(Some(0))
            .render(area, &mut buf);
        assert!(buf[(0, 0)].modifier.contains(Modifier::DIM));
        assert!(!buf[(0, 0)].modifier.contains(Modifier::REVERSED));
        assert_eq!(buf[(0, 0)].fg, Color::Yellow, "gradient dropped");
        let mut buf = Buffer::empty(Rect::new(0, 0, 8, 1));
        Rating::new(1.0, 2)
            .focus_style(Style::default().fg(Color::Red))
            .focused(true)
            .render(Rect::new(0, 0, 8, 1), &mut buf);
        assert_eq!(buf[(0, 0)].fg, Color::Red);
    }

    #[test]
    fn color_fn_wins() {
        let r = Rating::new(1.0, 2)
            .gap(0)
            .traffic_light()
            .color_fn(|i, fill| match (i, fill) {
                (0, ItemFill::Full) => Some(Color::Magenta),
                (1, ItemFill::Empty) => Some(Color::Cyan),
                _ => None,
            });
        assert_eq!(cell_fg(r.clone(), 0), Color::Magenta);
        assert_eq!(cell_fg(r.clone(), 1), Color::Cyan);
        assert!(format!("{r:?}").contains("ColorFn"));
    }

    #[test]
    fn hover_previews_and_highlights() {
        let r = Rating::new(1.0, 4).gap(0).hover(Some(3.0));
        assert_eq!(text(r.clone(), 4), "★★★☆");
        assert_eq!(r.value_text(), "3/4");
        let area = Rect::new(0, 0, 4, 1);
        let mut buf = Buffer::empty(area);
        r.clone().render(area, &mut buf);
        assert!(buf[(2, 0)].modifier.contains(Modifier::REVERSED));
        assert!(!buf[(1, 0)].modifier.contains(Modifier::REVERSED));
        assert_eq!(
            Rating::new(1.0, 4).hover(Some(f64::NAN)).display_value(),
            1.0
        );
        assert_eq!(Rating::new(1.0, 4).hover(Some(99.0)).display_value(), 4.0);
        let none = Rating::new(1.0, 4).hover(Some(0.0));
        assert_eq!(none.display_value(), 0.0);
    }

    #[test]
    fn presets_apply_and_can_be_overridden() {
        let r = Rating::new(1.0, 2).preset(RatingStyle::retro()).gap(0);
        assert_eq!(text(r.clone(), 2), "#.");
        assert_eq!(cell_fg(r.clone(), 0), Color::Rgb(51, 255, 51));
        let o = r.filled_color(Color::Red);
        assert_eq!(cell_fg(o, 0), Color::Red);
        let n = Rating::new(2.0, 2).preset(RatingStyle::neon()).gap(0);
        assert_eq!(cell_fg(n.clone(), 0), Color::Rgb(255, 0, 200));
        assert_eq!(cell_fg(n, 1), Color::Rgb(0, 255, 255));
    }

    #[test]
    fn hit_testing_horizontal() {
        let r = Rating::new(0.0, 3);
        let area = Rect::new(2, 1, 20, 1);
        // items at x = 2, 4, 6 on row 1
        assert_eq!(r.item_at(area, 2, 1), Some(0));
        assert_eq!(r.item_at(area, 6, 1), Some(2));
        assert_eq!(r.item_at(area, 3, 1), None, "gap");
        assert_eq!(r.item_at(area, 2, 0), None, "above");
        assert_eq!(r.item_at(area, 2, 2), None, "below");
        assert_eq!(r.item_at(area, 0, 1), None, "left of area");
        assert_eq!(r.item_at(area, 40, 1), None);
        assert_eq!(r.value_at(area, 4, 1), Some(2.0));
        // label shifts items
        let l = Rating::new(0.0, 3).label("ab");
        assert_eq!(l.item_at(area, 2 + 3, 1), Some(0));
        // right / center alignment, block, reversed
        let right = Rating::new(0.0, 3).alignment(Alignment::Right);
        assert_eq!(right.item_at(area, 21, 1), Some(2));
        let center = Rating::new(0.0, 3).alignment(Alignment::Center);
        assert_eq!(center.item_at(area, 2 + 7, 1), Some(0));
        let blk = Rating::new(0.0, 3).block(Block::bordered());
        let barea = Rect::new(0, 0, 20, 3);
        assert_eq!(blk.item_at(barea, 1, 1), Some(0));
        assert_eq!(blk.item_at(barea, 1, 0), None);
        let rev = Rating::new(0.0, 3).reversed(true);
        assert_eq!(rev.item_at(area, 2, 1), Some(2));
        assert_eq!(rev.item_at(area, 6, 1), Some(0));
        // focus brackets shift by one
        let f = Rating::new(0.0, 3).focused(true);
        assert_eq!(f.item_at(area, 3, 1), Some(0));
        // degenerate
        assert_eq!(Rating::new(0.0, 0).item_at(area, 2, 1), None);
        assert_eq!(r.item_at(Rect::new(0, 0, 0, 0), 0, 0), None);
    }

    #[test]
    fn hit_testing_half_and_vertical() {
        let area = Rect::new(0, 0, 20, 5);
        let wide = Rating::new(0.0, 3)
            .symbols(RatingSymbols::fire())
            .half_steps(true);
        assert_eq!(wide.value_at(area, 0, 0), Some(0.5));
        assert_eq!(wide.value_at(area, 1, 0), Some(1.0));
        assert_eq!(wide.value_at(area, 2, 0), None, "gap");
        assert_eq!(wide.value_at(area, 3, 0), Some(1.5));
        // single-width glyph cannot select half
        let narrow = Rating::new(0.0, 3).half_steps(true);
        assert_eq!(narrow.value_at(area, 0, 0), Some(1.0));
        // vertical: top row is the highest item
        let v = Rating::new(0.0, 3).orientation(Orientation::Vertical);
        assert_eq!(v.value_at(area, 0, 0), Some(3.0));
        assert_eq!(v.value_at(area, 0, 1), None, "gap row");
        assert_eq!(v.value_at(area, 0, 2), Some(2.0));
        assert_eq!(v.value_at(area, 0, 4), Some(1.0));
        assert_eq!(v.value_at(area, 5, 0), None);
    }

    #[test]
    fn stateful_render_uses_state() {
        use ratatui::widgets::StatefulWidget;
        let mut state = RatingState::new(2.0, 3).with_half_steps(false);
        let area = Rect::new(0, 0, 5, 1);
        let mut buf = Buffer::empty(area);
        Rating::new(0.0, 1)
            .gap(0)
            .stateful()
            .render(area, &mut buf, &mut state);
        assert_eq!(
            (0..3).map(|x| buf[(x, 0)].symbol()).collect::<String>(),
            "★★☆"
        );
        // hover + animation flow through
        state.set_hover(Some(3.0));
        let mut buf = Buffer::empty(area);
        let widget = Rating::new(0.0, 1).gap(0).stateful();
        (&widget).render(area, &mut buf, &mut state);
        assert_eq!(buf[(2, 0)].symbol(), "★");
        state.set_hover(None);
        state.animate_to_with(3.0, 4, crate::Easing::Linear);
        let mut buf = Buffer::empty(area);
        (&widget).render(area, &mut buf, &mut state);
        assert_eq!(buf[(2, 0)].symbol(), "☆", "animation still at 2.0");
        for _ in 0..4 {
            state.tick();
        }
        let mut buf = Buffer::empty(area);
        (&widget).render(area, &mut buf, &mut state);
        assert_eq!(buf[(2, 0)].symbol(), "★");
    }
}

#[cfg(feature = "theme")]
#[cfg(test)]
mod theme_tests {
    use super::*;

    fn fg(r: Rating<'_>, x: u16) -> Option<Color> {
        let area = Rect::new(0, 0, 20, 1);
        let mut buf = Buffer::empty(area);
        r.render(area, &mut buf);
        buf[(x, 0)].style().fg
    }

    #[test]
    fn maps_roles() {
        let t = Theme::nord();
        let r = Rating::new(1.0, 2).theme(&t);
        assert_eq!(fg(r.clone(), 0), Some(t.accent));
        assert_eq!(fg(r, 2), Some(t.disabled));
    }

    #[test]
    fn explicit_wins_any_order() {
        let t = Theme::nord();
        let a = Rating::new(1.0, 2).filled_color(Color::Red).theme(&t);
        let b = Rating::new(1.0, 2).theme(&t).filled_color(Color::Red);
        assert_eq!(fg(a, 0), Some(Color::Red));
        assert_eq!(fg(b, 0), Some(Color::Red));
    }

    #[test]
    fn all_themes_render() {
        for t in Theme::all() {
            let r = Rating::new(2.5, 5)
                .half_steps(true)
                .label("x")
                .show_value(true);
            let _ = fg(r.theme(t), 0);
        }
    }
}
