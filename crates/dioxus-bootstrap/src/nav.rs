use dioxus::prelude::*;

use crate::types::{Color, NavbarExpand};

/// Bootstrap Nav component — standalone nav (not inside a Navbar).
///
/// ```rust
/// rsx! {
///     Nav { pills: true, fill: true,
///         NavItem { NavLink { active: true, "Home" } }
///         NavItem { NavLink { "Profile" } }
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct NavProps {
    /// Use pill style.
    #[props(default)]
    pub pills: bool,
    /// Use tab style.
    #[props(default)]
    pub tabs: bool,
    /// Use underline style.
    #[props(default)]
    pub underline: bool,
    /// Fill available width equally.
    #[props(default)]
    pub fill: bool,
    /// Justify items to fill width (equal-width items).
    #[props(default)]
    pub justified: bool,
    /// Vertical layout.
    #[props(default)]
    pub vertical: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements (NavItems).
    pub children: Element,
}

#[component]
pub fn Nav(props: NavProps) -> Element {
    let mut classes = vec!["nav".to_string()];
    if props.pills {
        classes.push("nav-pills".to_string());
    }
    if props.tabs {
        classes.push("nav-tabs".to_string());
    }
    if props.underline {
        classes.push("nav-underline".to_string());
    }
    if props.fill {
        classes.push("nav-fill".to_string());
    }
    if props.justified {
        classes.push("nav-justified".to_string());
    }
    if props.vertical {
        classes.push("flex-column".to_string());
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let full_class = classes.join(" ");

    rsx! {
        ul { class: "{full_class}",
            {props.children}
        }
    }
}

/// Bootstrap Navbar component.
///
/// ```rust
/// let collapsed = use_signal(|| true);
/// rsx! {
///     Navbar { color: Color::Dark, expand: NavbarExpand::Lg,
///         brand: rsx! { a { class: "navbar-brand", href: "#", "My App" } },
///         NavbarCollapse { collapsed: collapsed,
///             NavItem { NavLink { href: "/", active: true, "Home" } }
///             NavItem { NavLink { href: "/about", "About" } }
///         }
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct NavbarProps {
    /// Navbar color scheme.
    #[props(default)]
    pub color: Option<Color>,
    /// Responsive expand breakpoint.
    #[props(default)]
    pub expand: NavbarExpand,
    /// Brand element (logo, app name).
    #[props(default)]
    pub brand: Option<Element>,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements (nav items, collapse, etc.).
    pub children: Element,
}

#[component]
pub fn Navbar(props: NavbarProps) -> Element {
    let mut classes = vec!["navbar".to_string(), props.expand.to_string()];

    if let Some(ref color) = props.color {
        match color {
            Color::Dark => {
                classes.push("bg-dark".to_string());
                classes.push("[data-bs-theme=dark]".to_string());
            }
            Color::Light => {
                classes.push("bg-light".to_string());
            }
            c => {
                classes.push(format!("bg-{c}"));
            }
        }
    }

    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }

    let full_class = classes.join(" ");

    rsx! {
        nav { class: "{full_class}",
            div { class: "container-fluid",
                if let Some(brand) = props.brand {
                    {brand}
                }
                {props.children}
            }
        }
    }
}

/// Navbar toggler button (hamburger menu) for responsive collapse.
///
/// ```rust
/// rsx! {
///     NavbarToggler { collapsed: collapsed_signal }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct NavbarTogglerProps {
    /// Signal to toggle — will invert the value on click.
    pub collapsed: Signal<bool>,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
}

#[component]
pub fn NavbarToggler(props: NavbarTogglerProps) -> Element {
    let is_collapsed = *props.collapsed.read();
    let mut signal = props.collapsed;

    let full_class = if props.class.is_empty() {
        "navbar-toggler".to_string()
    } else {
        format!("navbar-toggler {}", props.class)
    };

    rsx! {
        button {
            class: "{full_class}",
            r#type: "button",
            "aria-expanded": if !is_collapsed { "true" } else { "false" },
            "aria-label": "Toggle navigation",
            onclick: move |_| signal.set(!is_collapsed),
            span { class: "navbar-toggler-icon" }
        }
    }
}

/// Navbar collapsible content area.
///
/// ```rust
/// let collapsed = use_signal(|| true);
/// rsx! {
///     NavbarToggler { collapsed: collapsed }
///     NavbarCollapse { collapsed: collapsed,
///         NavItem { NavLink { href: "/", "Home" } }
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct NavbarCollapseProps {
    /// Signal controlling collapsed state.
    pub collapsed: Signal<bool>,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements.
    pub children: Element,
}

#[component]
pub fn NavbarCollapse(props: NavbarCollapseProps) -> Element {
    let is_collapsed = *props.collapsed.read();
    let show = if !is_collapsed { " show" } else { "" };

    let full_class = if props.class.is_empty() {
        format!("collapse navbar-collapse{show}")
    } else {
        format!("collapse navbar-collapse{show} {}", props.class)
    };

    rsx! {
        div { class: "{full_class}",
            ul { class: "navbar-nav me-auto mb-2 mb-lg-0",
                {props.children}
            }
        }
    }
}

/// Bootstrap NavItem component.
#[derive(Clone, PartialEq, Props)]
pub struct NavItemProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements (NavLink).
    pub children: Element,
}

#[component]
pub fn NavItem(props: NavItemProps) -> Element {
    let full_class = if props.class.is_empty() {
        "nav-item".to_string()
    } else {
        format!("nav-item {}", props.class)
    };

    rsx! {
        li { class: "{full_class}", {props.children} }
    }
}

/// Bootstrap NavLink component.
///
/// ```rust
/// rsx! {
///     NavLink { href: "/dashboard", active: true, "Dashboard" }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct NavLinkProps {
    /// Link href.
    #[props(default = "#".to_string())]
    pub href: String,
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
pub fn NavLink(props: NavLinkProps) -> Element {
    let mut classes = vec!["nav-link".to_string()];
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
        a {
            class: "{full_class}",
            href: "{props.href}",
            "aria-current": if props.active { "page" } else { "" },
            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            },
            {props.children}
        }
    }
}
