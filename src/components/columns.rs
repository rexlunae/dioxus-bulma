use dioxus::prelude::*;
use crate::utils::build_class;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColumnSize {
    ThreeQuarters,
    TwoThirds,
    Half,
    OneThird,
    OneQuarter,
    Full,
    FourFifths,
    ThreeFifths,
    TwoFifths,
    OneFifth,
    Narrow,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
}

impl ColumnSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            ColumnSize::ThreeQuarters => "is-three-quarters",
            ColumnSize::TwoThirds => "is-two-thirds",
            ColumnSize::Half => "is-half",
            ColumnSize::OneThird => "is-one-third",
            ColumnSize::OneQuarter => "is-one-quarter",
            ColumnSize::Full => "is-full",
            ColumnSize::FourFifths => "is-four-fifths",
            ColumnSize::ThreeFifths => "is-three-fifths",
            ColumnSize::TwoFifths => "is-two-fifths",
            ColumnSize::OneFifth => "is-one-fifth",
            ColumnSize::Narrow => "is-narrow",
            ColumnSize::One => "is-1",
            ColumnSize::Two => "is-2",
            ColumnSize::Three => "is-3",
            ColumnSize::Four => "is-4",
            ColumnSize::Five => "is-5",
            ColumnSize::Six => "is-6",
            ColumnSize::Seven => "is-7",
            ColumnSize::Eight => "is-8",
            ColumnSize::Nine => "is-9",
            ColumnSize::Ten => "is-10",
            ColumnSize::Eleven => "is-11",
            ColumnSize::Twelve => "is-12",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ColumnsProps {
    #[props(default)]
    pub multiline: Option<bool>,
    #[props(default)]
    pub gapless: Option<bool>,
    #[props(default)]
    pub mobile: Option<bool>,
    #[props(default)]
    pub desktop: Option<bool>,
    #[props(default)]
    pub centered: Option<bool>,
    #[props(default)]
    pub vcentered: Option<bool>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Columns(props: ColumnsProps) -> Element {
    let multiline = props.multiline.unwrap_or(false);
    let gapless = props.gapless.unwrap_or(false);
    let mobile = props.mobile.unwrap_or(false);
    let desktop = props.desktop.unwrap_or(false);
    let centered = props.centered.unwrap_or(false);
    let vcentered = props.vcentered.unwrap_or(false);
    
    let base_classes = vec!["columns"];
    
    let optional_classes = vec![
        if multiline { Some("is-multiline".to_string()) } else { None },
        if gapless { Some("is-gapless".to_string()) } else { None },
        if mobile { Some("is-mobile".to_string()) } else { None },
        if desktop { Some("is-desktop".to_string()) } else { None },
        if centered { Some("is-centered".to_string()) } else { None },
        if vcentered { Some("is-vcentered".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let columns_style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{columns_style}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ColumnProps {
    #[props(default)]
    pub size: Option<ColumnSize>,
    #[props(default)]
    pub offset: Option<ColumnSize>,
    #[props(default)]
    pub narrow: Option<bool>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn Column(props: ColumnProps) -> Element {
    let narrow = props.narrow.unwrap_or(false);
    
    let base_classes = vec!["column"];
    
    let optional_classes = vec![
        props.size.map(|s| s.as_str().to_string()),
        props.offset.map(|o| format!("is-offset-{}", o.as_str().strip_prefix("is-").unwrap_or(o.as_str()))),
        if narrow { Some("is-narrow".to_string()) } else { None },
        props.class.clone(),
    ];
    
    let final_class = build_class(&base_classes, &optional_classes);
    let column_style = props.style.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{final_class}",
            style: "{column_style}",
            {props.children}
        }
    }
}