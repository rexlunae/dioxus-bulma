use crate::theme::BulmaColor;
use crate::utils::build_class;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PanelProps {
    #[props(default)]
    pub color: Option<BulmaColor>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Panel(props: PanelProps) -> Element {
    let base_classes = vec!["panel"];
    let optional_classes = vec![
        props.color.map(|c| format!("is-{}", c.as_str())),
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let panel_style = props.style.as_deref().unwrap_or("");

    rsx! {
        nav {
            class: "{final_class}",
            style: "{panel_style}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PanelHeadingProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn PanelHeading(props: PanelHeadingProps) -> Element {
    let base_classes = vec!["panel-heading"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let heading_style = props.style.as_deref().unwrap_or("");

    rsx! {
        p {
            class: "{final_class}",
            style: "{heading_style}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PanelTabsProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn PanelTabs(props: PanelTabsProps) -> Element {
    let base_classes = vec!["panel-tabs"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let tabs_style = props.style.as_deref().unwrap_or("");

    rsx! {
        p {
            class: "{final_class}",
            style: "{tabs_style}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PanelBlockProps {
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
pub fn PanelBlock(props: PanelBlockProps) -> Element {
    let active = props.active.unwrap_or(false);
    
    let base_classes = vec!["panel-block"];
    let optional_classes = vec![
        if active { Some("is-active".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let block_style = props.style.as_deref().unwrap_or("");

    #[cfg(feature = "router")]
    if let Some(nav_target) = props.to {
        return rsx! {
            Link {
                to: nav_target,
                class: "{final_class}",
                style: "{block_style}",
                onclick: move |evt| {
                    if let Some(handler) = &props.onclick {
                        handler.call(evt);
                    }
                },
                {props.children}
            }
        };
    }

    rsx! {
        if let Some(href) = props.href {
            a {
                class: "{final_class}",
                style: "{block_style}",
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
                style: "{block_style}",
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
pub struct PanelIconProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn PanelIcon(props: PanelIconProps) -> Element {
    let base_classes = vec!["panel-icon"];
    let optional_classes = vec![props.class.clone()];
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
