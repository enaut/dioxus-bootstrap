use dioxus::prelude::*;

/// Bootstrap Accordion component — signal-driven, no JavaScript.
///
/// ```rust
/// let open = use_signal(|| Some(0usize)); // First item open by default
/// rsx! {
///     Accordion { open: open,
///         AccordionItem { index: 0, title: "Section 1",
///             p { "Content for section 1" }
///         }
///         AccordionItem { index: 1, title: "Section 2",
///             p { "Content for section 2" }
///         }
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct AccordionProps {
    /// Signal controlling which item is open (None = all closed).
    /// For "always open" mode, use AccordionAlwaysOpen instead.
    pub open: Signal<Option<usize>>,
    /// Remove borders and rounded corners.
    #[props(default)]
    pub flush: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements (AccordionItem components).
    pub children: Element,
}

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    let flush = if props.flush { " accordion-flush" } else { "" };
    let full_class = if props.class.is_empty() {
        format!("accordion{flush}")
    } else {
        format!("accordion{flush} {}", props.class)
    };

    rsx! {
        div { class: "{full_class}", {props.children} }
    }
}

/// A single item within an Accordion.
#[derive(Clone, PartialEq, Props)]
pub struct AccordionItemProps {
    /// Item index (must match position in accordion).
    pub index: usize,
    /// Header/title text.
    pub title: String,
    /// Signal controlling which item is open (shared with parent).
    pub open: Signal<Option<usize>>,
    /// Additional CSS classes for the accordion item.
    #[props(default)]
    pub class: String,
    /// Content (shown when expanded).
    pub children: Element,
}

#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
    let is_open = *props.open.read() == Some(props.index);
    let mut open_signal = props.open;
    let index = props.index;

    let button_class = if is_open {
        "accordion-button"
    } else {
        "accordion-button collapsed"
    };

    let body_class = if is_open {
        "accordion-collapse collapse show"
    } else {
        "accordion-collapse collapse"
    };

    let full_class = if props.class.is_empty() {
        "accordion-item".to_string()
    } else {
        format!("accordion-item {}", props.class)
    };

    rsx! {
        div { class: "{full_class}",
            h2 { class: "accordion-header",
                button {
                    class: "{button_class}",
                    r#type: "button",
                    "aria-expanded": if is_open { "true" } else { "false" },
                    onclick: move |_| {
                        if is_open {
                            open_signal.set(None);
                        } else {
                            open_signal.set(Some(index));
                        }
                    },
                    "{props.title}"
                }
            }
            div { class: "{body_class}",
                div { class: "accordion-body",
                    {props.children}
                }
            }
        }
    }
}
