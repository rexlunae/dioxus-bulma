use dioxus::prelude::*;
use crate::theme::BulmaSize;
use crate::utils::build_class;

#[cfg(feature = "router")]
use dioxus_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PaginationAlignment {
    Left,
    Centered,
    Right,
}

impl PaginationAlignment {
    pub fn as_str(&self) -> &'static str {
        match self {
            PaginationAlignment::Left => "",
            PaginationAlignment::Centered => "is-centered",
            PaginationAlignment::Right => "is-right",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PaginationProps {
    #[props(default)]
    pub size: Option<BulmaSize>,
    #[props(default)]
    pub alignment: Option<PaginationAlignment>,
    #[props(default)]
    pub rounded: Option<bool>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Pagination(props: PaginationProps) -> Element {
    let size = props.size.unwrap_or_default();
    let alignment = props.alignment.unwrap_or(PaginationAlignment::Left);
    let rounded = props.rounded.unwrap_or(false);
    
    let base_classes = vec!["pagination"];
    let optional_classes = vec![
        if size != BulmaSize::Normal { Some(size.as_class().to_string()) } else { None },
        if alignment != PaginationAlignment::Left { Some(alignment.as_str().to_string()) } else { None },
        if rounded { Some("is-rounded".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let pagination_style = props.style.as_deref().unwrap_or("");

    rsx! {
        nav {
            class: "{final_class}",
            style: "{pagination_style}",
            role: "navigation",
            "aria-label": "pagination",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PaginationPreviousProps {
    #[props(default)]
    pub disabled: Option<bool>,
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
pub fn PaginationPrevious(props: PaginationPreviousProps) -> Element {
    let disabled = props.disabled.unwrap_or(false);
    
    let base_classes = vec!["pagination-previous"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let style = props.style.as_deref().unwrap_or("");

    #[cfg(feature = "router")]
    if let Some(nav_target) = props.to {
        return rsx! {
            Link {
                to: nav_target,
                class: "{final_class}",
                style: "{style}",
                onclick: move |evt| {
                    if !disabled {
                        if let Some(handler) = &props.onclick {
                            handler.call(evt);
                        }
                    }
                },
                {props.children}
            }
        };
    }

    rsx! {
        a {
            class: "{final_class}",
            style: "{style}",
            href: if disabled { None } else { props.href.as_deref() },
            onclick: move |evt| {
                if !disabled {
                    if let Some(handler) = &props.onclick {
                        handler.call(evt);
                    }
                }
            },
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PaginationNextProps {
    #[props(default)]
    pub disabled: Option<bool>,
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
pub fn PaginationNext(props: PaginationNextProps) -> Element {
    let disabled = props.disabled.unwrap_or(false);
    
    let base_classes = vec!["pagination-next"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let style = props.style.as_deref().unwrap_or("");

    #[cfg(feature = "router")]
    if let Some(nav_target) = props.to {
        return rsx! {
            Link {
                to: nav_target,
                class: "{final_class}",
                style: "{style}",
                onclick: move |evt| {
                    if !disabled {
                        if let Some(handler) = &props.onclick {
                            handler.call(evt);
                        }
                    }
                },
                {props.children}
            }
        };
    }

    rsx! {
        a {
            class: "{final_class}",
            style: "{style}",
            href: if disabled { None } else { props.href.as_deref() },
            onclick: move |evt| {
                if !disabled {
                    if let Some(handler) = &props.onclick {
                        handler.call(evt);
                    }
                }
            },
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PaginationListProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn PaginationList(props: PaginationListProps) -> Element {
    let base_classes = vec!["pagination-list"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let style = props.style.as_deref().unwrap_or("");

    rsx! {
        ul {
            class: "{final_class}",
            style: "{style}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PaginationLinkProps {
    #[props(default)]
    pub current: Option<bool>,
    #[props(default)]
    pub disabled: Option<bool>,
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
pub fn PaginationLink(props: PaginationLinkProps) -> Element {
    let current = props.current.unwrap_or(false);
    let disabled = props.disabled.unwrap_or(false);
    
    let base_classes = vec!["pagination-link"];
    let optional_classes = vec![
        if current { Some("is-current".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let style = props.style.as_deref().unwrap_or("");

    #[cfg(feature = "router")]
    if let Some(nav_target) = props.to {
        return rsx! {
            li {
                Link {
                    to: nav_target,
                    class: "{final_class}",
                    style: "{style}",
                    "aria-current": if current { "page" } else { "" },
                    onclick: move |evt| {
                        if !disabled {
                            if let Some(handler) = &props.onclick {
                                handler.call(evt);
                            }
                        }
                    },
                    {props.children}
                }
            }
        };
    }

    rsx! {
        li {
            a {
                class: "{final_class}",
                style: "{style}",
                href: if disabled { None } else { props.href.as_deref() },
                "aria-current": if current { "page" } else { "" },
                onclick: move |evt| {
                    if !disabled {
                        if let Some(handler) = &props.onclick {
                            handler.call(evt);
                        }
                    }
                },
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PaginationEllipsisProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
}

#[component]
pub fn PaginationEllipsis(props: PaginationEllipsisProps) -> Element {
    let base_classes = vec!["pagination-ellipsis"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let style = props.style.as_deref().unwrap_or("");

    rsx! {
        li {
            span {
                class: "{final_class}",
                style: "{style}",
                "…"
            }
        }
    }
}
