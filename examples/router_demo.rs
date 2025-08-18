use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_bulma::*;

fn main() {
    launch(App);
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/layout")]
    Layout {},
    #[route("/elements")]
    Elements {},
    #[route("/forms")]
    Forms {},
    #[route("/navigation")]
    Navigation {},
    #[route("/components")]
    Components {},
}

#[component]
fn App() -> Element {
    rsx! {
        BulmaProvider {
            theme: BulmaTheme::Auto,
            load_bulma_css: true,
            
            Router::<Route> {}
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        Title { size: TitleSize::Is2, "Welcome to Dioxus Bulma Demo" }
        
        Content {
            "This comprehensive demo showcases all available components in the dioxus-bulma library, "
            "complete with code snippets and live examples. Use the navigation above to explore different component categories."
        }

        // Feature Highlights
        Columns {
            multiline: true,
            
            Column { size: ColumnSize::OneThird,
                Card {
                    CardContent {
                        div { class: "has-text-centered",
                            Title { size: TitleSize::Is4, "Complete Bulma Integration" }
                            Content { "All Bulma CSS components implemented as type-safe Dioxus components." }
                        }
                    }
                }
            }
            
            Column { size: ColumnSize::OneThird,
                Card {
                    CardContent {
                        div { class: "has-text-centered",
                            Title { size: TitleSize::Is4, "Router Integration" }
                            Content { "Seamless client-side navigation with dioxus-router support." }
                        }
                    }
                }
            }
            
            Column { size: ColumnSize::OneThird,
                Card {
                    CardContent {
                        div { class: "has-text-centered",
                            Title { size: TitleSize::Is4, "Type-Safe Components" }
                            Content { "Compile-time guarantees with Rust's type system for better DX." }
                        }
                    }
                }
            }
        }

        // Router Navigation Example using Link directly
        div { class: "mt-6",
            Title { size: TitleSize::Is3, "Router Navigation" }
            
            Field { grouped: true,
                Control {
                    Link {
                        to: Route::Layout {},
                        class: "button is-primary",
                        "View Layout Components"
                    }
                }
                Control {
                    Link {
                        to: Route::Elements {},
                        class: "button is-info",
                        "View Elements"
                    }
                }
                Control {
                    Link {
                        to: Route::Forms {},
                        class: "button is-success",
                        "View Forms"
                    }
                }
                Control {
                    Link {
                        to: Route::Navigation {},
                        class: "button is-warning",
                        "View Navigation"
                    }
                }
                Control {
                    Link {
                        to: Route::Components {},
                        class: "button is-danger",
                        "View Components"
                    }
                }
            }
        }
    }
}

#[component]
fn Layout() -> Element {
    rsx! {
        Title { size: TitleSize::Is2, "Layout Components" }
        Content { "Layout components help structure your application with responsive design patterns." }

        // Container Demo
        Title { size: TitleSize::Is3, "Container" }
        div { class: "box",
            Container {
                Content { "This content is centered and has responsive max-width based on breakpoints." }
            }
            
            details { class: "mt-4",
                summary { "View Code" }
                pre {
                    code {
                        "Container {{\n    Content {{ \"Responsive centered content\" }}\n}}"
                    }
                }
            }
        }

        // Columns Demo
        Title { size: TitleSize::Is3, "Columns & Grid System" }
        div { class: "box",
            Columns {
                Column { size: ColumnSize::Half,
                    div { class: "box has-background-primary-light",
                        Content { "Half width column" }
                    }
                }
                Column { size: ColumnSize::OneQuarter,
                    div { class: "box has-background-info-light",
                        Content { "Quarter width" }
                    }
                }
                Column { size: ColumnSize::OneQuarter,
                    div { class: "box has-background-success-light",
                        Content { "Quarter width" }
                    }
                }
            }
            
            Columns {
                mobile: true,
                Column { size: ColumnSize::OneThird,
                    div { class: "box has-background-warning-light",
                        Content { "One third" }
                    }
                }
                Column { size: ColumnSize::TwoThirds,
                    div { class: "box has-background-danger-light",
                        Content { "Two thirds" }
                    }
                }
            }
        }

        // Navigation
        div { class: "mt-6",
            Link {
                to: Route::Home {},
                class: "button",
                "← Back to Home"
            }
        }
    }
}

#[component]
fn Elements() -> Element {
    rsx! {
        Title { size: TitleSize::Is2, "Element Components" }
        Content { "Basic UI elements with various styling options and states." }

        // Button Demo
        Title { size: TitleSize::Is3, "Buttons" }
        div { class: "box",
            Title { size: TitleSize::Is4, "Colors" }
            Field { grouped: true, class: "is-grouped-multiline",
                Control { Button { color: BulmaColor::Primary, "Primary" } }
                Control { Button { color: BulmaColor::Link, "Link" } }
                Control { Button { color: BulmaColor::Info, "Info" } }
                Control { Button { color: BulmaColor::Success, "Success" } }
                Control { Button { color: BulmaColor::Warning, "Warning" } }
                Control { Button { color: BulmaColor::Danger, "Danger" } }
            }

            Title { size: TitleSize::Is5, "Variants" }
            Field { grouped: true, class: "is-grouped-multiline",
                Control { Button { color: BulmaColor::Primary, outlined: true, "Outlined" } }
                Control { Button { color: BulmaColor::Info, rounded: true, "Rounded" } }
                Control { Button { color: BulmaColor::Success, loading: true, "Loading" } }
                Control { Button { disabled: true, "Disabled" } }
            }
        }

        // Typography Demo
        Title { size: TitleSize::Is3, "Typography" }
        div { class: "box",
            Title { size: TitleSize::Is1, "Title Is1" }
            Title { size: TitleSize::Is2, "Title Is2" }
            Title { size: TitleSize::Is3, "Title Is3" }
            Title { size: TitleSize::Is4, "Title Is4" }
            Title { size: TitleSize::Is5, "Title Is5" }
            Title { size: TitleSize::Is6, "Title Is6" }
            
            Subtitle { size: TitleSize::Is3, "Subtitle Is3" }
            Subtitle { size: TitleSize::Is4, "Subtitle Is4" }
            Subtitle { size: TitleSize::Is5, "Subtitle Is5" }
            Subtitle { size: TitleSize::Is6, "Subtitle Is6" }
        }

        // Navigation
        div { class: "mt-6",
            Link {
                to: Route::Home {},
                class: "button",
                "← Back to Home"
            }
        }
    }
}

#[component]
fn Forms() -> Element {
    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);

    rsx! {
        Title { size: TitleSize::Is2, "Form Components" }
        Content { "Interactive form elements with validation states and various input types." }

        // Basic Form Demo
        Title { size: TitleSize::Is3, "Complete Form Example" }
        div { class: "box",
            Card {
                CardHeader { CardHeaderTitle { "Contact Form" } }
                CardContent {
                    // Text Input
                    Field {
                        Label { "Full Name" }
                        Control {
                            Input {
                                input_type: InputType::Text,
                                placeholder: "Enter your full name",
                                value: name(),
                                oninput: move |evt: FormEvent| name.set(evt.value())
                            }
                        }
                        Help { "Your full name as you'd like it to appear" }
                    }

                    // Email Input with Validation
                    Field {
                        Label { "Email Address" }
                        Control {
                            Input {
                                input_type: InputType::Email,
                                placeholder: "Enter your email",
                                value: email(),
                                color: if !email().is_empty() && 
                                       (!email().contains('@') || !email().contains('.')) {
                                    Some(BulmaColor::Danger)
                                } else if email().contains('@') && email().contains('.') {
                                    Some(BulmaColor::Success)
                                } else {
                                    None
                                },
                                oninput: move |evt: FormEvent| email.set(evt.value())
                            }
                        }
                        if !email().is_empty() && 
                           (!email().contains('@') || !email().contains('.')) {
                            Help { color: BulmaColor::Danger, "Please enter a valid email address" }
                        } else if email().contains('@') && email().contains('.') {
                            Help { color: BulmaColor::Success, "Email looks good!" }
                        }
                    }
                    
                    // Form Actions
                    Field { grouped: true,
                        Control {
                            Button {
                                color: BulmaColor::Primary,
                                size: BulmaSize::Medium,
                                onclick: move |_| {
                                    println!("Form submitted: {} - {}", name(), email());
                                },
                                "Send Message"
                            }
                        }
                        Control {
                            Button {
                                onclick: move |_| {
                                    name.set(String::new());
                                    email.set(String::new());
                                },
                                "Clear Form"
                            }
                        }
                    }
                }
            }
        }

        // Navigation
        div { class: "mt-6",
            Link {
                to: Route::Home {},
                class: "button",
                "← Back to Home"
            }
        }
    }
}

#[component]
fn Navigation() -> Element {
    let mut current_page = use_signal(|| 2);

    rsx! {
        Title { size: TitleSize::Is2, "Navigation Components" }
        Content { "Components for site navigation, pagination, and organizing content." }

        // Breadcrumb Demo (using href for now)
        Title { size: TitleSize::Is3, "Breadcrumb" }
        div { class: "box",
            Breadcrumb {
                size: BulmaSize::Medium,
                BreadcrumbItem { href: "/", "Home" }
                BreadcrumbItem { href: "/components", "Components" }
                BreadcrumbItem { href: "/navigation", "Navigation" }
                BreadcrumbItem { active: true, "Breadcrumb Demo" }
            }
        }

        // Pagination Demo
        Title { size: TitleSize::Is3, "Pagination" }
        div { class: "box",
            Content { "Current page: {current_page()}" }
            
            Pagination {
                size: BulmaSize::Medium,
                alignment: PaginationAlignment::Centered,
                PaginationPrevious { 
                    disabled: current_page() <= 1,
                    onclick: move |_| if current_page() > 1 { current_page.set(current_page() - 1) },
                    "Previous" 
                }
                PaginationNext { 
                    disabled: current_page() >= 10,
                    onclick: move |_| if current_page() < 10 { current_page.set(current_page() + 1) },
                    "Next page" 
                }
                PaginationList {
                    PaginationLink { 
                        current: current_page() == 1,
                        onclick: move |_| current_page.set(1),
                        "1" 
                    }
                    if current_page() > 3 {
                        PaginationEllipsis {}
                    }
                    
                    PaginationLink { 
                        current: current_page() == 10,
                        onclick: move |_| current_page.set(10),
                        "10" 
                    }
                }
            }
        }

        // Navigation
        div { class: "mt-6",
            Link {
                to: Route::Home {},
                class: "button",
                "← Back to Home"
            }
        }
    }
}

#[component]
fn Components() -> Element {
    let mut show_modal = use_signal(|| false);
    let mut dropdown_active = use_signal(|| false);

    rsx! {
        Title { size: TitleSize::Is2, "Advanced Components" }
        Content { "Complex interactive components for building rich user interfaces." }

        // Modal Demo
        Title { size: TitleSize::Is3, "Modal" }
        div { class: "box",
            Button {
                color: BulmaColor::Primary,
                size: BulmaSize::Large,
                onclick: move |_| show_modal.set(true),
                "Launch Modal"
            }

            Modal {
                active: show_modal(),
                onclose: move |_| show_modal.set(false),
                ModalCard {
                    ModalCardHead {
                        onclose: move |_| show_modal.set(false),
                        Title { size: TitleSize::Is4, "Modal Dialog" }
                    }
                    ModalCardBody {
                        Content {
                            p { "This is a modal dialog demonstrating the modal component structure." }
                            p { "It includes a header with close button, body content, and footer actions." }
                        }
                    }
                    ModalCardFoot {
                        Button {
                            color: BulmaColor::Success,
                            onclick: move |_| show_modal.set(false),
                            "Save Changes"
                        }
                        Button {
                            onclick: move |_| show_modal.set(false),
                            "Cancel"
                        }
                    }
                }
            }
        }

        // Dropdown Demo
        Title { size: TitleSize::Is3, "Dropdown Menu" }
        div { class: "box",
            Dropdown {
                active: dropdown_active(),
                DropdownTrigger {
                    onclick: move |_| dropdown_active.set(!dropdown_active()),
                    Button {
                        color: BulmaColor::Primary,
                        "Dropdown Menu"
                        span { class: "icon is-small ml-2", 
                            if dropdown_active() { "▲" } else { "▼" }
                        }
                    }
                }
                DropdownMenu {
                    DropdownItem { "Action" }
                    DropdownItem { "Another action" }
                    DropdownItem { "Something else here" }
                    DropdownDivider {}
                    DropdownItem { active: true, "Active item" }
                    DropdownItem { "Separated link" }
                }
            }
        }

        // Navigation
        div { class: "mt-6",
            Link {
                to: Route::Home {},
                class: "button",
                "← Back to Home"
            }
        }
    }
}