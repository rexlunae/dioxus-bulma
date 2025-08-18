# Dioxus Bulma

A comprehensive Rust component library bringing [Bulma CSS](https://bulma.io/) components to [Dioxus](https://dioxuslabs.com/) applications.

## Features

- 🎨 **Full Bulma Integration**: Complete set of Bulma CSS components
- ⚡ **Type-Safe**: Leverages Rust's type system for compile-time guarantees
- 🎯 **Dioxus Native**: Built specifically for the Dioxus ecosystem
- 📱 **Responsive**: Mobile-first responsive design out of the box
- 🌗 **Theme Support**: Light/dark/auto theme modes
- 🧩 **Composable**: Mix and match components as needed

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dioxus-bulma = "0.1.0"
dioxus = "0.6"

[features]
web = ["dioxus-bulma/web"]
```

## Quick Start

```rust
use dioxus::prelude::*;
use dioxus_bulma::*;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        BulmaProvider {
            theme: BulmaTheme::Auto,
            load_bulma_css: true,
            Section {
                Container {
                    Title { size: TitleSize::Is1, "Hello, Dioxus Bulma!" }
                    Subtitle { size: TitleSize::Is4, "Build beautiful web apps with Rust" }
                    
                    Button {
                        color: BulmaColor::Primary,
                        onclick: |_| println!("Button clicked!"),
                        "Click me!"
                    }
                }
            }
        }
    }
}
```

## Available Components

### Layout
- `Container` - Responsive container with breakpoint options
- `Section` - Page sections with size variants
- `Columns` / `Column` - Flexible grid system with responsive sizing
- `Hero` / `HeroBody` / `HeroHead` / `HeroFoot` - Hero banner component

### Elements
- `Button` - Full-featured buttons with colors, sizes, and states
- `Title` / `Subtitle` - Typography components with heading levels
- `Notification` - Dismissible alert notifications
- `Tag` / `Tags` - Label and tag components with colors and variants

### Form Components
- `Input` - Text inputs with validation states and various types
- `Field` - Form field wrapper with grouping options
- `Control` - Form control wrapper with icon support
- `Label` - Form labels
- `Help` - Form help text with color states

### Components
- `Card` / `CardHeader` / `CardContent` / `CardFooter` - Card components
- `Modal` / `ModalCard` / `ModalCardHead` / `ModalCardBody` / `ModalCardFoot` - Modal dialogs
- `Navbar` / `NavbarBrand` / `NavbarMenu` / `NavbarItem` - Navigation bars

## Examples

Run the examples to see all components in action:

```bash
# Basic demo
cargo run --example demo --features web

# Comprehensive showcase
cargo run --example showcase --features web
```

## Component Props

### Button
```rust
Button {
    color: BulmaColor::Primary,      // Color scheme
    size: BulmaSize::Medium,         // Button size
    outlined: true,                  // Outlined style
    rounded: true,                   // Rounded corners
    loading: false,                  // Loading state
    disabled: false,                 // Disabled state
    fullwidth: false,                // Full width
    onclick: |_| { /* handler */ },  // Click handler
    "Button Text"
}
```

### Input
```rust
Input {
    input_type: InputType::Email,    // Input type
    placeholder: "Enter email...",   // Placeholder text
    value: email_value,              // Controlled value
    color: BulmaColor::Success,      // Validation color
    size: BulmaSize::Large,          // Input size
    oninput: |evt| { /* handler */ } // Input handler
}
```

## Theme System

Wrap your app with `BulmaProvider` to enable theming:

```rust
BulmaProvider {
    theme: BulmaTheme::Auto,  // Auto, Light, or Dark
    load_bulma_css: true,     // Auto-load Bulma CSS from CDN
    // Your app components...
}
```

## Development Status

This library is under active development. Core components are functional, with more advanced components being added regularly.

### Implemented ✅
- Button (full-featured with all variants)
- Layout components (Container, Columns, Section, Hero)
- Typography (Title/Subtitle with proper heading levels)
- Form components (Input, Field, Control, Label, Help)
- UI components (Card, Modal, Navbar, Notification, Tag)
- Theme system with BulmaProvider

### In Progress 🚧
- Advanced form components (Select, Textarea, Checkbox, Radio)
- Data display components (Table, Image, Progress)
- Navigation components (Breadcrumb, Tabs, Pagination)
- Overlay components (Dropdown, Menu, Tooltip)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.