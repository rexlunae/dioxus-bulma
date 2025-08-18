use dioxus::prelude::*;
use crate::utils::build_class;

#[derive(Props, Clone, PartialEq)]
pub struct FieldProps {
    #[props(default)]
    pub grouped: Option<bool>,
    #[props(default)]
    pub addons: Option<bool>,
    #[props(default)]
    pub horizontal: Option<bool>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Field(props: FieldProps) -> Element {
    let grouped = props.grouped.unwrap_or(false);
    let addons = props.addons.unwrap_or(false);
    let horizontal = props.horizontal.unwrap_or(false);
    
    let base_classes = vec!["field"];
    let optional_classes = vec![
        if grouped { Some("is-grouped".to_string()) } else { None },
        if addons { Some("has-addons".to_string()) } else { None },
        if horizontal { Some("is-horizontal".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let field_style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{field_style}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct LabelProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Label(props: LabelProps) -> Element {
    let base_classes = vec!["label"];
    let optional_classes = vec![props.class.clone()];
    let final_class = build_class(&base_classes, &optional_classes);
    let label_style = props.style.as_deref().unwrap_or("");

    rsx! {
        label {
            class: "{final_class}",
            style: "{label_style}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct HelpProps {
    #[props(default)]
    pub color: Option<crate::theme::BulmaColor>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Help(props: HelpProps) -> Element {
    let base_classes = vec!["help"];
    let optional_classes = vec![
        props.color.map(|c| format!("is-{}", c.as_str())),
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let help_style = props.style.as_deref().unwrap_or("");

    rsx! {
        p {
            class: "{final_class}",
            style: "{help_style}",
            {props.children}
        }
    }
}
