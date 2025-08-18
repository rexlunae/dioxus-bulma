use dioxus::prelude::*;
use crate::utils::build_class;

#[cfg(feature = "router")]
use dioxus_router::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MenuProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Menu(props: MenuProps) -> Element {
    let base_classes = vec!["menu"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let menu_style = props.style.as_deref().unwrap_or("");

    rsx! {
        aside {
            class: "{final_class}",
            style: "{menu_style}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MenuLabelProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn MenuLabel(props: MenuLabelProps) -> Element {
    let base_classes = vec!["menu-label"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let label_style = props.style.as_deref().unwrap_or("");

    rsx! {
        p {
            class: "{final_class}",
            style: "{label_style}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MenuListProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn MenuList(props: MenuListProps) -> Element {
    let base_classes = vec!["menu-list"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let list_style = props.style.as_deref().unwrap_or("");

    rsx! {
        ul {
            class: "{final_class}",
            style: "{list_style}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MenuItemProps {
    #[props(default)]
    pub active: Option<bool>,
    #[props(default)]
    pub href: Option<String>,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// If present, use router navigation instead of href
    #[cfg(feature = "router")]
    #[props(default)]
    pub to: Option<NavigationTarget>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn MenuItem(props: MenuItemProps) -> Element {
    let active = props.active.unwrap_or(false);
    
    let base_classes = vec![];
    let optional_classes = vec![
        if active { Some("is-active".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let item_style = props.style.as_deref().unwrap_or("");

    #[cfg(feature = "router")]
    if let Some(nav_target) = props.to {
        return rsx! {
            li {
                Link {
                    to: nav_target,
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
        };
    }

    rsx! {
        li {
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
                a {
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
}
