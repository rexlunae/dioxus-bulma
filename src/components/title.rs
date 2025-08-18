use dioxus::prelude::*;
use crate::utils::build_class;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TitleSize {
    Is1,
    Is2,
    Is3,
    Is4,
    Is5,
    Is6,
}

impl TitleSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            TitleSize::Is1 => "is-1",
            TitleSize::Is2 => "is-2",
            TitleSize::Is3 => "is-3",
            TitleSize::Is4 => "is-4",
            TitleSize::Is5 => "is-5",
            TitleSize::Is6 => "is-6",
        }
    }
    
    pub fn as_tag(&self) -> &'static str {
        match self {
            TitleSize::Is1 => "h1",
            TitleSize::Is2 => "h2",
            TitleSize::Is3 => "h3",
            TitleSize::Is4 => "h4",
            TitleSize::Is5 => "h5",
            TitleSize::Is6 => "h6",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TitleProps {
    #[props(default = TitleSize::Is3)]
    pub size: TitleSize,
    #[props(default)]
    pub spaced: Option<bool>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Title(props: TitleProps) -> Element {
    let spaced = props.spaced.unwrap_or(false);
    
    let base_classes = vec!["title"];
    
    let optional_classes = vec![
        Some(props.size.as_str().to_string()),
        if spaced { Some("is-spaced".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let title_style = props.style.as_deref().unwrap_or("");

    match props.size.as_tag() {
        "h1" => rsx! {
            h1 {
                class: "{final_class}",
                style: "{title_style}",
                {props.children}
            }
        },
        "h2" => rsx! {
            h2 {
                class: "{final_class}",
                style: "{title_style}",
                {props.children}
            }
        },
        "h3" => rsx! {
            h3 {
                class: "{final_class}",
                style: "{title_style}",
                {props.children}
            }
        },
        "h4" => rsx! {
            h4 {
                class: "{final_class}",
                style: "{title_style}",
                {props.children}
            }
        },
        "h5" => rsx! {
            h5 {
                class: "{final_class}",
                style: "{title_style}",
                {props.children}
            }
        },
        _ => rsx! {
            h6 {
                class: "{final_class}",
                style: "{title_style}",
                {props.children}
            }
        },
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SubtitleProps {
    #[props(default = TitleSize::Is5)]
    pub size: TitleSize,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Subtitle(props: SubtitleProps) -> Element {
    let base_classes = vec!["subtitle"];
    
    let optional_classes = vec![
        Some(props.size.as_str().to_string()),
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let subtitle_style = props.style.as_deref().unwrap_or("");

    match props.size.as_tag() {
        "h1" => rsx! {
            h1 {
                class: "{final_class}",
                style: "{subtitle_style}",
                {props.children}
            }
        },
        "h2" => rsx! {
            h2 {
                class: "{final_class}",
                style: "{subtitle_style}",
                {props.children}
            }
        },
        "h3" => rsx! {
            h3 {
                class: "{final_class}",
                style: "{subtitle_style}",
                {props.children}
            }
        },
        "h4" => rsx! {
            h4 {
                class: "{final_class}",
                style: "{subtitle_style}",
                {props.children}
            }
        },
        "h5" => rsx! {
            h5 {
                class: "{final_class}",
                style: "{subtitle_style}",
                {props.children}
            }
        },
        _ => rsx! {
            h6 {
                class: "{final_class}",
                style: "{subtitle_style}",
                {props.children}
            }
        },
    }
}