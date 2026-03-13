use dioxus::prelude::*;

/// Bootstrap Collapse component — signal-driven, no JavaScript.
///
/// ```rust
/// let expanded = use_signal(|| false);
/// rsx! {
///     Button { onclick: move |_| expanded.toggle(), "Toggle Content" }
///     Collapse { expanded: expanded,
///         Card { body: rsx! { "Collapsible content here." } }
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct CollapseProps {
    /// Signal controlling expanded/collapsed state.
    pub expanded: Signal<bool>,
    /// Use horizontal collapse instead of vertical.
    #[props(default)]
    pub horizontal: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements.
    pub children: Element,
}

#[component]
pub fn Collapse(props: CollapseProps) -> Element {
    let is_expanded = *props.expanded.read();

    let horizontal = if props.horizontal {
        " collapse-horizontal"
    } else {
        ""
    };

    let show = if is_expanded { " show" } else { "" };

    let full_class = if props.class.is_empty() {
        format!("collapse{horizontal}{show}")
    } else {
        format!("collapse{horizontal}{show} {}", props.class)
    };

    rsx! {
        div { class: "{full_class}",
            {props.children}
        }
    }
}
