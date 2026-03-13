use dioxus::prelude::*;

use crate::types::Color;

/// Bootstrap ListGroup component.
///
/// ```rust
/// rsx! {
///     ListGroup {
///         ListGroupItem { "Item 1" }
///         ListGroupItem { active: true, "Item 2 (active)" }
///         ListGroupItem { color: Color::Danger, "Item 3 (danger)" }
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct ListGroupProps {
    /// Remove borders and rounded corners for use inside cards.
    #[props(default)]
    pub flush: bool,
    /// Use numbered list style.
    #[props(default)]
    pub numbered: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements (ListGroupItem components).
    pub children: Element,
}

#[component]
pub fn ListGroup(props: ListGroupProps) -> Element {
    let mut classes = vec!["list-group".to_string()];
    if props.flush {
        classes.push("list-group-flush".to_string());
    }
    if props.numbered {
        classes.push("list-group-numbered".to_string());
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let full_class = classes.join(" ");

    if props.numbered {
        rsx! {
            ol { class: "{full_class}", {props.children} }
        }
    } else {
        rsx! {
            ul { class: "{full_class}", {props.children} }
        }
    }
}

/// Bootstrap ListGroupItem component.
#[derive(Clone, PartialEq, Props)]
pub struct ListGroupItemProps {
    /// Active state.
    #[props(default)]
    pub active: bool,
    /// Disabled state.
    #[props(default)]
    pub disabled: bool,
    /// Item color variant.
    #[props(default)]
    pub color: Option<Color>,
    /// Click event handler. When set, renders as a button for interactivity.
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements.
    pub children: Element,
}

#[component]
pub fn ListGroupItem(props: ListGroupItemProps) -> Element {
    let mut classes = vec!["list-group-item".to_string()];
    if props.active {
        classes.push("active".to_string());
    }
    if props.disabled {
        classes.push("disabled".to_string());
    }
    if let Some(ref c) = props.color {
        classes.push(format!("list-group-item-{c}"));
    }
    if props.onclick.is_some() {
        classes.push("list-group-item-action".to_string());
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let full_class = classes.join(" ");

    if let Some(handler) = &props.onclick {
        let handler = handler.clone();
        rsx! {
            button {
                class: "{full_class}",
                r#type: "button",
                disabled: props.disabled,
                onclick: move |evt| handler.call(evt),
                {props.children}
            }
        }
    } else {
        rsx! {
            li { class: "{full_class}", {props.children} }
        }
    }
}
