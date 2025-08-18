use dioxus::prelude::*;
use crate::theme::{BulmaColor, BulmaSize};
use crate::utils::build_class;

#[derive(Props, Clone, PartialEq)]
pub struct MessageProps {
    #[props(default)]
    pub color: Option<BulmaColor>,
    #[props(default)]
    pub size: Option<BulmaSize>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Message(props: MessageProps) -> Element {
    let size = props.size.unwrap_or_default();
    
    let base_classes = vec!["message"];
    let optional_classes = vec![
        props.color.map(|c| format!("is-{}", c.as_str())),
        if size != BulmaSize::Normal { Some(size.as_class().to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let message_style = props.style.as_deref().unwrap_or("");

    rsx! {
        article {
            class: "{final_class}",
            style: "{message_style}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MessageHeaderProps {
    #[props(default)]
    pub closable: Option<bool>,
    #[props(default)]
    pub onclose: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn MessageHeader(props: MessageHeaderProps) -> Element {
    let closable = props.closable.unwrap_or(false);
    
    let base_classes = vec!["message-header"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let header_style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{header_style}",
            p { {props.children} }
            if closable {
                button {
                    class: "delete",
                    "aria-label": "delete",
                    onclick: move |evt| {
                        if let Some(handler) = &props.onclose {
                            handler.call(evt);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MessageBodyProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn MessageBody(props: MessageBodyProps) -> Element {
    let base_classes = vec!["message-body"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let body_style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{body_style}",
            {props.children}
        }
    }
}
