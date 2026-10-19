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
mod histogram;
#[cfg(feature = "crossterm")]
mod input;
mod preset;
mod rating;
mod state;
mod symbols;
mod util;

pub use animation::{Animation, Easing};
pub use badge::{BadgeKind, RatingBadge};
pub use bar::RatingBar;
pub use bipolar::BipolarRating;
pub use histogram::RatingHistogram;
pub use preset::RatingStyle;
pub use rating::{ItemFill, LabelPosition, Orientation, Rating, StatefulRating, ValueFormat};
pub use state::RatingState;
pub use symbols::RatingSymbols;
#[cfg(feature = "theme")]
pub use tui_nuances::{Status, Theme, ThemeProvider, ThemeSwitcher, Themed};
