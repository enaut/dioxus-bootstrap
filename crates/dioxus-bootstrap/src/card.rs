use dioxus::prelude::*;

/// Bootstrap Card component with optional header, body, and footer slots.
///
/// ```rust
/// rsx! {
///     Card {
///         header: rsx! { "Card Title" },
///         body: rsx! { p { "Card content goes here." } },
///         footer: rsx! { "Last updated 3 mins ago" },
///     }
///     // Body-only card
///     Card {
///         body: rsx! { "Simple card" },
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct CardProps {
    /// Card header content.
    #[props(default)]
    pub header: Option<Element>,
    /// Card body content.
    #[props(default)]
    pub body: Option<Element>,
    /// Card footer content.
    #[props(default)]
    pub footer: Option<Element>,
    /// Additional CSS classes for the card container.
    #[props(default)]
    pub class: String,
    /// Additional CSS classes for the card body.
    #[props(default)]
    pub body_class: String,
    /// Child elements (rendered inside card, outside body — for custom layouts).
    #[props(default)]
    pub children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    let full_class = if props.class.is_empty() {
        "card".to_string()
    } else {
        format!("card {}", props.class)
    };

    let body_class = if props.body_class.is_empty() {
        "card-body".to_string()
    } else {
        format!("card-body {}", props.body_class)
    };

    rsx! {
        div { class: "{full_class}",
            if let Some(header) = props.header {
                div { class: "card-header", {header} }
            }
            if let Some(body) = props.body {
                div { class: "{body_class}", {body} }
            }
            {props.children}
            if let Some(footer) = props.footer {
                div { class: "card-footer", {footer} }
            }
        }
    }
}
