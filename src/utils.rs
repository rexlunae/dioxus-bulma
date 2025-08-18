/// Utility function to build CSS class strings from optional classes
pub fn build_class(base_classes: &[&str], optional_classes: &[Option<String>]) -> String {
    let mut classes = base_classes.iter()
        .filter(|&&class| !class.is_empty())
        .map(|&class| class.to_string())
        .collect::<Vec<String>>();
    
    for optional in optional_classes {
        if let Some(class) = optional {
            if !class.is_empty() {
                classes.push(class.clone());
            }
        }
    }
    
    classes.join(" ")
}

/// Utility function to conditionally add classes
pub fn conditional_class(condition: bool, class: &str) -> Option<String> {
    if condition {
        Some(class.to_string())
    } else {
        None
    }
}