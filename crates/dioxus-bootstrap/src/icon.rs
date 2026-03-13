use dioxus::prelude::*;

/// Bootstrap Icon component.
///
/// Renders a Bootstrap Icon by name. See https://icons.getbootstrap.com/ for available icons.
///
/// ```rust
/// rsx! {
///     Icon { name: "search" }
///     Icon { name: "shield-lock", class: "me-2 fs-4" }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct IconProps {
    /// Icon name without the `bi-` prefix (e.g., "search", "shield-lock").
    pub name: String,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
}

#[component]
pub fn Icon(props: IconProps) -> Element {
    let icon_class = if props.class.is_empty() {
        format!("bi bi-{}", props.name)
    } else {
        format!("bi bi-{} {}", props.name, props.class)
    };
    rsx! {
        i { class: "{icon_class}" }
    }
}
