use dioxus::prelude::*;

use crate::types::Color;

/// Bootstrap Alert component.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<div class="alert alert-success">` | `Alert { color: Color::Success, "Text" }` |
/// | `<div class="alert alert-danger alert-dismissible">` | `Alert { color: Color::Danger, dismissible: true, "Text" }` |
///
/// ```rust,no_run
/// rsx! {
///     Alert { color: Color::Success, "Operation completed!" }
///     Alert { color: Color::Danger, dismissible: true,
///         strong { "Error! " }
///         "Something went wrong."
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct AlertProps {
    /// Alert color variant.
    #[props(default = Color::Primary)]
    pub color: Color,
    /// Show a dismiss button. When clicked, the alert hides itself.
    #[props(default)]
    pub dismissible: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Child elements.
    pub children: Element,
}

#[component]
pub fn Alert(props: AlertProps) -> Element {
    let mut visible = use_signal(|| true);

    if !*visible.read() {
        return rsx! {};
    }

    let dismiss_class = if props.dismissible {
        " alert-dismissible fade show"
    } else {
        ""
    };

    let full_class = if props.class.is_empty() {
        format!("alert alert-{}{dismiss_class}", props.color)
    } else {
        format!("alert alert-{}{dismiss_class} {}", props.color, props.class)
    };

    rsx! {
        div {
            class: "{full_class}",
            role: "alert",
            ..props.attributes,
            {props.children}
            if props.dismissible {
                button {
                    class: "btn-close",
                    r#type: "button",
                    "aria-label": "Close",
                    onclick: move |_| visible.set(false),
                }
            }
        }
    }
}
