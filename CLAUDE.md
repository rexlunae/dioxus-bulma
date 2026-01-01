# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust library crate (`dioxus-bulma`) designed to provide Dioxus components for the Bulma CSS framework. The library is fully compatible with **Dioxus 0.7**.

## Recent Changes: Dioxus 0.6 to 0.7 Upgrade

The codebase has been upgraded to Dioxus 0.7 with the following key changes:

### Breaking Changes Handled
1. **Children Field Requirements**: Updated all component Props to use `children: Element` (required) instead of `children: Option<Element>`
2. **Prelude Imports**: Changed from `pub use dioxus::prelude::*` to selective imports to avoid conflicts with Dioxus 0.7's built-in `Title` and `Subtitle` components
3. **Hooks API**: Uses `use_signal` (Dioxus 0.7) instead of `use_state` (Dioxus 0.6)

### Import Pattern
The library uses a careful import strategy to avoid component conflicts:
- `dioxus_bulma::prelude::*` exports all components EXCEPT `Title` and `Subtitle`
- Users must explicitly import `Title` and `Subtitle` from `dioxus_bulma::components` to avoid conflicts with Dioxus's built-in document components
- This is documented in the README and example code

## Development Commands

- **Build**: `cargo build`
- **Test**: `cargo test`
- **Run tests with output**: `cargo test -- --nocapture`
- **Run specific test**: `cargo test <test_name>`
- **Check code without building**: `cargo check`
- **Build documentation**: `cargo doc`
- **Build and open documentation**: `cargo doc --open`
- **Run demo**: `cargo run --example demo --features web`
- **Publish**: `cargo publish`

## Project Structure

- `src/lib.rs` - Main library entry point with re-exports and documentation
- `src/prelude.rs` - Prelude module with carefully selected exports (excludes Title/Subtitle to avoid conflicts)
- `src/theme.rs` - Theme system with BulmaProvider, colors, sizes, and enums
- `src/utils.rs` - Utility functions for CSS class building
- `src/components/` - Individual component implementations
- `src/components/mod.rs` - Re-exports all components
- `examples/demo.rs` - Comprehensive demo showing all components
- `tests/` - Integration and API tests
- `Cargo.toml` - Project configuration
- Edition 2021 is used

## Architecture Notes

This is a fully functional component library that bridges Dioxus (a React-like Rust web framework) with Bulma CSS components. The library follows established patterns from similar projects like dioxus-chakra and dioxus-bootstrap.

### Core Structure
- `src/lib.rs` - Main library entry point with re-exports
- `src/theme.rs` - Theme system with BulmaProvider, colors, sizes, and enums
- `src/utils.rs` - Utility functions for CSS class building
- `src/components/` - Individual component implementations
- `src/prelude.rs` - Carefully curated prelude to avoid naming conflicts

### Implemented Components

**Layout Components:**
- `Container` - Responsive layout container
- `Columns`/`Column` - Flexible grid system with responsive sizing
- `Section` - Layout section with size variants
- `Hero`/`HeroBody`/`HeroHead`/`HeroFoot` - Hero section components
- `Level`/`LevelLeft`/`LevelRight`/`LevelItem` - Horizontal level layouts
- `Media`/`MediaLeft`/`MediaContent`/`MediaRight` - Media object component
- `Tile` - Flexible tile layout system

**Elements:**
- `Button`/`Buttons` - Full-featured with colors, sizes, variants
- `Title`/`Subtitle` - Typography components with heading levels
- `Content` - Content wrapper with typography enhancements
- `Delete` - Delete button component
- `Icon` - Icon wrapper with Font Awesome support
- `Image` - Image component with responsive sizing
- `Notification` - Alert/notification component with dismissible functionality
- `Progress` - Progress bar component
- `Table`/`TableContainer` - Responsive data tables
- `Tag`/`Tags` - Tag/label components with colors and variants

**Form Components:**
- `Field`/`Label`/`Help` - Form field with label and help text
- `Control` - Form control wrapper with addons
- `Input` - Form input with various types and validation states
- `Textarea` - Multi-line text input
- `Select`/`Option` - Dropdown select with options
- `Checkbox` - Checkbox input component
- `Radio` - Radio button component
- `File` - File upload component

**Components:**
- `Card`/`CardHeader`/`CardHeaderTitle`/`CardContent`/`CardFooter`/`CardFooterItem` - Card components
- `Breadcrumb`/`BreadcrumbItem` - Breadcrumb navigation
- `Dropdown`/`DropdownTrigger`/`DropdownMenu`/`DropdownItem`/`DropdownDivider` - Dropdown menu
- `Menu`/`MenuLabel`/`MenuList`/`MenuItem` - Sidebar menu
- `Message`/`MessageHeader`/`MessageBody` - Message box component
- `Modal`/`ModalCard`/`ModalCardHead`/`ModalCardBody`/`ModalCardFoot` - Modal dialog components
- `Navbar`/`NavbarBrand`/`NavbarMenu`/`NavbarStart`/`NavbarEnd`/`NavbarItem` - Navigation bar
- `Pagination`/`PaginationPrevious`/`PaginationList`/`PaginationNext`/`PaginationLink`/`PaginationEllipsis` - Pagination
- `Panel`/`PanelHeading`/`PanelTabs`/`PanelBlock`/`PanelIcon` - Panel component
- `Tabs`/`Tab` - Tabbed interface component

### Component Patterns
- All components use `#[component]` macro and return `Element`
- Props structs derive `Props, Clone, PartialEq`
- Optional props use `Option<T>` with `#[props(default)]`
- Children field is `Element` (required, not optional) per Dioxus 0.7 requirements
- CSS classes built using utility functions for consistency
- Event handlers passed as `Option<EventHandler<T>>`

### Important: Title and Subtitle Components
These components must be explicitly imported from `dioxus_bulma::components` to avoid conflicts with Dioxus 0.7's built-in document components. They are NOT included in the prelude automatically.



### Theme System
- `BulmaProvider` component wraps the app and loads Bulma CSS
- Standardized color scheme with `BulmaColor` enum
- Size system with `BulmaSize` enum
- Automatic CSS class generation from enums

### Running Examples
- `cargo run --example demo --features web` - Interactive demo showcasing components