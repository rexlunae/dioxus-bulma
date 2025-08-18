use dioxus::prelude::*;
use crate::theme::{BulmaColor, BulmaSize};
use crate::utils::build_class;

#[derive(Props, Clone, PartialEq)]
pub struct TagProps {
    #[props(default)]
    pub color: Option<BulmaColor>,
    #[props(default)]
    pub size: Option<BulmaSize>,
    #[props(default)]
    pub light: Option<bool>,
    #[props(default)]
    pub rounded: Option<bool>,
    #[props(default)]
    pub delete: Option<bool>,
    #[props(default)]
    pub ondelete: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Tag(props: TagProps) -> Element {
    let light = props.light.unwrap_or(false);
    let rounded = props.rounded.unwrap_or(false);
    let delete = props.delete.unwrap_or(false);
    let size = props.size.unwrap_or_default();
    
    let base_classes = vec!["tag"];
    let optional_classes = vec![
        props.color.map(|c| format!("is-{}", c.as_str())),
        if size != BulmaSize::Normal { Some(size.as_class().to_string()) } else { None },
        if light { Some("is-light".to_string()) } else { None },
        if rounded { Some("is-rounded".to_string()) } else { None },
        if delete { Some("is-delete".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let tag_style = props.style.as_deref().unwrap_or("");

    if delete {
        rsx! {
            button {
                class: "{final_class}",
                style: "{tag_style}",
                onclick: move |evt| {
                    if let Some(handler) = &props.ondelete {
                        handler.call(evt);
                    }
                }
            }
        }
    } else {
        rsx! {
            span {
                class: "{final_class}",
                style: "{tag_style}",
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TagsProps {
    #[props(default)]
    pub addons: Option<bool>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Tags(props: TagsProps) -> Element {
    let addons = props.addons.unwrap_or(false);
    
    let base_classes = vec!["tags"];
    let optional_classes = vec![
        if addons { Some("has-addons".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let tags_style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{tags_style}",
            {props.children}
        }
    }
}
