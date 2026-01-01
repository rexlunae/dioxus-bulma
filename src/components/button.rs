use crate::theme::{BulmaColor, BulmaSize};
use crate::utils::build_class;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(default)]
    pub color: Option<BulmaColor>,
    #[props(default)]
    pub size: Option<BulmaSize>,
    #[props(default)]
    pub outlined: Option<bool>,
    #[props(default)]
    pub inverted: Option<bool>,
    #[props(default)]
    pub rounded: Option<bool>,
    #[props(default)]
    pub loading: Option<bool>,
    #[props(default)]
    pub disabled: Option<bool>,
    #[props(default)]
    pub fullwidth: Option<bool>,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    /// If present, generate the button as a Link component using the dioxus router
    #[cfg(feature = "router")]
    #[props(default)]
    pub to: Option<NavigationTarget>,
    pub children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let color = props.color.unwrap_or(BulmaColor::Primary);
    let size = props.size.unwrap_or_default();
    let outlined = props.outlined.unwrap_or(false);
    let inverted = props.inverted.unwrap_or(false);
    let rounded = props.rounded.unwrap_or(false);
    let loading = props.loading.unwrap_or(false);
    let disabled = props.disabled.unwrap_or(false);
    let fullwidth = props.fullwidth.unwrap_or(false);
    
    let base_classes = vec!["button"];
    
    let optional_classes = vec![
        Some(format!("is-{}", color.as_str())),
        if size != BulmaSize::Normal { Some(size.as_class().to_string()) } else { None },
        if outlined { Some("is-outlined".to_string()) } else { None },
        if inverted { Some("is-inverted".to_string()) } else { None },
        if rounded { Some("is-rounded".to_string()) } else { None },
        if loading { Some("is-loading".to_string()) } else { None },
        if fullwidth { Some("is-fullwidth".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let button_style = props.style.as_deref().unwrap_or("");

    #[cfg(feature = "router")]
    if let Some(nav_target) = props.to {
        return rsx! {
            Link {
                to: nav_target,
                class: "{final_class}",
                style: "{button_style}",
                onclick: move |evt| {
                    if !disabled && !loading {
                        if let Some(handler) = &props.onclick {
                            handler.call(evt);
                        }
                    }
                },
                {props.children}
            }
        };
    }

    rsx! {
        button {
            class: "{final_class}",
            style: "{button_style}",
            disabled: disabled,
            onclick: move |evt| {
                if !disabled && !loading {
                    if let Some(handler) = &props.onclick {
                        handler.call(evt);
                    }
                }
            },
            {props.children}
        }
    }
}