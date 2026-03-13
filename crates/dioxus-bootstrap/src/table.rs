use dioxus::prelude::*;

use crate::types::{Color, Size};

/// Bootstrap Table component.
///
/// ```rust
/// rsx! {
///     Table { striped: true, hover: true, responsive: true,
///         thead {
///             tr {
///                 th { "Name" }
///                 th { "Status" }
///             }
///         }
///         tbody {
///             tr {
///                 td { "Service A" }
///                 td { "Running" }
///             }
///         }
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct TableProps {
    /// Striped rows.
    #[props(default)]
    pub striped: bool,
    /// Highlight rows on hover.
    #[props(default)]
    pub hover: bool,
    /// Add borders to all cells.
    #[props(default)]
    pub bordered: bool,
    /// Remove all borders.
    #[props(default)]
    pub borderless: bool,
    /// Compact table with smaller padding.
    #[props(default)]
    pub size: Size,
    /// Table color variant.
    #[props(default)]
    pub color: Option<Color>,
    /// Wrap in a responsive container for horizontal scrolling.
    #[props(default)]
    pub responsive: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements (thead, tbody, etc.).
    pub children: Element,
}

#[component]
pub fn Table(props: TableProps) -> Element {
    let mut classes = vec!["table".to_string()];

    if props.striped {
        classes.push("table-striped".to_string());
    }
    if props.hover {
        classes.push("table-hover".to_string());
    }
    if props.bordered {
        classes.push("table-bordered".to_string());
    }
    if props.borderless {
        classes.push("table-borderless".to_string());
    }
    if let Size::Sm = props.size {
        classes.push("table-sm".to_string());
    }
    if let Some(ref c) = props.color {
        classes.push(format!("table-{c}"));
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }

    let full_class = classes.join(" ");

    if props.responsive {
        rsx! {
            div { class: "table-responsive",
                table { class: "{full_class}", {props.children} }
            }
        }
    } else {
        rsx! {
            table { class: "{full_class}", {props.children} }
        }
    }
}
