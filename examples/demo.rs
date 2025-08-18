use dioxus::prelude::*;
use dioxus_bulma::*;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0i32);
    let mut message = use_signal(|| String::from("Hello, Bulma!"));
    let mut show_notification = use_signal(|| true);

    rsx! {
        BulmaProvider {
            theme: BulmaTheme::Auto,
            load_bulma_css: true,
            Section {
                Container {
                    Title { size: TitleSize::Is1, "Dioxus Bulma Demo" }
                    Subtitle { size: TitleSize::Is4, "A demonstration of Bulma components in Dioxus" }

                    if show_notification() {
                        Notification {
                            color: BulmaColor::Info,
                            light: true,
                            dismissible: true,
                            onclose: move |_| show_notification.set(false),
                            "Welcome to the Dioxus Bulma component library! This notification can be dismissed."
                        }
                    }

                    Columns {
                        multiline: true,
                        Column {
                            size: ColumnSize::Half,
                            div { class: "box",
                                Title { size: TitleSize::Is4, "Counter Example" }
                                p { "Count: {count}" }
                                div { class: "buttons",
                                    Button {
                                        color: BulmaColor::Primary,
                                        onclick: move |_| count += 1,
                                        "Increment"
                                    }
                                    Button {
                                        color: BulmaColor::Warning,
                                        onclick: move |_| count -= 1,
                                        "Decrement"  
                                    }
                                    Button {
                                        color: BulmaColor::Danger,
                                        outlined: true,
                                        onclick: move |_| count.set(0),
                                        "Reset"
                                    }
                                }
                            }
                        }

                        Column {
                            size: ColumnSize::Half,
                            div { class: "box",
                                Title { size: TitleSize::Is4, "Input Example" }
                                div { class: "field",
                                    label { class: "label", "Message" }
                                    div { class: "control",
                                        Input {
                                            value: message.read().clone(),
                                            placeholder: "Enter a message...",
                                            oninput: move |evt: FormEvent| {
                                                message.set(evt.value());
                                            }
                                        }
                                    }
                                }
                                p { "Your message: {message}" }
                            }
                        }
                    }

                    Columns {
                        Column {
                            div { class: "box",
                                Title { size: TitleSize::Is4, "Button Variants" }
                                div { class: "buttons",
                                    Button { color: BulmaColor::Primary, "Primary" }
                                    Button { color: BulmaColor::Link, "Link" }
                                    Button { color: BulmaColor::Info, "Info" }
                                    Button { color: BulmaColor::Success, "Success" }
                                    Button { color: BulmaColor::Warning, "Warning" }
                                    Button { color: BulmaColor::Danger, "Danger" }
                                }

                                Title { size: TitleSize::Is5, "Outlined Buttons" }
                                div { class: "buttons",
                                    Button { color: BulmaColor::Primary, outlined: true, "Primary" }
                                    Button { color: BulmaColor::Link, outlined: true, "Link" }
                                    Button { color: BulmaColor::Info, outlined: true, "Info" }
                                    Button { color: BulmaColor::Success, outlined: true, "Success" }
                                    Button { color: BulmaColor::Warning, outlined: true, "Warning" }
                                    Button { color: BulmaColor::Danger, outlined: true, "Danger" }
                                }

                                Title { size: TitleSize::Is5, "Button Sizes" }
                                div { class: "buttons",
                                    Button { size: BulmaSize::Small, color: BulmaColor::Primary, "Small" }
                                    Button { size: BulmaSize::Normal, color: BulmaColor::Primary, "Normal" }
                                    Button { size: BulmaSize::Medium, color: BulmaColor::Primary, "Medium" }
                                    Button { size: BulmaSize::Large, color: BulmaColor::Primary, "Large" }
                                }

                                Title { size: TitleSize::Is5, "Special Buttons" }
                                div { class: "buttons",
                                    Button { color: BulmaColor::Primary, rounded: true, "Rounded" }
                                    Button { color: BulmaColor::Info, loading: true, "Loading" }
                                    Button { color: BulmaColor::Danger, disabled: true, "Disabled" }
                                }
                            }
                        }
                    }

                    Columns {
                        Column {
                            div { class: "box",
                                Title { size: TitleSize::Is4, "Notifications" }
                                Button {
                                    color: BulmaColor::Info,
                                    onclick: move |_| show_notification.set(true),
                                    "Show Notification Again"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}