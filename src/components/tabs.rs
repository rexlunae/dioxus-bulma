use crate::theme::BulmaSize;
use crate::utils::build_class;
use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TabsStyle {
    Default,
    Boxed,
    Toggle,
    ToggleRounded,
}

impl TabsStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            TabsStyle::Default => "",
            TabsStyle::Boxed => "is-boxed",
            TabsStyle::Toggle => "is-toggle",
            TabsStyle::ToggleRounded => "is-toggle is-toggle-rounded",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TabsAlignment {
    Left,
    Centered,
    Right,
}

impl TabsAlignment {
    pub fn as_str(&self) -> &'static str {
        match self {
            TabsAlignment::Left => "",
            TabsAlignment::Centered => "is-centered",
            TabsAlignment::Right => "is-right",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    #[props(default)]
    pub size: Option<BulmaSize>,
    #[props(default)]
    pub style: Option<TabsStyle>,
    #[props(default)]
    pub alignment: Option<TabsAlignment>,
    #[props(default)]
    pub fullwidth: Option<bool>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub css_style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let size = props.size.unwrap_or_default();
    let style = props.style.unwrap_or(TabsStyle::Default);
    let alignment = props.alignment.unwrap_or(TabsAlignment::Left);
    let fullwidth = props.fullwidth.unwrap_or(false);
    
    let base_classes = vec!["tabs"];
    let optional_classes = vec![
        if size != BulmaSize::Normal { Some(size.as_class().to_string()) } else { None },
        if style != TabsStyle::Default { 
            if style == TabsStyle::ToggleRounded {
                Some("is-toggle is-toggle-rounded".to_string())
            } else {
                Some(style.as_str().to_string())
            }
        } else { None },
        if alignment != TabsAlignment::Left { Some(alignment.as_str().to_string()) } else { None },
        if fullwidth { Some("is-fullwidth".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let tabs_style = props.css_style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{tabs_style}",
            ul {
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TabProps {
    #[props(default)]
    pub active: Option<bool>,
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
pub fn Tab(props: TabProps) -> Element {
    let active = props.active.unwrap_or(false);
    let disabled = props.disabled.unwrap_or(false);
    
    let base_classes = vec![];
    let optional_classes = vec![
        if active { Some("is-active".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let tab_style = props.style.as_deref().unwrap_or("");

    rsx! {
        li {
            class: "{final_class}",
            style: "{tab_style}",
            
            if let Some(href) = props.href {
                if !disabled {
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
                    span { {props.children} }
                }
            } else {
                a {
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
}
