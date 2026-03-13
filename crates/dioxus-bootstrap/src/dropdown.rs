use dioxus::prelude::*;

/// Bootstrap Dropdown component — signal-driven, no JavaScript.
///
/// ```rust
/// let open = use_signal(|| false);
/// rsx! {
///     Dropdown { open: open,
///         toggle: rsx! { "Actions" },
///         menu: rsx! {
///             DropdownItem { onclick: handler1, "Edit" }
///             DropdownItem { onclick: handler2, "Delete" }
///             DropdownDivider {}
///             DropdownItem { disabled: true, "Archived" }
///         },
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct DropdownProps {
    /// Signal controlling dropdown open state.
    pub open: Signal<bool>,
    /// Toggle button content.
    pub toggle: Element,
    /// Dropdown menu content (DropdownItem components).
    pub menu: Element,
    /// Additional CSS classes for the dropdown container.
    #[props(default)]
    pub class: String,
    /// Additional CSS classes for the toggle button.
    #[props(default)]
    pub toggle_class: String,
    /// Drop direction.
    #[props(default)]
    pub direction: DropDirection,
    /// Align menu to the end (right).
    #[props(default)]
    pub align_end: bool,
    /// Split button mode — toggle is a separate caret-only button.
    #[props(default)]
    pub split: bool,
    /// Color for split button mode (used for the main button).
    #[props(default)]
    pub color: Option<crate::types::Color>,
}

/// Dropdown direction.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum DropDirection {
    #[default]
    Down,
    Up,
    Start,
    End,
}

#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let is_open = *props.open.read();
    let mut open_signal = props.open;

    let dir_class = match props.direction {
        DropDirection::Down => "dropdown",
        DropDirection::Up => "dropup",
        DropDirection::Start => "dropstart",
        DropDirection::End => "dropend",
    };

    let container_class = if props.class.is_empty() {
        dir_class.to_string()
    } else {
        format!("{dir_class} {}", props.class)
    };

    let color_name = match &props.color {
        Some(c) => format!("{c}"),
        None => "secondary".to_string(),
    };

    let toggle_class = if props.split {
        format!("btn btn-{color_name} dropdown-toggle dropdown-toggle-split")
    } else if props.toggle_class.is_empty() {
        format!("btn btn-{color_name} dropdown-toggle")
    } else {
        format!("btn dropdown-toggle {}", props.toggle_class)
    };

    let menu_class = if is_open {
        if props.align_end {
            "dropdown-menu dropdown-menu-end show"
        } else {
            "dropdown-menu show"
        }
    } else if props.align_end {
        "dropdown-menu dropdown-menu-end"
    } else {
        "dropdown-menu"
    };

    rsx! {
        // Invisible overlay to close on outside click (only when open)
        if is_open {
            div {
                style: "position: fixed; inset: 0; z-index: 990;",
                onclick: move |_| open_signal.set(false),
            }
        }
        div { class: "{container_class}",
            style: if is_open { "position: relative; z-index: 991;" } else { "" },
            // Split mode: main button + separate toggle caret
            if props.split {
                button {
                    class: "btn btn-{color_name}",
                    r#type: "button",
                    {props.toggle.clone()}
                }
            }
            button {
                class: "{toggle_class}",
                r#type: "button",
                "aria-expanded": if is_open { "true" } else { "false" },
                onclick: move |evt| {
                    evt.stop_propagation();
                    open_signal.set(!is_open);
                },
                if !props.split {
                    {props.toggle}
                }
                if props.split {
                    span { class: "visually-hidden", "Toggle Dropdown" }
                }
            }
            ul { class: "{menu_class}",
                // Close dropdown when clicking an item
                onclick: move |_| open_signal.set(false),
                {props.menu}
            }
        }
    }
}

/// A single item in a Dropdown menu.
#[derive(Clone, PartialEq, Props)]
pub struct DropdownItemProps {
    /// Active state.
    #[props(default)]
    pub active: bool,
    /// Disabled state.
    #[props(default)]
    pub disabled: bool,
    /// Click event handler.
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements.
    pub children: Element,
}

#[component]
pub fn DropdownItem(props: DropdownItemProps) -> Element {
    let mut classes = vec!["dropdown-item".to_string()];
    if props.active {
        classes.push("active".to_string());
    }
    if props.disabled {
        classes.push("disabled".to_string());
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let full_class = classes.join(" ");

    rsx! {
        li {
            button {
                class: "{full_class}",
                r#type: "button",
                disabled: props.disabled,
                onclick: move |evt| {
                    if let Some(handler) = &props.onclick {
                        handler.call(evt);
                    }
                },
                {props.children}
            }
        }
    }
}

/// Dropdown menu divider.
#[component]
pub fn DropdownDivider() -> Element {
    rsx! {
        li { hr { class: "dropdown-divider" } }
    }
}

/// Dropdown menu header text.
#[derive(Clone, PartialEq, Props)]
pub struct DropdownHeaderProps {
    pub children: Element,
}

#[component]
pub fn DropdownHeader(props: DropdownHeaderProps) -> Element {
    rsx! {
        li { h6 { class: "dropdown-header", {props.children} } }
    }
}
