use dioxus::prelude::*;
use crate::utils::build_class;

#[derive(Props, Clone, PartialEq)]
pub struct ModalProps {
    #[props(default = false)]
    pub active: bool,
    #[props(default)]
    pub onclose: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Modal(props: ModalProps) -> Element {
    let base_classes = vec!["modal"];
    let optional_classes = vec![
        if props.active { Some("is-active".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let modal_style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{modal_style}",
            div {
                class: "modal-background",
                onclick: move |evt| {
                    if let Some(handler) = &props.onclose {
                        handler.call(evt);
                    }
                }
            }
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalCardProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn ModalCard(props: ModalCardProps) -> Element {
    let base_classes = vec!["modal-card"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{style}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalCardHeadProps {
    #[props(default)]
    pub onclose: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn ModalCardHead(props: ModalCardHeadProps) -> Element {
    let base_classes = vec!["modal-card-head"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let style = props.style.as_deref().unwrap_or("");

    rsx! {
        header {
            class: "{final_class}",
            style: "{style}",
            {props.children}
            button {
                class: "delete",
                "aria-label": "close",
                onclick: move |evt| {
                    if let Some(handler) = &props.onclose {
                        handler.call(evt);
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalCardBodyProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn ModalCardBody(props: ModalCardBodyProps) -> Element {
    let base_classes = vec!["modal-card-body"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let style = props.style.as_deref().unwrap_or("");

    rsx! {
        section {
            class: "{final_class}",
            style: "{style}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalCardFootProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn ModalCardFoot(props: ModalCardFootProps) -> Element {
    let base_classes = vec!["modal-card-foot"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let style = props.style.as_deref().unwrap_or("");

    rsx! {
        footer {
            class: "{final_class}",
            style: "{style}",
            {props.children}
        }
    }
}
