//! # tui-rating
//!
//! A customizable rating widget (stars, hearts, circles, …) for
//! [ratatui](https://ratatui.rs) applications, with half-step support.
//!
//! ## Quick start
//!
//! ```
//! use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
//! use tui_rating::{Rating, RatingState};
//!
//! let mut state = RatingState::new(3.0, 5).with_half_steps(true);
//! state.increment(); // 3.5
//!
//! let area = Rect::new(0, 0, 30, 1);
//! let mut buf = Buffer::empty(area);
//! Rating::from_state(&state).show_value(true).render(area, &mut buf);
//! ```
//!
//! ## Widgets
//!
//! - [`Rating`]: stars/hearts/… with half steps, smooth fractional fill,
//!   gradients, vertical layout, hover preview, mouse hit-testing
//!   (also a `StatefulWidget`)
//! - [`RatingBar`]: continuous gauge-style bar
//! - [`RatingBadge`]: `[ 4.2 ★ ]`, pill and vote-count badges
//! - [`RatingHistogram`]: Amazon-style review summary
//! - [`BipolarRating`]: `−n … +n` scales (👎 … 👍)
//! - [`NpsRating`] / [`NpsSummary`]: 0–10 Net Promoter Score selector and aggregate
//! - [`ThumbsRating`]: up/down votes with a net score ([`ThumbsState`])
//! - [`LikertScale`]: labelled agreement / satisfaction scales ([`LikertState`])
//! - [`SentimentFaces`]: 😡 🙁 😐 🙂 😍
//! - [`RatingRange`]: two-handle score band ([`RangeState`])
//! - [`MultiCriteria`]: several labelled ratings + overall ([`RatingForm`] state)
//! - [`RatingTrend`] / [`RatingDelta`]: sparkline and `▲ 0.3` change indicator
//! - [`RatingSlider`]: continuous slider with numeric entry ([`SliderState`])
//! - [`RatingCompare`]: two items side by side with the difference
//!
//! ## Support types
//!
//! - [`RatingState`]: value/max/step, hover, animation, input handling
//! - [`RatingSymbols`]: glyph presets (stars … emoji, smooth)
//! - [`RatingStyle`]: bundled looks (`netflix`, `github`, `neon`, `retro`, …)
//! - [`Easing`] / [`Animation`]: tick-based value animation
//!
//! ## Features
//!
//! - `theme`: shared theming via `tui-nuances`
//! - `crossterm`: `RatingState::handle_key` / `handle_mouse`
//! - `serde`: serialize `RatingState` and the small enums

mod animation;
mod badge;
mod bar;
mod bipolar;
mod compare;
mod faces;
mod histogram;
#[cfg(feature = "crossterm")]
mod input;
mod likert;
mod multi;
mod nps;
mod preset;
mod range;
mod rating;
mod slider;
mod state;
mod symbols;
mod thumbs;
mod trend;
mod util;

pub use animation::{Animation, Easing};
pub use badge::{BadgeKind, RatingBadge};
pub use bar::RatingBar;
pub use bipolar::BipolarRating;
pub use compare::RatingCompare;
pub use faces::SentimentFaces;
pub use histogram::RatingHistogram;
pub use likert::{LikertLabels, LikertScale, LikertState};
pub use multi::{MultiCriteria, RatingForm};
pub use nps::{NpsCategory, NpsRating, NpsSummary, NpsSummaryWidget};
pub use preset::RatingStyle;
pub use range::{Handle, RangeState, RatingRange};
pub use rating::{ItemFill, LabelPosition, Orientation, Rating, StatefulRating, ValueFormat};
pub use slider::{RatingSlider, SliderState};
pub use state::{Change, RatingState};
pub use symbols::RatingSymbols;
pub use thumbs::{ThumbsRating, ThumbsState, Vote};
pub use trend::{RatingDelta, RatingTrend};
#[cfg(feature = "theme")]
pub use tui_nuances::{Status, Theme, ThemeProvider, ThemeSwitcher, Themed};
