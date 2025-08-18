use dioxus::prelude::*;
use crate::theme::{BulmaColor, BulmaSize};
use crate::utils::build_class;

#[derive(Props, Clone, PartialEq)]
pub struct SelectProps {
    #[props(default)]
    pub value: Option<String>,
    #[props(default)]
    pub color: Option<BulmaColor>,
    #[props(default)]
    pub size: Option<BulmaSize>,
    #[props(default)]
    pub rounded: Option<bool>,
    #[props(default)]
    pub disabled: Option<bool>,
    #[props(default)]
    pub loading: Option<bool>,
    #[props(default)]
    pub multiple: Option<bool>,
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    let size = props.size.unwrap_or_default();
    let rounded = props.rounded.unwrap_or(false);
    let disabled = props.disabled.unwrap_or(false);
    let loading = props.loading.unwrap_or(false);
    let multiple = props.multiple.unwrap_or(false);
    
    let base_classes = vec!["select"];
    let optional_classes = vec![
        props.color.map(|c| format!("is-{}", c.as_str())),
        if size != BulmaSize::Normal { Some(size.as_class().to_string()) } else { None },
        if rounded { Some("is-rounded".to_string()) } else { None },
        if loading { Some("is-loading".to_string()) } else { None },
        if multiple { Some("is-multiple".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let select_style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{select_style}",
            select {
                value: props.value.as_deref().unwrap_or(""),
                disabled: disabled,
                multiple: multiple,
                onchange: move |evt| {
                    if let Some(handler) = &props.onchange {
                        handler.call(evt);
                    }
                },
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct OptionProps {
    #[props(default)]
    pub value: Option<String>,
    #[props(default)]
    pub selected: Option<bool>,
    #[props(default)]
    pub disabled: Option<bool>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Option(props: OptionProps) -> Element {
    let selected = props.selected.unwrap_or(false);
    let disabled = props.disabled.unwrap_or(false);
    let option_style = props.style.as_deref().unwrap_or("");
    let option_class = props.class.as_deref().unwrap_or("");

    rsx! {
        option {
            value: props.value.as_deref().unwrap_or(""),
            selected: selected,
            disabled: disabled,
            class: "{option_class}",
            style: "{option_style}",
            {props.children}
        }
    }
}
