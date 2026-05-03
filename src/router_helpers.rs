//! Router-related helpers used by components that optionally render a
//! [`dioxus_router::components::Link`] when the `router` feature is enabled.
//!
//! The main type here is [`MaybeNav`], a thin wrapper around
//! `Option<NavigationTarget>` that, combined with `#[props(into)]`, lets users
//! pass any value that implements `Into<NavigationTarget>` (such as a
//! `Routable` `Route` enum variant) directly to a `to` prop without having to
//! wrap it in `Some(...)` and call `.into()` themselves.
//!
//! Addresses [issue #6](https://github.com/rexlunae/dioxus-bulma/issues/6).
//!
//! # Example
//!
//! ```rust,ignore
//! use dioxus::prelude::*;
//! use dioxus_bulma::prelude::*;
//!
//! #[derive(Routable, Clone, PartialEq)]
//! enum Route {
//!     #[route("/")]
//!     Home,
//!     #[route("/devices")]
//!     DeviceList,
//! }
//!
//! rsx! {
//!     // Route::DeviceList works directly thanks to MaybeNav.
//!     MenuItem { to: Route::DeviceList, "Devices" }
//! }
//! ```

use dioxus::prelude::NavigationTarget;

/// Optional [`NavigationTarget`] wrapper that allows ergonomic conversion from
/// any `T: Into<NavigationTarget>` (including `Routable` enum variants) via
/// `#[props(into)]` on a Dioxus component prop.
#[derive(Clone, PartialEq, Debug)]
pub struct MaybeNav(pub Option<NavigationTarget>);

impl MaybeNav {
    /// Construct an empty `MaybeNav` (equivalent to `Option::None`).
    pub const fn none() -> Self {
        Self(None)
    }

    /// Returns `true` if no navigation target is set.
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    /// Returns the inner [`NavigationTarget`], if any.
    pub fn as_target(&self) -> Option<&NavigationTarget> {
        self.0.as_ref()
    }

    /// Consumes the wrapper and returns the inner `Option<NavigationTarget>`.
    pub fn into_inner(self) -> Option<NavigationTarget> {
        self.0
    }
}

impl Default for MaybeNav {
    fn default() -> Self {
        Self(None)
    }
}

// Blanket conversion: anything that already converts into `NavigationTarget`
// — including `NavigationTarget` itself (via the standard reflexive
// `Into<T> for T`) and any `Routable` route — converts into `MaybeNav` as
// `Some(target)`. This impl is what lets `to: Route::DeviceList` "just work".
impl<T: Into<NavigationTarget>> From<T> for MaybeNav {
    fn from(value: T) -> Self {
        Self(Some(value.into()))
    }
}
