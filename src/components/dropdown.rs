use dioxus::prelude::*;
use crate::utils::build_class;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DropdownTrigger {
    Hover,
    Click,
}

#[derive(Props, Clone, PartialEq)]
pub struct DropdownProps {
    #[props(default)]
    pub active: Option<bool>,
    #[props(default)]
    pub hoverable: Option<bool>,
    #[props(default)]
    pub right: Option<bool>,
    #[props(default)]
    pub up: Option<bool>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let active = props.active.unwrap_or(false);
    let hoverable = props.hoverable.unwrap_or(false);
    let right = props.right.unwrap_or(false);
    let up = props.up.unwrap_or(false);
    
    let base_classes = vec!["dropdown"];
    let optional_classes = vec![
        if active { Some("is-active".to_string()) } else { None },
        if hoverable { Some("is-hoverable".to_string()) } else { None },
        if right { Some("is-right".to_string()) } else { None },
        if up { Some("is-up".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let dropdown_style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{dropdown_style}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DropdownTriggerProps {
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn DropdownTrigger(props: DropdownTriggerProps) -> Element {
    let base_classes = vec!["dropdown-trigger"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let trigger_style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{trigger_style}",
            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            },
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    let base_classes = vec!["dropdown-menu"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let menu_style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{menu_style}",
            role: "menu",
            div {
                class: "dropdown-content",
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DropdownItemProps {
    #[props(default)]
    pub active: Option<bool>,
    #[props(default)]
    pub href: Option<String>,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn DropdownItem(props: DropdownItemProps) -> Element {
    let active = props.active.unwrap_or(false);
    
    let base_classes = vec!["dropdown-item"];
    let optional_classes = vec![
        if active { Some("is-active".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let item_style = props.style.as_deref().unwrap_or("");

    rsx! {
        if let Some(href) = props.href {
            a {
                class: "{final_class}",
                style: "{item_style}",
                href: "{href}",
                onclick: move |evt| {
                    if let Some(handler) = &props.onclick {
                        handler.call(evt);
                    }
                },
                {props.children}
            }
        } else {
            div {
                class: "{final_class}",
                style: "{item_style}",
                onclick: move |evt| {
                    if let Some(handler) = &props.onclick {
                        handler.call(evt);
                    }
                },
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DropdownDividerProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
}

#[component]
pub fn DropdownDivider(props: DropdownDividerProps) -> Element {
    let base_classes = vec!["dropdown-divider"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let divider_style = props.style.as_deref().unwrap_or("");

    rsx! {
        hr {
            class: "{final_class}",
            style: "{divider_style}"
        }
    }
}
