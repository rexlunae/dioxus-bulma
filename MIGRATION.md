# Migration Guide: Dioxus 0.6 to 0.7

This guide walks you through upgrading your dioxus-bulma application from Dioxus 0.6 to Dioxus 0.7.

## Overview of Changes

Dioxus 0.7 introduces several breaking changes that affect how components and hooks work. This library has been updated to be fully compatible with Dioxus 0.7.

## Step-by-Step Upgrade

### Step 1: Update Dependencies

Update your `Cargo.toml` to use Dioxus 0.7:

```toml
[dependencies]
dioxus = "0.7"
dioxus-bulma = "0.7"
dioxus-router = "0.7"  # if using router feature
manganis = "0.7"       # if using asset management
```

### Step 2: Update Imports

The most important change is how you import components from dioxus-bulma.

**Before (Dioxus 0.6):**

```rust
use dioxus::prelude::*;
use dioxus_bulma::*;
```

**After (Dioxus 0.7):**

```rust
use dioxus::prelude::*;
use dioxus_bulma::prelude::*;
use dioxus_bulma::components::{Title, Subtitle};  // Explicit imports required
```

### Why the Import Change?

Dioxus 0.7 introduced built-in document components (`Title`, `Subtitle`) that would conflict with the dioxus-bulma styled components of the same name. By requiring explicit imports of these components, we ensure you get the Bulma-styled versions when you import them.

### Step 3: Update Hook Usage

Dioxus 0.7 renamed the `use_state` hook to `use_signal`.

**Before:**

```rust
let mut count = use_state(|| 0);
let mut name = use_state(String::new);
```

**After:**

```rust
let mut count = use_signal(|| 0);
let mut name = use_signal(String::new);
```

### Step 4: Update Component Children Props

In Dioxus 0.7, component children are required, not optional. If you've created custom components with children, update their Props:

**Before:**

```rust
#[derive(Props, Clone, PartialEq)]
pub struct MyComponentProps {
    #[props(default)]
    pub children: Option<Element>,
}

#[component]
fn MyComponent(props: MyComponentProps) -> Element {
    match props.children {
        Some(children) => rsx! { div { {children} } },
        None => rsx! { div { "No content" } },
    }
}
```

**After:**

```rust
#[derive(Props, Clone, PartialEq)]
pub struct MyComponentProps {
    pub children: Element,  // Required, no default
}

#[component]
fn MyComponent(props: MyComponentProps) -> Element {
    rsx! {
        div {
            {props.children}
        }
    }
}
```

### Step 5: Update Signal/State Patterns

When accessing signal values, the syntax remains the same, but signals are now the primary way to manage state:

```rust
// Reading values
let current_value = *count.read();
// or use the deref shorthand
let current_value = *count;

// Writing values
count.set(count() + 1);
// or
count.with_mut(|v| *v += 1);
```

## Common Issues and Solutions

### Issue: Title/Subtitle Not Found

**Problem:**

```
error[E0599]: no method named `size` found for struct `dioxus::dioxus_document::TitlePropsBuilder`
```

**Solution:** Make sure you're explicitly importing the Bulma Title and Subtitle:

```rust
use dioxus_bulma::components::{Title, Subtitle};
```

### Issue: use_state Not Available

**Problem:**

```
error[E0425]: cannot find value `use_state` in this scope
```

**Solution:** Replace `use_state` with `use_signal`:

```rust
// Change this:
let mut count = use_state(|| 0);

// To this:
let mut count = use_signal(|| 0);
```

### Issue: Children Field Errors

**Problem:**

```
error[E0308]: mismatched types
expected enum `Option<Result<VNode, RenderError>>`
   found enum `Result<VNode, RenderError>`
```

**Solution:** Remove `Option` wrapper and `#[props(default)]` from children:

```rust
// Change this:
#[props(default)]
pub children: Option<Element>,

// To this:
pub children: Element,
```

## Example: Complete Migration

Here's a complete example showing a Dioxus 0.6 component migrated to Dioxus 0.7:

### Before (Dioxus 0.6):

```rust
use dioxus::prelude::*;
use dioxus_bulma::*;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let mut count = use_state(|| 0);

    rsx! {
        BulmaProvider {
            theme: BulmaTheme::Auto,
            load_bulma_css: true,

            Section {
                Container {
                    Title { size: TitleSize::Is1, "Counter App" }

                    div { class: "mb-3",
                        "Count: {count}"
                    }

                    Button {
                        color: BulmaColor::Primary,
                        onclick: |_| {
                            count += 1;
                        },
                        "Increment"
                    }
                }
            }
        }
    }
}
```

### After (Dioxus 0.7):

```rust
use dioxus::prelude::*;
use dioxus_bulma::prelude::*;
use dioxus_bulma::components::{Title, Subtitle};

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        BulmaProvider {
            theme: BulmaTheme::Auto,
            load_bulma_css: true,

            Section {
                Container {
                    Title { size: TitleSize::Is1, "Counter App" }

                    div { class: "mb-3",
                        "Count: {count}"
                    }

                    Button {
                        color: BulmaColor::Primary,
                        "Increment"
                    }
                }
            }
        }
    }
}
```

## Additional Resources

- [Dioxus 0.7 Release Notes](https://github.com/dioxuslabs/dioxus/releases/tag/v0.7.0)
- [Dioxus Signals Documentation](https://dioxuslabs.com/learn/0.7/guide/managing_state/)
- [Dioxus Components Guide](https://dioxuslabs.com/learn/0.7/guide/describing_ui)
- [dioxus-bulma Examples](https://github.com/rexlunae/dioxus-bulma/tree/main/examples)

## Need Help?

If you encounter issues during migration:

1. Check the [Troubleshooting](#common-issues-and-solutions) section above
2. Review the [example code](examples/) in the repository
3. Check the [README](README.md) for import patterns
4. Open an issue on GitHub with a minimal reproducible example

## Summary of Key Changes

| Aspect | Dioxus 0.6 | Dioxus 0.7 |
|--------|-----------|-----------|
| State Hook | `use_state` | `use_signal` |
| Import Pattern | `use dioxus_bulma::*` | `use dioxus_bulma::prelude::*` + explicit Title/Subtitle imports |
| Children Prop | `Option<Element>` | `Element` (required) |
| Component Definition | `#[component]` (same) | `#[component]` (same) |
| Props Derive | `#[derive(Props, Clone, PartialEq)]` | `#[derive(Props, Clone, PartialEq)]` (same) |
| Event Handlers | `Option<EventHandler<T>>` | `Option<EventHandler<T>>` (same) |
