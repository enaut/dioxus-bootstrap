use dioxus::prelude::*;

use crate::types::Color;

/// Bootstrap Toast notification — signal-driven, no JavaScript.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML (requires JavaScript) -->
/// <div class="toast show">
///   <div class="toast-header">
///     <strong class="me-auto">Notification</strong>
///     <small>just now</small>
///     <button class="btn-close" data-bs-dismiss="toast"></button>
///   </div>
///   <div class="toast-body">You have a new message.</div>
/// </div>
/// ```
///
/// ```rust,no_run
/// // Dioxus equivalent
/// let show = use_signal(|| true);
/// rsx! {
///     ToastContainer { position: ToastPosition::TopEnd,
///         Toast { show: show, title: "Notification", subtitle: "just now",
///             "You have a new message."
///         }
///     }
/// }
/// ```
///
/// # Props
///
/// - `show` — `Signal<bool>` controlling visibility
/// - `title` — toast header title
/// - `subtitle` — small text in header (e.g., "just now")
/// - `color` — background color variant
/// - `show_close` — show close button (default: true)
#[derive(Clone, PartialEq, Props)]
pub struct ToastProps {
    /// Signal controlling visibility.
    pub show: Signal<bool>,
    /// Toast title (shown in header).
    #[props(default)]
    pub title: String,
    /// Small text in header (e.g., "just now", "2 mins ago").
    #[props(default)]
    pub subtitle: String,
    /// Show close button.
    #[props(default = true)]
    pub show_close: bool,
    /// Toast color variant (applied as bg class).
    #[props(default)]
    pub color: Option<Color>,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Toast body content.
    pub children: Element,
}

#[component]
pub fn Toast(props: ToastProps) -> Element {
    let is_shown = *props.show.read();
    let mut show_signal = props.show;

    if !is_shown {
        return rsx! {};
    }

    let color_class = match &props.color {
        Some(c) => format!(" text-bg-{c}"),
        None => String::new(),
    };

    let full_class = if props.class.is_empty() {
        format!("toast show{color_class}")
    } else {
        format!("toast show{color_class} {}", props.class)
    };

    rsx! {
        div {
            class: "{full_class}",
            role: "alert",
            "aria-live": "assertive",
            "aria-atomic": "true",
            if !props.title.is_empty() {
                div { class: "toast-header",
                    strong { class: "me-auto", "{props.title}" }
                    if !props.subtitle.is_empty() {
                        small { "{props.subtitle}" }
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
            div { class: "toast-body", {props.children} }
        }
    }
}

/// Container for positioning toasts on screen.
///
/// ```rust
/// rsx! {
///     ToastContainer { position: ToastPosition::TopEnd,
///         Toast { show: signal1, title: "Success", "Saved!" }
///         Toast { show: signal2, title: "Error", color: Color::Danger, "Failed." }
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct ToastContainerProps {
    /// Position on screen.
    #[props(default)]
    pub position: ToastPosition,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements (Toast components).
    pub children: Element,
}

/// Toast position on screen.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ToastPosition {
    TopStart,
    TopCenter,
    #[default]
    TopEnd,
    MiddleCenter,
    BottomStart,
    BottomCenter,
    BottomEnd,
}

impl std::fmt::Display for ToastPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToastPosition::TopStart => write!(f, "top-0 start-0"),
            ToastPosition::TopCenter => write!(f, "top-0 start-50 translate-middle-x"),
            ToastPosition::TopEnd => write!(f, "top-0 end-0"),
            ToastPosition::MiddleCenter => {
                write!(f, "top-50 start-50 translate-middle")
            }
            ToastPosition::BottomStart => write!(f, "bottom-0 start-0"),
            ToastPosition::BottomCenter => {
                write!(f, "bottom-0 start-50 translate-middle-x")
            }
            ToastPosition::BottomEnd => write!(f, "bottom-0 end-0"),
        }
    }
}

#[component]
pub fn ToastContainer(props: ToastContainerProps) -> Element {
    let pos = props.position;
    let full_class = if props.class.is_empty() {
        format!("toast-container position-fixed p-3 {pos}")
    } else {
        format!("toast-container position-fixed p-3 {pos} {}", props.class)
    };

    rsx! {
        div { class: "{full_class}", {props.children} }
    }
}
