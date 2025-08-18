# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust library crate (`dioxus-bulma`) designed to provide Dioxus components for the Bulma CSS framework. The project is in its early stages with basic scaffolding in place.

## Development Commands

- **Build**: `cargo build`
- **Test**: `cargo test`
- **Run tests with output**: `cargo test -- --nocapture`
- **Run specific test**: `cargo test <test_name>`
- **Check code without building**: `cargo check`
- **Build documentation**: `cargo doc`
- **Build and open documentation**: `cargo doc --open`

## Project Structure

- `src/lib.rs` - Main library entry point (currently contains placeholder code)
- `Cargo.toml` - Project configuration with no dependencies yet
- Edition 2024 is used

## Architecture Notes

This is a fully functional component library that bridges Dioxus (a React-like Rust web framework) with Bulma CSS components. The library follows established patterns from similar projects like dioxus-chakra and dioxus-bootstrap.

### Core Structure
- `src/lib.rs` - Main library entry point with re-exports
- `src/theme.rs` - Theme system with BulmaProvider, colors, sizes, and enums
- `src/utils.rs` - Utility functions for CSS class building
- `src/components/` - Individual component implementations

### Implemented Components
- **Button**: Full-featured with colors, sizes, variants (outlined, rounded, loading, etc.)
- **Container**: Layout container with fluid and breakpoint options
- **Columns/Column**: Flexible grid system with responsive sizing
- **Title/Subtitle**: Typography components with heading levels
- **Section**: Layout section with size variants
- **Notification**: Alert/notification component with dismissible functionality
- **Input**: Form input with various types and states

### Component Patterns
- All components use `#[component]` macro and return `Element`
- Props structs derive `Props, Clone, PartialEq`
- Optional props use `Option<T>` with `#[props(default)]`
- CSS classes built using utility functions for consistency
- Event handlers passed as `Option<EventHandler<T>>`

### Theme System
- `BulmaProvider` component wraps the app and loads Bulma CSS
- Standardized color scheme with `BulmaColor` enum
- Size system with `BulmaSize` enum
- Automatic CSS class generation from enums

### Running Examples
- `cargo run --example demo --features web` - Interactive demo showcasing components