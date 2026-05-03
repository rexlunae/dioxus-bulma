//! Compile-only test verifying that router-enabled components accept a
//! `Routable` enum variant directly as the `to` prop. Regression test for
//! [issue #6](https://github.com/rexlunae/dioxus-bulma/issues/6).
#![cfg(feature = "router")]

use dioxus::prelude::*;
use dioxus_bulma::prelude::*;

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[route("/devices")]
    DeviceList {},
}

#[component]
fn Home() -> Element {
    rsx! { div {} }
}

#[component]
fn DeviceList() -> Element {
    rsx! { div {} }
}

// This function only needs to compile to demonstrate that all of the
// router-enabled components accept a bare `Route` value (no `Some(...)`,
// no `.into()`).
#[allow(dead_code)]
fn _accepts_route_directly() -> Element {
    rsx! {
        Button { to: Route::DeviceList {}, "Devices" }
        MenuItem { to: Route::DeviceList {}, "Devices" }
        BreadcrumbItem { to: Route::DeviceList {}, "Devices" }
        DropdownItem { to: Route::DeviceList {}, "Devices" }
        PanelBlock { to: Route::DeviceList {}, "Devices" }
        PaginationLink { to: Route::DeviceList {}, "1" }
        PaginationPrevious { to: Route::DeviceList {} }
        PaginationNext { to: Route::DeviceList {} }
        Tab { to: Route::DeviceList {}, "Devices" }
    }
}

#[test]
fn route_directly_compiles() {
    // The real assertion is the function above compiling.
    assert!(true);
}
