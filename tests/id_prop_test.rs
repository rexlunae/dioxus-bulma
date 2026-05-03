//! Compile/run tests for the new `id` prop on every Bulma component.
//!
//! Verifies that `id: "..."` works on representative components from every
//! category. Since we use `BulmaTitle`/`BulmaSubtitle` aliases in the prelude
//! (see [`dioxus_bulma::prelude`]), this also doubles as a regression test
//! for [issue #7](https://github.com/rexlunae/dioxus-bulma/issues/7).
use dioxus::prelude::*;
use dioxus_bulma::components::Option as BulmaOption;
use dioxus_bulma::prelude::*;

#[allow(dead_code)]
fn _every_component_accepts_id() -> Element {
    rsx! {
        BulmaProvider {
            theme: BulmaTheme::Light,
            // Layout
            Container { id: "page-container", "x" }
            Section { id: "main-section", "x" }
            Hero { id: "hero", "x" }
            // Elements
            BulmaBox { id: "summary", "Box content" }
            Block { id: "spacer", "Block content" }
            BulmaTitle { id: "page-title", "Hi" }
            BulmaSubtitle { id: "page-subtitle", "Hello" }
            Button { id: "save-btn", "Save" }
            Notification { id: "alert", "x" }
            Tag { id: "badge", "x" }
            Icon { id: "ico", "x" }
            // Form
            Field {
                id: "f1",
                FieldLabel { "n" }
                Control { Input { id: "i1", placeholder: "x" } }
            }
            Textarea { id: "ta", placeholder: "x" }
            Select { id: "sel", BulmaOption { id: "opt1", value: "1", "1" } }
            // Components
            Card {
                id: "card",
                CardHeader { id: "ch", CardHeaderTitle { id: "cht", "Title" } }
                CardContent { id: "cc", "body" }
                CardFooter { id: "cf", CardFooterItem { id: "cfi", "ok" } }
            }
            Modal { id: "m", active: true, "Modal" }
            Navbar { id: "nav", "x" }
            Tabs { id: "tabs", "x" }
        }
    }
}

#[test]
fn id_prop_compiles_for_components() {
    assert!(true);
}
