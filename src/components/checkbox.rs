use dioxus::prelude::*;
use crate::utils::build_class;

#[derive(Props, Clone, PartialEq)]
pub struct CheckboxProps {
    #[props(default)]
    pub checked: Option<bool>,
    #[props(default)]
    pub disabled: Option<bool>,
    #[props(default)]
    pub name: Option<String>,
    #[props(default)]
    pub value: Option<String>,
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let checked = props.checked.unwrap_or(false);
    let disabled = props.disabled.unwrap_or(false);
    
    let base_classes = vec!["checkbox"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let checkbox_style = props.style.as_deref().unwrap_or("");

    rsx! {
        label {
            class: "{final_class}",
            style: "{checkbox_style}",
            input {
                r#type: "checkbox",
                checked: checked,
                disabled: disabled,
                name: props.name.as_deref(),
                value: props.value.as_deref().unwrap_or("on"),
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
