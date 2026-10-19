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
//! ## Modules
//!
//! - [`Rating`]: the widget
//! - [`RatingState`]: value/max/step management
//! - [`RatingSymbols`]: glyph presets

mod rating;
mod state;
mod symbols;

pub use rating::{ItemFill, LabelPosition, Rating, ValueFormat};
pub use state::RatingState;
pub use symbols::RatingSymbols;
#[cfg(feature = "theme")]
pub use tui_nuances::{Status, Theme, ThemeProvider, ThemeSwitcher, Themed};
