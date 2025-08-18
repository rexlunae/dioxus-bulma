use dioxus::prelude::*;
use crate::utils::build_class;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImageSize {
    Is16x16,
    Is24x24,
    Is32x32,
    Is48x48,
    Is64x64,
    Is96x96,
    Is128x128,
    IsSquare,
    Is1by1,
    Is5by4,
    Is4by3,
    Is3by2,
    Is5by3,
    Is16by9,
    Is2by1,
    Is3by1,
    Is4by5,
    Is3by4,
    Is2by3,
    Is3by5,
    Is9by16,
    Is1by2,
    Is1by3,
}

impl ImageSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            ImageSize::Is16x16 => "is-16x16",
            ImageSize::Is24x24 => "is-24x24",
            ImageSize::Is32x32 => "is-32x32",
            ImageSize::Is48x48 => "is-48x48",
            ImageSize::Is64x64 => "is-64x64",
            ImageSize::Is96x96 => "is-96x96",
            ImageSize::Is128x128 => "is-128x128",
            ImageSize::IsSquare => "is-square",
            ImageSize::Is1by1 => "is-1by1",
            ImageSize::Is5by4 => "is-5by4",
            ImageSize::Is4by3 => "is-4by3",
            ImageSize::Is3by2 => "is-3by2",
            ImageSize::Is5by3 => "is-5by3",
            ImageSize::Is16by9 => "is-16by9",
            ImageSize::Is2by1 => "is-2by1",
            ImageSize::Is3by1 => "is-3by1",
            ImageSize::Is4by5 => "is-4by5",
            ImageSize::Is3by4 => "is-3by4",
            ImageSize::Is2by3 => "is-2by3",
            ImageSize::Is3by5 => "is-3by5",
            ImageSize::Is9by16 => "is-9by16",
            ImageSize::Is1by2 => "is-1by2",
            ImageSize::Is1by3 => "is-1by3",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ImageProps {
    #[props(default)]
    pub size: Option<ImageSize>,
    #[props(default)]
    pub rounded: Option<bool>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Image(props: ImageProps) -> Element {
    let rounded = props.rounded.unwrap_or(false);
    
    let base_classes = vec!["image"];
    let optional_classes = vec![
        props.size.map(|s| s.as_str().to_string()),
        if rounded { Some("is-rounded".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let image_style = props.style.as_deref().unwrap_or("");

    rsx! {
        figure {
            class: "{final_class}",
            style: "{image_style}",
            {props.children}
        }
    }
}
