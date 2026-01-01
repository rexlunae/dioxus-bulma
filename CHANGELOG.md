# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
