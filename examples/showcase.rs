use dioxus::prelude::*;
use dioxus_bulma::*;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let mut current_tab = use_signal(|| "buttons".to_string());
    let mut show_modal = use_signal(|| false);
    let mut show_notification = use_signal(|| true);
    let count = use_signal(|| 0i32);
    let name = use_signal(|| String::new());
    let email = use_signal(|| String::new());

    rsx! {
        BulmaProvider {
            theme: BulmaTheme::Auto,
            load_bulma_css: true,
            
            // Hero Section
            Hero {
                color: BulmaColor::Primary,
                size: BulmaSize::Medium,
                HeroBody {
                    Container {
                        Title { size: TitleSize::Is1, "Dioxus Bulma Showcase" }
                        Subtitle { size: TitleSize::Is4, "Explore all the beautiful components" }
                    }
                }
            }

            // Notification
            if show_notification() {
                Section {
                    Container {
                        Notification {
                            color: BulmaColor::Info,
                            light: true,
                            dismissible: true,
                            onclose: move |_| show_notification.set(false),
                            "Welcome! This showcase demonstrates the various Dioxus Bulma components."
                        }
                    }
                }
            }

            Section {
                Container {
                    // Tab Navigation
                    div { class: "tabs is-boxed is-large",
                        ul {
                            li { class: if current_tab() == "buttons" { "is-active" } else { "" },
                                a { onclick: move |_| current_tab.set("buttons".to_string()), "Buttons" }
                            }
                            li { class: if current_tab() == "forms" { "is-active" } else { "" },
                                a { onclick: move |_| current_tab.set("forms".to_string()), "Forms" }
                            }
                            li { class: if current_tab() == "layout" { "is-active" } else { "" },
                                a { onclick: move |_| current_tab.set("layout".to_string()), "Layout" }
                            }
                            li { class: if current_tab() == "components" { "is-active" } else { "" },
                                a { onclick: move |_| current_tab.set("components".to_string()), "Components" }
                            }
                        }
                    }

                    // Content based on selected tab
                    match current_tab().as_str() {
                        "buttons" => rsx! { ButtonsDemo { count } },
                        "forms" => rsx! { FormsDemo { name, email } },
                        "layout" => rsx! { LayoutDemo {} },
                        "components" => rsx! { ComponentsDemo { show_modal } },
                        _ => rsx! { ButtonsDemo { count } },
                    }
                }
            }

            // Modal
            Modal {
                active: show_modal(),
                onclose: move |_| show_modal.set(false),
                ModalCard {
                    ModalCardHead {
                        onclose: move |_| show_modal.set(false),
                        p { class: "modal-card-title", "Example Modal" }
                    }
                    ModalCardBody {
                        p { "This is an example modal built with the Modal component." }
                        p { "It demonstrates how modals work in the Dioxus Bulma library." }
                    }
                    ModalCardFoot {
                        Button {
                            color: BulmaColor::Success,
                            onclick: move |_| show_modal.set(false),
                            "Confirm"
                        }
                        Button {
                            onclick: move |_| show_modal.set(false),
                            "Cancel"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ButtonsDemo(count: Signal<i32>) -> Element {
    rsx! {
        Title { size: TitleSize::Is3, "Buttons" }
        
        // Interactive Counter
        Card {
            CardHeader { CardHeaderTitle { "Interactive Counter" } }
            CardContent {
                div { class: "has-text-centered",
                    Title { size: TitleSize::Is2, "Count: {count}" }
                    Field { grouped: true, class: "is-justify-content-center",
                        Control {
                            Button {
                                color: BulmaColor::Success,
                                size: BulmaSize::Large,
                                onclick: move |_| *count.write() += 1,
                                "+"
                            }
                        }
                        Control {
                            Button {
                                color: BulmaColor::Danger,
                                size: BulmaSize::Large,
                                onclick: move |_| *count.write() -= 1,
                                "-"
                            }
                        }
                        Control {
                            Button {
                                color: BulmaColor::Warning,
                                size: BulmaSize::Large,
                                outlined: true,
                                onclick: move |_| count.set(0),
                                "Reset"
                            }
                        }
                    }
                }
            }
        }
        
        // Button Colors
        div { class: "mt-5",
            Title { size: TitleSize::Is4, "Button Colors" }
            Field { grouped: true, class: "is-grouped-multiline",
                Control { Button { color: BulmaColor::Primary, "Primary" } }
                Control { Button { color: BulmaColor::Link, "Link" } }
                Control { Button { color: BulmaColor::Info, "Info" } }
                Control { Button { color: BulmaColor::Success, "Success" } }
                Control { Button { color: BulmaColor::Warning, "Warning" } }
                Control { Button { color: BulmaColor::Danger, "Danger" } }
            }
        }

        // Button Variants
        div { class: "mt-5",
            Title { size: TitleSize::Is4, "Button Variants" }
            Field { grouped: true, class: "is-grouped-multiline",
                Control { Button { color: BulmaColor::Primary, outlined: true, "Outlined" } }
                Control { Button { color: BulmaColor::Info, rounded: true, "Rounded" } }
                Control { Button { color: BulmaColor::Warning, loading: true, "Loading" } }
                Control { Button { disabled: true, "Disabled" } }
            }
        }

        // Button Sizes
        div { class: "mt-5",
            Title { size: TitleSize::Is4, "Button Sizes" }
            Field { grouped: true, class: "is-grouped-multiline",
                Control { Button { size: BulmaSize::Small, color: BulmaColor::Primary, "Small" } }
                Control { Button { size: BulmaSize::Normal, color: BulmaColor::Primary, "Normal" } }
                Control { Button { size: BulmaSize::Medium, color: BulmaColor::Primary, "Medium" } }
                Control { Button { size: BulmaSize::Large, color: BulmaColor::Primary, "Large" } }
            }
        }
    }
}

#[component]
fn FormsDemo(mut name: Signal<String>, mut email: Signal<String>) -> Element {
    rsx! {
        Title { size: TitleSize::Is3, "Form Components" }
        
        Card {
            CardHeader { CardHeaderTitle { "Contact Form" } }
            CardContent {
                Field {
                    Label { "Name" }
                    Control {
                        Input {
                            input_type: InputType::Text,
                            placeholder: "Enter your name",
                            value: name(),
                            oninput: move |evt: FormEvent| name.set(evt.value())
                        }
                    }
                }
                
                Field {
                    Label { "Email" }
                    Control {
                        Input {
                            input_type: InputType::Email,
                            placeholder: "Enter your email",
                            color: if email().contains('@') && email().contains('.') { 
                                Some(BulmaColor::Success) 
                            } else if !email().is_empty() { 
                                Some(BulmaColor::Danger) 
                            } else { 
                                None 
                            },
                            value: email(),
                            oninput: move |evt: FormEvent| email.set(evt.value())
                        }
                    }
                    if !email().is_empty() && (!email().contains('@') || !email().contains('.')) {
                        Help { color: BulmaColor::Danger, "Please enter a valid email address" }
                    } else if email().contains('@') && email().contains('.') {
                        Help { color: BulmaColor::Success, "Email looks good!" }
                    }
                }
                
                Field { grouped: true,
                    Control {
                        Button {
                            color: BulmaColor::Primary,
                            onclick: move |_| {
                                // Handle form submission
                                println!("Submitted: Name={}, Email={}", name(), email());
                            },
                            "Submit"
                        }
                    }
                    Control {
                        Button {
                            onclick: move |_| {
                                name.set(String::new());
                                email.set(String::new());
                            },
                            "Clear"
                        }
                    }
                }
            }
        }

        // Tags showcase
        div { class: "mt-5",
            Title { size: TitleSize::Is4, "Tags" }
            Tags {
                Tag { color: BulmaColor::Primary, "Rust" }
                Tag { color: BulmaColor::Info, light: true, "Dioxus" }
                Tag { color: BulmaColor::Success, rounded: true, "WebAssembly" }
                Tag { color: BulmaColor::Warning, "CSS" }
                Tag { color: BulmaColor::Danger, size: BulmaSize::Medium, "Bulma" }
            }
        }
    }
}

#[component]
fn LayoutDemo() -> Element {
    rsx! {
        Title { size: TitleSize::Is3, "Layout Components" }
        
        // Columns
        Title { size: TitleSize::Is4, "Columns" }
        Columns {
            multiline: true,
            Column { size: ColumnSize::Half,
                div { class: "box has-background-primary-light",
                    p { class: "has-text-centered", "Half width column" }
                }
            }
            Column { size: ColumnSize::OneQuarter,
                div { class: "box has-background-info-light",
                    p { class: "has-text-centered", "Quarter width" }
                }
            }
            Column { size: ColumnSize::OneQuarter,
                div { class: "box has-background-success-light", 
                    p { class: "has-text-centered", "Quarter width" }
                }
            }
        }

        Columns {
            Column { size: ColumnSize::OneThird,
                div { class: "box has-background-warning-light",
                    p { class: "has-text-centered", "One third" }
                }
            }
            Column { size: ColumnSize::TwoThirds,
                div { class: "box has-background-danger-light",
                    p { class: "has-text-centered", "Two thirds" }
                }
            }
        }

        // Hero
        div { class: "mt-5",
            Title { size: TitleSize::Is4, "Hero" }
            Hero {
                color: BulmaColor::Info,
                HeroBody {
                    Container {
                        Title { size: TitleSize::Is4, "Hero Title" }
                        Subtitle { size: TitleSize::Is6, "Hero subtitle" }
                    }
                }
            }
        }
    }
}

#[component]
fn ComponentsDemo(mut show_modal: Signal<bool>) -> Element {
    rsx! {
        Title { size: TitleSize::Is3, "Components" }
        
        // Cards
        Title { size: TitleSize::Is4, "Cards" }
        Columns {
            multiline: true,
            Column { size: ColumnSize::Half,
                Card {
                    CardHeader {
                        CardHeaderTitle { "Card with Header" }
                    }
                    CardContent {
                        div { class: "content",
                            p { "This is a card with a header. Cards are great for displaying content in a structured way." }
                            time { datetime: "2024-1-1", "Posted on January 1, 2024" }
                        }
                    }
                    CardFooter {
                        a { class: "card-footer-item", href: "#", "Save" }
                        a { class: "card-footer-item", href: "#", "Edit" }
                        a { class: "card-footer-item", href: "#", "Delete" }
                    }
                }
            }
            Column { size: ColumnSize::Half,
                Card {
                    CardContent {
                        div { class: "media",
                            div { class: "media-left",
                                figure { class: "image is-48x48",
                                    div { 
                                        class: "has-background-primary-light is-flex is-align-items-center is-justify-content-center",
                                        style: "width: 48px; height: 48px; border-radius: 6px;",
                                        span { class: "icon is-large has-text-primary", "📦" }
                                    }
                                }
                            }
                            div { class: "media-content",
                                p { class: "title is-4", "Media Card" }
                                p { class: "subtitle is-6", "With media object" }
                            }
                        }
                        div { class: "content",
                            "This card uses the media object pattern for structured content layout."
                        }
                    }
                }
            }
        }

        // Modal trigger
        div { class: "mt-5",
            Title { size: TitleSize::Is4, "Modal" }
            Field {
                Control {
                    Button {
                        color: BulmaColor::Primary,
                        size: BulmaSize::Large,
                        onclick: move |_| show_modal.set(true),
                        "Open Modal"
                    }
                }
            }
        }

        // Notifications
        div { class: "mt-5",
            Title { size: TitleSize::Is4, "Notifications" }
            div { class: "space-y-3",
                Notification {
                    color: BulmaColor::Success,
                    light: true,
                    "Success notification with light styling"
                }
                Notification {
                    color: BulmaColor::Warning,
                    "Warning notification"
                }
                Notification {
                    color: BulmaColor::Danger,
                    light: true,
                    "Danger notification with light styling"
                }
            }
        }
    }
}