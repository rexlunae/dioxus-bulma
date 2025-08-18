use dioxus::prelude::*;
use crate::theme::BulmaSize;
use crate::utils::build_class;

#[derive(Props, Clone, PartialEq)]
pub struct ControlProps {
    #[props(default)]
    pub has_icons_left: Option<bool>,
    #[props(default)]
    pub has_icons_right: Option<bool>,
    #[props(default)]
    pub loading: Option<bool>,
    #[props(default)]
    pub size: Option<BulmaSize>,
    #[props(default)]
    pub expanded: Option<bool>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Control(props: ControlProps) -> Element {
    let has_icons_left = props.has_icons_left.unwrap_or(false);
    let has_icons_right = props.has_icons_right.unwrap_or(false);
    let loading = props.loading.unwrap_or(false);
    let expanded = props.expanded.unwrap_or(false);
    let size = props.size.unwrap_or_default();
    
    let base_classes = vec!["control"];
    let optional_classes = vec![
        if has_icons_left { Some("has-icons-left".to_string()) } else { None },
        if has_icons_right { Some("has-icons-right".to_string()) } else { None },
        if loading { Some("is-loading".to_string()) } else { None },
        if expanded { Some("is-expanded".to_string()) } else { None },
        if size != BulmaSize::Normal { Some(size.as_class().to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let control_style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{control_style}",
            {props.children}
        }
    }
}
