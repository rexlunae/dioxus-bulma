use dioxus_bulma::prelude::*;

#[test]
fn test_prelude_imports() {
    // Test that all commonly used items are available from prelude

    // Theme items
    let _theme = BulmaTheme::Dark;
    let _color = BulmaColor::Primary;
    let _size = BulmaSize::Large;

    // Component enums
    let _alignment = ButtonsAlignment::Centered;
    let _input_type = InputType::Email;

    // Test enum methods
    assert_eq!(_color.as_str(), "primary");
    assert_eq!(_size.as_class(), "is-large");
    assert_eq!(_alignment.as_class(), "is-centered");
    assert_eq!(_input_type.as_str(), "email");

    // Test passes if everything compiles and enums work
    assert!(true);
}