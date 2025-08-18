use dioxus::prelude::*;
use crate::theme::BulmaSize;
use crate::utils::build_class;

#[derive(Props, Clone, PartialEq)]
pub struct ContentProps {
    #[props(default)]
    pub size: Option<BulmaSize>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Content(props: ContentProps) -> Element {
    let size = props.size.unwrap_or_default();
    
    let base_classes = vec!["content"];
    let optional_classes = vec![
        if size != BulmaSize::Normal { Some(size.as_class().to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let content_style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{content_style}",
            {props.children}
        }
    }
}
