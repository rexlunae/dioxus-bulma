use dioxus::prelude::*;
use crate::theme::BulmaSize;
use crate::utils::build_class;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BreadcrumbSeparator {
    Arrow,
    Bullet,
    Dot,
    Succeeds,
}

impl BreadcrumbSeparator {
    pub fn as_str(&self) -> &'static str {
        match self {
            BreadcrumbSeparator::Arrow => "has-arrow-separator",
            BreadcrumbSeparator::Bullet => "has-bullet-separator",
            BreadcrumbSeparator::Dot => "has-dot-separator",
            BreadcrumbSeparator::Succeeds => "has-succeeds-separator",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BreadcrumbAlignment {
    Left,
    Centered,
    Right,
}

impl BreadcrumbAlignment {
    pub fn as_str(&self) -> &'static str {
        match self {
            BreadcrumbAlignment::Left => "",
            BreadcrumbAlignment::Centered => "is-centered",
            BreadcrumbAlignment::Right => "is-right",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbProps {
    #[props(default)]
    pub size: Option<BulmaSize>,
    #[props(default)]
    pub alignment: Option<BreadcrumbAlignment>,
    #[props(default)]
    pub separator: Option<BreadcrumbSeparator>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    let size = props.size.unwrap_or_default();
    let alignment = props.alignment.unwrap_or(BreadcrumbAlignment::Left);
    
    let base_classes = vec!["breadcrumb"];
    let optional_classes = vec![
        if size != BulmaSize::Normal { Some(size.as_class().to_string()) } else { None },
        if alignment != BreadcrumbAlignment::Left { Some(alignment.as_str().to_string()) } else { None },
        props.separator.map(|s| s.as_str().to_string()),
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let breadcrumb_style = props.style.as_deref().unwrap_or("");

    rsx! {
        nav {
            class: "{final_class}",
            style: "{breadcrumb_style}",
            "aria-label": "breadcrumbs",
            ul {
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbItemProps {
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
pub fn BreadcrumbItem(props: BreadcrumbItemProps) -> Element {
    let active = props.active.unwrap_or(false);
    
    let base_classes = vec![];
    let optional_classes = vec![
        if active { Some("is-active".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let item_style = props.style.as_deref().unwrap_or("");

    rsx! {
        li {
            class: "{final_class}",
            style: "{item_style}",
            if let Some(href) = props.href {
                a {
                    href: "{href}",
                    onclick: move |evt| {
                        if let Some(handler) = &props.onclick {
                            handler.call(evt);
                        }
                    },
                    {props.children}
                }
            } else {
                span {
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
