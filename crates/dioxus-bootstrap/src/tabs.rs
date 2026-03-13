use dioxus::prelude::*;

use crate::types::Color;

/// Definition for a single tab.
#[derive(Clone, PartialEq)]
pub struct TabDef {
    /// Tab button label.
    pub label: String,
    /// Optional Bootstrap icon name (without "bi-" prefix).
    pub icon: Option<String>,
    /// Tab content.
    pub content: Element,
}

/// Bootstrap Tabs component — signal-driven, no JavaScript.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML (requires JavaScript) -->
/// <ul class="nav nav-tabs">
///   <li class="nav-item"><button class="nav-link active" data-bs-toggle="tab">Home</button></li>
///   <li class="nav-item"><button class="nav-link" data-bs-toggle="tab">Profile</button></li>
/// </ul>
/// <div class="tab-content">
///   <div class="tab-pane active">Home content</div>
///   <div class="tab-pane">Profile content</div>
/// </div>
/// ```
///
/// ```rust,no_run
/// // Dioxus equivalent — use Tab children or TabList with TabDef
/// let active = use_signal(|| 0usize);
/// rsx! {
///     Tabs { active: active,
///         Tab { label: "Home", index: 0, active: active,
///             p { "Home content" }
///         }
///         Tab { label: "Profile", index: 1, active: active, icon: "person",
///             p { "Profile content" }
///         }
///     }
/// }
/// ```
///
/// # Props
///
/// - `active` — `Signal<usize>` controlling active tab index
/// - `pills` — pill style instead of tabs
/// - `fill` — fill available width
/// - `justified` — equal-width items
/// - `vertical` — vertical tab layout
#[derive(Clone, PartialEq, Props)]
pub struct TabsProps {
    /// Signal controlling the active tab index.
    pub active: Signal<usize>,
    /// Use pill style instead of tabs.
    #[props(default)]
    pub pills: bool,
    /// Active tab color (for pills).
    #[props(default)]
    pub color: Option<Color>,
    /// Fill available width.
    #[props(default)]
    pub fill: bool,
    /// Justify items equally.
    #[props(default)]
    pub justified: bool,
    /// Vertical tabs layout.
    #[props(default)]
    pub vertical: bool,
    /// Additional CSS classes for the nav container.
    #[props(default)]
    pub class: String,
    /// Child elements (Tab components).
    pub children: Element,
}

#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let style = if props.pills { "nav-pills" } else { "nav-tabs" };
    let mut nav_classes = vec![format!("nav {style}")];
    if props.fill {
        nav_classes.push("nav-fill".to_string());
    }
    if props.justified {
        nav_classes.push("nav-justified".to_string());
    }
    if props.vertical {
        nav_classes.push("flex-column".to_string());
    }
    if !props.class.is_empty() {
        nav_classes.push(props.class.clone());
    }
    let nav_class = nav_classes.join(" ");

    rsx! {
        div {
            ul { class: "{nav_class}", role: "tablist",
                {props.children}
            }
        }
    }
}

/// A single Tab within a Tabs component.
///
/// Must be a direct child of Tabs.
#[derive(Clone, PartialEq, Props)]
pub struct TabProps {
    /// Tab button label.
    pub label: String,
    /// Optional Bootstrap icon name (without "bi-" prefix).
    #[props(default)]
    pub icon: String,
    /// Tab index (0-based). Set this to match the tab's position.
    pub index: usize,
    /// Signal controlling the active tab (shared with parent Tabs).
    pub active: Signal<usize>,
    /// Additional CSS classes for the tab pane.
    #[props(default)]
    pub class: String,
    /// Tab content.
    pub children: Element,
}

#[component]
pub fn Tab(props: TabProps) -> Element {
    let is_active = *props.active.read() == props.index;
    let mut active_signal = props.active;

    let btn_class = if is_active {
        "nav-link active"
    } else {
        "nav-link"
    };

    let pane_class = if is_active {
        "tab-pane fade show active"
    } else {
        "tab-pane fade"
    };

    let pane_class = if props.class.is_empty() {
        pane_class.to_string()
    } else {
        format!("{pane_class} {}", props.class)
    };

    let index = props.index;

    rsx! {
        // Tab button
        li { class: "nav-item", role: "presentation",
            button {
                class: "{btn_class}",
                r#type: "button",
                role: "tab",
                "aria-selected": if is_active { "true" } else { "false" },
                onclick: move |_| active_signal.set(index),
                if !props.icon.is_empty() {
                    i { class: "bi bi-{props.icon} me-1" }
                }
                "{props.label}"
            }
        }
        // Tab pane (rendered but hidden when not active via CSS classes)
        div { class: "{pane_class}", role: "tabpanel",
            {props.children}
        }
    }
}

/// A simpler Tabs API using TabDef structs instead of child components.
///
/// ```rust
/// let active = use_signal(|| 0usize);
/// rsx! {
///     TabList {
///         active: active,
///         tabs: vec![
///             TabDef { label: "Home".into(), icon: None, content: rsx! { "Home" } },
///             TabDef { label: "About".into(), icon: Some("info-circle".into()), content: rsx! { "About" } },
///         ],
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct TabListProps {
    /// Signal controlling the active tab index.
    pub active: Signal<usize>,
    /// Tab definitions.
    pub tabs: Vec<TabDef>,
    /// Use pill style.
    #[props(default)]
    pub pills: bool,
    /// Additional CSS classes for the nav.
    #[props(default)]
    pub class: String,
    /// Additional CSS classes for the tab content area.
    #[props(default)]
    pub content_class: String,
}

#[component]
pub fn TabList(props: TabListProps) -> Element {
    let current = *props.active.read();
    let mut active_signal = props.active;
    let style = if props.pills { "nav-pills" } else { "nav-tabs" };

    let nav_class = if props.class.is_empty() {
        format!("nav {style}")
    } else {
        format!("nav {style} {}", props.class)
    };

    let content_class = if props.content_class.is_empty() {
        "tab-content".to_string()
    } else {
        format!("tab-content {}", props.content_class)
    };

    rsx! {
        ul { class: "{nav_class}", role: "tablist",
            for (i, tab) in props.tabs.iter().enumerate() {
                li { class: "nav-item", role: "presentation",
                    button {
                        class: if current == i { "nav-link active" } else { "nav-link" },
                        r#type: "button",
                        role: "tab",
                        "aria-selected": if current == i { "true" } else { "false" },
                        onclick: move |_| active_signal.set(i),
                        if let Some(ref icon) = tab.icon {
                            i { class: "bi bi-{icon} me-1" }
                        }
                        "{tab.label}"
                    }
                }
            }
        }
        div { class: "{content_class}",
            for (i, tab) in props.tabs.iter().enumerate() {
                div {
                    class: if current == i { "tab-pane fade show active" } else { "tab-pane fade" },
                    role: "tabpanel",
                    if current == i {
                        {tab.content.clone()}
                    }
                }
            }
        }
    }
}
