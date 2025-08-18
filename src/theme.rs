use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BulmaTheme {
    Auto,
    Light,
    Dark,
}

impl Default for BulmaTheme {
    fn default() -> Self {
        BulmaTheme::Auto
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BulmaColor {
    White,
    Light,
    Dark,
    Black,
    Text,
    Ghost,
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BulmaSize {
    Small,
    Normal,
    Medium,
    Large,
}

impl Default for BulmaSize {
    fn default() -> Self {
        BulmaSize::Normal
    }
}

impl BulmaColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            BulmaColor::White => "white",
            BulmaColor::Light => "light",
            BulmaColor::Dark => "dark",
            BulmaColor::Black => "black",
            BulmaColor::Text => "text",
            BulmaColor::Ghost => "ghost",
            BulmaColor::Primary => "primary",
            BulmaColor::Link => "link",
            BulmaColor::Info => "info",
            BulmaColor::Success => "success",
            BulmaColor::Warning => "warning",
            BulmaColor::Danger => "danger",
        }
    }
}

impl BulmaSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            BulmaSize::Small => "small",
            BulmaSize::Normal => "normal",
            BulmaSize::Medium => "medium",
            BulmaSize::Large => "large",
        }
    }
    
    pub fn as_class(&self) -> &'static str {
        match self {
            BulmaSize::Small => "is-small",
            BulmaSize::Normal => "",
            BulmaSize::Medium => "is-medium", 
            BulmaSize::Large => "is-large",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct BulmaProviderProps {
    #[props(default)]
    pub theme: Option<BulmaTheme>,
    #[props(default = true)]
    pub load_bulma_css: bool,
    pub children: Element,
}

#[component]
pub fn BulmaProvider(props: BulmaProviderProps) -> Element {
    let theme_mode = props.theme.unwrap_or_default();
    
    let theme_class = match theme_mode {
        BulmaTheme::Auto => "theme-auto",
        BulmaTheme::Light => "theme-light", 
        BulmaTheme::Dark => "theme-dark",
    };
    
    rsx! {
        if props.load_bulma_css {
            document::Link {
                rel: "stylesheet",
                href: "https://cdn.jsdelivr.net/npm/bulma@1.0.0/css/bulma.min.css"
            }
        }
        div { 
            class: "{theme_class}",
            style: "min-height: 100vh;",
            {props.children}
        }
    }
}