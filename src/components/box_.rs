use dioxus::prelude::*;
use crate::utils::build_class;

/// A Bulma `Box` element — a simple white-on-grey container with a subtle border-radius and shadow.
///
/// See <https://bulma.io/documentation/elements/box/>.
///
/// Note: the type is named `BulmaBox` because `Box` collides with Rust's built-in
/// `Box<T>` smart pointer. The component is also re-exported as `Box` for convenience
/// in modules where `std::boxed::Box` is not used directly; prefer `BulmaBox` in new code
/// to avoid ambiguity.
#[derive(Props, Clone, PartialEq)]
pub struct BulmaBoxProps {
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub id: Option<String>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn BulmaBox(props: BulmaBoxProps) -> Element {
    let base_classes = vec!["box"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let box_style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{box_style}",
            id: props.id.clone(),
            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            },
            {props.children}
        }
    }
}

/// A Bulma `Block` element — a simple spacer block used to add consistent vertical
/// rhythm between elements. See <https://bulma.io/documentation/elements/block/>.
#[derive(Props, Clone, PartialEq)]
pub struct BlockProps {
    #[props(default)]
    pub id: Option<String>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Block(props: BlockProps) -> Element {
    let base_classes = vec!["block"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let block_style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{block_style}",
            id: props.id.clone(),
            {props.children}
        }
    }
}
