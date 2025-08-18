use dioxus::prelude::*;
use crate::theme::BulmaColor;
use crate::utils::build_class;

#[derive(Props, Clone, PartialEq)]
pub struct NavbarProps {
    #[props(default)]
    pub color: Option<BulmaColor>,
    #[props(default)]
    pub transparent: Option<bool>,
    #[props(default)]
    pub fixed_top: Option<bool>,
    #[props(default)]
    pub fixed_bottom: Option<bool>,
    #[props(default)]
    pub spaced: Option<bool>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Navbar(props: NavbarProps) -> Element {
    let transparent = props.transparent.unwrap_or(false);
    let fixed_top = props.fixed_top.unwrap_or(false);
    let fixed_bottom = props.fixed_bottom.unwrap_or(false);
    let spaced = props.spaced.unwrap_or(false);
    
    let base_classes = vec!["navbar"];
    let optional_classes = vec![
        props.color.map(|c| format!("is-{}", c.as_str())),
        if transparent { Some("is-transparent".to_string()) } else { None },
        if fixed_top { Some("is-fixed-top".to_string()) } else { None },
        if fixed_bottom { Some("is-fixed-bottom".to_string()) } else { None },
        if spaced { Some("is-spaced".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let navbar_style = props.style.as_deref().unwrap_or("");

    rsx! {
        nav {
            class: "{final_class}",
            style: "{navbar_style}",
            role: "navigation",
            "aria-label": "main navigation",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct NavbarBrandProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn NavbarBrand(props: NavbarBrandProps) -> Element {
    let base_classes = vec!["navbar-brand"];
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
pub struct NavbarMenuProps {
    #[props(default = false)]
    pub active: bool,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn NavbarMenu(props: NavbarMenuProps) -> Element {
    let base_classes = vec!["navbar-menu"];
    let optional_classes = vec![
        if props.active { Some("is-active".to_string()) } else { None },
        props.class.clone(),
    ];
    
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
pub struct NavbarStartProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn NavbarStart(props: NavbarStartProps) -> Element {
    let base_classes = vec!["navbar-start"];
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
pub struct NavbarEndProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn NavbarEnd(props: NavbarEndProps) -> Element {
    let base_classes = vec!["navbar-end"];
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
pub struct NavbarItemProps {
    #[props(default)]
    pub active: Option<bool>,
    #[props(default)]
    pub hoverable: Option<bool>,
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
pub fn NavbarItem(props: NavbarItemProps) -> Element {
    let active = props.active.unwrap_or(false);
    let hoverable = props.hoverable.unwrap_or(false);
    
    let base_classes = vec!["navbar-item"];
    let optional_classes = vec![
        if active { Some("is-active".to_string()) } else { None },
        if hoverable { Some("is-hoverable".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let style = props.style.as_deref().unwrap_or("");

    if let Some(href) = props.href {
        rsx! {
            a {
                class: "{final_class}",
                style: "{style}",
                href: "{href}",
                onclick: move |evt| {
                    if let Some(handler) = &props.onclick {
                        handler.call(evt);
                    }
                },
                {props.children}
            }
        }
    } else {
        rsx! {
            div {
                class: "{final_class}",
                style: "{style}",
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
