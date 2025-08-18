use dioxus::prelude::*;
use crate::utils::build_class;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContainerBreakpoint {
    Widescreen,
    FullHD,
    MaxDesktop,
    MaxWidescreen,
}

impl ContainerBreakpoint {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContainerBreakpoint::Widescreen => "is-widescreen",
            ContainerBreakpoint::FullHD => "is-fullhd", 
            ContainerBreakpoint::MaxDesktop => "is-max-desktop",
            ContainerBreakpoint::MaxWidescreen => "is-max-widescreen",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ContainerProps {
    #[props(default)]
    pub fluid: Option<bool>,
    #[props(default)]
    pub breakpoint: Option<ContainerBreakpoint>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Container(props: ContainerProps) -> Element {
    let fluid = props.fluid.unwrap_or(false);
    
    let base_classes = vec!["container"];
    
    let optional_classes = vec![
        if fluid { Some("is-fluid".to_string()) } else { None },
        props.breakpoint.map(|bp| bp.as_str().to_string()),
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let container_style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{container_style}",
            {props.children}
        }
    }
}