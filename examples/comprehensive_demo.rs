use dioxus::prelude::*;
use dioxus_bulma::*;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let mut active_dropdown = use_signal(|| false);
    let mut active_message = use_signal(|| true);
    let form_data = use_signal(|| FormData::default());
    let mut current_page = use_signal(|| 1);

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
                        Subtitle { size: TitleSize::Is4, "Showcasing all available components" }
                    }
                }
            }

            Section {
                Container {
                    // Navigation Components
                    Title { size: TitleSize::Is2, "Navigation Components" }
                    
                    // Breadcrumb
                    Title { size: TitleSize::Is4, "Breadcrumb" }
                    Breadcrumb {
                        size: BulmaSize::Medium,
                        BreadcrumbItem { href: "#", "Home" }
                        BreadcrumbItem { href: "#", "Documentation" }
                        BreadcrumbItem { active: true, "Components" }
                    }

                    // Tabs
                    div { class: "mt-5",
                        Title { size: TitleSize::Is4, "Tabs" }
                        Tabs {
                            style: TabsStyle::Toggle,
                            size: BulmaSize::Medium,
                            Tab { active: true, "Overview" }
                            Tab { "Components" }
                            Tab { "Examples" }
                            Tab { "Documentation" }
                        }
                    }

                    // Pagination
                    div { class: "mt-5",
                        Title { size: TitleSize::Is4, "Pagination" }
                        Pagination {
                            size: BulmaSize::Medium,
                            alignment: PaginationAlignment::Centered,
                            PaginationPrevious { "Previous" }
                            PaginationNext { "Next" }
                            PaginationList {
                                PaginationLink { href: "#", onclick: move |_| current_page.set(1), "1" }
                                PaginationLink { current: current_page() == 2, href: "#", onclick: move |_| current_page.set(2), "2" }
                                PaginationLink { href: "#", onclick: move |_| current_page.set(3), "3" }
                                PaginationEllipsis {}
                                PaginationLink { href: "#", onclick: move |_| current_page.set(45), "45" }
                            }
                        }
                    }
                }
            }

            Section {
                Container {
                    // Advanced Components
                    Title { size: TitleSize::Is2, "Advanced Components" }
                    
                    Columns {
                        multiline: true,
                        
                        // Dropdown
                        Column { size: ColumnSize::OneThird,
                            Card {
                                CardHeader { CardHeaderTitle { "Dropdown" } }
                                CardContent {
                                    Dropdown {
                                        active: active_dropdown(),
                                        DropdownTrigger {
                                            onclick: move |_| active_dropdown.set(!active_dropdown()),
                                            Button {
                                                color: BulmaColor::Primary,
                                                "Dropdown"
                                                span { class: "icon is-small", "▼" }
                                            }
                                        }
                                        DropdownMenu {
                                            DropdownItem { href: "#", "Action" }
                                            DropdownItem { "Another action" }
                                            DropdownDivider {}
                                            DropdownItem { active: true, "Active item" }
                                        }
                                    }
                                }
                            }
                        }

                        // Menu
                        Column { size: ColumnSize::OneThird,
                            Card {
                                CardHeader { CardHeaderTitle { "Menu" } }
                                CardContent {
                                    Menu {
                                        MenuLabel { "General" }
                                        MenuList {
                                            MenuItem { active: true, "Dashboard" }
                                            MenuItem { "Customers" }
                                        }
                                        MenuLabel { "Administration" }
                                        MenuList {
                                            MenuItem { "Team Settings" }
                                            MenuItem { "Manage Your Team" }
                                            MenuItem { "Invitations" }
                                        }
                                    }
                                }
                            }
                        }

                        // Panel
                        Column { size: ColumnSize::OneThird,
                            Panel {
                                color: BulmaColor::Primary,
                                PanelHeading { "Repositories" }
                                PanelTabs {
                                    "All"
                                    "Public"
                                    "Private" 
                                    "Sources"
                                    "Forks"
                                }
                                PanelBlock {
                                    PanelIcon { "📁" }
                                    "bulma"
                                }
                                PanelBlock {
                                    active: true,
                                    PanelIcon { "📁" }
                                    "dioxus-bulma"
                                }
                                PanelBlock {
                                    PanelIcon { "📁" }
                                    "dioxus"
                                }
                            }
                        }
                    }
                }
            }

            Section {
                Container {
                    // Message Component
                    if active_message() {
                        Message {
                            color: BulmaColor::Info,
                            MessageHeader {
                                closable: true,
                                onclose: move |_| active_message.set(false),
                                "Information"
                            }
                            MessageBody {
                                "This is a message component with a closable header. "
                                "You can dismiss it by clicking the close button."
                            }
                        }
                    }
                }
            }

            Section {
                Container {
                    // Form Components Showcase
                    Title { size: TitleSize::Is2, "Form Components" }
                    
                    FormDemo { form_data }
                }
            }

            Section {
                Container {
                    // Layout Components
                    Title { size: TitleSize::Is2, "Layout Components" }
                    
                    // Tile Layout
                    Title { size: TitleSize::Is4, "Tile Layout" }
                    Tile {
                        ancestor: true,
                        Tile {
                            vertical: true,
                            size: TileSize::Is8,
                            Tile {
                                Card {
                                    CardContent {
                                        Title { size: TitleSize::Is4, "Wide tile" }
                                        Subtitle { "Aligned with the right tile" }
                                        Content { "This tile is wider than its sibling." }
                                    }
                                }
                            }
                            Tile {
                                Card {
                                    CardContent {
                                        Title { size: TitleSize::Is4, "Wide tile" }
                                        Subtitle { "Aligned with the above tile" }
                                        Content { "This tile is stacked below the first one." }
                                    }
                                }
                            }
                        }
                        Tile {
                            parent: true,
                            Card {
                                CardContent {
                                    Title { size: TitleSize::Is4, "Tall tile" }
                                    Subtitle { "Aligned with the left tile" }
                                    Content { "This tile spans the full height." }
                                }
                            }
                        }
                    }

                    // Level
                    div { class: "mt-5",
                        Title { size: TitleSize::Is4, "Level" }
                        Level {
                            LevelLeft {
                                LevelItem {
                                    div {
                                        Title { size: TitleSize::Is5, "123" }
                                        div { class: "heading", "Posts" }
                                    }
                                }
                                LevelItem {
                                    div {
                                        Title { size: TitleSize::Is5, "456K" }
                                        div { class: "heading", "Following" }
                                    }
                                }
                            }
                            LevelRight {
                                LevelItem {
                                    Button { color: BulmaColor::Success, "New post" }
                                }
                            }
                        }
                    }
                }
            }

            Section {
                Container {
                    // Utility Components
                    Title { size: TitleSize::Is2, "Utility Components" }
                    
                    Columns {
                        // Progress
                        Column {
                            Title { size: TitleSize::Is4, "Progress" }
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
                        
                        // Table
                        Column {
                            Title { size: TitleSize::Is4, "Table" }
                            Table {
                                bordered: true,
                                striped: true,
                                hoverable: true,
                                thead {
                                    tr {
                                        th { "Name" }
                                        th { "Position" }
                                        th { "Date" }
                                    }
                                }
                                tbody {
                                    tr {
                                        td { "John Doe" }
                                        td { "Developer" }
                                        td { "2024-01-01" }
                                    }
                                    tr {
                                        td { "Jane Smith" }
                                        td { "Designer" }
                                        td { "2024-01-02" }
                                    }
                                    tr {
                                        td { "Bob Johnson" }
                                        td { "Manager" }
                                        td { "2024-01-03" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Footer
            footer { class: "footer",
                div { class: "content has-text-centered",
                    Content {
                        "Built with "
                        strong { "Dioxus" }
                        " and "
                        strong { "Bulma" }
                        ". The source code is available on GitHub."
                    }
                }
            }
        }
    }
}

#[derive(Default, Clone, PartialEq, Debug)]
struct FormData {
    name: String,
    email: String,
    message: String,
    newsletter: bool,
    priority: String,
    file_selected: bool,
}

#[component]
fn FormDemo(mut form_data: Signal<FormData>) -> Element {
    rsx! {
        Card {
            CardHeader { CardHeaderTitle { "Contact Form" } }
            CardContent {
                Columns {
                    Column {
                        // Text Input
                        Field {
                            Label { "Name" }
                            Control {
                                Input {
                                    input_type: InputType::Text,
                                    placeholder: "Enter your name",
                                    value: form_data().name,
                                    oninput: move |evt: FormEvent| {
                                        form_data.with_mut(|data| data.name = evt.value());
                                    }
                                }
                            }
                        }

                        // Email Input
                        Field {
                            Label { "Email" }
                            Control {
                                Input {
                                    input_type: InputType::Email,
                                    placeholder: "Enter your email",
                                    value: form_data().email,
                                    color: if !form_data().email.is_empty() && 
                                           (!form_data().email.contains('@') || !form_data().email.contains('.')) {
                                        Some(BulmaColor::Danger)
                                    } else if form_data().email.contains('@') && form_data().email.contains('.') {
                                        Some(BulmaColor::Success)
                                    } else {
                                        None
                                    },
                                    oninput: move |evt: FormEvent| {
                                        form_data.with_mut(|data| data.email = evt.value());
                                    }
                                }
                            }
                            if !form_data().email.is_empty() && 
                               (!form_data().email.contains('@') || !form_data().email.contains('.')) {
                                Help { color: BulmaColor::Danger, "Please enter a valid email" }
                            }
                        }

                        // Textarea
                        Field {
                            Label { "Message" }
                            Control {
                                Textarea {
                                    placeholder: "Enter your message here...",
                                    rows: 4,
                                    value: form_data().message,
                                    oninput: move |evt: FormEvent| {
                                        form_data.with_mut(|data| data.message = evt.value());
                                    }
                                }
                            }
                        }
                    }

                    Column {
                        // Select
                        Field {
                            Label { "Priority" }
                            Control {
                                Select {
                                    value: form_data().priority.clone(),
                                    onchange: move |evt: FormEvent| {
                                        form_data.with_mut(|data| data.priority = evt.value());
                                    },
                                    option { value: "", "Select priority..." }
                                    option { value: "low", "Low" }
                                    option { value: "medium", "Medium" }
                                    option { value: "high", "High" }
                                }
                            }
                        }

                        // Checkbox
                        Field {
                            Control {
                                Checkbox {
                                    checked: form_data().newsletter,
                                    onchange: move |evt: FormEvent| {
                                        form_data.with_mut(|data| data.newsletter = evt.checked());
                                    },
                                    "Subscribe to newsletter"
                                }
                            }
                        }

                        // Radio buttons
                        Field {
                            Label { "Contact method" }
                            Control {
                                Radio {
                                    name: "contact_method",
                                    value: "email",
                                    "Email"
                                }
                            }
                            Control {
                                Radio {
                                    name: "contact_method", 
                                    value: "phone",
                                    "Phone"
                                }
                            }
                            Control {
                                Radio {
                                    name: "contact_method",
                                    value: "both",
                                    "Both"
                                }
                            }
                        }

                        // File input
                        Field {
                            Label { "Attachment" }
                            File {
                                filename: if form_data().file_selected { "document.pdf" } else { "" },
                                input { r#type: "file" }
                                span { class: "file-cta",
                                    span { class: "file-icon", "📎" }
                                    span { class: "file-label", "Choose a file…" }
                                }
                            }
                        }

                        // Submit button
                        Field {
                            Control {
                                Button {
                                    color: BulmaColor::Primary,
                                    size: BulmaSize::Medium,
                                    fullwidth: true,
                                    onclick: move |_| {
                                        // Handle form submission
                                        let data = form_data();
                                        println!("Form submitted: {:#?}", data);
                                    },
                                    "Send Message"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}