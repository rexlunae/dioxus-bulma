//! # Dioxus Bulma
//!
//! A component library bringing Bulma CSS components to Dioxus applications.
//!
//! This library provides a comprehensive set of UI components styled with
//! Bulma CSS framework, designed to work seamlessly with Dioxus.
//!
//! ## Usage
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_bulma::prelude::*;
//! ```

pub mod components;
pub mod prelude;
pub mod theme;
pub mod utils;

// Re-export theme components at top level (no naming conflicts)
pub use theme::*;

// Re-export dioxus for convenience, but avoid ImageSize conflict
pub use dioxus::prelude::{component, Element, Props, rsx, EventHandler, MouseEvent, FormEvent};
