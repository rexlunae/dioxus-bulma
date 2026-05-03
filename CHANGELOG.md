# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.3] - 2026-05-03

### Added

- **Every component now exposes an `id: Option<String>` prop** that is
  forwarded to the rendered root element. This lets tooling (Playwright,
  Cypress, accessibility tooling, custom CSS) target individual components.
- New [`BulmaBox`] (also re-exported as `Box` from `dioxus_bulma::components`)
  component implementing Bulma's [`box`](https://bulma.io/documentation/elements/box/)
  element. Closes [#8](https://github.com/rexlunae/dioxus-bulma/issues/8).
- New `Block` helper component implementing Bulma's
  [`block`](https://bulma.io/documentation/elements/block/) spacer.
- New [`MaybeNav`] wrapper plus `#[props(into)]` on every router-enabled
  component's `to` prop, so users can now pass a `Routable` route directly
  (`MenuItem { to: Route::DeviceList, "Devices" }`) without having to write
  `Some(Route::DeviceList.into())`. Closes
  [#6](https://github.com/rexlunae/dioxus-bulma/issues/6).
- New top-level [`AGENTS.md`](AGENTS.md) document aimed at AI coding agents
  (Copilot, Claude Code, Cursor, ChatGPT, …) describing the crate's
  conventions, gotchas, and the full component inventory so generated code
  uses the available components instead of reinventing them with raw HTML.
- New regression tests `tests/id_prop_test.rs` and
  `tests/router_to_prop_test.rs`.

### Changed

- The prelude now exports Bulma's typography components as **`BulmaTitle`**
  and **`BulmaSubtitle`** instead of `Title`/`Subtitle`, to coexist cleanly
  with Dioxus 0.7's document `Title` component. The original
  `Title`/`Subtitle` names remain accessible via `dioxus_bulma::components`.
  Closes [#7](https://github.com/rexlunae/dioxus-bulma/issues/7).
- README updated to describe `id` support, the new `BulmaBox`/`Block`
  components, the new prelude naming, and the router `to` ergonomics.

### Fixed

- Confirmed the `router` feature builds cleanly on Dioxus 0.7
  ([#5](https://github.com/rexlunae/dioxus-bulma/issues/5)) — the published
  `0.7.2` version on crates.io still imported `dioxus_router::prelude::*`
  which had moved; this release re-publishes a working router build.

[`BulmaBox`]: https://docs.rs/dioxus-bulma/latest/dioxus_bulma/components/struct.BulmaBox.html
[`MaybeNav`]: https://docs.rs/dioxus-bulma/latest/dioxus_bulma/struct.MaybeNav.html

## [0.7.2] - 2025-01-01

### Changed

- Synchronized version numbers to roughly match dioxus version numbers. Some versions may slightly lag behind or run ahead in the revision number because of minor non-breaking changes to this crate or Dioxus.
- **BREAKING**: Upgraded to Dioxus 0.7 from 0.6
  - Updated all dependencies to Dioxus 0.7
  - Updated all component Props to use `children: Element` (required) instead of `children: Option<Element>`
  - Hooks API now uses `use_signal` instead of `use_state`
  - Removed `use_state` import from prelude

### Fixed

- Fixed naming conflicts with Dioxus 0.7's built-in `Title` and `Subtitle` components
  - These components are no longer automatically exported from prelude
  - Users must explicitly import them from `dioxus_bulma::components`
  - Updated all examples and tests to use correct import pattern
- Updated import strategy in `src/lib.rs` to selectively re-export components and avoid conflicts

### Documentation

- Updated README with Dioxus 0.7 installation and import instructions
- Added "Upgrade Guide: Dioxus 0.6 to 0.7" section to README
- Updated CLAUDE.md with information about Dioxus 0.7 changes and upgrade details
- Updated example code to show correct import patterns
- Updated docstring examples in prelude to show Title/Subtitle explicit import

### Migration Guide for Users

If upgrading from dioxus-bulma 0.1.2 (with Dioxus 0.6), follow these steps:

1. Update `Cargo.toml`:

   ```toml
   dioxus = "0.7"  # was "0.6"
   dioxus-bulma = "0.1.3"  # was "0.1.2"
   ```

2. Update imports in your code:

   ```rust
   // OLD (Dioxus 0.6):
   use dioxus::prelude::*;
   use dioxus_bulma::*;

   // NEW (Dioxus 0.7):
   use dioxus::prelude::*;
   use dioxus_bulma::prelude::*;
   use dioxus_bulma::components::{Title, Subtitle};  // Explicit imports
   ```

3. Update hooks:

   ```rust
   // OLD:
   let mut count = use_state(|| 0);

   // NEW:
   let mut count = use_signal(|| 0);
   ```

## [0.1.2] - 2024-12-XX

### Added

- Initial stable release with Dioxus 0.6 support
- Complete Bulma CSS component library
- Theme system with light/dark/auto modes
- Router integration support
- Comprehensive component examples

### Components

- Layout: Container, Columns, Section, Hero, Level, Media, Tile
- Elements: Button, Title, Content, Delete, Icon, Image, Notification, Progress, Table, Tag
- Forms: Field, Control, Input, Textarea, Select, Checkbox, Radio, File
- Navigation: Breadcrumb, Tabs, Pagination, Navbar
- Components: Card, Dropdown, Menu, Message, Modal, Panel
