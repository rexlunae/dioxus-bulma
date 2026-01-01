use dioxus::prelude::*;
use dioxus_bulma::prelude::*;
use dioxus_bulma::components::{
    Title, Subtitle, TitleSize,
    TabsAlignment, TabsStyle,
    PaginationAlignment,
    InputType,
    ColumnSize,
    ImageSize,
    Label, Help,
};

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let mut current_section = use_signal(|| "elements".to_string());
    let mut show_modal = use_signal(|| false);
    let mut show_notification = use_signal(|| true);
    let counter = use_signal(|| 0i32);
    let name = use_signal(String::new);
    let email = use_signal(String::new);
    let message = use_signal(String::new);
    let dropdown_active = use_signal(|| false);
    let current_page = use_signal(|| 1);

    rsx! {
        BulmaProvider {
            theme: BulmaTheme::Auto,
            load_bulma_css: true,
            
            // Hero Section
            Hero {
                color: BulmaColor::Primary,
                size: BulmaSize::Large,
                HeroBody {
                    Container {
                        Title { size: TitleSize::Is1, "Dioxus Bulma" }
                        Title { size: TitleSize::Is3, "Comprehensive Component Demo" }
                        Subtitle { size: TitleSize::Is4, "All components with live examples and code snippets" }
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
                            "Welcome to the comprehensive Dioxus Bulma demo! Explore all components with interactive examples."
                        }
                    }
                }
            }

            Section {
                Container {
                    // Navigation Tabs
                    Tabs {
                        style: TabsStyle::Boxed,
                        size: BulmaSize::Large,
                        // centered: true,
                        Tab {
                            active: current_section() == "elements",
                            onclick: move |_| current_section.set("elements".to_string()),
                            "Elements"
                        }
                        Tab {
                            active: current_section() == "forms",
                            onclick: move |_| current_section.set("forms".to_string()),
                            "Forms"
                        }
                        Tab {
                            active: current_section() == "layout",
                            onclick: move |_| current_section.set("layout".to_string()),
                            "Layout"
                        }
                        Tab {
                            active: current_section() == "components",
                            onclick: move |_| current_section.set("components".to_string()),
                            "Components"
                        }
                        Tab {
                            active: current_section() == "navigation",
                            onclick: move |_| current_section.set("navigation".to_string()),
                            "Navigation"
                        }
                    }

                    // Content based on selected section
                    match current_section().as_str() {
                        "elements" => rsx! { ElementsSection { counter } },
                        "forms" => rsx! { FormsSection { name, email, message } },
                        "layout" => rsx! { LayoutSection {} },
                        "components" => rsx! { ComponentsSection { show_modal, dropdown_active } },
                        "navigation" => rsx! { NavigationSection { current_page } },
                        _ => rsx! { ElementsSection { counter } },
                    }
                }
            }

            // Modal (positioned outside main content)
            Modal {
                active: show_modal(),
                onclose: move |_| show_modal.set(false),
                ModalCard {
                    ModalCardHead {
                        onclose: move |_| show_modal.set(false),
                        Title { size: TitleSize::Is4, "Example Modal Dialog" }
                    }
                    ModalCardBody {
                        Content {
                            p { "This is a comprehensive modal example demonstrating the modal component structure." }
                            p { "The modal includes a header with title and close button, body content area, and footer with action buttons." }
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
    }
}

#[component]
fn ElementsSection(counter: Signal<i32>) -> Element {
    rsx! {
        div { class: "content",
            Title { size: TitleSize::Is2, "Elements" }
            Subtitle { "Basic UI elements with various styling options" }

            // Interactive Counter Demo
            Card {
                CardHeader { CardHeaderTitle { "Interactive Counter Example" } }
                CardContent {
                    div { class: "has-text-centered",
                        Title { size: TitleSize::Is1, "{counter}" }
                        Field { grouped: true, class: "is-justify-content-center",
                            Control {
                                Button {
                                    color: BulmaColor::Success,
                                    size: BulmaSize::Large,
                                    onclick: move |_| counter += 1,
                                    "Increment"
                                }
                            }
                            Control {
                                Button {
                                    color: BulmaColor::Danger,
                                    size: BulmaSize::Large,
                                    onclick: move |_| counter -= 1,
                                    "Decrement"
                                }
                            }
                            Control {
                                Button {
                                    color: BulmaColor::Warning,
                                    size: BulmaSize::Large,
                                    outlined: true,
                                    onclick: move |_| counter.set(0),
                                    "Reset"
                                }
                            }
                        }
                    }
                }
            }

            // Button Showcase
            div { class: "mt-6",
                Title { size: TitleSize::Is3, "Buttons" }
                
                Card {
                    CardContent {
                        Title { size: TitleSize::Is4, "Button Colors" }
                        Field { grouped: true, class: "is-grouped-multiline",
                            Control { Button { color: BulmaColor::Primary, "Primary" } }
                            Control { Button { color: BulmaColor::Link, "Link" } }
                            Control { Button { color: BulmaColor::Info, "Info" } }
                            Control { Button { color: BulmaColor::Success, "Success" } }
                            Control { Button { color: BulmaColor::Warning, "Warning" } }
                            Control { Button { color: BulmaColor::Danger, "Danger" } }
                        }

                        Title { size: TitleSize::Is5, "Button Variants" }
                        Field { grouped: true, class: "is-grouped-multiline",
                            Control { Button { color: BulmaColor::Primary, outlined: true, "Outlined" } }
                            Control { Button { color: BulmaColor::Info, rounded: true, "Rounded" } }
                            Control { Button { color: BulmaColor::Success, loading: true, "Loading" } }
                            Control { Button { disabled: true, "Disabled" } }
                            Control { Button { color: BulmaColor::Warning, "Warning" } }
                        }

                        Title { size: TitleSize::Is5, "Button Sizes" }
                        Field { grouped: true, class: "is-grouped-multiline",
                            Control { Button { size: BulmaSize::Small, color: BulmaColor::Primary, "Small" } }
                            Control { Button { size: BulmaSize::Normal, color: BulmaColor::Primary, "Normal" } }
                            Control { Button { size: BulmaSize::Medium, color: BulmaColor::Primary, "Medium" } }
                            Control { Button { size: BulmaSize::Large, color: BulmaColor::Primary, "Large" } }
                        }
                    }
                }
            }

            // Typography
            div { class: "mt-6",
                Title { size: TitleSize::Is3, "Typography" }
                
                Card {
                    CardContent {
                        Title { size: TitleSize::Is1, "Title Is1 - Main Heading" }
                        Title { size: TitleSize::Is2, "Title Is2 - Section Heading" }
                        Title { size: TitleSize::Is3, "Title Is3 - Subsection" }
                        Title { size: TitleSize::Is4, "Title Is4 - Component Title" }
                        Title { size: TitleSize::Is5, "Title Is5 - Small Title" }
                        Title { size: TitleSize::Is6, "Title Is6 - Smallest Title" }
                        
                        hr {}
                        
                        Subtitle { size: TitleSize::Is3, "Subtitle Is3" }
                        Subtitle { size: TitleSize::Is4, "Subtitle Is4" }
                        Subtitle { size: TitleSize::Is5, "Subtitle Is5" }
                        Subtitle { size: TitleSize::Is6, "Subtitle Is6" }
                    }
                }
            }

            // Tags
            div { class: "mt-6",
                Title { size: TitleSize::Is3, "Tags" }
                
                Card {
                    CardContent {
                        Title { size: TitleSize::Is4, "Tag Colors & Variants" }
                        Tags {
                            Tag { color: BulmaColor::Primary, "Primary" }
                            Tag { color: BulmaColor::Info, "Info" }
                            Tag { color: BulmaColor::Success, "Success" }
                            Tag { color: BulmaColor::Warning, "Warning" }
                            Tag { color: BulmaColor::Danger, "Danger" }
                        }
                        
                        Tags {
                            Tag { color: BulmaColor::Primary, light: true, "Light Primary" }
                            Tag { color: BulmaColor::Info, rounded: true, "Rounded" }
                            Tag { color: BulmaColor::Success, size: BulmaSize::Medium, "Medium Size" }
                            Tag { color: BulmaColor::Warning, size: BulmaSize::Large, "Large Size" }
                        }
                    }
                }
            }

            // Notifications
            div { class: "mt-6",
                Title { size: TitleSize::Is3, "Notifications" }
                
                Card {
                    CardContent {
                        Notification {
                            color: BulmaColor::Primary,
                            "Primary notification - great for highlighting important information"
                        }
                        
                        Notification {
                            color: BulmaColor::Success,
                            light: true,
                            "Success notification with light styling - perfect for confirmation messages"
                        }
                        
                        Notification {
                            color: BulmaColor::Warning,
                            "Warning notification - use for cautionary messages"
                        }
                        
                        Notification {
                            color: BulmaColor::Danger,
                            light: true,
                            "Danger notification with light styling - for error messages"
                        }
                    }
                }
            }

            // Progress bars
            div { class: "mt-6",
                Title { size: TitleSize::Is3, "Progress" }
                
                Card {
                    CardContent {
                        Progress { 
                            color: BulmaColor::Primary, 
                            value: 15.0, 
                            max: 100.0,
                            "15%"
                        }
                        Progress { 
                            color: BulmaColor::Info, 
                            value: 30.0, 
                            max: 100.0,
                            "30%"
                        }
                        Progress { 
                            color: BulmaColor::Success, 
                            value: 45.0, 
                            max: 100.0,
                            "45%"
                        }
                        Progress { 
                            color: BulmaColor::Warning, 
                            value: 60.0, 
                            max: 100.0,
                            "60%"
                        }
                        Progress { 
                            color: BulmaColor::Danger, 
                            value: 90.0, 
                            max: 100.0,
                            "90%"
                        }
                    }
                }
            }

            // Table
            div { class: "mt-6",
                Title { size: TitleSize::Is3, "Table" }
                
                Card {
                    CardContent {
                        Table {
                            bordered: true,
                            striped: true,
                            hoverable: true,
                            thead {
                                tr {
                                    th { "Name" }
                                    th { "Position" }
                                    th { "Date" }
                                    th { "Status" }
                                }
                            }
                            tbody {
                                tr {
                                    td { "John Doe" }
                                    td { "Developer" }
                                    td { "2024-01-01" }
                                    td { Tag { color: BulmaColor::Success, "Active" } }
                                }
                                tr {
                                    td { "Jane Smith" }
                                    td { "Designer" }
                                    td { "2024-01-02" }
                                    td { Tag { color: BulmaColor::Info, "Away" } }
                                }
                                tr {
                                    td { "Bob Johnson" }
                                    td { "Manager" }
                                    td { "2024-01-03" }
                                    td { Tag { color: BulmaColor::Warning, "Busy" } }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FormsSection(name: Signal<String>, email: Signal<String>, message: Signal<String>) -> Element {
    let mut priority = use_signal(String::new);
    let mut newsletter = use_signal(|| false);
    let mut contact_method = use_signal(|| "email".to_string());

    rsx! {
        div { class: "content",
            Title { size: TitleSize::Is2, "Forms" }
            Subtitle { "Interactive form components with validation states" }

            Card {
                CardHeader { CardHeaderTitle { "Complete Form Example" } }
                CardContent {
                    Columns {
                        Column {
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
                                Help { "Your name as you'd like it to appear" }
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

                            // Textarea
                            Field {
                                Label { "Message" }
                                Control {
                                    Textarea {
                                        placeholder: "Enter your message here...",
                                        rows: 4,
                                        value: message(),
                                        oninput: move |evt: FormEvent| message.set(evt.value())
                                    }
                                }
                                Help { "Tell us what's on your mind" }
                            }
                        }

                        Column {
                            // Select
                            Field {
                                Label { "Priority Level" }
                                Control {
                                    Select {
                                        value: priority(),
                                        onchange: move |evt: FormEvent| priority.set(evt.value()),
                                        option { value: "", "Select priority..." }
                                        option { value: "low", "Low Priority" }
                                        option { value: "medium", "Medium Priority" }
                                        option { value: "high", "High Priority" }
                                        option { value: "urgent", "Urgent" }
                                    }
                                }
                            }

                            // Checkbox
                            Field {
                                Control {
                                    Checkbox {
                                        checked: newsletter(),
                                        onchange: move |evt: FormEvent| newsletter.set(evt.checked()),
                                        "Subscribe to our newsletter"
                                    }
                                }
                                Help { "We'll send you updates about new features" }
                            }

                            // Radio buttons
                            Field {
                                Label { "Preferred Contact Method" }
                                Control {
                                    Radio {
                                        name: "contact_method",
                                        value: "email",
                                        checked: contact_method() == "email",
                                        onchange: move |_| contact_method.set("email".to_string()),
                                        "Email"
                                    }
                                }
                                Control {
                                    Radio {
                                        name: "contact_method", 
                                        value: "phone",
                                        checked: contact_method() == "phone",
                                        onchange: move |_| contact_method.set("phone".to_string()),
                                        "Phone"
                                    }
                                }
                                Control {
                                    Radio {
                                        name: "contact_method",
                                        value: "both",
                                        checked: contact_method() == "both",
                                        onchange: move |_| contact_method.set("both".to_string()),
                                        "Both"
                                    }
                                }
                            }

                            // File input
                            Field {
                                Label { "Attachment (Optional)" }
                                File {
                                    input { r#type: "file", accept: ".pdf,.doc,.docx" }
                                    span { class: "file-cta",
                                        span { class: "file-icon", "📎" }
                                        span { class: "file-label", "Choose a file…" }
                                    }
                                }
                            }

                            // Display current form values
                            div { class: "mt-4",
                                Title { size: TitleSize::Is5, "Current Form Values" }
                                Content {
                                    p { "Name: {name()}" }
                                    p { "Email: {email()}" }
                                    p { "Message: {message()}" }
                                    p { "Priority: {priority()}" }
                                    p { "Newsletter: {newsletter()}" }
                                    p { "Contact Method: {contact_method()}" }
                                }
                            }
                        }
                    }

                    // Form Actions
                    Field { grouped: true, class: "is-justify-content-center",
                        Control {
                            Button {
                                color: BulmaColor::Primary,
                                size: BulmaSize::Medium,
                                onclick: move |_| {
                                    let form_data = format!(
                                        "Name: {}, Email: {}, Message: {}, Priority: {}, Newsletter: {}, Contact: {}",
                                        name(), email(), message(), priority(), newsletter(), contact_method()
                                    );
                                    println!("Form submitted: {}", form_data);
                                },
                                "Send Message"
                            }
                        }
                        Control {
                            Button {
                                onclick: move |_| {
                                    name.set(String::new());
                                    email.set(String::new());
                                    message.set(String::new());
                                    priority.set(String::new());
                                    newsletter.set(false);
                                    contact_method.set("email".to_string());
                                },
                                "Clear Form"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LayoutSection() -> Element {
    rsx! {
        div { class: "content",
            Title { size: TitleSize::Is2, "Layout Components" }
            Subtitle { "Structural components for organizing content" }

            // Container
            Title { size: TitleSize::Is3, "Container" }
            div { class: "box",
                Container {
                    Content { 
                        "This content is wrapped in a Container component, which provides responsive max-width "
                        "and centers content on larger screens."
                    }
                }
            }

            // Columns System
            div { class: "mt-6",
                Title { size: TitleSize::Is3, "Columns & Grid System" }
                
                Card {
                    CardContent {
                        Title { size: TitleSize::Is4, "Basic Columns" }
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

                        Title { size: TitleSize::Is5, "Responsive Columns" }
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

                        Title { size: TitleSize::Is5, "Multiline Columns" }
                        Columns {
                            multiline: true,
                            Column { size: ColumnSize::OneQuarter,
                                div { class: "box has-background-primary-light", Content { "1/4" } }
                            }
                            Column { size: ColumnSize::OneQuarter,
                                div { class: "box has-background-info-light", Content { "1/4" } }
                            }
                            Column { size: ColumnSize::OneQuarter,
                                div { class: "box has-background-success-light", Content { "1/4" } }
                            }
                            Column { size: ColumnSize::OneQuarter,
                                div { class: "box has-background-warning-light", Content { "1/4" } }
                            }
                            Column { size: ColumnSize::Half,
                                div { class: "box has-background-danger-light", Content { "1/2 (wraps to new line)" } }
                            }
                            Column { size: ColumnSize::OneQuarter,
                                div { class: "box has-background-primary-light", Content { "1/4" } }
                            }
                        }
                    }
                }
            }

            // Section
            div { class: "mt-6",
                Title { size: TitleSize::Is3, "Section" }
                
                Section {
                    div { class: "box has-background-info-light",
                        Title { size: TitleSize::Is4, "Small Section" }
                        Content { "This is a small section with reduced padding." }
                    }
                }
                
                Section {
                    div { class: "box has-background-success-light",
                        Title { size: TitleSize::Is4, "Medium Section" }
                        Content { "This is a medium section with standard padding." }
                    }
                }
                
                Section {
                    div { class: "box has-background-warning-light",
                        Title { size: TitleSize::Is4, "Large Section" }
                        Content { "This is a large section with extra padding." }
                    }
                }
            }

            // Hero
            div { class: "mt-6",
                Title { size: TitleSize::Is3, "Hero" }
                
                Hero {
                    color: BulmaColor::Primary,
                    size: BulmaSize::Medium,
                    HeroBody {
                        Container {
                            Title { size: TitleSize::Is2, "Hero Section" }
                            Subtitle { size: TitleSize::Is4, "Perfect for landing pages and feature highlights" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ComponentsSection(show_modal: Signal<bool>, dropdown_active: Signal<bool>) -> Element {
    let mut message_visible = use_signal(|| true);

    rsx! {
        div { class: "content",
            Title { size: TitleSize::Is2, "Components" }
            Subtitle { "Advanced interactive components" }

            // Cards
            Title { size: TitleSize::Is3, "Cards" }
            Columns {
                multiline: true,
                
                Column { size: ColumnSize::Half,
                    Card {
                        CardHeader {
                            CardHeaderTitle { "Card with Header" }
                            // CardHeaderIcon not available yet
                        }
                        CardContent {
                            Content {
                                "Cards are versatile containers for displaying structured content. "
                                "This card includes a header with title and icon, content area, and footer with actions."
                                br {}
                                small { "Last updated: January 2024" }
                            }
                        }
                        CardFooter {
                            CardFooterItem { "Save" }
                            CardFooterItem { "Edit" }
                            CardFooterItem { "Delete" }
                        }
                    }
                }
                
                Column { size: ColumnSize::Half,
                    Card {
                        CardContent {
                            Media {
                                MediaLeft {
                                    Image { 
                                        size: ImageSize::Is64x64,
                                        div {
                                            class: "has-background-primary has-text-white is-flex is-align-items-center is-justify-content-center",
                                            style: "width: 64px; height: 64px; border-radius: 6px; font-size: 24px;",
                                            "👤"
                                        }
                                    }
                                }
                                MediaContent {
                                    Title { size: TitleSize::Is4, "John Doe" }
                                    Subtitle { size: TitleSize::Is6, "@johndoe" }
                                }
                            }
                            Content {
                                "This card demonstrates the media object pattern, perfect for user profiles, "
                                "comments, or any content that combines an image with text."
                                br {}
                                small { "Member since 2020" }
                            }
                        }
                    }
                }
            }

            // Modal
            div { class: "mt-6",
                Title { size: TitleSize::Is3, "Modal" }
                
                Card {
                    CardContent {
                        Content { "Modals provide focused interactions without navigating away from the current page." }
                        
                        Button {
                            color: BulmaColor::Primary,
                            size: BulmaSize::Large,
                            onclick: move |_| show_modal.set(true),
                            "Launch Modal Demo"
                        }
                    }
                }
            }

            // Dropdown
            div { class: "mt-6",
                Title { size: TitleSize::Is3, "Dropdown" }
                
                Card {
                    CardContent {
                        Content { "Dropdown menus provide contextual actions and navigation options." }
                        
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
                                DropdownItem { href: "#", "Profile Settings" }
                                DropdownItem { href: "#", "Account" }
                                DropdownItem { href: "#", "Preferences" }
                                DropdownDivider {}
                                DropdownItem { active: true, "Current Item" }
                                DropdownItem { href: "#", "Help & Support" }
                                DropdownDivider {}
                                DropdownItem { href: "#", "Sign Out" }
                            }
                        }
                    }
                }
            }

            // Message
            div { class: "mt-6",
                Title { size: TitleSize::Is3, "Message" }
                
                if message_visible() {
                    Message {
                        color: BulmaColor::Info,
                        MessageHeader {
                            closable: true,
                            onclose: move |_| message_visible.set(false),
                            "Information Message"
                        }
                        MessageBody {
                            "This is a message component with a closable header. Messages are perfect for "
                            "displaying alerts, information, or temporary notifications that users can dismiss. "
                            "Click the × button to close this message."
                        }
                    }
                } else {
                    Card {
                        CardContent {
                            Content { "The message above was dismissed. " }
                            Button {
                                color: BulmaColor::Info,
                                onclick: move |_| message_visible.set(true),
                                "Show Message Again"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn NavigationSection(current_page: Signal<i32>) -> Element {
    let mut active_tab = use_signal(|| 0);

    rsx! {
        div { class: "content",
            Title { size: TitleSize::Is2, "Navigation" }
            Subtitle { "Components for site navigation and organization" }

            // Breadcrumb
            Title { size: TitleSize::Is3, "Breadcrumb" }
            Card {
                CardContent {
                    Breadcrumb {
                        size: BulmaSize::Medium,
                        BreadcrumbItem { href: "#", "Home" }
                        BreadcrumbItem { href: "#", "Documentation" }
                        BreadcrumbItem { href: "#", "Components" }
                        BreadcrumbItem { active: true, "Navigation" }
                    }
                }
            }

            // Tabs
            div { class: "mt-6",
                Title { size: TitleSize::Is3, "Tabs" }
                
                Card {
                    CardContent {
                        Tabs {
                            style: TabsStyle::Boxed,
                            size: BulmaSize::Medium,
                            Tab {
                                active: active_tab() == 0,
                                onclick: move |_| active_tab.set(0),
                                "Overview"
                            }
                            Tab {
                                active: active_tab() == 1,
                                onclick: move |_| active_tab.set(1),
                                "Features"
                            }
                            Tab {
                                active: active_tab() == 2,
                                onclick: move |_| active_tab.set(2),
                                "Documentation"
                            }
                            Tab {
                                active: active_tab() == 3,
                                onclick: move |_| active_tab.set(3),
                                "Support"
                            }
                        }
                        
                        div { class: "content mt-4",
                            match active_tab() {
                                0 => rsx! { 
                                    Title { size: TitleSize::Is4, "Overview" }
                                    Content { "This tab shows an overview of the content." }
                                },
                                1 => rsx! { 
                                    Title { size: TitleSize::Is4, "Features" }
                                    Content { "Here you can learn about the key features." }
                                },
                                2 => rsx! { 
                                    Title { size: TitleSize::Is4, "Documentation" }
                                    Content { "Complete documentation and API reference." }
                                },
                                3 => rsx! { 
                                    Title { size: TitleSize::Is4, "Support" }
                                    Content { "Get help and support for your questions." }
                                },
                                _ => rsx! { Content { "Select a tab above" } }
                            }
                        }
                    }
                }
            }

            // Pagination
            div { class: "mt-6",
                Title { size: TitleSize::Is3, "Pagination" }
                
                Card {
                    CardContent {
                        Content { "Current page: {current_page()}/10" }
                        
                        Pagination {
                            size: BulmaSize::Medium,
                            alignment: PaginationAlignment::Centered,
                            PaginationPrevious { 
                                disabled: current_page() <= 1,
                                onclick: move |_| if current_page() > 1 { 
                                    current_page.set(current_page() - 1) 
                                },
                                "Previous" 
                            }
                            PaginationNext { 
                                disabled: current_page() >= 10,
                                onclick: move |_| if current_page() < 10 { 
                                    current_page.set(current_page() + 1) 
                                },
                                "Next" 
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
                                if current_page() > 2 && current_page() < 9 {
                                    PaginationLink { 
                                        current: true,
                                        "{current_page()}"
                                    }
                                }
                                if current_page() < 8 {
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
                }
            }
        }
    }
}