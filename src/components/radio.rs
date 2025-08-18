use dioxus::prelude::*;
use crate::utils::build_class;

#[derive(Props, Clone, PartialEq)]
pub struct RadioProps {
    #[props(default)]
    pub checked: Option<bool>,
    #[props(default)]
    pub disabled: Option<bool>,
    pub name: String,
    pub value: String,
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Radio(props: RadioProps) -> Element {
    let checked = props.checked.unwrap_or(false);
    let disabled = props.disabled.unwrap_or(false);
    
    let base_classes = vec!["radio"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let radio_style = props.style.as_deref().unwrap_or("");

    rsx! {
        label {
            class: "{final_class}",
            style: "{radio_style}",
            input {
                r#type: "radio",
                name: "{props.name}",
                value: "{props.value}",
                checked: checked,
                disabled: disabled,
                onchange: move |evt| {
                    if let Some(handler) = &props.onchange {
                        handler.call(evt);
                    }
                }
            }
            " "
            {props.children}
        }
    }
}
