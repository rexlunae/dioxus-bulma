use dioxus::prelude::*;
use crate::theme::{BulmaColor, BulmaSize};
use crate::utils::build_class;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputType {
    Text,
    Password,
    Email,
    Tel,
}

impl InputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Password => "password",
            InputType::Email => "email",
            InputType::Tel => "tel",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    #[props(default = InputType::Text)]
    pub input_type: InputType,
    #[props(default)]
    pub value: Option<String>,
    #[props(default)]
    pub placeholder: Option<String>,
    #[props(default)]
    pub color: Option<BulmaColor>,
    #[props(default)]
    pub size: Option<BulmaSize>,
    #[props(default)]
    pub rounded: Option<bool>,
    #[props(default)]
    pub loading: Option<bool>,
    #[props(default)]
    pub disabled: Option<bool>,
    #[props(default)]
    pub readonly: Option<bool>,
    #[props(default)]
    pub focused: Option<bool>,
    #[props(default)]
    pub oninput: Option<EventHandler<FormEvent>>,
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,
    #[props(default)]
    pub onfocus: Option<EventHandler<FocusEvent>>,
    #[props(default)]
    pub onblur: Option<EventHandler<FocusEvent>>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    let size = props.size.unwrap_or_default();
    let rounded = props.rounded.unwrap_or(false);
    let loading = props.loading.unwrap_or(false);
    let disabled = props.disabled.unwrap_or(false);
    let readonly = props.readonly.unwrap_or(false);
    let focused = props.focused.unwrap_or(false);
    
    let base_classes = vec!["input"];
    
    let optional_classes = vec![
        props.color.map(|c| format!("is-{}", c.as_str())),
        if size != BulmaSize::Normal { Some(size.as_class().to_string()) } else { None },
        if rounded { Some("is-rounded".to_string()) } else { None },
        if loading { Some("is-loading".to_string()) } else { None },
        if focused { Some("is-focused".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let input_style = props.style.as_deref().unwrap_or("");

    rsx! {
        input {
            r#type: props.input_type.as_str(),
            class: "{final_class}",
            style: "{input_style}",
            value: props.value.as_deref().unwrap_or(""),
            placeholder: props.placeholder.as_deref().unwrap_or(""),
            disabled: disabled,
            readonly: readonly,
            oninput: move |evt| {
                if let Some(handler) = &props.oninput {
                    handler.call(evt);
                }
            },
            onchange: move |evt| {
                if let Some(handler) = &props.onchange {
                    handler.call(evt);
                }
            },
            onfocus: move |evt| {
                if let Some(handler) = &props.onfocus {
                    handler.call(evt);
                }
            },
            onblur: move |evt| {
                if let Some(handler) = &props.onblur {
                    handler.call(evt);
                }
            }
        }
    }
}