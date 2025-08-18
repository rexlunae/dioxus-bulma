use dioxus::prelude::*;
use crate::utils::build_class;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileSize {
    Is1,
    Is2,
    Is3,
    Is4,
    Is5,
    Is6,
    Is7,
    Is8,
    Is9,
    Is10,
    Is11,
    Is12,
}

impl TileSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            TileSize::Is1 => "is-1",
            TileSize::Is2 => "is-2",
            TileSize::Is3 => "is-3",
            TileSize::Is4 => "is-4",
            TileSize::Is5 => "is-5",
            TileSize::Is6 => "is-6",
            TileSize::Is7 => "is-7",
            TileSize::Is8 => "is-8",
            TileSize::Is9 => "is-9",
            TileSize::Is10 => "is-10",
            TileSize::Is11 => "is-11",
            TileSize::Is12 => "is-12",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TileProps {
    #[props(default)]
    pub ancestor: Option<bool>,
    #[props(default)]
    pub parent: Option<bool>,
    #[props(default)]
    pub child: Option<bool>,
    #[props(default)]
    pub vertical: Option<bool>,
    #[props(default)]
    pub size: Option<TileSize>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Tile(props: TileProps) -> Element {
    let ancestor = props.ancestor.unwrap_or(false);
    let parent = props.parent.unwrap_or(false);
    let child = props.child.unwrap_or(false);
    let vertical = props.vertical.unwrap_or(false);
    
    let base_classes = vec!["tile"];
    let optional_classes = vec![
        if ancestor { Some("is-ancestor".to_string()) } else { None },
        if parent { Some("is-parent".to_string()) } else { None },
        if child { Some("is-child".to_string()) } else { None },
        if vertical { Some("is-vertical".to_string()) } else { None },
        props.size.map(|s| s.as_str().to_string()),
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let tile_style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{tile_style}",
            {props.children}
        }
    }
}
