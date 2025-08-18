use dioxus::prelude::*;
use crate::theme::BulmaSize;
use crate::utils::build_class;

#[derive(Props, Clone, PartialEq)]
pub struct DeleteProps {
    #[props(default)]
    pub size: Option<BulmaSize>,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
}

#[component]
pub fn Delete(props: DeleteProps) -> Element {
    let size = props.size.unwrap_or_default();
    
    let base_classes = vec!["delete"];
    let optional_classes = vec![
        if size != BulmaSize::Normal { Some(size.as_class().to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let delete_style = props.style.as_deref().unwrap_or("");

    rsx! {
        button {
            class: "{final_class}",
            style: "{delete_style}",
            "aria-label": "delete",
            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            }
        }
    }
}
