use dioxus::prelude::*;
use crate::theme::BulmaColor;
use crate::utils::build_class;

#[derive(Props, Clone, PartialEq)]
pub struct NotificationProps {
    #[props(default)]
    pub color: Option<BulmaColor>,
    #[props(default)]
    pub light: Option<bool>,
    #[props(default)]
    pub dismissible: Option<bool>,
    #[props(default)]
    pub onclose: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Notification(props: NotificationProps) -> Element {
    let color = props.color.unwrap_or(BulmaColor::Primary);
    let light = props.light.unwrap_or(false);
    let dismissible = props.dismissible.unwrap_or(false);
    
    let base_classes = vec!["notification"];
    
    let optional_classes = vec![
        Some(format!("is-{}", color.as_str())),
        if light { Some("is-light".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let notification_style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{notification_style}",
            if dismissible {
                button {
                    class: "delete",
                    onclick: move |evt| {
                        if let Some(handler) = &props.onclose {
                            handler.call(evt);
                        }
                    }
                }
            }
            {props.children}
        }
    }
}