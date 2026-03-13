use dioxus::prelude::*;

use crate::types::Size;

/// Bootstrap InputGroup component.
///
/// ```rust
/// rsx! {
///     InputGroup {
///         InputGroupText { "@" }
///         Input { placeholder: "Username" }
///     }
///     InputGroup {
///         Input { placeholder: "Search..." }
///         InputGroupText {
///             Button { color: Color::Primary, Icon { name: "search" } }
///         }
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct InputGroupProps {
    /// Input group size.
    #[props(default)]
    pub size: Size,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements (inputs, InputGroupText, buttons).
    pub children: Element,
}

#[component]
pub fn InputGroup(props: InputGroupProps) -> Element {
    let size_class = match props.size {
        Size::Md => String::new(),
        s => format!(" input-group-{s}"),
    };

    let full_class = if props.class.is_empty() {
        format!("input-group{size_class}")
    } else {
        format!("input-group{size_class} {}", props.class)
    };

    rsx! {
        div { class: "{full_class}", {props.children} }
    }
}

/// Text addon for InputGroup.
#[derive(Clone, PartialEq, Props)]
pub struct InputGroupTextProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements.
    pub children: Element,
}

#[component]
pub fn InputGroupText(props: InputGroupTextProps) -> Element {
    let full_class = if props.class.is_empty() {
        "input-group-text".to_string()
    } else {
        format!("input-group-text {}", props.class)
    };

    rsx! {
        span { class: "{full_class}", {props.children} }
    }
}
