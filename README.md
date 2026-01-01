# Dioxus Bulma

A comprehensive Rust component library bringing [Bulma CSS](https://bulma.io/) components to [Dioxus](https://dioxuslabs.com/) applications.

Build beautiful, responsive web applications with the power of Rust and the elegance of Bulma CSS.

## Features

- 🎨 **Complete Bulma Integration**: All Bulma CSS components implemented as Dioxus components
- ⚡ **Type-Safe**: Leverages Rust's type system for compile-time guarantees and better developer experience
- 🎯 **Dioxus Native**: Built specifically for the Dioxus ecosystem with proper event handling
- 📱 **Responsive Design**: Mobile-first responsive design patterns out of the box
- 🌗 **Theme Support**: Light/dark/auto theme modes with CSS custom properties
- 🧩 **Composable Architecture**: Mix and match components with consistent APIs
- 🎛️ **Rich Props System**: Extensive customization through typed props
- 📦 **Zero Runtime Dependencies**: Pure Rust implementation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dioxus-bulma = "0.7"
dioxus = "0.7"
```

For web applications, enable the web feature:

```toml
[features]
default = []
web = ["dioxus/web"]
router = ["dioxus-bulma/router"]  # Optional: Enable router integration
```

### Router Integration

Enable router support for seamless navigation with dioxus-router:

```toml
[dependencies]
dioxus-bulma = { version = "0.7", features = ["router"] }
dioxus-router = "0.7"
```

## Dioxus 0.7 Compatibility

This library is fully compatible with Dioxus 0.7. If you're upgrading from Dioxus 0.6, see the [Upgrade Guide](#upgrade-guide) section below.

## Getting Started

### 1. Basic Setup

Start by wrapping your app with `BulmaProvider` to enable theme support and CSS loading:

```rust
use dioxus::prelude::*;
use dioxus_bulma::prelude::*;
use dioxus_bulma::components::{Title, Subtitle};

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        BulmaProvider {
            theme: BulmaTheme::Auto,      // Auto-detect system theme
            load_bulma_css: true,         // Auto-load Bulma CSS from CDN

            Section {
                Container {
                    Title { size: TitleSize::Is1, "Hello, Dioxus Bulma!" }
                    Subtitle { "Build beautiful web apps with Rust" }

                    Button {
                        color: BulmaColor::Primary,
                        size: BulmaSize::Large,
                        "Get Started"
                    }
                }
            }
        }
    }
}
```

### 2. Import Patterns

The library provides two import patterns:

**Recommended: Using the prelude**
```rust
use dioxus::prelude::*;
use dioxus_bulma::prelude::*;
use dioxus_bulma::components::{Title, Subtitle}; // Import Title/Subtitle explicitly
```

**Alternative: Component module**
```rust
use dioxus::prelude::*;
use dioxus_bulma::prelude::*;
use dioxus_bulma::components::*;
```

> **Note**: `Title` and `Subtitle` must be explicitly imported to avoid conflicts with Dioxus 0.7's built-in document components.

### 3. Building Forms

Create interactive forms with validation:

```rust
#[component]
fn ContactForm() -> Element {
    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);

    rsx! {
        Card {
            CardHeader { CardHeaderTitle { "Contact Us" } }
            CardContent {
                Field {
                    Label { "Name" }
                    Control {
                        Input {
                            input_type: InputType::Text,
                            placeholder: "Enter your name",
                            value: name(),
                            oninput: move |evt: FormEvent| name.set(evt.value())
                        }
                    }
                }

                Field {
                    Label { "Email" }
                    Control {
                        Input {
                            input_type: InputType::Email,
                            placeholder: "Enter your email",
                            color: if email().contains('@') {
                                Some(BulmaColor::Success)
                            } else {
                                None
                            },
                            value: email(),
                            oninput: move |evt: FormEvent| email.set(evt.value())
                        }
                    }
                    Help { color: BulmaColor::Info, "We'll never share your email" }
                }

                Field { grouped: true,
                    Control {
                        Button {
                            color: BulmaColor::Primary,
                            onclick: move |_| {
                                println!("Submitted: {} - {}", name(), email());
                            },
                            "Submit"
                        }
                    }
                    Control {
                        Button {
                            onclick: move |_| {
                                name.set(String::new());
                                email.set(String::new());
                            },
                            "Clear"
                        }
                    }
                }
            }
        }
    }
}
```

### 3. Layout with Grid System

Use Bulma's flexible grid system:

```rust
#[component]
fn GridLayout() -> Element {
    rsx! {
        Container {
            Columns {
                multiline: true,

                Column { size: ColumnSize::Half,
                    Card {
                        CardContent {
                            Title { size: TitleSize::Is4, "Half Width" }
                            Content { "This column takes half the available width." }
                        }
                    }
                }

                Column { size: ColumnSize::OneQuarter,
                    Card {
                        CardContent {
                            Title { size: TitleSize::Is4, "Quarter Width" }
                            Content { "This column takes one quarter of the width." }
                        }
                    }
                }

                Column { size: ColumnSize::OneQuarter,
                    Card {
                        CardContent {
                            Title { size: TitleSize::Is4, "Quarter Width" }
                            Content { "Another quarter width column." }
                        }
                    }
                }
            }
        }
    }
}
```

### 4. Router Integration

Use components with dioxus-router for client-side navigation:

```rust
use dioxus::prelude::*;
use dioxus_bulma::*;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
    #[route("/contact")]
    Contact {},
}

#[component]
fn App() -> Element {
    rsx! {
        BulmaProvider {
            theme: BulmaTheme::Auto,
            load_bulma_css: true,

            Router::<Route> {}

            // Navigation with router-enabled components
            Section {
                Container {
                    // Button navigation
                    Field { grouped: true,
                        Control {
                            Button {
                                color: BulmaColor::Primary,
                                to: Route::Home {},
                                "Home"
                            }
                        }
                        Control {
                            Button {
                                color: BulmaColor::Link,
                                to: Route::About {},
                                "About"
                            }
                        }
                        Control {
                            Button {
                                color: BulmaColor::Info,
                                to: Route::Contact {},
                                "Contact"
                            }
                        }
                    }

                    // Breadcrumb navigation
                    Breadcrumb {
                        BreadcrumbItem { to: Route::Home {}, "Home" }
                        BreadcrumbItem { to: Route::About {}, "About" }
                        BreadcrumbItem { active: true, "Current Page" }
                    }

                    // Content
                    Outlet::<Route> {}
                }
            }
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        Title { size: TitleSize::Is2, "Welcome Home" }
        Content { "This is the home page." }
    }
}

#[component]
fn About() -> Element {
    rsx! {
        Title { size: TitleSize::Is2, "About Us" }
        Content { "Learn more about our company." }
    }
}

#[component]
fn Contact() -> Element {
    rsx! {
        Title { size: TitleSize::Is2, "Contact" }
        Content { "Get in touch with us." }
    }
}
```

### 5. Advanced Components

Leverage advanced components like dropdowns and modals:

```rust
#[component]
fn AdvancedDemo() -> Element {
    let mut show_modal = use_signal(|| false);
    let mut dropdown_active = use_signal(|| false);

    rsx! {
        // Dropdown
        Dropdown {
            active: dropdown_active(),
            DropdownTrigger {
                onclick: move |_| dropdown_active.set(!dropdown_active()),
                Button { color: BulmaColor::Primary, "Options ▼" }
            }
            DropdownMenu {
                DropdownItem { "Action" }
                DropdownItem { "Another action" }
                DropdownDivider {}
                DropdownItem { active: true, "Active item" }
            }
        }

        // Modal trigger
        Button {
            color: BulmaColor::Info,
            onclick: move |_| show_modal.set(true),
            "Open Modal"
        }

        // Modal
        Modal {
            active: show_modal(),
            onclose: move |_| show_modal.set(false),
            ModalCard {
                ModalCardHead {
                    onclose: move |_| show_modal.set(false),
                    Title { size: TitleSize::Is4, "Modal Title" }
                }
                ModalCardBody {
                    Content { "This is the modal content!" }
                }
                ModalCardFoot {
                    Button {
                        color: BulmaColor::Success,
                        onclick: move |_| show_modal.set(false),
                        "Save"
                    }
                    Button {
                        onclick: move |_| show_modal.set(false),
                        "Cancel"
                    }
                }
            }
        }
    }
}
```

## Available Components

### Layout Components
- `Container` - Responsive container with breakpoint options
- `Section` - Page sections with size variants
- `Columns` / `Column` - Flexible grid system with responsive sizing and offsets
- `Hero` / `HeroBody` / `HeroHead` / `HeroFoot` - Hero banner components with sizes
- `Level` / `LevelLeft` / `LevelRight` / `LevelItem` - Horizontal level layout
- `Media` / `MediaLeft` / `MediaContent` / `MediaRight` - Media object layout
- `Tile` - Metro-style tile layout system

### Elements
- `Button` - Full-featured buttons with colors, sizes, states, and variants
- `Title` / `Subtitle` - Typography components with proper heading levels
- `Content` - Rich content container with typography styles
- `Delete` - Delete button with size variants
- `Icon` - Icon wrapper with size and color options
- `Image` - Image container with responsive sizing options
- `Notification` - Dismissible alert notifications with colors and light variants
- `Progress` - Progress bars with colors and values
- `Table` - Data tables with styling options (bordered, striped, hoverable)
- `Tag` / `Tags` - Label and tag components with colors, sizes, and variants

### Form Components
- `Input` - Text inputs with validation states, icons, and various types
- `Textarea` - Multi-line text areas with rows and validation
- `Select` - Dropdown select inputs with multiple options
- `Checkbox` - Checkbox inputs with labels
- `Radio` - Radio button inputs with grouping
- `File` - File input components with custom styling
- `Field` - Form field wrapper with grouping and addon options
- `Control` - Form control wrapper with icon and loading support
- `Label` - Form labels with proper association
- `Help` - Form help text with color states

### Navigation Components
- `Breadcrumb` / `BreadcrumbItem` - Breadcrumb navigation with separators
- `Tabs` / `Tab` - Tab navigation with styles (default, boxed, toggle)
- `Pagination` / `PaginationPrevious` / `PaginationNext` / `PaginationList` / `PaginationLink` / `PaginationEllipsis` - Pagination controls

### Components
- `Card` / `CardHeader` / `CardHeaderTitle` / `CardContent` / `CardFooter` / `CardFooterItem` - Card components
- `Dropdown` / `DropdownTrigger` / `DropdownMenu` / `DropdownItem` / `DropdownDivider` - Dropdown menus
- `Menu` / `MenuLabel` / `MenuList` / `MenuItem` - Vertical navigation menus
- `Message` / `MessageHeader` / `MessageBody` - Message components with colors and close functionality
- `Modal` / `ModalCard` / `ModalCardHead` / `ModalCardBody` / `ModalCardFoot` - Modal dialogs
- `Navbar` / `NavbarBrand` / `NavbarMenu` / `NavbarItem` - Navigation bars
- `Panel` / `PanelHeading` / `PanelTabs` / `PanelBlock` / `PanelIcon` - Panel components

### Router-Enabled Components 🚀

When the `router` feature is enabled, these components support the `to` prop for client-side navigation:

- `Button` - Navigate on click instead of form submission
- `BreadcrumbItem` - Router-aware breadcrumb navigation
- `DropdownItem` - Navigate from dropdown menus
- `MenuItem` - Navigate from vertical menus
- `PanelBlock` - Navigate from panel items
- `PaginationPrevious` / `PaginationNext` / `PaginationLink` - Navigate between pages

**Router Props**:
- `to: Route` - Navigate to route (takes priority over `href`)
- `href: String` - Fallback for regular link behavior
- All existing styling and event props work normally

## Examples

Run the examples to see all components in action:

```bash
# Basic demo with core components
cargo run --example demo --features web

# Interactive showcase with all components
cargo run --example showcase --features web

# Comprehensive demo showcasing all features
cargo run --example comprehensive_demo --features web
```

Each example demonstrates different aspects:
- **demo**: Basic usage patterns and core components
- **showcase**: Interactive components with state management
- **comprehensive_demo**: Complete feature set including forms, navigation, and advanced components

## Component Props Reference

### Core Colors and Sizes

All components support Bulma's color system:
```rust
BulmaColor::Primary | Link | Info | Success | Warning | Danger |
           White | Light | Dark | Black | Text | Ghost
```

Size variants available:
```rust
BulmaSize::Small | Normal | Medium | Large
```

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
    input_type: InputType::Email,    // Text, Email, Password, etc.
    placeholder: "Enter email...",   // Placeholder text
    value: email_value,              // Controlled value
    color: BulmaColor::Success,      // Validation color
    size: BulmaSize::Large,          // Input size
    readonly: false,                 // Read-only state
    disabled: false,                 // Disabled state
    oninput: |evt| { /* handler */ } // Input handler
}
```

### Card Structure
```rust
Card {
    CardHeader {
        CardHeaderTitle { "Card Title" }
    }
    CardContent {
        // Main content
    }
    CardFooter {
        CardFooterItem { href: "#", "Action" }
        CardFooterItem { "Another Action" }
    }
}
```

### Layout Grid
```rust
Columns {
    multiline: true,                 // Allow wrapping
    centered: false,                 // Center columns
    vcentered: false,                // Vertical centering
    mobile: false,                   // Force on mobile

    Column {
        size: ColumnSize::Half,      // Fractional sizing
        offset: ColumnSize::OneQuarter, // Column offset
        // Content
    }
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

This library provides a complete implementation of Bulma CSS components for Dioxus applications.

### ✅ Fully Implemented
- **Layout Components**: Container, Columns/Column, Section, Hero, Level, Media, Tile
- **Elements**: Button, Title/Subtitle, Content, Delete, Icon, Image, Notification, Progress, Table, Tag/Tags
- **Form Components**: Input, Textarea, Select, Checkbox, Radio, File, Field, Control, Label, Help
- **Navigation**: Breadcrumb, Tabs, Pagination with all sub-components
- **Advanced Components**: Card, Dropdown, Menu, Message, Modal, Navbar, Panel
- **Theme System**: Complete BulmaProvider with light/dark/auto themes

### 🎯 Features
- Type-safe props with comprehensive validation
- Consistent event handling across all components
- Responsive design patterns built-in
- Full Bulma CSS class coverage
- Zero runtime dependencies
- Comprehensive examples and documentation

### 🚀 Production Ready
All core Bulma components are implemented and thoroughly tested. The library is suitable for production use in Dioxus web applications.

## Best Practices

### Component Composition
```rust
// Prefer semantic composition
Card {
    CardHeader { CardHeaderTitle { "User Profile" } }
    CardContent {
        Media {
            MediaLeft { Image { /* avatar */ } }
            MediaContent { /* user info */ }
        }
    }
    CardFooter {
        CardFooterItem { "Edit" }
        CardFooterItem { "Delete" }
    }
}

// Over flat structures
div { class: "card",
    div { class: "card-header", /* ... */ }
    div { class: "card-content", /* ... */ }
}
```

### Form Validation
```rust
// Use color props for visual validation feedback
Input {
    color: if is_valid { BulmaColor::Success } else { BulmaColor::Danger },
    // ...
}
Help {
    color: BulmaColor::Danger,
    "Please enter a valid email address"
}
```

### Responsive Design
```rust
// Leverage Bulma's responsive system
Columns {
    Column { size: ColumnSize::IsFullMobile, /* mobile: full width */ }
    Column { size: ColumnSize::IsHalfTablet,  /* tablet: half width */ }
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup
```bash
git clone https://github.com/your-username/dioxus-bulma
cd dioxus-bulma
cargo check
cargo run --example comprehensive_demo --features web
```

## Upgrade Guide: Dioxus 0.6 to 0.7

If you're upgrading from Dioxus 0.6, follow these steps to update your dioxus-bulma integration:

### 1. Update Dependencies

```toml
[dependencies]
dioxus = "0.7"
dioxus-bulma = "0.7"
dioxus-router = "0.7"  # if using router
```

### 2. Update Imports

**Before (Dioxus 0.6):**
```rust
use dioxus::prelude::*;
use dioxus_bulma::*;
```

**After (Dioxus 0.7):**
```rust
use dioxus::prelude::*;
use dioxus_bulma::prelude::*;
use dioxus_bulma::components::{Title, Subtitle};  // Explicit imports to avoid conflicts
```

### 3. Hooks API Changes

Dioxus 0.7 renamed `use_state` to `use_signal`:

**Before:**
```rust
let mut count = use_state(|| 0);
```

**After:**
```rust
let mut count = use_signal(|| 0);
```

### 4. Children Prop Changes

In Dioxus 0.7, the `children` prop in component props is required (no longer optional):

**Before:**
```rust
#[derive(Props, Clone, PartialEq)]
pub struct MyComponentProps {
    #[props(default)]
    pub children: Option<Element>,
}
```

**After:**
```rust
#[derive(Props, Clone, PartialEq)]
pub struct MyComponentProps {
    pub children: Element,  // Required, no default
}
```

### Component Naming Conflicts

Dioxus 0.7 introduces built-in document components (`Title`, `Subtitle`) that may conflict with dioxus-bulma components. Always import them explicitly from the components module to ensure you're using the Bulma versions.

For more detailed information, refer to the [Dioxus 0.7 Release Notes](https://github.com/dioxuslabs/dioxus/releases/tag/v0.7.0).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.