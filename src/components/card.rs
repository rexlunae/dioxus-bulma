use dioxus::prelude::*;
use crate::utils::build_class;

#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    let base_classes = vec!["card"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let card_style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{card_style}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardHeaderProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    let base_classes = vec!["card-header"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let style = props.style.as_deref().unwrap_or("");

    rsx! {
        header {
            class: "{final_class}",
            style: "{style}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardHeaderTitleProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn CardHeaderTitle(props: CardHeaderTitleProps) -> Element {
    let base_classes = vec!["card-header-title"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let style = props.style.as_deref().unwrap_or("");

    rsx! {
        p {
            class: "{final_class}",
            style: "{style}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardContentProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn CardContent(props: CardContentProps) -> Element {
    let base_classes = vec!["card-content"];
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
pub struct CardFooterProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn CardFooter(props: CardFooterProps) -> Element {
    let base_classes = vec!["card-footer"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let style = props.style.as_deref().unwrap_or("");

    rsx! {
        footer {
            class: "{final_class}",
            style: "{style}",
            {props.children}
        }
    }
}
