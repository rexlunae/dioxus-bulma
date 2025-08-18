use dioxus::prelude::*;
use crate::theme::BulmaSize;
use crate::utils::build_class;

#[derive(Props, Clone, PartialEq)]
pub struct SectionProps {
    #[props(default)]
    pub size: Option<BulmaSize>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Section(props: SectionProps) -> Element {
    let base_classes = vec!["section"];
    
    let optional_classes = vec![
        props.size.and_then(|s| if s != BulmaSize::Normal { 
            Some(s.as_class().to_string()) 
        } else { 
            None 
        }),
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let section_style = props.style.as_deref().unwrap_or("");

    rsx! {
        section {
            class: "{final_class}",
            style: "{section_style}",
            {props.children}
        }
    }
}