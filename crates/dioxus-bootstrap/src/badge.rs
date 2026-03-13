use dioxus::prelude::*;

use crate::types::Color;

/// Bootstrap Badge component.
///
/// ```rust
/// rsx! {
///     Badge { color: Color::Primary, "New" }
///     Badge { color: Color::Danger, pill: true, "99+" }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct BadgeProps {
    /// Badge color variant.
    #[props(default = Color::Primary)]
    pub color: Color,
    /// Use pill (rounded) style.
    #[props(default)]
    pub pill: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements.
    pub children: Element,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let pill = if props.pill { " rounded-pill" } else { "" };
    let full_class = if props.class.is_empty() {
        format!("badge text-bg-{}{pill}", props.color)
    } else {
        format!("badge text-bg-{}{pill} {}", props.color, props.class)
    };

    rsx! {
        span { class: "{full_class}", {props.children} }
    }
}
