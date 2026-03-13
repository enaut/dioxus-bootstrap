use dioxus::prelude::*;

/// Bootstrap Offcanvas component — signal-driven, no JavaScript.
///
/// ```rust
/// let show = use_signal(|| false);
/// rsx! {
///     Button { onclick: move |_| show.set(true), "Open Sidebar" }
///     Offcanvas { show: show, title: "Menu", placement: OffcanvasPlacement::Start,
///         p { "Sidebar content" }
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct OffcanvasProps {
    /// Signal controlling visibility.
    pub show: Signal<bool>,
    /// Title shown in the offcanvas header.
    #[props(default)]
    pub title: String,
    /// Placement (which side it slides in from).
    #[props(default)]
    pub placement: OffcanvasPlacement,
    /// Close when clicking the backdrop.
    #[props(default = true)]
    pub backdrop_close: bool,
    /// Show backdrop overlay.
    #[props(default = true)]
    pub backdrop: bool,
    /// Show close button.
    #[props(default = true)]
    pub show_close: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements (offcanvas body content).
    pub children: Element,
}

/// Offcanvas slide-in direction.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum OffcanvasPlacement {
    #[default]
    Start,
    End,
    Top,
    Bottom,
}

impl std::fmt::Display for OffcanvasPlacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OffcanvasPlacement::Start => write!(f, "offcanvas-start"),
            OffcanvasPlacement::End => write!(f, "offcanvas-end"),
            OffcanvasPlacement::Top => write!(f, "offcanvas-top"),
            OffcanvasPlacement::Bottom => write!(f, "offcanvas-bottom"),
        }
    }
}

#[component]
pub fn Offcanvas(props: OffcanvasProps) -> Element {
    let is_shown = *props.show.read();
    let mut show_signal = props.show;

    if !is_shown {
        return rsx! {};
    }

    let placement = props.placement;
    let show = " show";

    let full_class = if props.class.is_empty() {
        format!("offcanvas {placement}{show}")
    } else {
        format!("offcanvas {placement}{show} {}", props.class)
    };

    let backdrop_close = props.backdrop_close;

    rsx! {
        // Backdrop
        if props.backdrop {
            div {
                class: "offcanvas-backdrop fade show",
                onclick: move |_| {
                    if backdrop_close {
                        show_signal.set(false);
                    }
                },
            }
        }
        // Offcanvas panel
        div {
            class: "{full_class}",
            style: "visibility: visible;",
            tabindex: "-1",
            "aria-modal": "true",
            role: "dialog",
            if !props.title.is_empty() || props.show_close {
                div { class: "offcanvas-header",
                    if !props.title.is_empty() {
                        h5 { class: "offcanvas-title", "{props.title}" }
                    }
                    if props.show_close {
                        button {
                            class: "btn-close",
                            r#type: "button",
                            "aria-label": "Close",
                            onclick: move |_| show_signal.set(false),
                        }
                    }
                }
            }
            div { class: "offcanvas-body",
                {props.children}
            }
        }
    }
}
