# AGENTS.md — Guide for AI coding agents using `dioxus-bulma`

This file is written for AI coding agents (Copilot, Claude Code, Cursor,
ChatGPT, etc.) so they can write idiomatic, working code with this crate
without trial-and-error. Human contributors are welcome to read it too — see
[`README.md`](README.md) for a more prose-style introduction.

If you are an agent: **read this whole file before generating code that uses
`dioxus-bulma`**. It is short and saves users from having to hand-correct
common mistakes.

---

## TL;DR

```toml
# Cargo.toml
[dependencies]
dioxus       = "0.7"
dioxus-bulma = "0.7"

# Optional, only if the user is using dioxus-router:
# dioxus-bulma = { version = "0.7", features = ["router"] }
```

```rust
use dioxus::prelude::*;
use dioxus_bulma::prelude::*;

fn App() -> Element {
    rsx! {
        BulmaProvider {
            theme: BulmaTheme::Light,
            load_bulma_css: true,
            BulmaTitle { "Hello, Dioxus Bulma!" }
            BulmaSubtitle { "Build beautiful web apps with Rust" }
            Button {
                id: "primary-cta",
                color: BulmaColor::Primary,
                size: BulmaSize::Large,
                onclick: move |_| (),
                "Click me"
            }
        }
    }
}

fn main() { dioxus::launch(App); }
```

---

## What this crate provides

A complete set of typed Dioxus components for [Bulma CSS](https://bulma.io/).
Every component:

- Is a Dioxus `#[component]` with a strongly-typed `Props` struct.
- Accepts an **`id: Option<String>`** prop forwarded to the rendered root
  element — use this for end-to-end tests, accessibility hooks, CSS targeting,
  etc.
- Accepts **`class: Option<String>`** (appended to the Bulma classes) and
  **`style: Option<String>`** (forwarded as `style="..."`).
- Uses Bulma's modifier conventions through the `BulmaColor` and `BulmaSize`
  enums where applicable.
- Takes `children: Element` when the underlying Bulma element supports
  children.

A non-exhaustive list, grouped roughly the same way as Bulma:

| Group | Components |
| --- | --- |
| Layout | `Container`, `Section`, `Columns`/`Column`, `Hero`/`HeroBody`/`HeroHead`/`HeroFoot`, `Level`/`LevelLeft`/`LevelRight`/`LevelItem`, `Media`/`MediaLeft`/`MediaContent`/`MediaRight`, `Tile` |
| Elements | `Button`, `Buttons`, `BulmaBox` (a.k.a. `Box`), `Block`, `Content`, `Delete`, `Icon`, `Image`, `Notification`, `Progress`, `Table`/`TableContainer`, `Tag`/`Tags`, `Title`/`Subtitle` (`BulmaTitle`/`BulmaSubtitle` in the prelude — see below) |
| Form | `Field`, `FieldLabel` (the `Label` component, renamed in the prelude), `Help`, `Control`, `Input` (+ `InputType`), `Textarea`, `Select`/`Option`, `Checkbox`, `Radio`, `File` |
| Components | `Card` (`CardHeader`/`CardHeaderTitle`/`CardContent`/`CardFooter`/`CardFooterItem`), `Breadcrumb`/`BreadcrumbItem`, `Dropdown` (+ `DropdownTrigger`/`DropdownMenu`/`DropdownItem`/`DropdownDivider`), `Menu`/`MenuLabel`/`MenuList`/`MenuItem`, `Message`/`MessageHeader`/`MessageBody`, `Modal`/`ModalCard`/..., `Navbar`/`NavbarBrand`/`NavbarMenu`/..., `Pagination`/..., `Panel`/..., `Tabs`/`Tab` |

For the canonical inventory, look at
[`src/prelude.rs`](src/prelude.rs) and
[`src/components/mod.rs`](src/components/mod.rs).

---

## Things agents commonly get wrong

### 1. Use the prelude — but watch out for renames

```rust
use dioxus::prelude::*;
use dioxus_bulma::prelude::*;
```

The prelude renames a couple of items to avoid collisions with `dioxus`,
`std`, etc.:

| Bulma name (in `dioxus_bulma::components`) | In `dioxus_bulma::prelude` |
| --- | --- |
| `Title` | `BulmaTitle` |
| `Subtitle` | `BulmaSubtitle` |
| `Box` (aliased to `BulmaBox`) | `BulmaBox` (and `Block`) |
| `Label` (form field label) | `FieldLabel` |

Why:

- `dioxus::prelude` already exports a *document* `Title` component (it sets
  `<title>`). The Bulma typography component lives at `BulmaTitle` so both
  can coexist.
- `Box` would collide with Rust's `std::boxed::Box`. The component type is
  named `BulmaBox`; the file also re-exports it as `Box` for users who want
  the original Bulma name and don't use `std::boxed::Box`.
- `Label` is too generic and was overloaded; the prelude calls it
  `FieldLabel`.

If you want the original names, import them explicitly from
`dioxus_bulma::components`:

```rust
use dioxus_bulma::components::{Title, Subtitle, Label, Box, Option as BulmaOption};
```

### 2. Setting an `id` (or any other unique attribute)

Just use the `id` prop. It is `Option<String>` on every component.

```rust
Button { id: "save-btn", "Save" }
Notification { id: "alert-1", color: BulmaColor::Warning, "Heads up" }
```

The `id` is rendered on the *root* HTML element of the component, which is
also the element that carries the Bulma class (`button`, `notification`,
`box`, etc.). This is the right thing to target from Playwright/Cypress and
from CSS.

### 3. Colors, sizes and modifiers — use the enums

Don't write `class: "is-primary is-large"` by hand. Use:

```rust
Button {
    color: BulmaColor::Primary,
    size: BulmaSize::Large,
    rounded: true,
    outlined: true,
    "Action"
}
```

Available enums: `BulmaColor` (Primary, Link, Info, Success, Warning, Danger,
Light, Dark, …), `BulmaSize` (Small, Normal, Medium, Large), plus
component-specific ones like `ButtonsAlignment`, `TabsStyle`, `TabsAlignment`,
`PaginationAlignment`, `ColumnSize`, `ImageSize`, `TitleSize`, `InputType`.

Most modifier props are `Option<bool>` — pass `true` to enable, omit to leave
default (false).

### 4. Wrap your app in `BulmaProvider`

`BulmaProvider` is responsible for loading the Bulma stylesheet and providing
the theme context. It must wrap any tree that uses Bulma components:

```rust
BulmaProvider {
    theme: BulmaTheme::Light, // or Dark, Auto
    load_bulma_css: true,     // injects the official Bulma CSS automatically
    // your app here
}
```

If you set `load_bulma_css: false`, you are responsible for including Bulma's
CSS yourself (e.g. via `<link>` in `index.html` or `manganis`).

### 5. Forms: nest `Field { Control { Input { ... } } }`

```rust
Field {
    FieldLabel { "Email" }
    Control {
        Input { input_type: InputType::Email, placeholder: "you@example.com" }
    }
    Help { "We'll never share your email." }
}
```

Don't put an `Input` directly inside a `Field` — Bulma's CSS expects the
`Control` wrapper.

### 6. Router integration (`feature = "router"`)

Enable the feature in `Cargo.toml`:

```toml
dioxus-bulma = { version = "0.7", features = ["router"] }
dioxus-router = "0.7"
```

Then components that can act as links (`Button`, `MenuItem`, `BreadcrumbItem`,
`DropdownItem`, `PanelBlock`, `PaginationLink`, `PaginationPrevious`,
`PaginationNext`, `Tab`) accept a `to` prop that takes **any `Routable` value
or `NavigationTarget`** directly — no `Some(...)` and no `.into()` required:

```rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")] Home {},
    #[route("/devices")] DeviceList {},
}

rsx! {
    Menu {
        MenuList {
            MenuItem { to: Route::Home {},        "Home" }
            MenuItem { to: Route::DeviceList {},  "Devices" }
        }
    }
}
```

This works because the `to` prop is typed as
[`MaybeNav`](src/router_helpers.rs) with `#[props(into)]`. If you're working
without the `router` feature, the `to` prop simply isn't present — use the
`href: Option<String>` prop on the same components instead.

### 7. Event handlers are `Option<EventHandler<...>>`

Pass closures directly; Dioxus builders accept `move |evt| { ... }` thanks to
its `SuperFrom` trait:

```rust
Button { onclick: move |_| println!("clicked"), "Click" }
Input  { oninput: move |evt: FormEvent| value.set(evt.value()), value: value() }
```

You generally don't need `EventHandler::new(...)`.

### 8. Children are required where they make sense

In Dioxus 0.7, `pub children: Element` is **required**. If a component has a
`children` field, you must pass either content or `{}`:

```rust
BulmaBox { } // ❌ fails to compile (missing children)
BulmaBox { "" } // ✅ empty children
BulmaBox { "Hello" } // ✅
```

A small number of components (`Input`, `Textarea`, `Delete`) are leaf
components without children.

### 9. Don't reinvent components

Before writing a raw `<div class="...">` element, check whether a
`dioxus-bulma` component already exists. The list above is exhaustive for
Bulma's official components and covers ~all common UI needs.

---

## Building, testing, examples

```bash
cargo build                          # default features
cargo build --features web           # for web targets
cargo build --features router        # router-enabled components
cargo test                           # runs all tests including compile-only
cargo test --features router         # also exercises router test
cargo run --example demo --features web
```

Tests of interest:

- `tests/api_test.rs` — top-level API smoke tests.
- `tests/prelude_test.rs` — verifies the prelude exports.
- `tests/id_prop_test.rs` — verifies `id` works on representative components.
- `tests/router_to_prop_test.rs` — verifies `to: Route::Variant {}` compiles
  without `Some(...).into()`.

---

## When in doubt

- The most authoritative source is the components themselves under
  [`src/components/`](src/components/). Each file is small (often <200 lines)
  and shows exactly which props the component supports.
- The prelude in [`src/prelude.rs`](src/prelude.rs) is the canonical export
  list.
- `examples/demo.rs` shows almost every component in use.
- For Bulma styling questions, the official docs at <https://bulma.io/> are
  the source of truth — `dioxus-bulma` aims to be a 1:1 typed wrapper.
