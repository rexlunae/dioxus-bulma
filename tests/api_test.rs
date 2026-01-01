use dioxus_bulma::prelude::*;
use dioxus::prelude::*;

#[test]
fn test_enum_visibility() {
    // Test that all enums are publicly accessible
    let _color = BulmaColor::Primary;
    let _size = BulmaSize::Large;
    let _input_type = InputType::Email;
    let _theme = BulmaTheme::Dark;
    let _alignment = ButtonsAlignment::Centered;

    // Test enum methods
    assert_eq!(_color.as_str(), "primary");
    assert_eq!(_size.as_str(), "large");
    assert_eq!(_size.as_class(), "is-large");
    assert_eq!(_input_type.as_str(), "email");
    assert_eq!(_alignment.as_class(), "is-centered");
}

#[test]
fn test_component_construction() {
    // Test that component functions are accessible and can be called
    let _app = move || rsx! {
        BulmaProvider {
            theme: BulmaTheme::Light,
            load_bulma_css: true,
            Button {
                color: BulmaColor::Primary,
                size: BulmaSize::Large,
                onclick: |_| {},
                "Click me"
            }
            Input {
                input_type: InputType::Text,
                placeholder: "Enter text",
                color: BulmaColor::Success,
                size: BulmaSize::Medium
            }
            Buttons {
                size: BulmaSize::Large,
                alignment: ButtonsAlignment::Centered,
                addons: true,
                Button { "Save" }
                Button { "Cancel" }
            }
        }
    };

    // If this compiles, the API is working
    assert!(true);
}