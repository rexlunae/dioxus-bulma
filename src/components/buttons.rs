use dioxus::prelude::*;
use crate::theme::BulmaSize;
use crate::utils::build_class;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonsAlignment {
    Left,
    Centered,
    Right,
}

impl ButtonsAlignment {
    pub fn as_class(&self) -> &'static str {
        match self {
            ButtonsAlignment::Left => "",
            ButtonsAlignment::Centered => "is-centered",
            ButtonsAlignment::Right => "is-right",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonsProps {
    #[props(default)]
    pub size: Option<BulmaSize>,
    #[props(default)]
    pub alignment: Option<ButtonsAlignment>,
    #[props(default)]
    pub addons: Option<bool>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Buttons(props: ButtonsProps) -> Element {
    let size = props.size.unwrap_or_default();
    let alignment = props.alignment.unwrap_or(ButtonsAlignment::Left);
    let addons = props.addons.unwrap_or(false);

    let base_classes = vec!["buttons"];

    let optional_classes = vec![
        if size != BulmaSize::Normal { Some(size.as_class().to_string()) } else { None },
        if alignment != ButtonsAlignment::Left { Some(alignment.as_class().to_string()) } else { None },
        if addons { Some("has-addons".to_string()) } else { None },
        props.class.clone(),
    ];

    let final_class = build_class(&base_classes, &optional_classes);
    let buttons_style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{buttons_style}",
            {props.children}
        }
    }
}