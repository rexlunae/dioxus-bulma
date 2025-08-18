use dioxus::prelude::*;
use crate::theme::{BulmaColor, BulmaSize};
use crate::utils::build_class;

#[derive(Props, Clone, PartialEq)]
pub struct FileProps {
    #[props(default)]
    pub color: Option<BulmaColor>,
    #[props(default)]
    pub size: Option<BulmaSize>,
    #[props(default)]
    pub boxed: Option<bool>,
    #[props(default)]
    pub centered: Option<bool>,
    #[props(default)]
    pub right: Option<bool>,
    #[props(default)]
    pub fullwidth: Option<bool>,
    #[props(default)]
    pub has_name: Option<bool>,
    #[props(default)]
    pub accept: Option<String>,
    #[props(default)]
    pub multiple: Option<bool>,
    #[props(default)]
    pub disabled: Option<bool>,
    #[props(default)]
    pub name: Option<String>,
    #[props(default)]
    pub filename: Option<String>,
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn File(props: FileProps) -> Element {
    let size = props.size.unwrap_or_default();
    let boxed = props.boxed.unwrap_or(false);
    let centered = props.centered.unwrap_or(false);
    let right = props.right.unwrap_or(false);
    let fullwidth = props.fullwidth.unwrap_or(false);
    let has_name = props.has_name.unwrap_or(false);
    let multiple = props.multiple.unwrap_or(false);
    let disabled = props.disabled.unwrap_or(false);
    
    let base_classes = vec!["file"];
    let optional_classes = vec![
        props.color.map(|c| format!("is-{}", c.as_str())),
        if size != BulmaSize::Normal { Some(size.as_class().to_string()) } else { None },
        if boxed { Some("is-boxed".to_string()) } else { None },
        if centered { Some("is-centered".to_string()) } else { None },
        if right { Some("is-right".to_string()) } else { None },
        if fullwidth { Some("is-fullwidth".to_string()) } else { None },
        if has_name { Some("has-name".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let file_style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{file_style}",
            label {
                class: "file-label",
                input {
                    class: "file-input",
                    r#type: "file",
                    accept: props.accept.as_deref(),
                    multiple: multiple,
                    disabled: disabled,
                    name: props.name.as_deref(),
                    onchange: move |evt| {
                        if let Some(handler) = &props.onchange {
                            handler.call(evt);
                        }
                    }
                }
                span {
                    class: "file-cta",
                    span {
                        class: "file-icon",
                        i { class: "fas fa-upload" }
                    }
                    span {
                        class: "file-label",
                        {props.children}
                    }
                }
                if has_name {
                    span {
                        class: "file-name",
                        {props.filename.as_deref().unwrap_or("No file chosen")}
                    }
                }
            }
        }
    }
}
