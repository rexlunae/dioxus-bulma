use dioxus::prelude::*;
use crate::theme::{BulmaColor, BulmaSize};
use crate::utils::build_class;

#[derive(Props, Clone, PartialEq)]
pub struct TextareaProps {
    #[props(default)]
    pub value: Option<String>,
    #[props(default)]
    pub placeholder: Option<String>,
    #[props(default)]
    pub color: Option<BulmaColor>,
    #[props(default)]
    pub size: Option<BulmaSize>,
    #[props(default)]
    pub disabled: Option<bool>,
    #[props(default)]
    pub readonly: Option<bool>,
    #[props(default)]
    pub rows: Option<u32>,
    #[props(default)]
    pub cols: Option<u32>,
    #[props(default)]
    pub has_fixed_size: Option<bool>,
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
pub fn Textarea(props: TextareaProps) -> Element {
    let size = props.size.unwrap_or_default();
    let disabled = props.disabled.unwrap_or(false);
    let readonly = props.readonly.unwrap_or(false);
    let has_fixed_size = props.has_fixed_size.unwrap_or(false);
    
    let base_classes = vec!["textarea"];
    let optional_classes = vec![
        props.color.map(|c| format!("is-{}", c.as_str())),
        if size != BulmaSize::Normal { Some(size.as_class().to_string()) } else { None },
        if has_fixed_size { Some("has-fixed-size".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let textarea_style = props.style.as_deref().unwrap_or("");

    rsx! {
        textarea {
            class: "{final_class}",
            style: "{textarea_style}",
            value: props.value.as_deref().unwrap_or(""),
            placeholder: props.placeholder.as_deref().unwrap_or(""),
            disabled: disabled,
            readonly: readonly,
            rows: props.rows.map(|r| r.to_string()).as_deref().unwrap_or("4"),
            cols: props.cols.map(|c| c.to_string()).as_deref(),
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
