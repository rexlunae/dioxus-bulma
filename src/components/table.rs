use dioxus::prelude::*;
use crate::theme::BulmaSize;
use crate::utils::build_class;

#[derive(Props, Clone, PartialEq)]
pub struct TableProps {
    #[props(default)]
    pub bordered: Option<bool>,
    #[props(default)]
    pub striped: Option<bool>,
    #[props(default)]
    pub narrow: Option<bool>,
    #[props(default)]
    pub hoverable: Option<bool>,
    #[props(default)]
    pub fullwidth: Option<bool>,
    #[props(default)]
    pub size: Option<BulmaSize>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Table(props: TableProps) -> Element {
    let bordered = props.bordered.unwrap_or(false);
    let striped = props.striped.unwrap_or(false);
    let narrow = props.narrow.unwrap_or(false);
    let hoverable = props.hoverable.unwrap_or(false);
    let fullwidth = props.fullwidth.unwrap_or(false);
    let size = props.size.unwrap_or_default();
    
    let base_classes = vec!["table"];
    let optional_classes = vec![
        if bordered { Some("is-bordered".to_string()) } else { None },
        if striped { Some("is-striped".to_string()) } else { None },
        if narrow { Some("is-narrow".to_string()) } else { None },
        if hoverable { Some("is-hoverable".to_string()) } else { None },
        if fullwidth { Some("is-fullwidth".to_string()) } else { None },
        if size != BulmaSize::Normal { Some(size.as_class().to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let table_style = props.style.as_deref().unwrap_or("");

    rsx! {
        table {
            class: "{final_class}",
            style: "{table_style}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TableContainerProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn TableContainer(props: TableContainerProps) -> Element {
    let base_classes = vec!["table-container"];
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
