use dioxus::prelude::*;
use crate::theme::{BulmaColor, BulmaSize};
use crate::utils::build_class;

#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {
    #[props(default)]
    pub value: Option<f32>,
    #[props(default = 100.0)]
    pub max: f32,
    #[props(default)]
    pub color: Option<BulmaColor>,
    #[props(default)]
    pub size: Option<BulmaSize>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Progress(props: ProgressProps) -> Element {
    let size = props.size.unwrap_or_default();

    let base_classes = vec!["progress"];
    let optional_classes = vec![
        props.color.map(|c| format!("is-{}", c.as_str())),
        if size != BulmaSize::Normal { Some(size.as_class().to_string()) } else { None },
        props.class.clone(),
    ];

    let final_class = build_class(&base_classes, &optional_classes);
    let progress_style = props.style.as_deref().unwrap_or("");
    let value_str = props.value.map(|v| v.to_string());
    let max_str = props.max.to_string();

    rsx! {
        progress {
            class: "{final_class}",
            style: "{progress_style}",
            value: value_str.as_deref(),
            max: "{max_str}",
            if props.value.is_some() {
                {props.children}
            } else if let Some(value) = props.value {
                "{value}%"
            }
        }
    }
}
