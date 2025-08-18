use dioxus::prelude::*;
use crate::theme::{BulmaColor, BulmaSize};
use crate::utils::build_class;

#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    #[props(default)]
    pub size: Option<BulmaSize>,
    #[props(default)]
    pub color: Option<BulmaColor>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Icon(props: IconProps) -> Element {
    let size = props.size.unwrap_or_default();
    
    let base_classes = vec!["icon"];
    let optional_classes = vec![
        if size != BulmaSize::Normal { Some(size.as_class().to_string()) } else { None },
        props.color.map(|c| format!("has-text-{}", c.as_str())),
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let icon_style = props.style.as_deref().unwrap_or("");

    rsx! {
        span {
            class: "{final_class}",
            style: "{icon_style}",
            {props.children}
        }
    }
}
