use dioxus::prelude::*;
use crate::utils::build_class;

#[derive(Props, Clone, PartialEq)]
pub struct MediaProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Media(props: MediaProps) -> Element {
    let base_classes = vec!["media"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let media_style = props.style.as_deref().unwrap_or("");

    rsx! {
        article {
            class: "{final_class}",
            style: "{media_style}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MediaLeftProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn MediaLeft(props: MediaLeftProps) -> Element {
    let base_classes = vec!["media-left"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{style}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MediaRightProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn MediaRight(props: MediaRightProps) -> Element {
    let base_classes = vec!["media-right"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{style}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MediaContentProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn MediaContent(props: MediaContentProps) -> Element {
    let base_classes = vec!["media-content"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{style}",
            {props.children}
        }
    }
}
