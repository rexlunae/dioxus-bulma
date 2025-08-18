use dioxus::prelude::*;
use crate::utils::build_class;

#[derive(Props, Clone, PartialEq)]
pub struct LevelProps {
    #[props(default)]
    pub mobile: Option<bool>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Level(props: LevelProps) -> Element {
    let mobile = props.mobile.unwrap_or(false);
    
    let base_classes = vec!["level"];
    let optional_classes = vec![
        if mobile { Some("is-mobile".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let level_style = props.style.as_deref().unwrap_or("");

    rsx! {
        nav {
            class: "{final_class}",
            style: "{level_style}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct LevelLeftProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn LevelLeft(props: LevelLeftProps) -> Element {
    let base_classes = vec!["level-left"];
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
pub struct LevelRightProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn LevelRight(props: LevelRightProps) -> Element {
    let base_classes = vec!["level-right"];
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
pub struct LevelItemProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn LevelItem(props: LevelItemProps) -> Element {
    let base_classes = vec!["level-item"];
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
