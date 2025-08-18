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
dioxus-bulma = "0.1.0"
dioxus = "0.6"
```

For web applications, enable the web feature:

```toml
[features]
default = []
web = ["dioxus/web"]
```

## Getting Started

### 1. Basic Setup

Start by wrapping your app with `BulmaProvider` to enable theme support and CSS loading:

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
            theme: BulmaTheme::Auto,      // Auto-detect system theme
            load_bulma_css: true,         // Auto-load Bulma CSS from CDN
            
            Section {
                Container {
                    Title { size: TitleSize::Is1, "Hello, Dioxus Bulma!" }
                    Subtitle { size: TitleSize::Is4, "Build beautiful web apps with Rust" }
                    
                    Button {
                        color: BulmaColor::Primary,
                        size: BulmaSize::Large,
                        onclick: |_| println!("Button clicked!"),
                        "Get Started"
                    }
                }
            }
        }
    }
}
```

### 2. Building Forms

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

### 4. Advanced Components

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